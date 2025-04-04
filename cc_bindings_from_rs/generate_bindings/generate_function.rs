// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::format_type::{format_cc_ident, format_ty_for_cc};
use crate::generate_doc_comment;
use crate::generate_function_thunk::{
    generate_thunk_decl, generate_thunk_impl, ident_or_opt_ident, is_thunk_required,
};
use crate::{
    format_param_types_for_cc, format_region_as_cc_lifetime, format_ret_ty_for_cc,
    generate_deprecated_tag, is_bridged_type, is_c_abi_compatible_by_value,
    liberate_and_deanonymize_late_bound_regions, post_analysis_typing_env, AllowReferences,
    BridgedType, CcType, FullyQualifiedName, RsSnippet,
};
use arc_anyhow::{Context, Result};
use code_gen_utils::{
    escape_non_identifier_chars, expect_format_cc_ident, make_rs_ident, CcInclude,
};
use database::code_snippet::{ApiSnippets, CcPrerequisites, CcSnippet};
use database::BindingsGenerator;
use database::{SugaredTy, TypeLocation};
use error_report::{anyhow, bail, ensure};
use itertools::Itertools;
use proc_macro2::{Literal, TokenStream};
use quote::quote;
use rustc_hir::Node;
use rustc_middle::mir::Mutability;
use rustc_middle::ty::{self, Ty, TyCtxt};
use rustc_span::def_id::{DefId, LocalDefId, LOCAL_CRATE};
use rustc_span::symbol::{kw, Symbol};
use std::collections::BTreeSet;

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

