// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Create the right string reprensentation of a type or an identifier.

use crate::generate_function::check_fn_sig;
use crate::generate_function_thunk::is_thunk_required;
use crate::{
    check_feature_enabled_on_self_and_all_deps, check_slice_layout, count_regions, get_layout,
    is_public_or_supported_export, matches_qualified_name, AllowReferences, CcType,
};
use arc_anyhow::{Context, Result};
use code_gen_utils::{CcInclude, NamespaceQualifier};
use database::code_snippet::{CcPrerequisites, CcSnippet};
use database::BindingsGenerator;
use database::{FineGrainedFeature, FullyQualifiedName, SugaredTy, TypeLocation};
use error_report::{anyhow, bail, ensure};
use proc_macro2::TokenStream;
use quote::quote;
use rustc_abi::{BackendRepr, HasDataLayout, Integer, Primitive, Scalar};
use rustc_hir::def::Res;
use rustc_middle::mir::Mutability;
use rustc_middle::ty::{self, AdtDef, GenericArg, Ty};
use rustc_span::def_id::{CrateNum, DefId, LOCAL_CRATE};
use rustc_span::symbol::{sym, Symbol};
use std::rc::Rc;

pub fn format_top_level_ns_for_crate(db: &dyn BindingsGenerator<'_>, krate: CrateNum) -> Symbol {
    let crate_name = if krate == LOCAL_CRATE {
        "self".to_string()
    } else {
        db.tcx().crate_name(krate).to_string()
    };
    if let Some(namespace) = db.crate_name_to_namespace().get(crate_name.as_str()) {
        Symbol::intern(namespace)
    } else {
        db.tcx().crate_name(krate)
    }
}

pub fn format_cc_ident_symbol(db: &dyn BindingsGenerator, ident: Symbol) -> Result<TokenStream> {
    format_cc_ident(db, ident.as_str())
}

pub fn format_cc_ident(db: &dyn BindingsGenerator, ident: &str) -> Result<TokenStream> {
    // TODO(b/254104998): Check whether the crate where the identifier is defined is
    // enabled for the feature. Right now if the dep enables the feature but the
    // current crate doesn't, we will escape the identifier in the dep but
    // consider it failed in the current crate.
    if code_gen_utils::is_cpp_reserved_keyword(ident)
        && check_feature_enabled_on_self_and_all_deps(
            db,
            FineGrainedFeature::EscapeCppReservedKeyword,
        )
    {
        let ident = format!("{ident}_");
        code_gen_utils::format_cc_ident(&ident)
    } else {
        code_gen_utils::format_cc_ident(ident)
    }
}

