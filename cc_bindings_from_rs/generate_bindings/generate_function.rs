// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::format_type::{format_cc_ident, has_elided_region, region_is_elided};
use crate::generate_doc_comment;
use crate::generate_function_thunk::{
    generate_thunk_decl, generate_thunk_impl, ident_or_opt_ident, is_thunk_required,
};
use crate::{
    format_param_types_for_cc, format_region_as_cc_lifetime, format_ret_ty_for_cc,
    generate_deprecated_tag, is_bridged_type, is_c_abi_compatible_by_value,
    liberate_and_deanonymize_late_bound_regions, BridgedType, CcType, FullyQualifiedName,
    RsSnippet,
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
use query_compiler::{is_copy, post_analysis_typing_env};
use quote::quote;
use rustc_hir::attrs::AttributeKind;
use rustc_hir::{self as hir, def::DefKind};
use rustc_middle::mir::Mutability;
use rustc_middle::ty::{self, Ty, TyCtxt};
use rustc_span::def_id::{DefId, LOCAL_CRATE};
use rustc_span::symbol::Symbol;
use std::collections::BTreeSet;

#[derive(Debug, Eq, PartialEq)]
enum FunctionKind {
    /// Free function (i.e. not a method).
    Free,

    /// Non-method associated function (i.e. the first parameter is not named `self`).
    AssociatedFn,

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
            FunctionKind::Free | FunctionKind::AssociatedFn => false,
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
    Ok(if let Some(bridged_type) = is_bridged_type(db, ty.mid())? {
        match bridged_type {
            BridgedType::Legacy { cpp_type, .. } => {
                if let CcType::Pointer { .. } = cpp_type {
                    cc_ident
                } else {
                    quote! { & #cc_ident }
                }
            }
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
        db.format_ty_for_cc(ty, TypeLocation::Other)?;
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
    if let Some(bridged_type) = is_bridged_type(db, ty.mid())? {
        match bridged_type {
            BridgedType::Legacy { cpp_type, .. } => {
                let cpp_type = format_cc_ident(db, cpp_type.as_ref())?;
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
            }
        }
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

/// Returns the kind of a function (free, method, etc.) or an error if the self type is unsupported.
fn function_kind<'tcx>(
    tcx: TyCtxt<'tcx>,
    def_id: DefId,
    self_ty: Option<Ty<'tcx>>,
    param_types: &[Ty<'tcx>],
) -> Result<FunctionKind> {
    match tcx.def_kind(def_id) {
        DefKind::Fn => return Ok(FunctionKind::Free),
        DefKind::AssocFn => {}
        other => panic!("Unexpected HIR node kind: {other:?}"),
    }
    if !tcx.associated_item(def_id).is_method() {
        return Ok(FunctionKind::AssociatedFn);
    }
    let self_ty = self_ty.expect("ImplItem => non-None `self_ty`");
    if param_types[0] == self_ty {
        return Ok(FunctionKind::MethodTakingSelfByValue);
    }
    match param_types[0].kind() {
        ty::TyKind::Ref(_, referent_ty, _) if *referent_ty == self_ty => {
            Ok(FunctionKind::MethodTakingSelfByRef)
        }
        _ => bail!("Unsupported `self` type `{}`", param_types[0]),
    }
}

fn self_ty_of_method<'tcx>(tcx: TyCtxt<'tcx>, def_id: DefId) -> Option<Ty<'tcx>> {
    #[rustversion::before(2025-07-29)]
    let impl_id = tcx.impl_of_method(def_id)?;
    #[rustversion::since(2025-07-29)]
    let impl_id = tcx.impl_of_assoc(def_id)?;
    match tcx.impl_subject(impl_id).instantiate_identity() {
        ty::ImplSubject::Inherent(ty) => Some(ty),
        ty::ImplSubject::Trait(_) => panic!("Trait methods should be filtered by caller"),
    }
}

fn export_name_and_no_mangle_attrs_of<'tcx>(
    tcx: TyCtxt<'tcx>,
    def_id: DefId,
) -> (Option<Symbol>, bool) {
    let mut export_name: Option<Symbol> = None;
    let mut no_mangle = false;
    for attr in tcx.get_all_attrs(def_id) {
        match attr {
            hir::Attribute::Parsed(AttributeKind::ExportName { name, .. }) => {
                export_name = Some(*name);
            }
            hir::Attribute::Parsed(AttributeKind::NoMangle(..)) => {
                no_mangle = true;
            }
            _ => {}
        }
    }
    (export_name, no_mangle)
}

pub(crate) struct MustUseAttr {
    pub reason: Option<Symbol>,
}

pub(crate) fn must_use_attr_of<'tcx>(tcx: TyCtxt<'tcx>, def_id: DefId) -> Option<MustUseAttr> {
    for attr in tcx.get_all_attrs(def_id) {
        if let hir::Attribute::Parsed(AttributeKind::MustUse { reason, .. }) = attr {
            return Some(MustUseAttr { reason: *reason });
        }
    }
    None
}

struct Param<'tcx> {
    cc_name: TokenStream,
    cpp_type: TokenStream,
    ty: SugaredTy<'tcx>,
}

fn can_shared_refs_to_ty_alias_mut_refs<'tcx>(tcx: TyCtxt<'tcx>, target_ty: Ty<'tcx>) -> bool {
    let is_zero_sized =
        query_compiler::get_layout(tcx, target_ty).map(|layout| layout.is_zst()).unwrap_or(false);
    if is_zero_sized {
        return true;
    }

    // Shared references to types which contain `UnsafeCell` may alias with mutable references.
    if !target_ty.is_freeze(tcx, ty::TypingEnv::fully_monomorphized()) {
        return true;
    }

    false
}

#[derive(Default)]
struct RefsToCheckForAliasing<'a, 'tcx> {
    mutable: Vec<&'a Param<'tcx>>,
    shared: Vec<&'a Param<'tcx>>,
}

/// Returns function parameters which need to be checked for possible illegal mutable aliasing.
///
/// Rust does not allow mutable references to alias with other references (shared or mutable).
/// C++ does not have this requirement, so we insert checks in the generated bindings to ensure that
/// this requirement is not violated.
fn refs_to_check_for_aliasing<'tcx, 'a>(
    db: &dyn BindingsGenerator<'tcx>,
    params: &'a [Param<'tcx>],
) -> Option<RefsToCheckForAliasing<'a, 'tcx>> {
    let tcx = db.tcx();
    let mut refs = RefsToCheckForAliasing::default();
    // TODO: b/351876244 - Apply this check to public reference fields of ADTs, not just top-level
    // reference function parameters.
    //
    // TODO: b/351876244 - Apply this check to reference-like types such as
    // `cpp_std::string_view` and `absl::Span`.
    for param in params {
        if let ty::TyKind::Ref(_region, target_ty, mutability) = param.ty.mid().kind() {
            if mutability.is_mut() {
                refs.mutable.push(param);
            } else if !can_shared_refs_to_ty_alias_mut_refs(tcx, *target_ty) {
                refs.shared.push(param);
            }
        }
    }
    if refs.mutable.is_empty() || (refs.shared.len() + refs.mutable.len() < 2) {
        return None;
    }
    Some(refs)
}

