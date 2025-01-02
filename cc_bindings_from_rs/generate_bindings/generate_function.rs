// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::db::BindingsGenerator;

use crate::code_snippet::{ApiSnippets, CcPrerequisites, CcSnippet};
use crate::format_cc_ident;
use crate::generate_doc_comment;
use crate::generate_function_thunk::{generate_thunk_decl, generate_thunk_impl, is_thunk_required};
use crate::{
    format_param_types_for_cc, format_region_as_cc_lifetime, format_ret_ty_for_cc,
    generate_deprecated_tag, is_bridged_type, is_c_abi_compatible_by_value,
    liberate_and_deanonymize_late_bound_regions, post_analysis_typing_env, AllowReferences,
    BridgedType, CcType, FullyQualifiedName, RsSnippet,
};
use arc_anyhow::{Context, Result};
use code_gen_utils::escape_non_identifier_chars;
use code_gen_utils::make_rs_ident;
use code_gen_utils::CcInclude;
use error_report::{anyhow, bail, ensure};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;
use rustc_hir::Node;
use rustc_middle::mir::Mutability;
use rustc_middle::ty::{self, Ty, TyCtxt};
use rustc_span::def_id::{DefId, LocalDefId, LOCAL_CRATE};
use rustc_span::symbol::kw;

#[derive(Debug, Eq, PartialEq)]
enum FunctionKind {
    /// Free function (i.e. not a method).
    Free,

    /// Static method (i.e. the first parameter is not named `self`).
    StaticMethod,

    /// Instance method taking `self` by value (i.e. `self: Self`).
    MethodTakingSelfByValue,

    /// Instance method taking `self` by reference (i.e. `&self` or `&mut
    /// self`).
    MethodTakingSelfByRef,
}

impl FunctionKind {
    fn has_self_param(&self) -> bool {
        match self {
            FunctionKind::MethodTakingSelfByValue | FunctionKind::MethodTakingSelfByRef => true,
            FunctionKind::Free | FunctionKind::StaticMethod => false,
        }
    }
}