pub fn create_canonical_name_from_foreign_path(
    db: &dyn BindingsGenerator<'_>,
    path_segments: &[rustc_hir::PathSegment<'_>],
    res: &Res,
) -> Option<(DefId, FullyQualifiedName)> {
    let tcx = db.tcx();
    let Res::Def(_, def_id) = res else {
        return None;
    };
    if def_id.is_local() {
        return None;
    }
    let mut segments = path_segments.to_vec();
    if segments.is_empty() {
        return None;
    }

    // The starting `::` will become `{{root}}` and should be removed.
    if segments[0].ident.name.as_str() == "{{root}}" {
        segments.remove(0);
    }
    let segment_len = segments.len();
    if segment_len < 2 {
        return None;
    }

    let krate = tcx.crate_name(def_id.krate);
    // If the crate name is different from the first segment, the path is using an
    // local alias.
    if krate.as_str() != segments[0].ident.name.as_str() {
        return None;
    }
    let item_name = tcx.opt_item_name(*def_id)?;
    let rs_name = Some(item_name);
    let cpp_name = rs_name;
    let rs_mod_path = NamespaceQualifier::new(
        segments[1..segment_len - 1].iter().map(|s| Rc::<str>::from(s.ident.name.as_str())),
    );
    let cpp_ns_path = rs_mod_path.clone();
    let attributes = crubit_attr::get_attrs(tcx, *def_id).unwrap();
    let cpp_type = attributes.cpp_type;
    Some((
        *def_id,
        FullyQualifiedName {
            krate,
            rs_name,
            rs_mod_path,
            cpp_top_level_ns: format_top_level_ns_for_crate(db, def_id.krate),
            cpp_ns_path,
            cpp_name,
            cpp_type,
        },
    ))
}

pub fn format_pointer_or_reference_ty_for_cc<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    pointee: SugaredTy<'tcx>,
    mutability: rustc_middle::mir::Mutability,
    pointer_sigil: TokenStream,
) -> Result<CcSnippet> {
    let tcx = db.tcx();
    let const_qualifier = match mutability {
        Mutability::Mut => quote! {},
        Mutability::Not => quote! { const },
    };
    if pointee.mid().is_c_void(tcx) {
        return Ok(CcSnippet { tokens: quote! { #const_qualifier void* }, ..Default::default() });
    }
    let CcSnippet { tokens, mut prereqs } = db.format_ty_for_cc(pointee, TypeLocation::Other)?;
    prereqs.move_defs_to_fwd_decls();
    Ok(CcSnippet { prereqs, tokens: quote! { #tokens #const_qualifier #pointer_sigil } })
}

pub fn format_slice_pointer_for_cc<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    slice_ty: SugaredTy<'tcx>,
    mutability: rustc_middle::mir::Mutability,
) -> Result<CcSnippet> {
    let const_qualifier = match mutability {
        Mutability::Mut => quote! {},
        Mutability::Not => quote! { const },
    };

    let CcSnippet { tokens, mut prereqs } =
        db.format_ty_for_cc(slice_ty, TypeLocation::Other).with_context(|| {
            format!("Failed to format the inner type of the slice type `{slice_ty}`")
        })?;
    prereqs.includes.insert(db.support_header("rs_std/slice_ref.h"));

    Ok(CcSnippet {
        prereqs,
        tokens: quote! {
            rs_std::SliceRef<
                #const_qualifier #tokens
            >
        },
    })
}

pub fn format_transparent_pointee_or_reference_for_cc<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    referent_ty: Ty<'tcx>,
    referer_hir: Option<&rustc_hir::Ty<'tcx>>,
    mutability: rustc_middle::mir::Mutability,
    pointer_sigil: TokenStream,
) -> Option<CcSnippet> {
    let ty::TyKind::Adt(adt, substs) = referent_ty.kind() else {
        return None;
    };

    if !matches_qualified_name(db, adt.did(), ":: core :: mem :: maybe_uninit :: MaybeUninit")
        || substs.len() != 1
    {
        return None;
    }

    let referent_mid = substs[0].expect_ty();
    let referent = SugaredTy::new(referent_mid, referer_hir);
    format_pointer_or_reference_ty_for_cc(db, referent, mutability, pointer_sigil).ok()
}

/// Formats `ty` into a `CcSnippet` that represents how the type should be
/// spelled in a C++ declaration of a function parameter or field.
pub fn format_ty_for_cc<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    ty: SugaredTy<'tcx>,
    location: TypeLocation,
) -> Result<CcSnippet> {
    let tcx = db.tcx();
    fn cstdint(tokens: TokenStream) -> CcSnippet {
        CcSnippet::with_include(tokens, CcInclude::cstdint())
    }
    fn keyword(tokens: TokenStream) -> CcSnippet {
        CcSnippet::new(tokens)
    }

    if let Some(alias) = format_core_alias_for_cc(db, ty) {
        return Ok(alias);
    }

    Ok(match ty.mid().kind() {
        ty::TyKind::Never => match location {
            TypeLocation::FnReturn => keyword(quote! { void }),
            _ => {
                // TODO(b/254507801): Maybe translate into `crubit::Never`?
                bail!("The never type `!` is only supported as a return type (b/254507801)");
            }
        },
        ty::TyKind::Tuple(_) => {
            let types = ty.as_tuple(db).unwrap();
            if types.is_empty() && matches!(location, TypeLocation::FnReturn) {
                keyword(quote! { void })
            } else {
                let mut prereqs = CcPrerequisites::default();
                prereqs.includes.insert(CcInclude::tuple());

                let mut cc_types = Vec::with_capacity(types.len());
                for element_type in types {
                    cc_types.push(
                        db.format_ty_for_cc(element_type, TypeLocation::Other)?
                            .into_tokens(&mut prereqs),
                    );
                }
                CcSnippet { prereqs, tokens: quote! { std::tuple<#(#cc_types),*> } }
            }
        }

        // https://rust-lang.github.io/unsafe-code-guidelines/layout/scalars.html#bool documents
        // that "Rust's bool has the same layout as C17's _Bool".  The details (e.g. size, valid
        // bit patterns) are implementation-defined, but this is okay, because `bool` in the
        // `extern "C"` functions in the generated `..._cc_api.h` will also be the C17's _Bool.
        ty::TyKind::Bool => keyword(quote! { bool }),

        // https://rust-lang.github.io/unsafe-code-guidelines/layout/scalars.html#fixed-width-floating-point-types
        // documents that "When the platforms' "math.h" header defines the __STDC_IEC_559__ macro,
        // Rust's floating-point types are safe to use directly in C FFI where the appropriate C
        // types are expected (f32 for float, f64 for double)."
        //
        // TODO(b/255768062): Generated bindings should explicitly check `__STDC_IEC_559__`
        ty::TyKind::Float(ty::FloatTy::F32) => keyword(quote! { float }),
        ty::TyKind::Float(ty::FloatTy::F64) => keyword(quote! { double }),

        // ABI compatibility and other details are described in the doc comments in
        // `crubit/support/rs_std/rs_char.h` and `crubit/support/rs_std/char_test.cc` (search for
        // "Layout tests").
        ty::TyKind::Char => {
            // Asserting that the target architecture meets the assumption from Crubit's
            // `rust_builtin_type_abi_assumptions.md` - we assume that Rust's `char` has the
            // same ABI as `u32`.
            let layout = tcx
                .layout_of(
                    ty::TypingEnv {
                        typing_mode: ty::TypingMode::PostAnalysis,
                        param_env: ty::ParamEnv::empty(),
                    }
                    .as_query_input(ty.mid()),
                )
                .expect("`layout_of` is expected to succeed for the builtin `char` type")
                .layout;
            assert_eq!(4, layout.align().abi.bytes());
            assert_eq!(4, layout.size().bytes());
            assert!(matches!(
                layout.backend_repr(),
                BackendRepr::Scalar(Scalar::Initialized {
                    value: Primitive::Int(Integer::I32, /* signedness = */ false),
                    ..
                })
            ));

            let mut cc_type = CcSnippet::with_include(
                quote! { rs_std::rs_char },
                db.support_header("rs_std/rs_char.h"),
            );
            cc_type.prereqs.required_features |= FineGrainedFeature::RustChar;
            cc_type
        }

        // https://rust-lang.github.io/unsafe-code-guidelines/layout/scalars.html#isize-and-usize
        // documents that "Rust's signed and unsigned fixed-width integer types {i,u}{8,16,32,64}
        // have the same layout the C fixed-width integer types from the <stdint.h> header
        // {u,}int{8,16,32,64}_t. These fixed-width integer types are therefore safe to use
        // directly in C FFI where the corresponding C fixed-width integer types are expected.
        //
        // https://rust-lang.github.io/unsafe-code-guidelines/layout/scalars.html#layout-compatibility-with-c-native-integer-types
        // documents that "Rust does not support C platforms on which the C native integer type are
        // not compatible with any of Rust's fixed-width integer type (e.g. because of
        // padding-bits, lack of 2's complement, etc.)."
        ty::TyKind::Int(ty::IntTy::I8) => cstdint(quote! { std::int8_t }),
        ty::TyKind::Int(ty::IntTy::I16) => cstdint(quote! { std::int16_t }),
        ty::TyKind::Int(ty::IntTy::I32) => cstdint(quote! { std::int32_t }),
        ty::TyKind::Int(ty::IntTy::I64) => cstdint(quote! { std::int64_t }),
        ty::TyKind::Uint(ty::UintTy::U8) => cstdint(quote! { std::uint8_t }),
        ty::TyKind::Uint(ty::UintTy::U16) => cstdint(quote! { std::uint16_t }),
        ty::TyKind::Uint(ty::UintTy::U32) => cstdint(quote! { std::uint32_t }),
        ty::TyKind::Uint(ty::UintTy::U64) => cstdint(quote! { std::uint64_t }),

        // https://rust-lang.github.io/unsafe-code-guidelines/layout/scalars.html#isize-and-usize
        // documents that "The isize and usize types are [...] layout compatible with C's uintptr_t
        // and intptr_t types.".
        ty::TyKind::Int(ty::IntTy::Isize) => cstdint(quote! { std::intptr_t }),
        ty::TyKind::Uint(ty::UintTy::Usize) => cstdint(quote! { std::uintptr_t }),

        ty::TyKind::Int(ty::IntTy::I128) | ty::TyKind::Uint(ty::UintTy::U128) => {
            // Note that "the alignment of Rust's {i,u}128 is unspecified and allowed to
            // change" according to
            // https://rust-lang.github.io/unsafe-code-guidelines/layout/scalars.html#fixed-width-integer-types
            //
            // TODO(b/254094650): Consider mapping this to Clang's (and GCC's) `__int128`
            // or to `absl::in128`.
            bail!("C++ doesn't have a standard equivalent of `{ty}` (b/254094650)");
        }

        ty::TyKind::Adt(adt, substs) => {
            let def_id = adt.did();
            let mut prereqs = CcPrerequisites::default();

            if let Some(BridgedType { cpp_type_include, .. }) = is_bridged_type(db, ty.mid())? {
                prereqs.includes.insert(CcInclude::from_path(cpp_type_include.as_str()));
            } else {
                ensure!(substs.len() == 0, "Generic types are not supported yet (b/259749095)");
                ensure!(
                    is_public_or_supported_export(db, adt.did()),
                    "Not a public or a supported reexported type (b/262052635)."
                );

                if def_id.krate == LOCAL_CRATE {
                    prereqs.defs.insert(def_id.expect_local());
                } else {
                    let other_crate_name = tcx.crate_name(def_id.krate);
                    let crate_name_to_include_paths = db.crate_name_to_include_paths();
                    let includes = crate_name_to_include_paths
                        .get(other_crate_name.as_str())
                        .ok_or_else(|| {
                            anyhow!(
                                "Type `{ty}` comes from the `{other_crate_name}` crate, \
                                 but no `--crate-header` was specified for this crate"
                            )
                        })?;
                    prereqs.includes.extend(includes.iter().cloned());
                }

                // Verify if definition of `ty` can be succesfully imported and bail otherwise.
                db.generate_adt_core(def_id).with_context(|| {
                    format!("Failed to generate bindings for the definition of `{ty}`")
                })?;
            }

            CcSnippet { tokens: FullyQualifiedName::new(db, def_id).format_for_cc(db)?, prereqs }
        }

        ty::TyKind::RawPtr(pointee_mid, mutbl) => {
            if let ty::TyKind::Slice(slice_ty) = pointee_mid.kind() {
                check_slice_layout(db.tcx(), ty.mid());
                let mut slice_hir_ty = None;
                if let Some(hir) = ty.hir(db) {
                    if let rustc_hir::TyKind::Ptr(pointee) = &hir.kind {
                        if let rustc_hir::TyKind::Slice(slice_ty) = &pointee.ty.kind {
                            slice_hir_ty = Some(*slice_ty);
                        }
                    }
                }
                return format_slice_pointer_for_cc(
                    db,
                    SugaredTy::new(*slice_ty, slice_hir_ty),
                    *mutbl,
                );
            }
            let mut pointee_hir = None;
            if let Some(hir) = ty.hir(db) {
                if let rustc_hir::TyKind::Ptr(mut_p) = hir.kind {
                    pointee_hir = Some(mut_p.ty);
                }
            }

            // Early return in case we handle a transparent pointer type.
            if let Some(snippet) = format_transparent_pointee_or_reference_for_cc(
                db,
                *pointee_mid,
                pointee_hir,
                *mutbl,
                quote! { * },
            ) {
                return Ok(snippet);
            }

            let pointee = SugaredTy::new(*pointee_mid, pointee_hir);
            format_pointer_or_reference_ty_for_cc(db, pointee, *mutbl, quote! { * }).with_context(
                || format!("Failed to format the pointee of the pointer type `{ty}`"),
            )?
        }

        ty::TyKind::Ref(region, referent_mid, mutability) => {
            if let ty::TyKind::Slice(_) = referent_mid.kind() {
                check_slice_layout(db.tcx(), ty.mid());
            }

            let mut referent_hir = None;
            if let Some(hir) = ty.hir(db) {
                if let rustc_hir::TyKind::Ref(_, mut_p, ..) = &hir.kind {
                    referent_hir = Some(mut_p.ty);
                }
            }

            match location {
                TypeLocation::FnReturn | TypeLocation::FnParam => (),
                TypeLocation::Other => bail!(
                    "Can't format `{ty}`, because references are only supported in \
                     function parameter types and return types (b/286256327)",
                ),
            };
            let lifetime = format_region_as_cc_lifetime(region);

            // Early return in case we handle a transparent reference type.
            if let Some(snippet) = format_transparent_pointee_or_reference_for_cc(
                db,
                *referent_mid,
                referent_hir,
                *mutability,
                quote! { & #lifetime },
            ) {
                return Ok(snippet);
            }

            let referent = SugaredTy::new(*referent_mid, referent_hir);
            let mut cc_type = format_pointer_or_reference_ty_for_cc(
                db,
                referent,
                *mutability,
                quote! { & #lifetime },
            )
            .with_context(|| {
                format!("Failed to format the referent of the reference type `{ty}`")
            })?;
            // For function parameters which are `'_`, we allow the caller to decide whether
            // to require the reference feature. Some use cases are safe (e.g.
            // if it's the only reference/pointer parameter.)
            //
            // In all other cases, we assume it is unsafe and require references to be
            // enabled.
            if location != TypeLocation::FnParam {
                cc_type.prereqs.required_features |= FineGrainedFeature::References;
            } else if !region.is_param() {
                cc_type.prereqs.required_features |= FineGrainedFeature::NonFreeReferenceParams;
            }
            cc_type
        }
        ty::TyKind::FnPtr(sig_tys, fn_header) => {
            let sig = {
                let sig_tys = match sig_tys.no_bound_vars() {
                    None => bail!("Generic functions are not supported yet (b/259749023)"),
                    Some(sig_tys) => sig_tys,
                };
                rustc_middle::ty::FnSig {
                    inputs_and_output: sig_tys.inputs_and_output,
                    c_variadic: fn_header.c_variadic,
                    safety: fn_header.safety,
                    abi: fn_header.abi,
                }
            };

            check_fn_sig(&sig)?;
            is_thunk_required(&sig).context("Function pointers can't have a thunk")?;

            // `is_thunk_required` check above implies `extern "C"` (or `"C-unwind"`).
            // This assertion reinforces that the generated C++ code doesn't need
            // to use calling convention attributes like `_stdcall`, etc.
            assert!(matches!(sig.abi, rustc_abi::ExternAbi::C { .. }));

            // C++ references are not rebindable and therefore can't be used to replicate
            // semantics of Rust field types (or, say, element types of Rust
            // arrays).  Because of this, C++ references are only used for
            // top-level return types and parameter types (and pointers are used
            // in other locations).
            let ptr_or_ref_sigil = match location {
                TypeLocation::FnReturn | TypeLocation::FnParam => quote! { & },
                TypeLocation::Other => quote! { * },
            };

            let mut prereqs = CcPrerequisites::default();
            prereqs.includes.insert(db.support_header("internal/cxx20_backports.h"));

            let mut sig_hir = None;
            if let Some(hir) = ty.hir(db) {
                if let rustc_hir::TyKind::BareFn(bare_fn) = &hir.kind {
                    sig_hir = Some(bare_fn.decl);
                }
            }
            let ret_type = format_ret_ty_for_cc(db, &sig, sig_hir)?.into_tokens(&mut prereqs);
            let param_types = format_param_types_for_cc(db, &sig, sig_hir, AllowReferences::Safe)?
                .into_iter()
                .map(|snippet| snippet.into_tokens(&mut prereqs));
            let tokens = quote! {
                crubit::type_identity_t<
                    #ret_type( #( #param_types ),* )
                > #ptr_or_ref_sigil
            };

            CcSnippet { tokens, prereqs }
        }

        // TODO(b/260268230, b/260729464): When recursively processing nested types (e.g. an
        // element type of an Array, a referent of a Ref, a parameter type of an FnPtr, etc), one
        // should also 1) propagate `CcPrerequisites::defs`, 2) cover `CcPrerequisites::defs` in
        // `test_format_ty_for_cc...`.  For ptr/ref it might be possible to use
        // `CcPrerequisites::move_defs_to_fwd_decls`.
        _ => bail!("The following Rust type is not supported yet: {ty}"),
    })
}

/// Returns `Some(CcSnippet)` if `ty` is a special-cased alias type from
/// `core::ffi` (AKA `std::ffi`).
///
/// TODO(b/283258442): Also handle `libc` aliases.
fn format_core_alias_for_cc<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    ty: SugaredTy<'tcx>,
) -> Option<CcSnippet> {
    use rustc_hir::definitions::{DefPathData::TypeNs, DisambiguatedDefPathData};
    fn matches_type_path(actual: &[DisambiguatedDefPathData], expected: &[&str]) -> bool {
        if actual.len() != expected.len() {
            return false;
        }
        for i in 0..actual.len() {
            let TypeNs(actual_elem) = actual[i].data else {
                return false;
            };
            if actual_elem.as_str() != expected[i] {
                return false;
            }
        }
        true
    }

    let tcx = db.tcx();
    let hir_ty = ty.hir(db)?;
    let rustc_hir::TyKind::Path(rustc_hir::QPath::Resolved(None, path)) = &hir_ty.kind else {
        return None;
    };
    let rustc_hir::def::Res::Def(rustc_hir::def::DefKind::TyAlias, alias_def_id) = &path.res else {
        return None;
    };
    let def_path = tcx.def_path(*alias_def_id);

    // Note: the `std::ffi` aliases are still originally defined in `core::ffi`, so
    // we only need to check for a crate name of `core` here.
    if tcx.crate_name(def_path.krate) != sym::core {
        return None;
    };
    let [module_path @ .., item] = def_path.data.as_slice() else { return None };
    // Primitives are defined in both `core::ffi` and `core::ffi::primitives
    if !matches_type_path(module_path, &["ffi"])
        && !matches_type_path(module_path, &["ffi", "primitives"])
    {
        return None;
    }
    let TypeNs(item) = item.data else {
        return None;
    };
    let cpp_type = match item.as_str() {
        "c_char" => quote! { char},
        "c_schar" => quote! { signed char},
        "c_uchar" => quote! { unsigned char},
        "c_short" => quote! { short},
        "c_ushort" => quote! { unsigned short},
        "c_int" => quote! { int},
        "c_uint" => quote! { unsigned int},
        "c_long" => quote! { long},
        "c_ulong" => quote! { unsigned long},
        "c_longlong" => quote! { long long},
        "c_ulonglong" => quote! { unsigned long long},
        _ => return None,
    };
    Some(CcSnippet::new(cpp_type))
}

/// Returns the C++ return type.
///
/// `sig_hir` is the optional HIR `FnDecl`, if available. This is used to
/// retrieve alias information.
pub fn format_ret_ty_for_cc<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    sig_mid: &ty::FnSig<'tcx>,
    sig_hir: Option<&rustc_hir::FnDecl<'tcx>>,
) -> Result<CcSnippet> {
    db.format_ty_for_cc(SugaredTy::fn_output(sig_mid, sig_hir), TypeLocation::FnReturn)
        .context("Error formatting function return type")
}

/// Returns the C++ parameter types.
///
/// `sig_hir` is the optional HIR FnSig, if available. This is used to retrieve
/// alias information.
///
/// if `allow_references` is `Safe`, then this only allows exactly one reference
/// parameter.
pub fn format_param_types_for_cc<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    sig_mid: &ty::FnSig<'tcx>,
    sig_hir: Option<&rustc_hir::FnDecl<'tcx>>,
    allow_references: AllowReferences,
) -> Result<Vec<CcSnippet>> {
    let region_counts = std::cell::LazyCell::new(|| count_regions(sig_mid));

    let param_types = SugaredTy::fn_inputs(sig_mid, sig_hir);
    let mut snippets = Vec::with_capacity(param_types.len());
    for i in 0..param_types.len() {
        let mut cc_type = db
            .format_ty_for_cc(param_types.index(i), TypeLocation::FnParam)
            .with_context(|| format!("Error handling parameter #{i}"))?;
        if allow_references == AllowReferences::Safe {
            // In parameter position, format_ty_for_cc defaults to allowing free
            // (non-static) references. We need to decide which references we
            // allow -- in this case, we choose to allow references _only_ if
            // the reference cannot mutably alias, and does not have any lifetime
            // requirements from the caller.
            match param_types.index(i).mid().kind() {
                ty::TyKind::Ref(input_region, .., Mutability::Not) => {
                    if region_counts[input_region] > 1 {
                        cc_type.prereqs.required_features |= FineGrainedFeature::LifetimeReuse;
                    }
                }
                ty::TyKind::Ref(input_region, .., Mutability::Mut) => {
                    if region_counts.len() > 1 || region_counts[input_region] > 1 {
                        cc_type.prereqs.required_features |=
                            FineGrainedFeature::PossibleMutableAliasing;
                    }
                }
                _ => {}
            }
        }
        snippets.push(cc_type);
    }
    Ok(snippets)
}

/// Format a supported `repr(transparent)` pointee type
pub fn format_transparent_pointee<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    ty: &Ty<'tcx>,
) -> Result<TokenStream> {
    if let ty::TyKind::Adt(adt, substs) = ty.kind() {
        if matches_qualified_name(db, adt.did(), ":: core :: mem :: maybe_uninit :: MaybeUninit") {
            let generic_ty = format_ty_for_rs(db, substs[0].expect_ty())?;
            return Ok(quote! { std::mem::MaybeUninit<#generic_ty> });
        }
    }
    bail!("unable to generate bindings for anything other than `MaybeUninit<T>`")
}

/// Formats `ty` for Rust - to be used in `..._cc_api_impl.rs` (e.g. as a type
/// of a parameter in a Rust thunk).  Because `..._cc_api_impl.rs` is a
/// distinct, separate crate, the returned `TokenStream` uses crate-qualified
/// names whenever necessary - for example: `target_crate::SomeStruct` rather
/// than just `SomeStruct`.
//
// TODO(b/259724276): This function's results should be memoized.
pub fn format_ty_for_rs<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    ty: Ty<'tcx>,
) -> Result<TokenStream> {
    Ok(match ty.kind() {
        ty::TyKind::Bool
        | ty::TyKind::Float(_)
        | ty::TyKind::Char
        | ty::TyKind::Int(_)
        | ty::TyKind::Uint(_)
        | ty::TyKind::FnPtr { .. }
        | ty::TyKind::Never => ty
            .to_string()
            .parse()
            .expect("rustc_middle::ty::Ty::to_string() should produce no parsing errors"),
        ty::TyKind::Tuple(types) => {
            let rs_types = types
                .iter()
                .map(|ty| format_ty_for_rs(db, ty))
                .collect::<Result<Vec<TokenStream>>>()?;
            quote! { (#(#rs_types,)*) }
        }
        ty::TyKind::Adt(adt, substs) => {
            let is_bridged_type = is_bridged_type(db, ty)?.is_some();
            ensure!(
                is_bridged_type || substs.len() == 0,
                "Generic types are not supported yet (b/259749095)"
            );
            FullyQualifiedName::new(db, adt.did()).format_for_rs()
        }
        ty::TyKind::RawPtr(pointee_ty, mutbl) => {
            let qualifier = match mutbl {
                Mutability::Mut => quote! { mut },
                Mutability::Not => quote! { const },
            };
            let ty = match format_transparent_pointee(db, pointee_ty) {
                Ok(generic_ty) => generic_ty,
                Err(_) => format_ty_for_rs(db, *pointee_ty).with_context(|| {
                    format!("Failed to format the pointee of the pointer type `{ty}`")
                })?,
            };
            quote! { * #qualifier #ty }
        }
        ty::TyKind::Ref(region, referent_ty, mutability) => {
            let mutability = match mutability {
                Mutability::Mut => quote! { mut },
                Mutability::Not => quote! {},
            };
            let ty = match format_transparent_pointee(db, referent_ty) {
                Ok(generic_ty) => generic_ty,
                Err(_) => format_ty_for_rs(db, *referent_ty).with_context(|| {
                    format!("Failed to format the referent of the reference type `{ty}`")
                })?,
            };
            let lifetime = format_region_as_rs_lifetime(region);
            quote! { & #lifetime #mutability #ty }
        }
        ty::TyKind::Slice(slice_ty) => {
            let ty = format_ty_for_rs(db, *slice_ty).with_context(|| {
                format!("Failed to format the element type of the slice type `{ty}`")
            })?;
            quote! { [#ty] }
        }
        _ => bail!("The following Rust type is not supported yet: {ty}"),
    })
}

pub fn format_region_as_cc_lifetime(region: &ty::Region) -> TokenStream {
    let name =
        region.get_name().expect("Caller should use `liberate_and_deanonymize_late_bound_regions`");
    let name = name
        .as_str()
        .strip_prefix('\'')
        .expect("All Rust lifetimes are expected to begin with the \"'\" character");

    // TODO(b/286299326): Use `$a` or `$(foo)` or `$static` syntax below.
    quote! { [[clang::annotate_type("lifetime", #name)]] }
}

pub fn format_region_as_rs_lifetime(region: &ty::Region) -> TokenStream {
    let name =
        region.get_name().expect("Caller should use `liberate_and_deanonymize_late_bound_regions`");
    let lifetime = syn::Lifetime::new(name.as_str(), proc_macro2::Span::call_site());
    quote! { #lifetime }
}

/// A Rust type that bridges to a particular pre-existing C++ type.
///
/// Bridged types may be representation-equivalent such that pointers to one may be treated as
/// pointers to the other, or they may require conversion functions (in which case they can only
/// be passed by-value).
pub struct BridgedType {
    /// The spelling of the C++ type of the item.
    pub cpp_type: CcType,
    /// Path to the header file that declares the type specified in `cpp_type`.
    pub cpp_type_include: Symbol,
    pub conversion_info: BridgedTypeConversionInfo,
}

/// A description of what method is used to convert between values of the Rust and C++ types.
pub enum BridgedTypeConversionInfo {
    /// The types are representation-equivalent and can be transmuted using simple pointer casts.
    PointerLikeTransmute,
    ExternCFuncConverters {
        cpp_to_rust_converter: Symbol,
        rust_to_cpp_converter: Symbol,
    },
}

/// Returns an error if `ty` is not pointer-like.
pub fn ensure_ty_is_pointer_like<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    ty: Ty<'tcx>,
) -> Result<()> {
    if let ty::TyKind::Adt(adt, _) = ty.kind() {
        if !adt.repr().transparent() {
            bail!("Can't convert {ty} to a C++ pointer as it's not `repr(transparent)`");
        }

        if !get_layout(db.tcx(), ty)?.is_pointer_like(db.tcx().data_layout()) {
            bail!(
                "Can't convert {ty} to a C++ pointer as its layout is not pointer-like. \
                To be considered pointer-like it may only have one non-ZST field that needs \
                to be a C ABI compatible pointer."
            );
        }
        Ok(())
    } else {
        bail!("Can't convert {ty} to a C++ pointer because it's not an ADT");
    }
}

/// Returns true if #[__crubit::annotate(...)] is configured to require type
/// bridging.
///
/// A type is said to require type bridging ("bridged type") if either
/// `cpp_to_rust_converter` or `rust_to_cpp_converter` is set in the
/// #[__crubit::annotate(...)] attribute. The idea is that for a Rust
/// type with an equivalent but not ABI compatible C++ type, conversion
/// functions that turn one type into another can be specified.
pub fn requires_type_bridging<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    ty: Ty<'tcx>,
    attrs: crubit_attr::CrubitAttrs,
) -> Result<Option<BridgedType>> {
    match attrs {
        crubit_attr::CrubitAttrs {
            cpp_type: Some(cpp_type),
            cpp_type_include: Some(cpp_type_include),
            cpp_to_rust_converter: None,
            rust_to_cpp_converter: None,
            ..
        } => {
            let ts = cpp_type.as_str().parse::<TokenStream>().map_err(|err| {
                anyhow!("Failed to parse CrubitAttrs.cpp_type = {cpp_type}: {err}")
            })?;

            match code_gen_utils::is_cpp_pointer_type(ts) {
                Some(cv) => {
                    ensure_ty_is_pointer_like(db, ty)?;

                    Ok(Some(BridgedType {
                        cpp_type: CcType::Pointer { cpp_type, cv },
                        cpp_type_include,
                        conversion_info: BridgedTypeConversionInfo::PointerLikeTransmute,
                    }))
                }
                None => Ok(None),
            }
        }
        crubit_attr::CrubitAttrs {
            cpp_type: Some(cpp_type),
            cpp_type_include: Some(cpp_type_include),
            cpp_to_rust_converter: Some(cpp_to_rust_converter),
            rust_to_cpp_converter: Some(rust_to_cpp_converter),
            ..
        } => {
            let ts = cpp_type.as_str().parse::<TokenStream>().map_err(|err| {
                anyhow!("Failed to parse CrubitAttrs.cpp_type = {cpp_type}: {err}")
            })?;

            match code_gen_utils::is_cpp_pointer_type(ts) {
                Some(cv) => Ok(Some(BridgedType {
                    cpp_type: CcType::Pointer { cpp_type, cv },
                    cpp_type_include,
                    conversion_info: BridgedTypeConversionInfo::ExternCFuncConverters {
                        cpp_to_rust_converter,
                        rust_to_cpp_converter,
                    },
                })),
                None => Ok(Some(BridgedType {
                    cpp_type: CcType::Other(cpp_type),
                    cpp_type_include,
                    conversion_info: BridgedTypeConversionInfo::ExternCFuncConverters {
                        cpp_to_rust_converter,
                        rust_to_cpp_converter,
                    },
                })),
            }
        }
        crubit_attr::CrubitAttrs { cpp_to_rust_converter: Some(cpp_to_rust_converter), .. } => {
            bail!(
                "Invalid state of  #[__crubit::annotate(...)] attribute. rust_to_cpp_converter \
                    set ({cpp_to_rust_converter}), but cpp_type not set."
            )
        }
        crubit_attr::CrubitAttrs { rust_to_cpp_converter: Some(rust_to_cpp_converter), .. } => {
            bail!(
                "Invalid state of  #[__crubit::annotate(...)] attribute. cpp_to_rust_converter \
                    set ({rust_to_cpp_converter}), but cpp_type not set."
            )
        }
        _ => Ok(None),
    }
}

/// Returns the contents of the `__crubit_annotate` attribute if type bridging
/// is configured. An error is returned if the attribute could not be parsed or
/// is in an invalid state.
pub fn is_bridged_adt<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    ty: Ty<'tcx>,
    adt: &AdtDef<'_>,
    substs: &[GenericArg<'tcx>],
) -> Result<Option<BridgedType>> {
    match crubit_attr::get_attrs(db.tcx(), adt.did()) {
        Ok(attrs) => {
            if let Some(attrs) = requires_type_bridging(db, ty, attrs)? {
                Ok(Some(attrs))
            } else {
                // The ADT does not need to be bridged, but check if it has generic types that
                // need to be bridged e.g. Box<BridgedType> cannot be formated at
                // the moment. If we encounter a type like this we return an error.
                let res = substs.iter().flat_map(|a| a.walk()).try_for_each(|a| {
                    if let Some(ty) = a.as_type() {
                        if is_bridged_type(db, ty)?.is_some() {
                            bail!(
                                "Can't format ADT as it has a generic type `{ty}` that is a \
                                    bridged type",
                            );
                        }
                    }
                    Ok(())
                });
                match res {
                    Ok(_) => Ok(None),
                    Err(err) => Err(anyhow!(err)),
                }
            }
        }
        Err(err) => Err(anyhow!(err)),
    }
}

/// Returns the contents of the `__crubit_annotate` attribute if type bridging
/// is configured. An error is returned if the type is a pointer or reference or
/// the attribute could not be parsed or is in an invalid state.
pub fn is_bridged_type<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    ty: Ty<'tcx>,
) -> Result<Option<BridgedType>> {
    match ty.kind() {
        ty::TyKind::Ref(_, referent_mid, _) if is_bridged_type(db, *referent_mid)?.is_some() => {
            bail!(
                "Can't format reference type `{ty}` because the referent is a bridged type. \
                    Passing bridged types by reference is not supported."
            )
        }
        ty::TyKind::RawPtr(pointee_mid, _) if is_bridged_type(db, *pointee_mid)?.is_some() => {
            bail!(
                "Can't format pointer type `{ty}` because the pointee is a bridged type. \
                    Passing bridged types by pointer is not supported."
            )
        }
        ty::TyKind::Adt(adt, substs) => is_bridged_adt(db, ty, adt, substs),
        _ => Ok(None),
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::tests::*;
    use code_gen_utils::format_cc_includes;
    use quote::quote;
    use run_compiler_test_support::find_def_id_by_name;
    use token_stream_matchers::assert_cc_matches;

    /// `test_format_ret_ty_for_cc_successes` provides test coverage for cases
    /// where `format_ty_for_cc` takes `TypeLocation::FnReturn` and returns
    /// an `Ok(...)`.  Additional testcases are covered by
    /// `test_format_ty_for_cc_successes`.
    #[test]
    fn test_format_ret_ty_for_cc_successes() {
        let testcases = [
            // ( <Rust type>, <expected C++ type> )
            ("bool", "bool"), // TyKind::Bool
            ("()", "void"),
            // TODO(b/254507801): Expect `crubit::Never` instead (see the bug for more
            // details).
            ("!", "void"),
            (
                "extern \"C\" fn (f32, f32) -> f32",
                "crubit :: type_identity_t < float (float , float) > &",
            ),
        ];
        test_ty(TypeLocation::FnReturn, &testcases, quote! {}, |desc, tcx, ty, expected| {
            let actual = {
                let db = bindings_db_for_tests(tcx);
                let cc_snippet = format_ty_for_cc(&db, ty, TypeLocation::FnReturn).unwrap();
                cc_snippet.tokens.to_string()
            };
            let expected = expected.parse::<TokenStream>().unwrap().to_string();
            assert_eq!(actual, expected, "{desc}");
        });
    }

    /// `test_format_ty_for_cc_successes` provides test coverage for cases where
    /// `format_ty_for_cc` returns an `Ok(...)`.
    ///
    /// Note that using `std::int8_t` (instead of `::std::int8_t`) has been an
    /// explicit decision. The "Google C++ Style Guide" suggests to "avoid
    /// nested namespaces that match well-known top-level namespaces" and "in
    /// particular, [...] not create any nested std namespaces.".  It
    /// seems desirable if the generated bindings conform to this aspect of the
    /// style guide, because it makes things easier for *users* of these
    /// bindings.
    #[test]
    fn test_format_ty_for_cc_successes() {
        struct FormatCcExpectation {
            expected_tokens: &'static str,
            expected_includes: Vec<&'static str>,
            expected_prereq_def: Option<&'static str>,
            expected_prereq_fwd_decl: Option<&'static str>,
        }

        // Helper macro to create a `FormatCcExpectation`s. Handles all a variation of
        // relevant fields (e.g. expected includes or forward decls).
        macro_rules! case {
            (rs: $input_rust_ty:expr, cc: $expected_cc_ty:expr, includes: [$($includes:expr),*], prereq_def: $expected_prereq_def:expr, prereq_fwd_decl: $expected_prereq_fwd_decl:expr) => {
                (
                    $input_rust_ty,
                    FormatCcExpectation {
                        expected_tokens: $expected_cc_ty,
                        expected_includes: Vec::<&'static str>::from([$($includes),*]),
                        expected_prereq_def: $expected_prereq_def,
                        expected_prereq_fwd_decl: $expected_prereq_fwd_decl,
                    }
            )
            };
            (rs: $input_rust_ty:expr, cc: $expected_cc_ty:expr) => {
                case!(rs: $input_rust_ty, cc: $expected_cc_ty, includes: [], prereq_def: None, prereq_fwd_decl: None)
            };
            (rs: $input_rust_ty:expr, cc: $expected_cc_ty:expr, includes: [$($includes:expr),*]) => {
                case!(rs: $input_rust_ty, cc: $expected_cc_ty, includes: [$($includes),*], prereq_def: None, prereq_fwd_decl: None)
            };
            (rs: $input_rust_ty:expr, cc: $expected_cc_ty:expr, includes: [$($includes:expr),*], prereq_def: $expected_prereq_def:expr) => {
                case!(rs: $input_rust_ty, cc: $expected_cc_ty, includes: [$($includes),*], prereq_def: Some($expected_prereq_def), prereq_fwd_decl: None)
            };
            (rs: $input_rust_ty:expr, cc: $expected_cc_ty:expr, includes: [$($includes:expr),*], prereq_fwd_decl: $expected_prereq_fwd_decl:expr) => {
                case!(rs: $input_rust_ty, cc: $expected_cc_ty, includes: [$($includes),*], prereq_def: None, prereq_fwd_decl: Some($expected_prereq_fwd_decl))
            };
        }

        let testcases = [
            case!(rs: "bool", cc:  "bool"),
            case!(rs: "f32", cc: "float"),
            case!(rs: "f64", cc: "double"),
            // The ffi aliases are special-cased to refer to the C++ fundamental integer types,
            // if the type alias information is not lost (e.g. from generics).
            case!(rs: "std::ffi::c_char", cc:  "char"),
            case!(rs: "::std::ffi::c_char", cc:  "char"),
            case!(rs: "core::ffi::c_char", cc:  "char"),
            case!(rs: "::core::ffi::c_char", cc:  "char"),
            case!(rs: "std::ffi::c_uchar", cc: "unsigned char"),
            case!(rs: "std::ffi::c_longlong", cc: "long long"),
            case!(rs: "c_char", cc:  "char"),
            // Simple pointers/references do not lose the type alias information.
            case!(rs: "*const std::ffi::c_uchar", cc: "unsigned char const *"),
            case!(
                rs: "&'static std::ffi::c_uchar",
                cc: "unsigned char const & [[clang :: annotate_type (\"lifetime\" , \"static\")]]"
            ),
            // Generics lose type alias information.
            case!(rs: "Identity<std::ffi::c_longlong>", cc: "std::int64_t", includes: ["<cstdint>"]),
            case!(rs: "i8", cc: "std::int8_t", includes: ["<cstdint>"]),
            case!(rs: "i16", cc:  "std::int16_t", includes: ["<cstdint>"]),
            case!(rs: "i32", cc:  "std::int32_t", includes: ["<cstdint>"]),
            case!(rs: "i64", cc:  "std::int64_t", includes: ["<cstdint>"]),
            case!(rs: "isize", cc: "std::intptr_t", includes: ["<cstdint>"]),
            case!(rs: "u8", cc: "std::uint8_t", includes: ["<cstdint>"]),
            case!(rs: "u16", cc: "std::uint16_t", includes: ["<cstdint>"]),
            case!(rs: "u32", cc: "std::uint32_t", includes: ["<cstdint>"]),
            case!(rs: "u64", cc: "std::uint64_t", includes: ["<cstdint>"]),
            case!(rs: "usize", cc: "std::uintptr_t", includes: ["<cstdint>"]),
            case!(
                rs: "char",
                cc: "rs_std::rs_char",
                includes: ["<crubit/support/for/tests/rs_std/rs_char.h>"]
            ),
            case!(rs: "SomeStruct", cc: "::rust_out::SomeStruct", includes: [],  prereq_def: "SomeStruct"),
            case!(rs: "SomeEnum", cc: "::rust_out::SomeEnum", includes: [], prereq_def: "SomeEnum"),
            case!(rs: "SomeUnion", cc: "::rust_out::SomeUnion", includes: [], prereq_def: "SomeUnion"),
            case!(rs: "*const i32", cc: "std :: int32_t const *", includes: ["<cstdint>"]),
            case!(rs: "*mut i32", cc: "std :: int32_t *", includes: ["<cstdint>"]),
            case!(
                rs: "&'static i32",
                cc: "std :: int32_t const & [[clang :: annotate_type (\"lifetime\" , \"static\")]]",
                includes: ["<cstdint>"]
            ),
            case!(
                rs: "&'static mut i32",
                cc: "std :: int32_t & [[clang :: annotate_type (\"lifetime\" , \"static\")]]",
                includes: ["<cstdint>"]
            ),
            // Slice pointers:
            case!(
                rs: "*const [i8]",
                cc: "rs_std::SliceRef<const std::int8_t>",
                includes: ["<cstdint>", "<crubit/support/for/tests/rs_std/slice_ref.h>"]
            ),
            case!(
                rs: "*mut [i64]",
                cc: "rs_std::SliceRef<std::int64_t>",
                includes: ["<cstdint>", "<crubit/support/for/tests/rs_std/slice_ref.h>"]
            ),
            case!(
                rs: "*const [c_char]",
                cc: "rs_std::SliceRef<const char>",
                includes: ["<crubit/support/for/tests/rs_std/slice_ref.h>"]
            ),
            case!(
                rs: "*mut [SomeStruct]",
                cc: "rs_std::SliceRef< ::rust_out::SomeStruct>",
                includes: [ "<crubit/support/for/tests/rs_std/slice_ref.h>"],
                prereq_def: "SomeStruct"

            ),
            // `SomeStruct` is a `fwd_decls` prerequisite (not `defs` prerequisite):
            case!(
                rs: "*mut SomeStruct",
                cc: "::rust_out::SomeStruct*",
                includes: [],
                prereq_fwd_decl: "SomeStruct"
            ),
            // Testing propagation of deeper/nested `fwd_decls`:
            case!(
                rs: "*mut *mut SomeStruct",
                cc: ":: rust_out :: SomeStruct * *",
                includes: [],
                prereq_fwd_decl: "SomeStruct"
            ),
            // Testing propagation of `const` / `mut` qualifiers:
            case!(rs: "*mut *const f32", cc: "float const * *"),
            case!(rs: "*const *mut f32", cc: "float * const *"),
            // Rust function pointers are non-nullable, so when function pointers are used as a
            // parameter type (i.e. in `TypeLocation::FnParam`) then we can translate to
            // generate a C++ function *reference*, rather than a C++ function *pointer*.
            case!(
                rs: "extern \"C\" fn (f32, f32) -> f32",
                cc: "crubit :: type_identity_t < float (float , float) > &",
                includes: ["<crubit/support/for/tests/internal/cxx20_backports.h>"]
            ),
            // Unsafe extern "C" function pointers are, to C++, just function pointers.
            case!(
                rs: "unsafe extern \"C\" fn(f32, f32) -> f32",
                cc: "crubit :: type_identity_t < float (float , float) > &",
                includes: ["<crubit/support/for/tests/internal/cxx20_backports.h>"]
            ),
            // Nested function pointer (i.e. `TypeLocation::Other`) means that
            // we need to generate a C++ function *pointer*, rather than a C++
            // function *reference*.
            case!(
                rs: "*const extern \"C\" fn (f32, f32) -> f32",
                cc: "crubit :: type_identity_t < float (float , float) > * const *",
                includes: ["<crubit/support/for/tests/internal/cxx20_backports.h>"]
            ),
            // Extra parens/sugar are expected to be ignored:
            case!(rs: "(bool)", cc: "bool"),
            // References to MaybeUninit:
            case!(
                rs: "*const std::mem::MaybeUninit<i32>",
                cc: "std :: int32_t const *",
                includes: ["<cstdint>"]
            ),
            case!(
                rs: "&mut std::mem::MaybeUninit<i32>",
                cc: "std :: int32_t & [[clang :: annotate_type (\"lifetime\" , \"__anon1\")]]",
                includes: ["<cstdint>"]
            ),
            case!(
                rs: "()",
                cc: "std::tuple < >",
                includes: ["<tuple>"]
            ),
            case!(
                rs: "(i32,)",
                cc: "std::tuple<std::int32_t>",
                includes: ["<cstdint>", "<tuple>"]
            ),
            case!(
                rs: "(i32, i32)",
                cc: "std::tuple<std::int32_t, std::int32_t>",
                includes: ["<cstdint>", "<tuple>"]
            ),
        ];
        let preamble = quote! {
            #![allow(unused_parens)]
            #![feature(register_tool)]
            #![register_tool(__crubit)]

            pub struct SomeStruct {
                pub x: i32,
                pub y: i32,
            }
            pub enum SomeEnum {
                Cartesian{x: f64, y: f64},
                Polar{angle: f64, dist: f64},
            }
            pub union SomeUnion {
                pub x: i32,
                pub y: i32,
            }

            #[allow(unused)]
            type Identity<T> = T;

            pub use core::ffi::c_char;
            // TODO(b/283258442): Correctly handle something like this:
            // pub type MyChar = core::ffi::c_char;
        };
        test_ty(
            TypeLocation::FnParam,
            &testcases,
            preamble,
            |desc,
             tcx,
             ty,
             FormatCcExpectation {
                 expected_tokens,
                 expected_includes,
                 expected_prereq_def,
                 expected_prereq_fwd_decl,
             }| {
                let (actual_tokens, actual_prereqs) = {
                    let db = bindings_db_for_tests(tcx);
                    let s = format_ty_for_cc(&db, ty, TypeLocation::FnParam).unwrap();
                    (s.tokens.to_string(), s.prereqs)
                };
                let (actual_includes, actual_prereq_defs, actual_prereq_fwd_decls) =
                    (actual_prereqs.includes, actual_prereqs.defs, actual_prereqs.fwd_decls);

                let expected_tokens = expected_tokens.parse::<TokenStream>().unwrap().to_string();
                assert_eq!(actual_tokens, expected_tokens, "{desc}");

                assert!(
                    expected_includes.len() == actual_includes.len(),
                    "{desc}: `actual_includes` is unexpectedly not of the same length as `expected_includes`. actual_includes: {actual_includes:#?}; expected_includes: {expected_includes:#?}"
                );

                if expected_includes.len() > 0 {
                    let expected_includes = expected_includes
                        .into_iter()
                        .map(|include| include.parse::<TokenStream>().unwrap())
                        .collect::<Vec<_>>();
                    assert_cc_matches!(
                        format_cc_includes(&actual_includes),
                        quote! { #( __HASH_TOKEN__ include #expected_includes )*}
                    );
                }

                if let Some(expected_prereq_def) = expected_prereq_def {
                    let expected_def_id = find_def_id_by_name(tcx, expected_prereq_def);
                    assert_eq!(1, actual_prereq_defs.len());
                    assert_eq!(expected_def_id, actual_prereq_defs.into_iter().next().unwrap());
                } else {
                    assert!(
                        actual_prereq_defs.is_empty(),
                        "{desc}: `actual_prereq_defs` is unexpectedly non-empty",
                    );
                }

                if let Some(expected_prereq_fwd_decl) = expected_prereq_fwd_decl {
                    let expected_def_id = find_def_id_by_name(tcx, expected_prereq_fwd_decl);
                    assert_eq!(1, actual_prereq_fwd_decls.len());
                    assert_eq!(
                        expected_def_id,
                        actual_prereq_fwd_decls.into_iter().next().unwrap()
                    );
                } else {
                    assert!(
                        actual_prereq_fwd_decls.is_empty(),
                        "{desc}: `actual_prereq_fwd_decls` is unexpectedly non-empty",
                    );
                }
            },
        );
    }

    /// `test_format_ty_for_cc_failures` provides test coverage for cases where
    /// `format_ty_for_cc` returns an `Err(...)`.
    ///
    /// It seems okay to have no test coverage for now for the following types
    /// (which should never be encountered when generating bindings and where
    /// `format_ty_for_cc` should panic):
    /// - TyKind::Closure
    /// - TyKind::Error
    /// - TyKind::FnDef
    /// - TyKind::Infer
    ///
    /// TODO(lukasza): Add test coverage (here and in the "for_rs" flavours)
    /// for:
    /// - TyKind::Bound
    /// - TyKind::Dynamic (`dyn Eq`)
    /// - TyKind::Foreign (`extern type T`)
    /// - https://doc.rust-lang.org/beta/unstable-book/language-features/generators.html:
    ///   TyKind::Generator, TyKind::GeneratorWitness
    /// - TyKind::Param
    /// - TyKind::Placeholder
    #[test]
    fn test_format_ty_for_cc_failures() {
        let testcases = [
            // ( <Rust type>, <expected error message> )
            (
                // TODO(b/254507801): Expect `crubit::Never` instead (see the bug for more
                // details).
                "!", // TyKind::Never
                "The never type `!` is only supported as a return type (b/254507801)",
            ),
            (
                "&'static &'static i32", // TyKind::Ref (nested reference - referent of reference)
                "Failed to format the referent of the reference type `&'static &'static i32`: \
                 Can't format `&'static i32`, because references are only supported \
                 in function parameter types and return types (b/286256327)",
            ),
            (
                "extern \"C\" fn (&i32)", // TyKind::Ref (nested reference - underneath fn ptr)
                "Generic functions are not supported yet (b/259749023)",
            ),
            (
                "[i32; 42]", // TyKind::Array
                "The following Rust type is not supported yet: [i32; 42]",
            ),
            (
                // Check that the failure for slices is about not being supported and not failed
                // asserts about ABI and layout.
                "&'static [i32]", // TyKind::Slice (nested underneath TyKind::Ref)
                "Failed to format the referent of the reference type `&'static [i32]`: \
                 The following Rust type is not supported yet: [i32]",
            ),
            (
                "&'static str", // TyKind::Str (nested underneath TyKind::Ref)
                "Failed to format the referent of the reference type `&'static str`: \
                 The following Rust type is not supported yet: str",
            ),
            (
                "impl Eq", // TyKind::Alias
                "The following Rust type is not supported yet: impl Eq",
            ),
            (
                "fn(i32) -> i32", // TyKind::FnPtr (default ABI = "Rust")
                "Function pointers can't have a thunk: \
                 Any calling convention other than `extern \"C\"` requires a thunk",
            ),
            (
                "extern \"C\" fn (SomeStruct, f32) -> f32",
                "Function pointers can't have a thunk: Type of parameter #0 requires a thunk",
            ),
            (
                "extern \"C\" fn (f32, f32) -> SomeStruct",
                "Function pointers can't have a thunk: Return type requires a thunk",
            ),
            // TODO(b/254094650): Consider mapping this to Clang's (and GCC's) `__int128`
            // or to `absl::in128`.
            ("i128", "C++ doesn't have a standard equivalent of `i128` (b/254094650)"),
            ("u128", "C++ doesn't have a standard equivalent of `u128` (b/254094650)"),
            ("ConstGenericStruct<42>", "Generic types are not supported yet (b/259749095)"),
            ("TypeGenericStruct<u8>", "Generic types are not supported yet (b/259749095)"),
            (
                // This double-checks that TyKind::Adt(..., substs) are present
                // even if the type parameter argument is not explicitly specified
                // (here it comes from the default: `...Struct<T = u8>`).
                "TypeGenericStruct",
                "Generic types are not supported yet (b/259749095)",
            ),
            ("LifetimeGenericStruct<'static>", "Generic types are not supported yet (b/259749095)"),
            (
                "std::cmp::Ordering",
                "Type `std::cmp::Ordering` comes from the `core` crate, \
                 but no `--crate-header` was specified for this crate",
            ),
            ("Option<i8>", "Generic types are not supported yet (b/259749095)"),
            (
                // TODO(b/258261328): Once cross-crate bindings are supported we should try
                // to test them via a test crate that we control (rather than testing via
                // implementation details of the std crate).
                "core::alloc::LayoutError",
                "Type `std::alloc::LayoutError` comes from the `core` crate, but no `--crate-header` was specified for this crate",
            ),
            (
                "*const Option<i8>",
                "Failed to format the pointee \
                 of the pointer type `*const std::option::Option<i8>`: \
                 Generic types are not supported yet (b/259749095)",
            ),
        ];
        let preamble = quote! {
            #![feature(never_type)]

            #[repr(C)]
            pub struct SomeStruct {
                pub x: i32,
                pub y: i32,
            }

            pub struct ConstGenericStruct<const N: usize> {
                pub arr: [u8; N],
            }

            pub struct TypeGenericStruct<T = u8> {
                pub t: T,
            }

            pub struct LifetimeGenericStruct<'a> {
                pub reference: &'a u8,
            }
        };
        test_ty(TypeLocation::FnParam, &testcases, preamble, |desc, tcx, ty, expected_msg| {
            let db = bindings_db_for_tests(tcx);
            let anyhow_err = format_ty_for_cc(&db, ty, TypeLocation::FnParam)
                .expect_err(&format!("Expecting error for: {desc}"));
            let actual_msg = format!("{anyhow_err:#}");
            assert_eq!(&actual_msg, *expected_msg, "{desc}");
        });
    }

    #[test]
    fn test_format_ty_for_rs_successes() {
        // Test coverage for cases where `format_ty_for_rs` returns an `Ok(...)`.
        let testcases = [
            // ( <Rust type>, <expected Rust spelling for ..._cc_api_impl.rs> )
            ("bool", "bool"),
            ("f32", "f32"),
            ("f64", "f64"),
            ("i8", "i8"),
            ("i16", "i16"),
            ("i32", "i32"),
            ("i64", "i64"),
            ("i128", "i128"),
            ("isize", "isize"),
            ("u8", "u8"),
            ("u16", "u16"),
            ("u32", "u32"),
            ("u64", "u64"),
            ("u128", "u128"),
            ("usize", "usize"),
            ("char", "char"),
            ("!", "!"),
            ("()", "()"),
            // ADTs:
            ("SomeStruct", "::rust_out::SomeStruct"),
            ("SomeEnum", "::rust_out::SomeEnum"),
            ("SomeUnion", "::rust_out::SomeUnion"),
            // Type from another crate:
            ("std::cmp::Ordering", "::core::cmp::Ordering"),
            // `const` and `mut` pointers:
            ("*const i32", "*const i32"),
            ("*mut i32", "*mut i32"),
            // References:
            ("&i32", "& '__anon1 i32"),
            ("&mut i32", "& '__anon1 mut i32"),
            ("&'_ i32", "& '__anon1 i32"),
            ("&'static i32", "& 'static i32"),
            // Pointer to an ADT:
            ("*mut SomeStruct", "* mut :: rust_out :: SomeStruct"),
            ("extern \"C\" fn(i32) -> i32", "extern \"C\" fn(i32) -> i32"),
            // Pointer to a Slice:
            ("*mut [i32]", "*mut [i32]"),
            // MaybeUninit:
            ("&'static std::mem::MaybeUninit<i32>", "& 'static std :: mem :: MaybeUninit < i32 >"),
            (
                "&'static mut std::mem::MaybeUninit<i32>",
                "& 'static mut std :: mem :: MaybeUninit < i32 >",
            ),
            ("*const std::mem::MaybeUninit<i32>", "*const std::mem::MaybeUninit<i32>"),
            ("*mut std::mem::MaybeUninit<i32>", "*mut std::mem::MaybeUninit<i32>"),
        ];
        let preamble = quote! {
            #![feature(never_type)]

            pub struct SomeStruct {
                pub x: i32,
                pub y: i32,
            }
            pub enum SomeEnum {
                Cartesian{x: f64, y: f64},
                Polar{angle: f64, dist: f64},
            }
            pub union SomeUnion {
                pub x: i32,
                pub y: i32,
            }
        };
        test_ty(TypeLocation::FnParam, &testcases, preamble, |desc, tcx, ty, expected_tokens| {
            let db = bindings_db_for_tests(tcx);
            let actual_tokens = format_ty_for_rs(&db, ty.mid()).unwrap().to_string();
            let expected_tokens = expected_tokens.parse::<TokenStream>().unwrap().to_string();
            assert_eq!(actual_tokens, expected_tokens, "{desc}");
        });
    }

    #[test]
    fn test_format_ty_for_rs_failures() {
        // This test provides coverage for cases where `format_ty_for_rs` returns an
        // `Err(...)`.
        let testcases = [
            // ( <Rust type>, <expected error message> )
            (
                "[i32; 42]", // TyKind::Array
                "The following Rust type is not supported yet: [i32; 42]",
            ),
            (
                "&'static str", // TyKind::Str (nested underneath TyKind::Ref)
                "Failed to format the referent of the reference type `&'static str`: \
                 The following Rust type is not supported yet: str",
            ),
            (
                "impl Eq", // TyKind::Alias
                "The following Rust type is not supported yet: impl Eq",
            ),
            (
                "Option<i8>", // TyKind::Adt - generic + different crate
                "Generic types are not supported yet (b/259749095)",
            ),
        ];
        let preamble = quote! {};
        test_ty(TypeLocation::FnParam, &testcases, preamble, |desc, tcx, ty, expected_err| {
            let db = bindings_db_for_tests(tcx);
            let anyhow_err =
                format_ty_for_rs(&db, ty.mid()).expect_err(&format!("Expecting error for: {desc}"));
            let actual_err = format!("{anyhow_err:#}");
            assert_eq!(&actual_err, *expected_err, "{desc}");
        });
    }
}