/// Implementation of `BindingsGenerator::generate_function`.
pub fn generate_function(db: &dyn BindingsGenerator<'_>, def_id: DefId) -> Result<ApiSnippets> {
    let tcx = db.tcx();
    ensure!(
        !query_compiler::has_non_lifetime_generics(tcx, def_id),
        "Generic functions are not supported yet (b/259749023)"
    );

    let (sig_mid, sig_hir) = get_fn_sig(tcx, def_id);
    check_fn_sig(&sig_mid)?;
    let self_ty = self_ty_of_method(tcx, def_id);
    let function_kind = function_kind(tcx, def_id, self_ty, sig_mid.inputs())?;
    // TODO(b/262904507): Don't require thunks for mangled extern "C" functions.
    let (export_name, has_no_mangle) = export_name_and_no_mangle_attrs_of(tcx, def_id);
    let has_export_name = export_name.is_some();
    let needs_thunk = is_thunk_required(&sig_mid).is_err() || (!has_no_mangle && !has_export_name);
    let thunk_name = thunk_name(db, def_id, export_name, needs_thunk);

    let fully_qualified_fn_name = FullyQualifiedName::new(db, def_id);
    let unqualified_rust_fn_name =
        fully_qualified_fn_name.rs_name.expect("Functions are assumed to always have a name");
    let main_api_fn_name = format_cc_ident(db, fully_qualified_fn_name.cpp_name.unwrap().as_str())
        .context("Error formatting function name")?;

    let mut main_api_prereqs = CcPrerequisites::default();
    let main_api_ret_type =
        format_ret_ty_for_cc(db, &sig_mid, sig_hir)?.into_tokens(&mut main_api_prereqs);

    let params = {
        let names = tcx.fn_arg_idents(def_id).iter();
        let cpp_types =
            format_param_types_for_cc(db, &sig_mid, sig_hir, function_kind.has_self_param())?;
        names
            .enumerate()
            .zip(SugaredTy::fn_inputs(&sig_mid, sig_hir))
            .zip(cpp_types)
            .map(|(((i, name), ty), cpp_type)| {
                // TODO(jeanpierreda): deduplicate this with thunk_param_names.
                let mut cc_name = None;
                if let Some(ident) = ident_or_opt_ident(name) {
                    if ident.name.as_str() != "_" {
                        if let Ok(name) = format_cc_ident(db, ident.name.as_str()) {
                            cc_name = Some(name);
                        }
                    }
                }
                let cc_name = if let Some(cc_name) = cc_name {
                    cc_name
                } else {
                    expect_format_cc_ident(&format!("__param_{i}"))
                };
                let cpp_type = cpp_type.into_tokens(&mut main_api_prereqs);
                Param { cc_name, cpp_type, ty }
            })
            .collect_vec()
    };

    let mut takes_self_by_copy = false;
    let method_qualifiers = match function_kind {
        FunctionKind::Free | FunctionKind::AssociatedFn => quote! {},
        FunctionKind::MethodTakingSelfByValue => {
            let self_ty = params[0].ty.mid();
            if is_copy(tcx, def_id, self_ty) {
                takes_self_by_copy = true;
                quote! { const }
            } else {
                quote! { && }
            }
        }
        FunctionKind::MethodTakingSelfByRef => match params[0].ty.mid().kind() {
            ty::TyKind::Ref(region, _, mutability) => {
                let tcx = db.tcx();
                // Ref-qualify if the lifetime of `&self` is a named lifetime or if the elided
                // lifetime appears in the return type.
                // See <internal link> for more details on the motivation.
                let ref_qualifier = if !region_is_elided(tcx, *region)
                    || has_elided_region(tcx, sig_mid.output())
                {
                    let lifetime_annotation = format_region_as_cc_lifetime(tcx, region);
                    quote! { & #lifetime_annotation }
                } else {
                    quote! {}
                };
                let mutability = match mutability {
                    Mutability::Mut => quote! {},
                    Mutability::Not => quote! { const },
                };
                quote! { #mutability #ref_qualifier }
            }
            _ => panic!("Expecting TyKind::Ref for MethodKind...Self...Ref"),
        },
    };

    fn has_non_lifetime_substs(substs: &[ty::GenericArg]) -> bool {
        substs.iter().any(|subst| subst.as_region().is_none())
    }

    let struct_name = match self_ty {
        Some(ty) => match ty.kind() {
            ty::TyKind::Adt(adt, substs) => {
                assert!(!has_non_lifetime_substs(substs), "Callers should filter out generics");
                Some(FullyQualifiedName::new(db, adt.did()))
            }
            _ => panic!("Non-ADT `impl`s should be filtered by caller"),
        },
        None => None,
    };
    let needs_definition = unqualified_rust_fn_name.as_str() != thunk_name;
    let main_api_params = params
        .iter()
        .skip(if function_kind.has_self_param() { 1 } else { 0 })
        .map(|Param { cc_name, cpp_type, .. }| quote! { #cpp_type #cc_name })
        .collect_vec();
    let rs_return_type = SugaredTy::fn_output(&sig_mid, sig_hir);
    let fn_never_returns = *rs_return_type.mid().kind() == ty::TyKind::Never;
    let main_api = {
        let doc_comment = {
            let doc_comment = generate_doc_comment(tcx, def_id);
            quote! { __NEWLINE__ #doc_comment }
        };

        let mut prereqs = main_api_prereqs.clone();
        prereqs.move_defs_to_fwd_decls();

        let static_ = if function_kind == FunctionKind::AssociatedFn {
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
        if let Some(must_use_attr) = must_use_attr_of(tcx, def_id) {
            match must_use_attr.reason {
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
        // Attribute: noreturn
        if fn_never_returns {
            attributes.push(quote! {[[noreturn]]});
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
        let thunk_decl = generate_thunk_decl(
            db,
            &sig_mid,
            sig_hir,
            &thunk_name,
            function_kind.has_self_param(),
        )?
        .into_tokens(&mut prereqs);

        let mut statements = TokenStream::new();
        let mut thunk_args = params
            .iter()
            .enumerate()
            .map(|(i, Param { cc_name, ty, .. })| {
                if i == 0 && function_kind.has_self_param() {
                    if takes_self_by_copy {
                        // Self-by-copy methods are `const` qualified. The Rust thunk does not
                        // accept a const pointer, but we can just const_cast since underlying C++
                        // object is not modified: Rust copies the object before passing it into
                        // the by-value method.
                        statements.extend(quote! {
                            auto& #cc_name = const_cast<
                                std::remove_cvref_t<decltype(*this)>&>(*this);
                        });
                    } else {
                        statements.extend(quote! { auto&& #cc_name = *this; });
                    }
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

        if let Some(refs_to_check) = refs_to_check_for_aliasing(db, &params) {
            let mut_cpp_tys = refs_to_check.mutable.iter().map(|param| &param.cpp_type);
            let mut_cpp_names = refs_to_check.mutable.iter().map(|param| &param.cc_name);
            let shared_cpp_tys = refs_to_check.shared.iter().map(|param| &param.cpp_type);
            let shared_cpp_names = refs_to_check.shared.iter().map(|param| &param.cc_name);
            prereqs.includes.insert(db.support_header("internal/check_no_mutable_aliasing.h"));
            statements.extend(quote! {
                __NEWLINE__
                crubit::internal::CheckNoMutableAliasing(
                    crubit::internal::AsMutPtrDatas<#( #mut_cpp_tys ),*>( #( #mut_cpp_names ),* ),
                    crubit::internal::AsPtrDatas<#( #shared_cpp_tys ),*>(
                        #( #shared_cpp_names ),* )
                );
                __NEWLINE__
            });
        }

        let impl_body: TokenStream = if is_bridged_type(db, rs_return_type.mid())?.is_none()
            && is_c_abi_compatible_by_value(rs_return_type.mid())
        {
            // C++ compilers can emit diagnostics if a function marked [[noreturn]] looks like it
            // might return. In this scenario, we just call the (also [[noreturn]]) thunk.
            let return_expr = if fn_never_returns {
                quote! {}
            } else {
                quote! {return}
            };
            quote! {
                #return_expr __crubit_internal::#thunk_name(#( #thunk_args ),*);
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
            // We don't have to worry about the [[noreturn]] situation described above because all
            // [[noreturn]] functions will take that branch.
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
pub fn get_fn_sig<'tcx>(
    tcx: TyCtxt<'tcx>,
    def_id: DefId,
) -> (ty::FnSig, Option<&rustc_hir::FnDecl>) {
    let sig_mid = liberate_and_deanonymize_late_bound_regions(
        tcx,
        tcx.fn_sig(def_id).instantiate_identity(),
        def_id,
    );
    let hir_decl = def_id
        .as_local()
        .map(|local_def_id| tcx.hir_node_by_def_id(local_def_id).fn_sig().unwrap().decl);
    (sig_mid, hir_decl)
}