/// Formats a function with the given `local_def_id`.
///
/// Will panic if `local_def_id`
/// - is invalid
/// - doesn't identify a function,
pub fn generate_function(
    db: &dyn BindingsGenerator<'_>,
    local_def_id: LocalDefId,
) -> Result<ApiSnippets> {
    let tcx = db.tcx();
    let def_id: DefId = local_def_id.to_def_id(); // Convert LocalDefId to DefId.

    ensure!(
        tcx.generics_of(def_id).count() == 0,
        "Generic functions are not supported yet (b/259749023)"
    );

    let (sig_mid, sig_hir) = get_fn_sig(tcx, local_def_id);
    check_fn_sig(&sig_mid)?;
    // TODO(b/262904507): Don't require thunks for mangled extern "C" functions.
    let has_export_name = tcx.get_attr(def_id, rustc_span::symbol::sym::export_name).is_some();
    let has_no_mangle = tcx.get_attr(def_id, rustc_span::symbol::sym::no_mangle).is_some();
    let needs_thunk = is_thunk_required(&sig_mid).is_err() || (!has_no_mangle && !has_export_name);
    let thunk_name = {
        let symbol_name = if db.no_thunk_name_mangling() {
            if has_export_name {
                tcx.get_attr(def_id, rustc_span::symbol::sym::export_name)
                    .unwrap()
                    .value_str()
                    .expect("export_name is a string")
                    .to_string()
            } else {
                FullyQualifiedName::new(db, def_id)
                    .rs_name
                    .expect("Functions are assumed to always have a name")
                    .to_string()
            }
        } else {
            // Call to `mono` is ok - `generics_of` have been checked above.
            let instance = ty::Instance::mono(tcx, def_id);
            tcx.symbol_name(instance).name.to_string()
        };
        let target_path_mangled_hash = if db.no_thunk_name_mangling() {
            "".to_string()
        } else {
            format!("{}_", tcx.crate_hash(LOCAL_CRATE).to_hex())
        };
        if needs_thunk {
            format!(
                "__crubit_thunk_{}{}",
                target_path_mangled_hash,
                &escape_non_identifier_chars(&symbol_name)
            )
        } else {
            symbol_name.to_string()
        }
    };

    let fully_qualified_fn_name = FullyQualifiedName::new(db, def_id);
    let unqualified_rust_fn_name =
        fully_qualified_fn_name.rs_name.expect("Functions are assumed to always have a name");
    let main_api_fn_name = format_cc_ident(db, fully_qualified_fn_name.cpp_name.unwrap().as_str())
        .context("Error formatting function name")?;

    let mut main_api_prereqs = CcPrerequisites::default();
    let main_api_ret_type =
        format_ret_ty_for_cc(db, &sig_mid, Some(sig_hir))?.into_tokens(&mut main_api_prereqs);

    struct Param<'tcx> {
        cc_name: TokenStream,
        cpp_type: TokenStream,
        ty: Ty<'tcx>,
    }
    let params = {
        let names = tcx.fn_arg_names(def_id).iter();
        let cpp_types =
            format_param_types_for_cc(db, &sig_mid, Some(sig_hir), AllowReferences::Safe)?;
        names
            .enumerate()
            .zip(sig_mid.inputs().iter())
            .zip(cpp_types)
            .map(|(((i, name), &ty), cpp_type)| {
                let cc_name = format_cc_ident(db, name.as_str())
                    .unwrap_or_else(|_err| format_cc_ident(db, &format!("__param_{i}")).unwrap());
                let cpp_type = cpp_type.into_tokens(&mut main_api_prereqs);
                Param { cc_name, cpp_type, ty }
            })
            .collect_vec()
    };

    let self_ty: Option<Ty> = match tcx.impl_of_method(def_id) {
        Some(impl_id) => match tcx.impl_subject(impl_id).instantiate_identity() {
            ty::ImplSubject::Inherent(ty) => Some(ty),
            ty::ImplSubject::Trait(_) => panic!("Trait methods should be filtered by caller"),
        },
        None => None,
    };

    let method_kind = match tcx.hir_node_by_def_id(local_def_id) {
        Node::Item(_) => FunctionKind::Free,
        Node::ImplItem(_) => match tcx.fn_arg_names(def_id).first() {
            Some(arg_name) if arg_name.name == kw::SelfLower => {
                let self_ty = self_ty.expect("ImplItem => non-None `self_ty`");
                if params[0].ty == self_ty {
                    FunctionKind::MethodTakingSelfByValue
                } else {
                    match params[0].ty.kind() {
                        ty::TyKind::Ref(_, referent_ty, _) if *referent_ty == self_ty => {
                            FunctionKind::MethodTakingSelfByRef
                        }
                        _ => bail!("Unsupported `self` type"),
                    }
                }
            }
            _ => FunctionKind::StaticMethod,
        },
        other => panic!("Unexpected HIR node kind: {other:?}"),
    };
    let method_qualifiers = match method_kind {
        FunctionKind::Free | FunctionKind::StaticMethod => quote! {},
        FunctionKind::MethodTakingSelfByValue => quote! { && },
        FunctionKind::MethodTakingSelfByRef => match params[0].ty.kind() {
            ty::TyKind::Ref(region, _, mutability) => {
                let lifetime_annotation = format_region_as_cc_lifetime(region);
                let mutability = match mutability {
                    Mutability::Mut => quote! {},
                    Mutability::Not => quote! { const },
                };
                quote! { #mutability #lifetime_annotation }
            }
            _ => panic!("Expecting TyKind::Ref for MethodKind...Self...Ref"),
        },
    };

    let struct_name = match self_ty {
        Some(ty) => match ty.kind() {
            ty::TyKind::Adt(adt, substs) => {
                assert_eq!(0, substs.len(), "Callers should filter out generics");
                Some(FullyQualifiedName::new(db, adt.did()))
            }
            _ => panic!("Non-ADT `impl`s should be filtered by caller"),
        },
        None => None,
    };
    let needs_definition = unqualified_rust_fn_name.as_str() != thunk_name;
    let main_api_params = params
        .iter()
        .skip(if method_kind.has_self_param() { 1 } else { 0 })
        .map(|Param { cc_name, cpp_type, .. }| quote! { #cpp_type #cc_name })
        .collect_vec();
    let main_api = {
        let doc_comment = {
            let doc_comment = generate_doc_comment(tcx, local_def_id);
            quote! { __NEWLINE__ #doc_comment }
        };

        let mut prereqs = main_api_prereqs.clone();
        prereqs.move_defs_to_fwd_decls();

        let static_ = if method_kind == FunctionKind::StaticMethod {
            quote! { static }
        } else {
            quote! {}
        };
        let extern_c = if !needs_definition {
            quote! { extern "C" }
        } else {
            quote! {}
        };

        let mut attributes = vec![];
        // Attribute: must_use
        if let Some(must_use_attr) = tcx.get_attr(def_id, rustc_span::symbol::sym::must_use) {
            match must_use_attr.value_str() {
                None => attributes.push(quote! {[[nodiscard]]}),
                Some(symbol) => {
                    let message = symbol.as_str();
                    attributes.push(quote! {[[nodiscard(#message)]]});
                }
            };
        }
        // Attribute: deprecated
        if let Some(cc_deprecated_tag) = generate_deprecated_tag(tcx, def_id) {
            attributes.push(cc_deprecated_tag);
        }
        // Also check the impl block to which this function belongs (if there is one).
        // Note: parent_def_id can be Some(...) even if the function is not inside an
        // impl block.
        if let Some(parent_def_id) = tcx.opt_parent(def_id) {
            if let Some(cc_deprecated_tag) = generate_deprecated_tag(tcx, parent_def_id) {
                attributes.push(cc_deprecated_tag);
            }
        }

        CcSnippet {
            prereqs,
            tokens: quote! {
                __NEWLINE__
                #doc_comment
                #extern_c #(#attributes)* #static_
                    #main_api_ret_type #main_api_fn_name (
                        #( #main_api_params ),*
                    ) #method_qualifiers;
                __NEWLINE__
            },
        }
    };
    let cc_details = if !needs_definition {
        CcSnippet::default()
    } else {
        let thunk_name = format_cc_ident(db, &thunk_name).context("Error formatting thunk name")?;
        let struct_name = match struct_name.as_ref() {
            None => quote! {},
            Some(fully_qualified_name) => {
                let name = fully_qualified_name.cpp_name.expect("Structs always have a name");
                let name = format_cc_ident(db, name.as_str()).expect(
                    "Caller of generate_function should verify struct via generate_adt_core",
                );
                quote! { #name :: }
            }
        };

        let mut prereqs = main_api_prereqs;
        let thunk_decl =
            generate_thunk_decl(db, &sig_mid, Some(sig_hir), &thunk_name, AllowReferences::Safe)?
                .into_tokens(&mut prereqs);

        let mut thunk_args = params
            .iter()
            .enumerate()
            .map(|(i, Param { cc_name, cpp_type, ty, .. })| {
                if let Some(BridgedType { cpp_type: cpp_type2, .. }) = is_bridged_type(db, *ty)? {
                    if let CcType::Pointer { .. } = cpp_type2 {
                        Ok(quote! { #cc_name })
                    } else {
                        Ok(quote! { & #cc_name })
                    }
                } else if i == 0 && method_kind.has_self_param() {
                    if method_kind == FunctionKind::MethodTakingSelfByValue {
                        Ok(quote! { this })
                    } else {
                        Ok(quote! { *this })
                    }
                } else if is_c_abi_compatible_by_value(*ty) {
                    Ok(quote! { #cc_name })
                } else if !ty.needs_drop(tcx, post_analysis_typing_env(tcx, def_id)) {
                    // As an optimization, if the type is trivially destructible, we don't
                    // need to move it to a new NoDestructor location. We can directly copy the
                    // bytes.
                    Ok(quote! { & #cc_name })
                } else {
                    // The implementation will copy the bytes, we just need to leave the variable
                    // behind in a valid moved-from state.
                    // TODO(jeanpierreda): Ideally, the Rust code should C++-move instead of memcpy,
                    // allowing us to avoid one extra memcpy: we could move it directly into its
                    // target location, instead of moving to a temporary that we memcpy to its
                    // target location.
                    prereqs.includes.insert(db.support_header("internal/slot.h"));
                    Ok(quote! { crubit::Slot<#cpp_type>(std::move(#cc_name)).Get() })
                }
            })
            .collect::<Result<Vec<_>>>()?;
        let impl_body: TokenStream;
        if let Some(attrs) = is_bridged_type(db, sig_mid.output())? {
            let cpp_type = format_cc_ident(db, attrs.cpp_type.as_ref())?;
            thunk_args.push(quote! { &__ret_val_holder.val });

            // Below, we use a union to allocate uninitialized memory that fits cpp_type.
            // The union prevents the type from being default constructed. It's
            // the responsibility of the thunk to properly initialize the
            // memory. In the union's destructor we use std::destroy_at to call
            // the cpp_type's destructor after the value has been moved on return.
            impl_body = quote! {
                union __crubit_return_union {
                    constexpr __crubit_return_union() {}
                    ~__crubit_return_union() { std::destroy_at(&this->val); }
                    #cpp_type val;
                } __ret_val_holder;

                __crubit_internal :: #thunk_name( #( #thunk_args ),* );

                return std::move(__ret_val_holder.val);
            };
        } else if is_c_abi_compatible_by_value(sig_mid.output()) {
            impl_body = quote! {
                return __crubit_internal :: #thunk_name( #( #thunk_args ),* );
            };
        } else {
            if let Some(adt_def) = sig_mid.output().ty_adt_def() {
                let core = db.generate_adt_core(adt_def.did())?;
                db.generate_move_ctor_and_assignment_operator(core).map_err(|_| {
                    anyhow!("Can't pass the return type by value without a move constructor")
                })?;
            }
            thunk_args.push(quote! { __ret_slot.Get() });
            impl_body = quote! {
                crubit::Slot<#main_api_ret_type> __ret_slot;
                __crubit_internal :: #thunk_name( #( #thunk_args ),* );
                return std::move(__ret_slot).AssumeInitAndTakeValue();
            };
            prereqs.includes.insert(CcInclude::utility()); // for `std::move`
            prereqs.includes.insert(db.support_header("internal/slot.h"));
        };
        CcSnippet {
            prereqs,
            tokens: quote! {
                __NEWLINE__
                #thunk_decl
                inline #main_api_ret_type #struct_name #main_api_fn_name (
                        #( #main_api_params ),* ) #method_qualifiers {
                    #impl_body
                }
                __NEWLINE__
            },
        }
    };

    let rs_details = if !needs_thunk {
        RsSnippet::default()
    } else {
        let fully_qualified_fn_name = match struct_name.as_ref() {
            None => fully_qualified_fn_name.format_for_rs(),
            Some(struct_name) => {
                let fn_name = make_rs_ident(unqualified_rust_fn_name.as_str());
                let struct_name = struct_name.format_for_rs();
                quote! { #struct_name :: #fn_name }
            }
        };
        generate_thunk_impl(db, def_id, &sig_mid, &thunk_name, fully_qualified_fn_name)?
    };

    Ok(ApiSnippets { main_api, cc_details, rs_details })
}

pub fn check_fn_sig(sig: &ty::FnSig) -> Result<()> {
    if sig.c_variadic {
        // TODO(b/254097223): Add support for variadic functions.
        bail!("C variadic functions are not supported (b/254097223)");
    }

    Ok(())
}

/// Returns the rustc_middle and rustc_hir function signatures.
///
/// In the case of rustc_hir, this returns the `FnDecl`, not the
/// `rustc_hir::FnSig`, because the `FnDecl` type is used for both function
/// pointers and actual functions. This makes it a more useful vocabulary type.
/// `FnDecl` does drop information, but that information is already on the
/// rustc_middle `FnSig`, so there is no loss.
pub fn get_fn_sig(tcx: TyCtxt, local_def_id: LocalDefId) -> (ty::FnSig, &rustc_hir::FnDecl) {
    let def_id = local_def_id.to_def_id();
    let sig_mid = liberate_and_deanonymize_late_bound_regions(
        tcx,
        tcx.fn_sig(def_id).instantiate_identity(),
        def_id,
    );
    let sig_hir = tcx.hir_node_by_def_id(local_def_id).fn_sig().unwrap();
    (sig_mid, sig_hir.decl)
}