fn thunk_name(
    db: &dyn BindingsGenerator,
    def_id: DefId,
    export_name: Option<Symbol>,
    needs_thunk: bool,
) -> String {
    let tcx = db.tcx();
    let symbol_name = if db.no_thunk_name_mangling() {
        if let Some(export_name) = export_name {
            export_name.to_string()
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
        symbol_name
    }
}

/// Returns a vector of identifiers `{prefix}_{i}` for `i` in `[0, n)`.
fn ident_for_each(prefix: &str, n: usize) -> Vec<TokenStream> {
    (0..n).map(|i| expect_format_cc_ident(&format!("{prefix}_{i}"))).collect()
}

/// Converts a C++ value to a C-ABI-compatible type.
///
/// * `db` - the bindings generator
/// * `cc_ident` - the name of the C++ lvalue.
/// * `ty` - the Rust type of the parameter
/// * `post_analysis_typing_env` - the typing environment, used to determine if the type is
///   trivially destructible (no drop glue).
/// * `includes` - an output parameter used to store the set of C++ includes required
/// * `statements` - an output parameter used to store the C++ statements performing the conversion
///
/// Returns a `TokenStream` containing an expression that evaluates to the
/// C-ABI-compatible version of the type.
fn cc_param_to_c_abi<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    cc_ident: TokenStream,
    ty: SugaredTy<'tcx>,
    post_analysis_typing_env: ty::TypingEnv<'tcx>,
    includes: &mut BTreeSet<CcInclude>,
    statements: &mut TokenStream,
) -> Result<TokenStream> {
    Ok(if let Some(BridgedType { cpp_type, .. }) = is_bridged_type(db, ty.mid())? {
        if let CcType::Pointer { .. } = cpp_type {
            cc_ident
        } else {
            quote! { & #cc_ident }
        }
    } else if is_c_abi_compatible_by_value(ty.mid()) {
        cc_ident
    } else if let Some(tuple_tys) = ty.as_tuple(db) {
        let n = tuple_tys.len();
        let c_abi_names = ident_for_each(&format!("{cc_ident}_cabi"), n);

        // Create a statement defining a local for the C ABI representation of each tuple element.
        // This is necessary in order to ensure that we have a non-temporary value to point to
        // in the `result_name` array below.
        //
        // Locals of unknown type use `auto&&` in order to avoid changing the type of the
        // expression.
        for (i, c_abi_name) in c_abi_names.iter().enumerate() {
            let tuple_element_name = expect_format_cc_ident(&format!("{cc_ident}_{i}"));
            // Needed to avoid `proc_macro2` interpolating `1usize` instead of `1`.
            let i_literal = Literal::usize_unsuffixed(i);
            statements.extend(quote! {
                auto&& #tuple_element_name = std::get<#i_literal>(#cc_ident);
            });
            let converted_value = cc_param_to_c_abi(
                db,
                tuple_element_name.clone(),
                tuple_tys.index(i),
                post_analysis_typing_env,
                includes,
                statements,
            )?;
            if matches!(tuple_tys.index(i).mid().kind(), ty::TyKind::Tuple(_)) {
                // Elements which are arrays must be referenced again in order
                // to properly convert them to pointers.
                //
                // Note that `converted_value` here is a `result_name` array lvalue,
                // never a temporary, so it's fine to take its address in the RHS.
                statements.extend(quote! {
                    auto* #c_abi_name = &#converted_value;
                });
            } else {
                statements.extend(quote! {
                    auto&& #c_abi_name = #converted_value;
                });
            }
        }
        let result_name = expect_format_cc_ident(&format!("{cc_ident}_cabi"));
        statements.extend(quote! {
            void* #result_name[] = { #(&#c_abi_names),* };
        });
        quote! {
            #result_name
        }
    } else if !ty.mid().needs_drop(db.tcx(), post_analysis_typing_env) {
        // As an optimization, if the type is trivially destructible, we don't
        // need to move it to a new NoDestructor location. We can directly copy the
        // bytes.
        quote! { & #cc_ident }
    } else {
        // The implementation will copy the bytes, we just need to leave the variable
        // behind in a valid moved-from state.
        // TODO(jeanpierreda): Ideally, the Rust code should C++-move instead of memcpy,
        // allowing us to avoid one extra memcpy: we could move it directly into its
        // target location, instead of moving to a temporary that we memcpy to its
        // target location.
        includes.insert(db.support_header("internal/slot.h"));
        let slot_name = &expect_format_cc_ident(&format!("{cc_ident}_slot"));
        statements.extend(quote! {
            crubit::Slot #slot_name((std::move(#cc_ident)));
        });
        quote! { #slot_name.Get() }
    })
}

struct ReturnConversion {
    /// The name of a variable holding a pointer to storage of the C-ABI-compatible version of
    /// the return type.
    storage_name: TokenStream,
    /// An expression that unpacks the return value from the storage location.
    unpack_expr: TokenStream,
}

fn format_ty_for_cc_amending_prereqs<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    ty: SugaredTy<'tcx>,
    prereqs: &mut CcPrerequisites,
) -> Result<TokenStream> {
    let CcSnippet { tokens: cc_type, prereqs: ty_prereqs } =
        format_ty_for_cc(db, ty, TypeLocation::Other)?;
    *prereqs += ty_prereqs;
    Ok(cc_type)
}

fn cc_return_value_from_c_abi<'tcx>(
    db: &dyn BindingsGenerator<'tcx>,
    ident: TokenStream,
    ty: SugaredTy<'tcx>,
    prereqs: &mut CcPrerequisites,
    storage_statements: &mut TokenStream,
    recursive: bool,
) -> Result<ReturnConversion> {
    let storage_name = &expect_format_cc_ident(&format!("__{ident}_storage"));
    if let Some(attrs) = is_bridged_type(db, ty.mid())? {
        let cpp_type = format_cc_ident(db, attrs.cpp_type.as_ref())?;
        // Below, we use a union to allocate uninitialized memory that fits cpp_type.
        // The union prevents the type from being default constructed. It's
        // the responsibility of the thunk to properly initialize the
        // memory. In the union's destructor we use std::destroy_at to call
        // the cpp_type's destructor after the value has been moved on return.
        let union_type = expect_format_cc_ident(&format!("__{ident}_crubit_return_union"));
        let local_name = expect_format_cc_ident(&format!("__{ident}_ret_val_holder"));
        storage_statements.extend(quote! {
            union #union_type {
                constexpr #union_type() {}
                ~#union_type() { std::destroy_at(&this->val); }
                #cpp_type val;
            } #local_name;
            auto* #storage_name = &#local_name.val;
        });
        Ok(ReturnConversion {
            storage_name: storage_name.clone(),
            unpack_expr: quote! {
                std::move(#local_name.val)
            },
        })
    } else if is_c_abi_compatible_by_value(ty.mid()) {
        let cc_type = &format_ty_for_cc_amending_prereqs(db, ty, prereqs)?;
        let local_name = &expect_format_cc_ident(&format!("__{ident}_ret_val_holder"));
        storage_statements.extend(quote! {
            #cc_type #local_name;
            #cc_type* #storage_name = &#local_name;
        });
        Ok(ReturnConversion {
            storage_name: storage_name.clone(),
            unpack_expr: quote! { *#storage_name },
        })
    } else if let Some(tuple_tys) = ty.as_tuple(db) {
        let n = tuple_tys.len();
        let mut storage_names = Vec::with_capacity(n);
        let mut unpack_exprs = Vec::with_capacity(n);
        for i in 0..n {
            let tuple_element_ident = expect_format_cc_ident(&format!("{ident}_{i}"));
            let ReturnConversion {
                storage_name: element_storage_name,
                unpack_expr: element_unpack_expr,
            } = cc_return_value_from_c_abi(
                db,
                tuple_element_ident,
                tuple_tys.index(i),
                prereqs,
                storage_statements,
                /*recursive=*/ true,
            )?;
            storage_names.push(element_storage_name);
            unpack_exprs.push(element_unpack_expr);
        }
        storage_statements.extend(quote! {
            void* #storage_name[] = { #(#storage_names),* };
        });
        Ok(ReturnConversion {
            storage_name: storage_name.clone(),
            unpack_expr: quote! { std::make_tuple(#(#unpack_exprs),*) },
        })
    } else {
        if recursive {
            if let Some(adt_def) = ty.mid().ty_adt_def() {
                let core = db.generate_adt_core(adt_def.did())?;
                // Note: the error here is an ApiSnippets which is not propagated.
                db.generate_move_ctor_and_assignment_operator(core).map_err(|_| {
                    anyhow!("Can't return a type by value inside a compound data type without a move constructor")
                })?;
            }
        }
        let local_name = expect_format_cc_ident(&format!("__{ident}_ret_val_holder"));
        let cc_type = format_ty_for_cc_amending_prereqs(db, ty, prereqs)?;
        storage_statements.extend(quote! {
            crubit::Slot<#cc_type> #local_name;
            auto* #storage_name = #local_name.Get();
        });
        prereqs.includes.insert(CcInclude::utility()); // for `std::move`
        prereqs.includes.insert(db.support_header("internal/slot.h"));
        Ok(ReturnConversion {
            storage_name: storage_name.clone(),
            unpack_expr: quote! {
                std::move(#local_name).AssumeInitAndTakeValue()
            },
        })
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
    let export_name: Option<Symbol> = tcx
        .get_attr(def_id, rustc_span::symbol::sym::export_name)
        .map(|attr| attr.value_str().expect("export_name is a string"));
    let has_export_name = export_name.is_some();
    let has_no_mangle = tcx.get_attr(def_id, rustc_span::symbol::sym::no_mangle).is_some();
    let needs_thunk = is_thunk_required(&sig_mid).is_err() || (!has_no_mangle && !has_export_name);
    let thunk_name = thunk_name(db, def_id, export_name, needs_thunk);

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
        ty: SugaredTy<'tcx>,
    }
    let params = {
        let names = tcx.fn_arg_names(def_id).iter();
        let cpp_types =
            format_param_types_for_cc(db, &sig_mid, Some(sig_hir), AllowReferences::Safe)?;
        names
            .enumerate()
            .zip(SugaredTy::fn_inputs(&sig_mid, Some(sig_hir)))
            .zip(cpp_types)
            .map(|(((i, name), ty), cpp_type)| {
                let mut cc_name = format_cc_ident(db, ident_or_opt_ident(name).as_str())
                    .unwrap_or_else(|_err| expect_format_cc_ident(&format!("__param_{i}")));
                if ident_or_opt_ident(name).as_str() == "_" {
                    cc_name = expect_format_cc_ident(&format!("__param_{i}"));
                }
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
            Some(arg_name) if ident_or_opt_ident(arg_name).name == kw::SelfLower => {
                let self_ty = self_ty.expect("ImplItem => non-None `self_ty`");
                if params[0].ty.mid() == self_ty {
                    FunctionKind::MethodTakingSelfByValue
                } else {
                    match params[0].ty.mid().kind() {
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
        FunctionKind::MethodTakingSelfByRef => match params[0].ty.mid().kind() {
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

        let mut statements = TokenStream::new();
        let mut thunk_args = params
            .iter()
            .enumerate()
            .map(|(i, Param { cc_name, ty, .. })| {
                if i == 0 && method_kind.has_self_param() {
                    statements.extend(quote! { auto&& #cc_name = *this; });
                }
                cc_param_to_c_abi(
                    db,
                    cc_name.clone(),
                    *ty,
                    post_analysis_typing_env(tcx, def_id),
                    &mut prereqs.includes,
                    &mut statements,
                )
            })
            .collect::<Result<Vec<TokenStream>>>()?;

        let rs_return_type = SugaredTy::fn_output(&sig_mid, Some(sig_hir));
        let impl_body: TokenStream = if is_bridged_type(db, rs_return_type.mid())?.is_none()
            && is_c_abi_compatible_by_value(rs_return_type.mid())
        {
            quote! {
                return __crubit_internal::#thunk_name(#( #thunk_args ),*);
            }
        } else {
            let ReturnConversion { storage_name, unpack_expr } = cc_return_value_from_c_abi(
                db,
                expect_format_cc_ident("return_value"),
                rs_return_type,
                &mut prereqs,
                &mut statements,
                /*recursive=*/ false,
            )?;
            thunk_args.push(quote! { #storage_name });
            quote! {
                __crubit_internal::#thunk_name(#( #thunk_args ),*);
                return #unpack_expr;
            }
        };

        CcSnippet {
            prereqs,
            tokens: quote! {
                __NEWLINE__
                #thunk_decl
                inline #main_api_ret_type #struct_name #main_api_fn_name (
                        #( #main_api_params ),* ) #method_qualifiers {
                    #statements
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

#[cfg(test)]
pub mod tests {
    use crate::tests::*;
    use code_gen_utils::format_cc_includes;
    use quote::quote;
    use token_stream_matchers::{assert_cc_matches, assert_rs_matches};

    /// `test_generated_bindings_fn_export_name` covers a scenario where
    /// `MixedSnippet::cc` is present but `MixedSnippet::rs` is empty
    /// (because no Rust thunks are needed).
    #[test]
    fn test_generated_bindings_fn_export_name() {
        let test_src = r#"
                #[unsafe(export_name = "export_name")]
                pub extern "C" fn public_function(x: f64, y: f64) -> f64 { x + y }
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            assert_cc_matches!(
                bindings.cc_api,
                quote! {
                    namespace rust_out {
                        ...
                        double public_function(double x, double y);
                        namespace __crubit_internal {
                            extern "C" double export_name(double, double);
                        }
                        inline double public_function(double x, double y) {
                            return __crubit_internal::export_name(x, y);
                        }
                    }
                }
            );
        });
    }

    /// The `test_generated_bindings_impl` test covers only a single example of
    /// a non-trait `impl`. Additional coverage of how items are formatted
    /// should be provided in the future by `test_format_item_...` tests.
    ///
    /// We don't want to duplicate coverage already provided by
    /// `test_format_item_static_method`, but we do want to verify that
    /// * `generate_crate` won't process the `impl` as a standalone HIR item
    /// * The actual shape of the bindings still looks okay at this level.
    #[test]
    fn test_generated_bindings_impl() {
        let test_src = r#"
                #![allow(dead_code)]

                pub struct SomeStruct(i32);

                impl SomeStruct {
                    pub fn public_static_method() -> i32 { 123 }

                    fn private_static_method() -> i32 { 123 }
                }
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            assert_cc_matches!(
                bindings.cc_api,
                quote! {
                    namespace rust_out {
                        ...
                        struct ... SomeStruct ... {
                            // No point replicating test coverage of
                            // `test_format_item_static_method`.
                            ...
                            std::int32_t public_static_method();
                            ...
                        };
                        ...
                        std::int32_t SomeStruct::public_static_method() {
                            ...
                        }
                        ...
                    }  // namespace rust_out
                }
            );
            assert_rs_matches!(
                bindings.cc_api_impl,
                quote! {
                    unsafe extern "C" fn ...() -> i32 {
                        unsafe { ::rust_out::SomeStruct::public_static_method() }
                    }
                }
            );
        });
    }

    #[test]
    fn test_generated_bindings_includes() {
        let test_src = r#"
                #[unsafe(no_mangle)]
                pub extern "C" fn public_function(i: i32, d: isize, u: u64) {
                    dbg!(i);
                    dbg!(d);
                    dbg!(u);
                }
            "#;
        test_generated_bindings(test_src, |bindings| {
            let bindings = bindings.unwrap();
            assert_cc_matches!(
                bindings.cc_api,
                quote! {
                    __HASH_TOKEN__ include <cstdint> ...
                    namespace ... {
                        ...
                        extern "C" void public_function(
                            std::int32_t i,
                            std::intptr_t d,
                            std::uint64_t u);
                    }
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_extern_c_no_mangle_no_params_no_return_type() {
        let test_src = r#"
                #[unsafe(no_mangle)]
                pub extern "C" fn public_function() {}
            "#;
        test_format_item(test_src, "public_function", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    extern "C" void public_function();
                }
            );

            // Sufficient to just re-declare the Rust API in C++.
            // (i.e. there is no need to have a C++-side definition of `public_function`).
            assert!(result.cc_details.tokens.is_empty());

            // There is no need to have a separate thunk for an `extern "C"` function.
            assert!(result.rs_details.tokens.is_empty());
        });
    }

    /// The `test_format_item_fn_explicit_unit_return_type` test below is very
    /// similar to the
    /// `test_format_item_fn_extern_c_no_mangle_no_params_no_return_type` above,
    /// except that the return type is explicitly spelled out.  There is no
    /// difference in `ty::FnSig` so our code behaves exactly the same, but the
    /// test has been planned based on earlier, hir-focused approach and having
    /// this extra test coverage shouldn't hurt. (`hir::FnSig`
    /// and `hir::FnRetTy` _would_ see a difference between the two tests, even
    /// though there is no different in the current `bindings.rs` code).
    #[test]
    fn test_format_item_fn_explicit_unit_return_type() {
        let test_src = r#"
                #[unsafe(no_mangle)]
                pub extern "C" fn explicit_unit_return_type() -> () {}
            "#;
        test_format_item(test_src, "explicit_unit_return_type", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    extern "C" void explicit_unit_return_type();
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_never_return_type() {
        let test_src = r#"
                #[unsafe(no_mangle)]
                pub extern "C" fn never_returning_function() -> ! {
                    panic!("This function panics and therefore never returns");
                }
            "#;
        test_format_item(test_src, "never_returning_function", |result| {
            // TODO(b/254507801): The function should be annotated with the `[[noreturn]]`
            // attribute.
            // TODO(b/254507801): Expect `crubit::Never` instead (see the bug for more
            // details).
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    extern "C" void never_returning_function();
                }
            );
        })
    }

    /// `test_format_item_fn_mangling` checks that bindings can be generated for
    /// `extern "C"` functions that do *not* have `#[unsafe(no_mangle)]`
    /// attribute.  The test elides away the mangled name in the
    /// `assert_cc_matches` checks below, but end-to-end test coverage
    /// should eventually be provided by `test/functions` (see b/262904507).
    #[test]
    fn test_format_item_fn_mangling() {
        let test_src = r#"
                pub extern "C" fn public_function(x: f64, y: f64) -> f64 { x + y }
            "#;
        test_format_item(test_src, "public_function", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    double public_function(double x, double y);
                }
            );
            // TODO(b/262904507): omit the thunk and uncomment the next line.
            // assert!(result.rs_details.tokens.is_empty());
            assert!(result.cc_details.prereqs.is_empty());
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                        extern "C" double ...(double, double);
                    }
                    ...
                    inline double public_function(double x, double y) {
                        return __crubit_internal::...(x, y);
                    }
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_export_name() {
        let test_src = r#"
                #[unsafe(export_name = "export_name")]
                pub extern "C" fn public_function(x: f64, y: f64) -> f64 { x + y }
            "#;
        test_format_item(test_src, "public_function", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    double public_function(double x, double y);
                }
            );

            // There is no need to have a separate thunk for an `extern "C"` function.
            assert!(result.rs_details.tokens.is_empty());

            // We generate a C++-side definition of `public_function` so that we
            // can call a differently-named (but same-signature) `export_name` function.
            assert!(result.cc_details.prereqs.is_empty());
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                        extern "C" double export_name(double, double);
                    }
                    ...
                    inline double public_function(double x, double y) {
                        return __crubit_internal::export_name(x, y);
                    }
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_extern_c_unsafe() {
        let test_src = r#"
                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn foo() {}
            "#;
        test_format_item(test_src, "foo", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    void foo();
                }
            );
            assert!(result.rs_details.tokens.is_empty());
        });
    }

    /// For non-extern "C" unsafe functions, we need a thunk, and it needs some
    /// `unsafe`.
    ///
    /// The thunk itself needs to be unsafe, because it wraps an unsafe function
    /// and is still in-principle itself directly callable. It also needs to
    /// have an unsafe block inside of it due to RFC #2585
    /// `unsafe_block_in_unsafe_fn`.
    #[test]
    fn test_format_item_fn_unsafe() {
        let test_src = r#"
                #[unsafe(no_mangle)]
                pub unsafe fn foo() {}
            "#;
        test_format_item(test_src, "foo", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    void foo();
                }
            );
            assert_cc_matches!(
                result.rs_details.tokens,
                quote! {
                    #[unsafe(no_mangle)]
                    unsafe extern "C" fn __crubit_thunk_foo() -> () {
                        unsafe { ::rust_out::foo() }
                    }
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_references() {
        let test_src = r#"
                #[unsafe(no_mangle)]
                pub fn foo(_x: &i32, _y: &i32) {}
            "#;
        test_format_item_with_features(
            test_src,
            "foo",
            <flagset::FlagSet<crubit_feature::CrubitFeature>>::default(),
            |result| {
                let main_api = result.unwrap().unwrap().main_api;
                assert_cc_matches!(
                    main_api.tokens,
                    quote! {
                        void foo(
                            std::int32_t const& [[clang::annotate_type("lifetime" , "__anon1")]] _x,
                            std::int32_t const& [[clang::annotate_type("lifetime" , "__anon2")]] _y );
                    }
                );
            },
        );
    }

    #[test]
    fn test_format_item_fn_risky_mut_reference() {
        let test_src = r#"
                #[unsafe(no_mangle)]
                pub fn foo(_x: &mut i32, _y: &i32) {}
            "#;
        test_format_item_with_features(
            test_src,
            "foo",
            <flagset::FlagSet<crubit_feature::CrubitFeature>>::default(),
            |result| {
                assert_eq!(
                    result.unwrap_err(),
                    "support for functions taking a mutable reference, and which may alias in C++, requires //features:experimental"
                )
            },
        );
    }

    #[test]
    fn test_format_item_fn_static_reference() {
        let test_src = r#"
                #[unsafe(no_mangle)]
                pub fn foo(_x: &'static i32) {}
            "#;
        test_format_item_with_features(
            test_src,
            "foo",
            <flagset::FlagSet<crubit_feature::CrubitFeature>>::default(),
            |result| {
                assert_eq!(
                    result.unwrap_err(),
                    "support for bound reference lifetimes (such as 'static) requires //features:experimental"
                )
            },
        );
    }

    // NOTE: If we gain support for references as non-parameter types, we must
    // _still_ require :experimental.
    #[test]
    fn test_format_item_fn_nested_reference() {
        let test_src = r#"
                #[unsafe(no_mangle)]
                pub fn foo(_x: &&i32) {}
            "#;
        test_format_item_with_features(
            test_src,
            "foo",
            <flagset::FlagSet<crubit_feature::CrubitFeature>>::default(),
            |result| {
                assert_eq!(
                    result.unwrap_err(),
                    "Error handling parameter #0 of type `&'__anon1 &'__anon2 i32`: Failed to format the referent of the reference type `&'__anon1 &'__anon2 i32`: Can't format `&'__anon2 i32`, because references are only supported in function parameter types and return types (b/286256327)"
                )
            },
        );
    }

    #[test]
    fn test_format_item_fn_returned_static_reference() {
        let test_src = r#"
                #[unsafe(no_mangle)]
                pub fn foo() -> &'static i32 {todo!()}
            "#;
        test_format_item_with_features(
            test_src,
            "foo",
            <flagset::FlagSet<crubit_feature::CrubitFeature>>::default(),
            |result| {
                assert_eq!(
                    result.unwrap_err(),
                    "support for references of non-function-param types requires //features:experimental"
                )
            },
        );
    }

    #[test]
    fn test_format_item_fn_reused_reference_lifetime() {
        let test_src = r#"
                #[unsafe(no_mangle)]
                pub fn foo<'a>(_x: &'a i32, _y: &'a i32) {}
            "#;
        test_format_item_with_features(
            test_src,
            "foo",
            <flagset::FlagSet<crubit_feature::CrubitFeature>>::default(),
            |result| {
                assert_eq!(
                    result.unwrap_err(),
                    "support for multiple uses of a lifetime parameter requires //features:experimental"
                )
            },
        );
    }

    // NOTE: If we gain support for lifetime generic parameters, we must _still_
    // require :experimental.
    #[test]
    fn test_format_item_fn_reused_reference_lifetime_struct() {
        let test_src = r#"
                pub struct Foo<'a>(pub i32, core::marker::PhantomData<&'a i32>);
                #[unsafe(no_mangle)]
                pub fn foo<'a>(_x: &'a Foo<'a>) {}
            "#;
        test_format_item_with_features(
            test_src,
            "foo",
            <flagset::FlagSet<crubit_feature::CrubitFeature>>::default(),
            |result| {
                assert_eq!(
                    result.unwrap_err(),
                    "Error handling parameter #0 of type `&'a Foo<'a>`: Failed to format the referent of the reference type `&'a Foo<'a>`: Generic types are not supported yet (b/259749095)"
                )
            },
        );
    }

    #[test]
    fn test_format_item_fn_char() {
        let test_src = r#"
                #[unsafe(no_mangle)]
                pub fn foo(_x: char) {}
            "#;
        test_format_item_with_features(
            test_src,
            "foo",
            <flagset::FlagSet<crubit_feature::CrubitFeature>>::default(),
            |result| {
                assert_eq!(
                    result.unwrap_err(),
                    "support for the Rust `char` type requires //features:experimental"
                )
            },
        );
    }

    #[test]
    fn test_format_fn_cpp_name() {
        let test_src = r#"
                #[doc="CRUBIT_ANNOTATE: cpp_name=Create"]
                pub fn foo() {}
            "#;
        test_format_item(test_src, "foo", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());

            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    #[unsafe(no_mangle)]
                    unsafe extern "C" fn __crubit_thunk_foo() -> () {
                         unsafe { ::rust_out::foo() }
                    }
                }
            );

            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    void Create();
                }
            );
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                        extern "C" void __crubit_thunk_foo();
                    }
                    ...
                    inline void Create() {
                        return __crubit_internal::__crubit_thunk_foo();
                    }
                }
            );
        });
    }

    /// `test_format_item_fn_const` tests how bindings for an `const fn` are
    /// generated.
    ///
    /// Right now the `const` qualifier is ignored, but one can imagine that in
    /// the (very) long-term future such functions (including their bodies)
    /// could be translated into C++ `consteval` functions.
    #[test]
    fn test_format_item_fn_const() {
        let test_src = r#"
                pub const fn foo(i: i32) -> i32 { i * 42 }
            "#;
        test_format_item(test_src, "foo", |result| {
            // TODO(b/254095787): Update test expectations below once `const fn` from Rust
            // is translated into a `consteval` C++ function.
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    std::int32_t foo(std::int32_t i);
                }
            );
            assert!(!result.cc_details.prereqs.is_empty());
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                        extern "C" std::int32_t ...( std::int32_t);
                    }
                    ...
                    inline std::int32_t foo(std::int32_t i) {
                        return __crubit_internal::...(i);
                    }
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    #[unsafe(no_mangle)]
                    unsafe extern "C"
                    fn ...(i: i32) -> i32 {
                        unsafe { ::rust_out::foo(i) }
                    }
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_with_c_unwind_abi() {
        // See also https://rust-lang.github.io/rfcs/2945-c-unwind-abi.html
        let test_src = r#"
                #![feature(c_unwind)]

                #[unsafe(no_mangle)]
                pub extern "C-unwind" fn may_throw() {}
            "#;
        test_format_item(test_src, "may_throw", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    extern "C" void may_throw();
                }
            );
        });
    }

    /// This test mainly verifies that `generate_item` correctly propagates
    /// `CcPrerequisites` of parameter types and return type.
    #[test]
    fn test_format_item_fn_cc_prerequisites_if_cpp_definition_needed() {
        let test_src = r#"
                #![allow(dead_code)]

                pub fn foo(_i: i32) -> S { panic!("foo") }
                pub struct S(i32);
            "#;
        test_format_item(test_src, "foo", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;

            // Minimal coverage, just to double-check that the test setup works.
            //
            // Note that this is a definition, and therefore `S` should be defined
            // earlier (not just forward declared).
            assert_cc_matches!(main_api.tokens, quote! { S foo(std::int32_t _i);});
            assert_cc_matches!(result.cc_details.tokens, quote! { S foo(std::int32_t _i) { ... }});

            // Main checks: `CcPrerequisites::includes`.
            assert_cc_matches!(
                format_cc_includes(&main_api.prereqs.includes),
                quote! { include <cstdint> }
            );
            assert_cc_matches!(
                format_cc_includes(&result.cc_details.prereqs.includes),
                quote! { include <cstdint> }
            );

            // Main checks: `CcPrerequisites::defs` and `CcPrerequisites::fwd_decls`.
            //
            // Verifying the actual def_id is tricky, because `test_format_item` doesn't
            // expose `tcx` to the verification function (and therefore calling
            // `find_def_id_by_name` is not easily possible).
            //
            // Note that `main_api` and `impl_details` have different expectations.
            assert_eq!(0, main_api.prereqs.defs.len());
            assert_eq!(1, main_api.prereqs.fwd_decls.len());
            assert_eq!(1, result.cc_details.prereqs.defs.len());
            assert_eq!(0, result.cc_details.prereqs.fwd_decls.len());
        });
    }

    /// This test verifies that `generate_item` uses
    /// `CcPrerequisites::fwd_decls` rather than `CcPrerequisites::defs` for
    /// function declarations in the `main_api`.
    #[test]
    fn test_format_item_fn_cc_prerequisites_if_only_cpp_declaration_needed() {
        let test_src = r#"
                #[unsafe(no_mangle)]
                pub extern "C" fn foo(s: S) -> bool { s.0 }

                #[repr(C)]
                pub struct S(bool);
            "#;
        test_format_item(test_src, "foo", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;

            // Minimal coverage, just to double-check that the test setup works.
            //
            // Note that this is only a function *declaration* (not a function definition -
            // there is no function body), and therefore `S` just needs to be
            // forward-declared earlier.
            assert_cc_matches!(main_api.tokens, quote! { bool foo(::rust_out::S s); });

            // Main checks: `CcPrerequisites::defs` and `CcPrerequisites::fwd_decls`.
            //
            // Verifying the actual def_id is tricky, because `test_format_item` doesn't
            // expose `tcx` to the verification function (and therefore calling
            // `find_def_id_by_name` is not easily possible).
            assert_eq!(0, main_api.prereqs.defs.len());
            assert_eq!(1, main_api.prereqs.fwd_decls.len());
        });
    }

    #[test]
    fn test_format_item_fn_with_type_aliased_return_type() {
        // Type aliases disappear at the `rustc_middle::ty::Ty` level and therefore in
        // the short-term the generated bindings also ignore type aliases.
        //
        // TODO(b/254096006): Consider preserving `type` aliases when generating
        // bindings.
        let test_src = r#"
                type MyTypeAlias = f64;

                #[unsafe(no_mangle)]
                pub extern "C" fn type_aliased_return() -> MyTypeAlias { 42.0 }
            "#;
        test_format_item(test_src, "type_aliased_return", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    extern "C" double type_aliased_return();
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_with_doc_comment_with_unmangled_name() {
        let test_src = r#"
            /// Outer line doc.
            /** Outer block doc that spans lines.
             */
            #[doc = "Doc comment via doc attribute."]
            #[unsafe(no_mangle)]
            pub extern "C" fn fn_with_doc_comment_with_unmangled_name() {}
          "#;
        test_format_item(test_src, "fn_with_doc_comment_with_unmangled_name", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());
            let doc_comments = [
                " Outer line doc.",
                "",
                " Outer block doc that spans lines.",
                "             ",
                "",
                "Doc comment via doc attribute.",
                "",
                "Generated from: <crubit_unittests.rs>;l=7",
            ]
            .join("\n");
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    __COMMENT__ #doc_comments
                    extern "C" void fn_with_doc_comment_with_unmangled_name();
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_with_inner_doc_comment_with_unmangled_name() {
        let test_src = r#"
            /// Outer doc comment.
            #[unsafe(no_mangle)]
            pub extern "C" fn fn_with_inner_doc_comment_with_unmangled_name() {
                //! Inner doc comment.
            }
          "#;
        test_format_item(test_src, "fn_with_inner_doc_comment_with_unmangled_name", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());
            let doc_comments = [
                " Outer doc comment.",
                " Inner doc comment.",
                "Generated from: <crubit_unittests.rs>;l=4",
            ]
            .join("\n\n");
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    __COMMENT__ #doc_comments
                    extern "C" void fn_with_inner_doc_comment_with_unmangled_name();
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_with_doc_comment_with_mangled_name() {
        let test_src = r#"
                /// Doc comment of a function with mangled name.
                pub extern "C" fn fn_with_doc_comment_with_mangled_name() {}
            "#;
        test_format_item(test_src, "fn_with_doc_comment_with_mangled_name", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());
            let comment = " Doc comment of a function with mangled name.\n\n\
                           Generated from: <crubit_unittests.rs>;l=3";
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    __COMMENT__ #comment
                    void fn_with_doc_comment_with_mangled_name();
                }
            );
        });
    }

    #[test]
    fn test_format_item_unsupported_fn_name_is_reserved_cpp_keyword() {
        let test_src = r#"
                #[unsafe(no_mangle)]
                pub extern "C" fn reinterpret_cast() -> () {}
            "#;
        test_format_item(test_src, "reinterpret_cast", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    extern "C" void reinterpret_cast_();
                }
            );
        });
    }

    /// This test verifies handling of inferred, anonymous lifetimes.
    ///
    /// Note that `Region::get_name_or_anon()` may return the same name (e.g.
    /// `"anon"` for both lifetimes, but bindings should use 2 distinct
    /// lifetime names in the generated bindings and in the thunk impl.
    #[test]
    fn test_format_item_lifetime_generic_fn_with_inferred_lifetimes() {
        let test_src = r#"
                pub fn foo(arg: &i32) -> &i32 {
                    unimplemented!("arg = {arg}")
                }
            "#;
        test_format_item(test_src, "foo", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    std::int32_t const& [[clang::annotate_type("lifetime", "__anon1")]]
                    foo(std::int32_t const& [[clang::annotate_type("lifetime", "__anon1")]] arg);
                }
            );
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                    extern "C"
                    std::int32_t const& [[clang::annotate_type("lifetime", "__anon1")]] ...(
                        std::int32_t const& [[clang::annotate_type("lifetime", "__anon1")]]);
                    }
                    inline
                    std::int32_t const& [[clang::annotate_type("lifetime", "__anon1")]]
                    foo(std::int32_t const& [[clang::annotate_type("lifetime", "__anon1")]] arg) {
                      return __crubit_internal::...(arg);
                    }
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    #[unsafe(no_mangle)]
                    unsafe extern "C" fn ...<'__anon1>(arg: &'__anon1 i32) -> &'__anon1 i32 {
                        unsafe { ::rust_out::foo(arg) }
                    }
                }
            );
        });
    }

    /// This test verifies handling of various explicit (i.e. non-inferred)
    /// lifetimes.
    ///
    /// * Note that the two `'_` specify two distinct lifetimes (i.e. two
    ///   distinct names need to be used in the generated bindings and thunk
    ///   impl).
    /// * Note that `'static` doesn't need to be listed in the generic
    ///   parameters of the thunk impl
    /// * Note that even though `'foo` is used in 2 parameter types, it should
    ///   only appear once in the list of generic parameters of the thunk impl
    /// * Note that in the future the following translation may be preferable:
    ///     * `'a` => `$a` (no parens)
    ///     * `'foo` => `$(foo)` (note the extra parens)
    #[test]
    fn test_format_item_lifetime_generic_fn_with_various_lifetimes() {
        let test_src = r#"
                pub fn foo<'a, 'foo>(
                    arg1: &'a i32,  // Single letter lifetime = `$a` is possible
                    arg2: &'foo i32,  // Multi-character lifetime
                    arg3: &'foo i32,  // Same lifetime used for 2 places
                    arg4: &'static i32,
                    arg5: &'_ i32,
                    arg6: &'_ i32,
                ) -> &'foo i32 {
                    unimplemented!("args: {arg1}, {arg2}, {arg3}, {arg4}, {arg5}, {arg6}")
                }
            "#;
        test_format_item(test_src, "foo", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                  std::int32_t const& [[clang::annotate_type("lifetime", "foo")]]
                  foo(
                    std::int32_t const& [[clang::annotate_type("lifetime", "a")]] arg1,
                    std::int32_t const& [[clang::annotate_type("lifetime", "foo")]] arg2,
                    std::int32_t const& [[clang::annotate_type("lifetime", "foo")]] arg3,
                    std::int32_t const& [[clang::annotate_type("lifetime", "static")]] arg4,
                    std::int32_t const& [[clang::annotate_type("lifetime", "__anon1")]] arg5,
                    std::int32_t const& [[clang::annotate_type("lifetime", "__anon2")]] arg6);
                }
            );
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                    extern "C"
                    std::int32_t const& [[clang::annotate_type("lifetime", "foo")]]
                    ...(
                        std::int32_t const& [[clang::annotate_type("lifetime", "a")]],
                        std::int32_t const& [[clang::annotate_type("lifetime", "foo")]],
                        std::int32_t const& [[clang::annotate_type("lifetime", "foo")]],
                        std::int32_t const& [[clang::annotate_type("lifetime", "static")]],
                        std::int32_t const& [[clang::annotate_type("lifetime", "__anon1")]],
                        std::int32_t const& [[clang::annotate_type("lifetime", "__anon2")]]);
                    }
                    inline
                    std::int32_t const& [[clang::annotate_type("lifetime", "foo")]]
                    foo(
                        std::int32_t const& [[clang::annotate_type("lifetime", "a")]] arg1,
                        std::int32_t const& [[clang::annotate_type("lifetime", "foo")]] arg2,
                        std::int32_t const& [[clang::annotate_type("lifetime", "foo")]] arg3,
                        std::int32_t const& [[clang::annotate_type("lifetime", "static")]] arg4,
                        std::int32_t const& [[clang::annotate_type("lifetime", "__anon1")]] arg5,
                        std::int32_t const& [[clang::annotate_type("lifetime", "__anon2")]] arg6) {
                      return __crubit_internal::...(arg1, arg2, arg3, arg4, arg5, arg6);
                    }
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    #[unsafe(no_mangle)]
                    unsafe extern "C" fn ...<'__anon1, '__anon2, 'a, 'foo>(
                        arg1: &'a i32,
                        arg2: &'foo i32,
                        arg3: &'foo i32,
                        arg4: &'static i32,
                        arg5: &'__anon1 i32,
                        arg6: &'__anon2 i32
                    ) -> &'foo i32 {
                        unsafe { ::rust_out::foo(arg1, arg2, arg3, arg4, arg5, arg6) }
                    }
                }
            );
        });
    }

    /// Test of lifetime-generic function with a `where` clause.
    ///
    /// The `where` constraint below is a bit silly (why not just use `'static`
    /// directly), but it seems prudent to test and confirm that we disable
    /// generation of bindings for generic functions with `where` clauses
    /// (because it is unclear if such constraints can be replicated
    /// in C++).
    #[test]
    fn test_format_item_lifetime_generic_fn_with_where_clause() {
        let test_src = r#"
                pub fn foo<'a>(arg: &'a i32) where 'a : 'static {
                    unimplemented!("{arg}")
                }
            "#;
        test_format_item(test_src, "foo", |result| {
            let err = result.unwrap_err();
            assert_eq!(err, "Generic functions are not supported yet (b/259749023)");
        });
    }

    #[test]
    fn test_format_item_unsupported_type_generic_fn() {
        let test_src = r#"
                use std::fmt::Display;
                pub fn generic_function<T: Default + Display>() {
                    println!("{}", T::default());
                }
            "#;
        test_format_item(test_src, "generic_function", |result| {
            let err = result.unwrap_err();
            assert_eq!(err, "Generic functions are not supported yet (b/259749023)");
        });
    }

    #[test]
    fn test_format_item_unsupported_fn_async() {
        let test_src = r#"
                pub async fn async_function() {}
            "#;
        test_format_item(test_src, "async_function", |result| {
            let err = result.unwrap_err();
            assert_eq!(
                err,
                "Error formatting function return type `impl std::future::Future<Output = ()>`: \
                             The following Rust type is not supported yet: \
                             impl std::future::Future<Output = ()>"
            );
        });
    }

    #[test]
    fn test_format_item_fn_rust_abi() {
        let test_src = r#"
                pub fn add(x: f64, y: f64) -> f64 { x * y }
            "#;
        test_format_item(test_src, "add", |result| {
            // TODO(b/261074843): Re-add thunk name verification once we are using stable
            // name mangling (which may be coming in Q1 2023).  (This might mean
            // reverting cl/492333432 + manual review and tweaks.)
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    double add(double x, double y);
                }
            );
            assert!(result.cc_details.prereqs.is_empty());
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                        extern "C" double ...(double, double);
                    }
                    ...
                    inline double add(double x, double y) {
                        return __crubit_internal::...(x, y);
                    }
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    #[unsafe(no_mangle)]
                    unsafe extern "C"
                    fn ...(x: f64, y: f64) -> f64 {
                        unsafe { ::rust_out::add(x, y) }
                    }
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_rust_abi_with_param_taking_struct_by_value22() {
        let test_src = r#"
                use std::slice;
                pub struct S(i32);
                pub unsafe fn transmute_slice(
                    slice_ptr: *const u8,
                    slice_len: usize,
                    element_size: usize,
                    s: S,
                ) -> i32 {
                    let len_in_bytes = slice_len * element_size;
                    let b = slice::from_raw_parts(slice_ptr as *const u8, len_in_bytes);
                    if b.len() == len_in_bytes {
                        s.0
                    } else {
                        0
                    }
                }
            "#;
        test_format_item(test_src, "transmute_slice", |result| {
            let result = result.unwrap().unwrap();
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    #[unsafe(no_mangle)]
                    unsafe extern "C"
                    fn ...(...) -> i32 {
                        unsafe {
                            let s = s.assume_init_read();
                            ::rust_out::transmute_slice(..., ..., ..., s)
                        }
                    }
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_rust_abi_with_param_taking_struct_by_value() {
        let test_src = r#"
                pub struct S(i32);
                pub fn into_i32(s: S) -> i32 { s.0 }
            "#;
        test_format_item(test_src, "into_i32", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    std::int32_t into_i32(::rust_out::S s);
                }
            );
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                        extern "C" std::int32_t ...(::rust_out::S*);
                    }
                    ...
                    inline std::int32_t into_i32(::rust_out::S s) {
                        return __crubit_internal::...(&s);
                    }
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    #[unsafe(no_mangle)]
                    unsafe extern "C"
                    fn ...(s: &mut ::core::mem::MaybeUninit<::rust_out::S>) -> i32 {
                        unsafe {
                            let s = s.assume_init_read();
                            ::rust_out::into_i32(s)
                        }
                    }
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_rust_abi_returning_struct_by_value() {
        let test_src = r#"
                #![allow(dead_code)]

                pub struct S(i32);
                pub fn create(i: i32) -> S { S(i) }
            "#;
        test_format_item(test_src, "create", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ::rust_out::S create(std::int32_t i);
                }
            );
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                        extern "C" void ...(std::int32_t, ::rust_out::S* __ret_ptr);
                    }
                    ...
                    inline ::rust_out::S create(std::int32_t i) {
                        crubit::Slot<::rust_out::S> __return_value_ret_val_holder;
                        auto* __return_value_storage = __return_value_ret_val_holder.Get();
                        __crubit_internal::...(i, __return_value_storage);
                        return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
                    }
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    #[unsafe(no_mangle)]
                    unsafe extern "C"
                    fn ...(
                        i: i32,
                        __ret_ptr: *mut core::ffi::c_void
                    ) -> () {
                        unsafe {
                            let __rs_return_value = ::rust_out::create(i);
                            (__ret_ptr as *mut ::rust_out::S).write(__rs_return_value);
                        }
                    }
                }
            );
        });
    }

    /// `test_format_item_fn_rust_abi` tests a function call that is not a
    /// C-ABI, and is not the default Rust ABI.  It can't use `"stdcall"`,
    /// because it is not supported on the targets where Crubit's tests run.
    /// So, it ended up using `"vectorcall"`.
    ///
    /// This test almost entirely replicates `test_format_item_fn_rust_abi`,
    /// except for the `extern "vectorcall"` part in the `test_src` test
    /// input.
    ///
    /// This test verifies the current behavior that gives reasonable and
    /// functional FFI bindings.  OTOH, in the future we may decide to avoid
    /// having the extra thunk for cases where the given non-C-ABI function
    /// call convention is supported by both C++ and Rust
    /// (see also `format_cc_call_conv_as_clang_attribute` in
    /// `rs_bindings_from_cc/src_code_gen.rs`)
    #[test]
    fn test_format_item_fn_vectorcall_abi() {
        let test_src = r#"
                #![feature(abi_vectorcall)]
                pub extern "vectorcall" fn add(x: f64, y: f64) -> f64 { x * y }
            "#;
        test_format_item(test_src, "add", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    double add(double x, double y);
                }
            );
            assert!(result.cc_details.prereqs.is_empty());
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                        extern "C" double ...(double, double);
                    }
                    ...
                    inline double add(double x, double y) {
                        return __crubit_internal::...(x, y);
                    }
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    #[unsafe(no_mangle)]
                    unsafe extern "C"
                    fn ...(x: f64, y: f64) -> f64 {
                        unsafe { ::rust_out::add(x, y) }
                    }
                }
            );
        });
    }

    #[test]
    fn test_format_item_unsupported_fn_variadic() {
        let test_src = r#"
                #![feature(c_variadic)]

                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn variadic_function(_fmt: *const u8, ...) {}
            "#;
        test_format_item(test_src, "variadic_function", |result| {
            // TODO(b/254097223): Add support for variadic functions.
            let err = result.unwrap_err();
            assert_eq!(err, "C variadic functions are not supported (b/254097223)");
        });
    }

    #[test]
    fn test_format_item_fn_params() {
        let test_src = r#"
                #[allow(unused_variables)]
                #[unsafe(no_mangle)]
                pub extern "C" fn foo(b: bool, f: f64) {}
            "#;
        test_format_item(test_src, "foo", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    extern "C" void foo(bool b, double f);
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_param_name_reserved_keyword() {
        let test_src = r#"
                #[allow(unused_variables)]
                #[unsafe(no_mangle)]
                pub extern "C" fn some_function(reinterpret_cast: f64) {}
            "#;
        test_format_item(test_src, "some_function", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    ...
                    extern "C" void some_function(double reinterpret_cast_);
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_with_multiple_anonymous_parameter_names() {
        let test_src = r#"
                pub fn foo(_: f64, _: f64) {}
            "#;
        test_format_item(test_src, "foo", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    void foo(double __param_0, double __param_1);
                }
            );
            assert!(result.cc_details.prereqs.is_empty());
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                        extern "C" void ...(double, double);
                    }
                    ...
                    inline void foo(double __param_0, double __param_1) {
                        return __crubit_internal::...(__param_0, __param_1);
                    }
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    #[unsafe(no_mangle)]
                    unsafe extern "C" fn ...(__param_0: f64, __param_1: f64) -> () {
                        unsafe { ::rust_out::foo(__param_0, __param_1) }
                    }
                }
            );
        });
    }

    #[test]
    fn test_format_item_fn_with_destructuring_parameter_name() {
        let test_src = r#"
                pub struct S {
                    pub f1: i32,
                    pub f2: i32,
                }

                // This test mostly focuses on the weird parameter "name" below.
                // See also
                // https://doc.rust-lang.org/reference/items/functions.html#function-parameters
                // which points out that function parameters are just irrefutable patterns.
                pub fn func(S{f1, f2}: S) -> i32 { f1 + f2 }
            "#;
        test_format_item(test_src, "func", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    std::int32_t func(::rust_out::S __param_0);
                }
            );
            assert_cc_matches!(
                result.cc_details.tokens,
                quote! {
                    namespace __crubit_internal {
                        extern "C" std::int32_t ...(::rust_out::S*);
                    }
                    ...
                    inline std::int32_t func(::rust_out::S __param_0) {
                        return __crubit_internal::...(&__param_0);
                    }
                }
            );
            assert_rs_matches!(
                result.rs_details.tokens,
                quote! {
                    #[unsafe(no_mangle)]
                    unsafe extern "C" fn ...(
                        __param_0: &mut ::core::mem::MaybeUninit<::rust_out::S>
                    ) -> i32 {
                        unsafe {
                            let __param_0 = __param_0.assume_init_read();
                            ::rust_out::func(__param_0)
                        }
                    }
                }
            );
        });
    }

    #[test]
    fn test_format_item_unsupported_fn_param_type_never() {
        let test_src = r#"
                #![feature(never_type)]

                #[unsafe(no_mangle)]
                pub extern "C" fn fn_with_params(_param: !) {}
            "#;
        test_format_item(test_src, "fn_with_params", |result| {
            let err = result.unwrap_err();
            assert_eq!(
                err,
                "Error handling parameter #0 of type `!`: \
                 The never type `!` is only supported as a return type (b/254507801)"
            );
        });
    }

    #[test]
    fn test_must_use_attr_for_fn_no_msg() {
        let test_src = r#"
        #[must_use]
        pub fn add(x: i32, y: i32) -> i32 {
            x + y
        }"#;

        test_format_item(test_src, "add", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    [[nodiscard]] std::int32_t add(std::int32_t x, std::int32_t y);
                }
            )
        })
    }

    #[test]
    fn test_must_use_attr_for_fn_msg() {
        let test_src = r#"
        #[must_use = "hello!"]
        pub fn add(x: i32, y: i32) -> i32 {
            x + y
        }"#;

        test_format_item(test_src, "add", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    [[nodiscard("hello!")]] std::int32_t add(std::int32_t x, std::int32_t y);
                }
            )
        })
    }

    #[test]
    fn test_deprecated_attr_for_fn_no_args() {
        let test_src = r#"
        #[deprecated]
        pub fn add(x: i32, y: i32) -> i32 {
            x + y
        }"#;

        test_format_item(test_src, "add", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    [[deprecated]] std::int32_t add(std::int32_t x, std::int32_t y);
                }
            )
        })
    }

    #[test]
    fn test_deprecated_attr_for_fn_with_message() {
        let test_src = r#"
        #[deprecated = "Use add_i32 instead"]
        pub fn add(x: i32, y: i32) -> i32 {
            x + y
        }"#;

        test_format_item(test_src, "add", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    [[deprecated("Use add_i32 instead")]] std::int32_t add(std::int32_t x, std::int32_t y);
                }
            )
        })
    }

    #[test]
    fn test_deprecated_attr_for_fn_with_named_args() {
        let test_src = r#"
        #[deprecated(since = "3.14", note = "Use add_i32 instead")]
        pub fn add(x: i32, y: i32) -> i32 {
            x + y
        }"#;

        test_format_item(test_src, "add", |result| {
            let result = result.unwrap().unwrap();
            let main_api = &result.main_api;
            assert!(!main_api.prereqs.is_empty());
            assert_cc_matches!(
                main_api.tokens,
                quote! {
                    [[deprecated("Use add_i32 instead")]] std::int32_t add(std::int32_t x, std::int32_t y);
                }
            )
        })
    }
}
