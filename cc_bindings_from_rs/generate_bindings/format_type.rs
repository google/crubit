// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Create the right string reprensentation of a type or an identifier.

use crate::code_snippet::{CcPrerequisites, CcSnippet};
use crate::db::BindingsGenerator;
use crate::generate_function::check_fn_sig;
use crate::generate_function_thunk::is_thunk_required;
use crate::{
    check_feature_enabled_on_self_and_all_deps, check_slice_layout, count_regions, get_layout,
    is_public_or_supported_export, matches_qualified_name, AllowReferences, CcType,
    FineGrainedFeature, FullyQualifiedName, SugaredTy, TypeLocation,
};
use arc_anyhow::{Context, Result};
use code_gen_utils::{CcInclude, NamespaceQualifier};
use error_report::{anyhow, bail, ensure};
use proc_macro2::TokenStream;
use quote::quote;
use rustc_hir::def::Res;
use rustc_middle::mir::Mutability;
use rustc_middle::ty::{self, AdtDef, GenericArg, Ty};
use rustc_span::def_id::{CrateNum, DefId, LOCAL_CRATE};
use rustc_span::symbol::{sym, Symbol};
use rustc_target::abi::{BackendRepr, HasDataLayout, Integer, Primitive, Scalar};
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

pub fn format_ns_path_for_cc(
    db: &dyn BindingsGenerator<'_>,
    ns: &NamespaceQualifier,
) -> Result<TokenStream> {
    let idents = ns.0.iter().map(|s| format_cc_ident(db, s)).collect::<Result<Vec<_>>>()?;
    Ok(quote! { #(#idents::)* })
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
        ty::TyKind::Tuple(types) => {
            if types.len() == 0 {
                match location {
                    TypeLocation::FnReturn => keyword(quote! { void }),
                    _ => {
                        // TODO(b/254507801): Maybe translate into `crubit::Unit`?
                        bail!("`()` / `void` is only supported as a return type (b/254507801)");
                    }
                }
            } else {
                // TODO(b/254099023): Add support for tuples.
                bail!("Tuples are not supported yet: {} (b/254099023)", ty);
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
                prereqs.includes.insert(CcInclude::user_header(cpp_type_include.as_str().into()));
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
            assert!(matches!(sig.abi, rustc_target::spec::abi::Abi::C { .. }));

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
pub fn format_core_alias_for_cc<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    ty: SugaredTy<'tcx>,
) -> Option<CcSnippet> {
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
    let [module, item] = def_path.data.as_slice() else {
        return None;
    };
    if module.data != rustc_hir::definitions::DefPathData::TypeNs(sym::ffi) {
        return None;
    };
    let rustc_hir::definitions::DefPathData::TypeNs(item) = item.data else {
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
    let hir = sig_hir.and_then(|sig_hir| match sig_hir.output {
        rustc_hir::FnRetTy::Return(hir_ty) => Some(hir_ty),
        _ => None,
    });
    db.format_ty_for_cc(SugaredTy::new(sig_mid.output(), hir), TypeLocation::FnReturn)
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
    if let Some(sig_hir) = sig_hir {
        assert_eq!(
            sig_mid.inputs().len(),
            sig_hir.inputs.len(),
            "internal error: MIR and HIR function signatures do not line up"
        );
    }

    let region_counts = std::cell::LazyCell::new(|| count_regions(sig_mid));

    sig_mid
        .inputs()
        .iter()
        .enumerate()
        .map(|(i, &mid)| {
            let hir = sig_hir.map(|sig_hir| &sig_hir.inputs[i]);
            let mut cc_type = db
                .format_ty_for_cc(SugaredTy::new(mid, hir), TypeLocation::FnParam)
                .with_context(|| format!("Error handling parameter #{i}"))?;
            if allow_references == AllowReferences::Safe {
                // In parameter position, format_ty_for_cc defaults to allowing free
                // (non-static) references. We need to decide which references we
                // allow -- in this case, we choose to allow references _only_ if
                // the reference cannot mutably alias, and does not have any lifetime
                // requirements from the caller.
                match mid.kind() {
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

            Ok(cc_type)
        })
        .collect()
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
            if types.len() == 0 {
                quote! { () }
            } else {
                // TODO(b/254099023): Add support for tuples.
                bail!("Tuples are not supported yet: {} (b/254099023)", ty);
            }
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

pub struct BridgedType {
    pub cpp_type: CcType,
    pub cpp_type_include: Symbol,
    pub conversion_info: BridgedTypeConversionInfo,
}

pub enum BridgedTypeConversionInfo {
    PointerLikeTransmute,
    ExternCFuncConverters { cpp_to_rust_converter: Symbol, rust_to_cpp_converter: Symbol },
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