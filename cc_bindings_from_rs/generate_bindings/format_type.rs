// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Create the right string reprensentation of a type or an identifier.

extern crate rustc_abi;
extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_span;

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
use rustc_abi::{BackendRepr, HasDataLayout, Integer, Layout, Primitive, Scalar, TargetDataLayout};
use rustc_hir::def::Res;
use rustc_middle::mir::Mutability;
use rustc_middle::ty::{self, AdtDef, GenericArg, Ty, TyCtxt};
use rustc_span::def_id::{CrateNum, DefId, LOCAL_CRATE};
use rustc_span::symbol::{sym, Symbol};
use std::rc::Rc;

/// Implementation of `BindingsGenerator::format_top_level_ns_for_crate`.
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

/// Implementation of `BindingsGenerator::format_cc_ident`.
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

/// Returns a CcSnippet referencing `rs_std::StrRef` and its include path.
pub fn format_str_ref_for_cc(db: &dyn BindingsGenerator<'_>) -> CcSnippet {
    CcSnippet::with_include(quote! { rs_std::StrRef }, db.support_header("rs_std/str_ref.h"))
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

/// Implementation of `BindingsGenerator::format_ty_for_cc`.
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
        // `crubit/support/rs_std/char.h` and `crubit/support/rs_std/char_test.cc` (search for
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
                quote! { rs_std::char_ },
                db.support_header("rs_std/char.h"),
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

            if let Some(BridgedType { include_path, .. }) = is_bridged_type(db, ty.mid())? {
                prereqs.includes.insert(CcInclude::from_path(include_path.as_str()));
            } else {
                let has_cpp_type = crubit_attr::get_attrs(db.tcx(), adt.did())?.cpp_type.is_some();
                ensure!(
                    has_cpp_type || !has_non_lifetime_substs(substs),
                    "Generic types are not supported yet (b/259749095)"
                );
                ensure!(
                    is_public_or_supported_export(db, adt.did()),
                    "Not a public or a supported reexported type (b/262052635)."
                );

                if def_id.krate == LOCAL_CRATE {
                    prereqs.defs.insert(def_id);
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
            match location {
                TypeLocation::FnReturn | TypeLocation::FnParam { .. } | TypeLocation::Const => (),
                TypeLocation::Other => bail!(
                    "Can't format `{ty}`, because references are only supported in \
                     function parameter types, return types, and consts (b/286256327)",
                ),
            };

            if matches!(referent_mid.kind(), ty::TyKind::Slice(_)) {
                check_slice_layout(db.tcx(), ty.mid());
            }

            if matches!(referent_mid.kind(), ty::TyKind::Str) {
                check_slice_layout(db.tcx(), ty.mid());
                if mutability.is_mut() {
                    bail!("Mutable references to `str` are not yet supported.")
                }
                return Ok(format_str_ref_for_cc(db));
            }

            let mut referent_hir = None;
            if let Some(hir) = ty.hir(db) {
                if let rustc_hir::TyKind::Ref(_, mut_p, ..) = &hir.kind {
                    referent_hir = Some(mut_p.ty);
                }
            }

            let lifetime = format_region_as_cc_lifetime(tcx, region);
            let treat_ref_as_ptr: bool = (|| {
                // Parameter type references are only converted to C++ references if they are
                // valid exclusively for the lifetime of the function.
                //
                // References with a more complex lifetime are converted to pointers.
                // See <internal link> for more details on the motivation.
                let TypeLocation::FnParam { is_self_param, elided_is_output } = location else {
                    return false;
                };
                // `self` parameters are always passed by-ref, never by pointer.
                if is_self_param {
                    return false;
                }
                // Explicit lifetimes are always converted to pointers.
                if !region_is_elided(tcx, *region) {
                    return true;
                }
                // Elided lifetimes are converted to pointers if the elided lifetime is captured by
                // the output of the function.
                if elided_is_output {
                    return true;
                }
                false
            })();

            let ptr_or_ref_prefix = if treat_ref_as_ptr {
                quote! { * #lifetime }
            } else {
                quote! { & #lifetime }
            };

            // Early return in case we handle a transparent reference type.
            if let Some(snippet) = format_transparent_pointee_or_reference_for_cc(
                db,
                *referent_mid,
                referent_hir,
                *mutability,
                ptr_or_ref_prefix.clone(),
            ) {
                return Ok(snippet);
            }

            let referent = SugaredTy::new(*referent_mid, referent_hir);
            format_pointer_or_reference_ty_for_cc(db, referent, *mutability, ptr_or_ref_prefix)
                .with_context(|| {
                    format!("Failed to format the referent of the reference type `{ty}`")
                })?
        }
        ty::TyKind::FnPtr(sig_tys, fn_header) => {
            let sig = {
                let sig_tys = match sig_tys.no_bound_vars() {
                    None => bail!("Generic function pointers are not supported yet (b/259749023)"),
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
                TypeLocation::FnReturn | TypeLocation::FnParam { .. } | TypeLocation::Const => {
                    quote! { & }
                }
                TypeLocation::Other => quote! { * },
            };

            let mut prereqs = CcPrerequisites::default();
            prereqs.includes.insert(db.support_header("internal/cxx20_backports.h"));

            let mut sig_hir = None;
            if let Some(hir) = ty.hir(db) {
                if let rustc_hir::TyKind::FnPtr(bare_fn) = &hir.kind {
                    sig_hir = Some(bare_fn.decl);
                }
            }
            let ret_type = format_ret_ty_for_cc(db, &sig, sig_hir)?.into_tokens(&mut prereqs);
            let param_types = format_param_types_for_cc(
                db,
                &sig,
                sig_hir,
                AllowReferences::Safe,
                /*has_self_param=*/ false,
            )?
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
    let output_ty = SugaredTy::fn_output(sig_mid, sig_hir);
    db.format_ty_for_cc(output_ty, TypeLocation::FnReturn)
        .with_context(|| format!("Error formatting function return type `{output_ty}`"))
}

pub fn has_elided_region<'tcx>(tcx: TyCtxt<'tcx>, search_ty: ty::Ty<'tcx>) -> bool {
    use core::ops::ControlFlow;
    use rustc_middle::ty::{Region, TyCtxt, TypeVisitor};

    struct HasUnnamedRegion;
    struct Searcher<'tcx> {
        tcx: TyCtxt<'tcx>,
    }
    impl<'tcx> TypeVisitor<TyCtxt<'tcx>> for Searcher<'tcx> {
        type Result = ControlFlow<HasUnnamedRegion>;
        fn visit_region(&mut self, region: Region<'tcx>) -> ControlFlow<HasUnnamedRegion> {
            if region_is_elided(self.tcx, region) {
                ControlFlow::Break(HasUnnamedRegion)
            } else {
                ControlFlow::Continue(())
            }
        }
    }
    match (Searcher { tcx }).visit_ty(search_ty) {
        ControlFlow::Break(HasUnnamedRegion) => true,
        ControlFlow::Continue(()) => false,
    }
}

pub fn region_is_elided<'tcx>(tcx: TyCtxt<'tcx>, region: ty::Region<'tcx>) -> bool {
    match region.get_name(tcx) {
        Some(name) => name.as_str().starts_with(query_compiler::ANON_REGION_PREFIX),
        None => true,
    }
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
    has_self_param: bool,
) -> Result<Vec<CcSnippet>> {
    let region_counts = std::cell::LazyCell::new(|| count_regions(sig_mid));
    let elided_is_output = has_elided_region(db.tcx(), sig_mid.output());
    let param_types = SugaredTy::fn_inputs(sig_mid, sig_hir);
    let mut snippets = Vec::with_capacity(param_types.len());
    for (i, param_type) in param_types.enumerate() {
        let is_self_param = i == 0 && has_self_param;
        let mut cc_type = db
            .format_ty_for_cc(param_type, TypeLocation::FnParam { elided_is_output, is_self_param })
            .with_context(|| format!("Error handling parameter #{i} of type `{param_type}`"))?;
        if allow_references == AllowReferences::Safe {
            // In parameter position, format_ty_for_cc defaults to allowing free
            // (non-static) references. We need to decide which references we
            // allow -- in this case, we choose to allow references _only_ if
            // the reference cannot mutably alias.
            match param_type.mid().kind() {
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

fn has_non_lifetime_substs(substs: &[ty::GenericArg]) -> bool {
    substs.iter().any(|subst| subst.as_region().is_none())
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
            let has_cpp_type = crubit_attr::get_attrs(db.tcx(), adt.did())?.cpp_type.is_some();
            ensure!(
                has_cpp_type || !has_non_lifetime_substs(substs),
                "Generic types are not supported yet (b/259749095)"
            );
            let type_name = FullyQualifiedName::new(db, adt.did()).format_for_rs();
            let generic_params = if substs.len() == 0 {
                quote! {}
            } else {
                let generic_params = substs
                    .iter()
                    .map(|subst| match subst.kind() {
                        ty::GenericArgKind::Type(ty) => format_ty_for_rs(db, ty),
                        ty::GenericArgKind::Lifetime(region) => {
                            assert_eq!(
                                region.kind(),
                                ty::RegionKind::ReStatic,
                                "We should never format types with non-'static regions, as \
                                    thunks should first call `replace_all_regions_with_static`."
                            );
                            Ok(quote! { 'static })
                        }
                        ty::GenericArgKind::Const(_) => {
                            panic!("Const parameters are not supported, but found {ty}")
                        }
                    })
                    .collect::<Result<Vec<TokenStream>>>()?;
                quote! { < #(#generic_params),* > }
            };
            quote! { #type_name #generic_params }
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
            let lifetime = format_region_as_rs_lifetime(db.tcx(), region);
            if matches!(referent_ty.kind(), ty::TyKind::Str) && mutability.is_not() {
                return Ok(quote! { & #lifetime str });
            }
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

pub fn format_region_as_cc_lifetime<'tcx>(
    tcx: TyCtxt<'tcx>,
    region: &ty::Region<'tcx>,
) -> TokenStream {
    let name = region
        .get_name(tcx)
        .expect("Caller should use `liberate_and_deanonymize_late_bound_regions`");
    let name = name
        .as_str()
        .strip_prefix('\'')
        .expect("All Rust lifetimes are expected to begin with the \"'\" character");

    // TODO(b/286299326): Use `$a` or `$(foo)` or `$static` syntax below.
    quote! { [[clang::annotate_type("lifetime", #name)]] }
}

pub fn format_region_as_rs_lifetime<'tcx>(
    tcx: TyCtxt<'tcx>,
    region: &ty::Region<'tcx>,
) -> TokenStream {
    let name = region
        .get_name(tcx)
        .expect("Caller should use `liberate_and_deanonymize_late_bound_regions`");
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
    pub include_path: Symbol,
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

/// Whether the layout is from a type that implements [`std::marker::PointerLike`].
///
/// Currently, that means that the type is pointer-sized, pointer-aligned,
/// and has a initialized (non-union), scalar ABI.
fn layout_pointer_like(from: &Layout, data_layout: &TargetDataLayout) -> bool {
    from.size() == data_layout.pointer_size()
        && from.align().abi == data_layout.pointer_align().abi
        && matches!(from.backend_repr(), BackendRepr::Scalar(Scalar::Initialized { .. }))
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

        if !layout_pointer_like(&get_layout(db.tcx(), ty)?, db.tcx().data_layout()) {
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

/// Returns `Ok(Some(BridgedType)))` if #[crubit_annotate::...] is configured to require type
/// bridging.
///
/// A type is said to require type bridging ("bridged type") if either
/// `cpp_to_rust_converter` or `rust_to_cpp_converter` is set in the
/// #[crubit_annotate::...] attribute. The idea is that for a Rust
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
            include_path,
            cpp_to_rust_converter: None,
            rust_to_cpp_converter: None,
            ..
        } => {
            let Some(include_path) = include_path else {
                // NOTE: this branch is surprising, and the annotations should probably be rewritten
                // to be more explicit.
                //
                // Specifically when an include path is specified we treat the type as
                // a pointer-like transmute bridged type rather than a non-bridged type.
                // When no include path is specified, we treat the type as non-bridged.
                return Ok(None);
            };

            let ts = cpp_type.as_str().parse::<TokenStream>().unwrap_or_else(|err| {
                panic!("Failed to parse CrubitAttrs.cpp_type for {ty} = {cpp_type}: {err}")
            });

            match code_gen_utils::is_cpp_pointer_type(ts) {
                Some(cv) => {
                    ensure_ty_is_pointer_like(db, ty)?;

                    Ok(Some(BridgedType {
                        cpp_type: CcType::Pointer { cpp_type, cv },
                        include_path,
                        conversion_info: BridgedTypeConversionInfo::PointerLikeTransmute,
                    }))
                }
                None => Ok(None),
            }
        }
        crubit_attr::CrubitAttrs {
            cpp_type: Some(cpp_type),
            include_path,
            cpp_to_rust_converter: Some(cpp_to_rust_converter),
            rust_to_cpp_converter: Some(rust_to_cpp_converter),
            ..
        } => {
            let Some(include_path) = include_path else {
                panic!("Failed to parse CrubitAttrs.include_path for {ty} = {cpp_type}: missing include_path")
            };

            let ts = cpp_type.as_str().parse::<TokenStream>().unwrap_or_else(|err| {
                panic!("Failed to parse CrubitAttrs.cpp_type for {ty} = {cpp_type}: {err}")
            });

            match code_gen_utils::is_cpp_pointer_type(ts) {
                Some(cv) => Ok(Some(BridgedType {
                    cpp_type: CcType::Pointer { cpp_type, cv },
                    include_path,
                    conversion_info: BridgedTypeConversionInfo::ExternCFuncConverters {
                        cpp_to_rust_converter,
                        rust_to_cpp_converter,
                    },
                })),
                None => Ok(Some(BridgedType {
                    cpp_type: CcType::Other(cpp_type),
                    include_path,
                    conversion_info: BridgedTypeConversionInfo::ExternCFuncConverters {
                        cpp_to_rust_converter,
                        rust_to_cpp_converter,
                    },
                })),
            }
        }
        crubit_attr::CrubitAttrs { cpp_to_rust_converter: Some(cpp_to_rust_converter), .. } => {
            bail!(
                "Invalid state of  #[crubit_annotate::...] attribute. rust_to_cpp_converter \
                    set ({cpp_to_rust_converter}), but cpp_type not set."
            )
        }
        crubit_attr::CrubitAttrs { rust_to_cpp_converter: Some(rust_to_cpp_converter), .. } => {
            bail!(
                "Invalid state of  #[crubit_annotate::...] attribute. cpp_to_rust_converter \
                    set ({rust_to_cpp_converter}), but cpp_type not set."
            )
        }
        crubit_attr::CrubitAttrs {
            cpp_type: None,
            include_path: _,
            cpp_to_rust_converter: None,
            rust_to_cpp_converter: None,
            ..
        } => Ok(None),
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
    let attrs = crubit_attr::get_attrs(db.tcx(), adt.did())
        .unwrap_or_else(|e| panic!("Invalid attrs for {ty}: {e}"));
    if let Some(attrs) = requires_type_bridging(db, ty, attrs)? {
        return Ok(Some(attrs));
    };

    // The ADT does not need to be bridged, but check if it has generic types that
    // need to be bridged e.g. Box<BridgedType> cannot be formated at
    // the moment. If we encounter a type like this we return an error.
    substs.iter().flat_map(|a| a.walk()).try_for_each(|a| {
        let Some(ty) = a.as_type() else {
            return Ok(());
        };
        if is_bridged_type(db, ty)?.is_some() {
            bail!(
                "Can't format ADT as it has a generic type `{ty}` that is a \
                    bridged type",
            );
        }
        Ok(())
    })?;
    Ok(None)
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
