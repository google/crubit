// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use arc_anyhow::{ensure, Context, Result};
use code_gen_utils::make_rs_ident;
use crubit_abi_type::{CrubitAbiTypeToRustExprTokens, CrubitAbiTypeToRustTokens};
use database::code_snippet::{ApiSnippets, Feature, GeneratedItem, Thunk, Visibility};
use database::function_types::{FunctionId, GeneratedFunction, ImplFor, ImplKind, TraitName};
use database::rs_snippet::{
    format_generic_params, format_generic_params_replacing_by_self, should_derive_clone,
    unique_lifetimes, Lifetime, LifetimeOptions, Mutability, PassingConvention, RsTypeKind,
};
use database::BindingsGenerator;
use error_report::{anyhow, bail, ErrorList};
use errors::{bail_to_errors, Errors, ErrorsOr};
use flagset::FlagSet;
use generate_comment::generate_doc_comment;
use generate_function_thunk::{
    generate_function_assertation, generate_function_thunk, generate_function_thunk_impl,
    thunk_ident,
};
use ir::*;
use itertools::Itertools;
use lifetime_defaults_transform::lifetime_defaults_transform_func;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;
use std::ptr;
use std::rc::Rc;
use std::sync::LazyLock;

/// Similar to to_tokens but removing a given record type from the list of
/// generic args
///
/// This is used to remove the record whose trait implementation is being
/// generated.
fn trait_name_to_token_stream_removing_trait_record(
    db: &dyn BindingsGenerator,
    trait_name: &TraitName,
    trait_record: Option<&Record>,
) -> TokenStream {
    use TraitName::*;
    match trait_name {
        UnpinConstructor { name, params } | Other { name, params, .. } => {
            let name_as_token_stream = name.parse::<TokenStream>().unwrap();
            let formatted_params =
                format_generic_params_replacing_by_self(db, &**params, trait_record);
            quote! {#name_as_token_stream #formatted_params}
        }
        PartialEq { param, .. } => {
            if trait_record.is_some_and(|trait_record| param.is_record(trait_record)) {
                quote! {PartialEq}
            } else {
                let formatted_params = format_generic_params_replacing_by_self(
                    db,
                    core::slice::from_ref(&**param),
                    trait_record,
                );
                quote! {PartialEq #formatted_params}
            }
        }
        PartialOrd { param } => {
            if trait_record.is_some() && param.is_record(trait_record.unwrap()) {
                quote! {PartialOrd}
            } else {
                let formatted_params = format_generic_params_replacing_by_self(
                    db,
                    core::slice::from_ref(&**param),
                    trait_record,
                );
                quote! {PartialOrd #formatted_params}
            }
        }
        CtorNew(arg_types) => {
            let formatted_arg_types =
                format_tuple_except_singleton_replacing_by_self(db, arg_types, trait_record);
            quote! { ::ctor::CtorNew < #formatted_arg_types > }
        }
        Clone => {
            quote! { Clone }
        }
    }
}

fn trait_name_to_token_stream(db: &dyn BindingsGenerator, trait_name: &TraitName) -> TokenStream {
    trait_name_to_token_stream_removing_trait_record(db, trait_name, None)
}

/// Returns whether an argument of this type causes ADL to include the `record`.
fn adl_expands_to(record: &Record, rs_type_kind: &RsTypeKind) -> bool {
    match rs_type_kind {
        RsTypeKind::Record { record: nested_record, .. } => ptr::eq(record, &**nested_record),
        RsTypeKind::Reference { referent, .. } => adl_expands_to(record, referent),
        RsTypeKind::RvalueReference { referent, .. } => adl_expands_to(record, referent),
        _ => false,
    }
}

/// Returns whether any type in `param_types` causes ADL to include `record`.
///
/// This is an under-approximation. Things not considered include class template
/// arguments and the parameters and return type of function types.
///
/// See https://en.cppreference.com/w/cpp/language/adl
fn is_visible_by_adl(enclosing_record: &Record, param_types: &[RsTypeKind]) -> bool {
    param_types.iter().any(|param_type| adl_expands_to(enclosing_record, param_type))
}

#[derive(Debug)]
struct OperatorMetadata {
    by_cc_name_and_params: HashMap<(&'static str, usize), OperatorMetadataEntry>,
}

#[derive(Clone, Copy, Debug)]
struct OperatorMetadataEntry {
    cc_name: &'static str,
    cc_params: usize,
    trait_name: &'static str,
    method_name: &'static str,
    is_compound_assignment: bool,
}

impl OperatorMetadataEntry {
    const fn unary(
        cc_name: &'static str,
        trait_name: &'static str,
        method_name: &'static str,
    ) -> Self {
        Self { cc_name, cc_params: 1, trait_name, method_name, is_compound_assignment: false }
    }

    const fn binary(
        cc_name: &'static str,
        trait_name: &'static str,
        method_name: &'static str,
    ) -> Self {
        Self { cc_name, cc_params: 2, trait_name, method_name, is_compound_assignment: false }
    }

    const fn assign(
        cc_name: &'static str,
        trait_name: &'static str,
        method_name: &'static str,
    ) -> Self {
        Self { cc_name, cc_params: 2, trait_name, method_name, is_compound_assignment: true }
    }
}

static OPERATOR_METADATA: LazyLock<OperatorMetadata> = LazyLock::new(|| {
    const ENTRIES: &[OperatorMetadataEntry] = &[
        OperatorMetadataEntry::unary("-", "Neg", "neg"),
        // The Rust `Not` trait matches with both the C++ `!` and `~` operators to some extent. The
        // two operators appear with similar frequency in our target codebase so it's not clear
        // which is better to map here. Mapping `operator!` to `Not` as chosen here means that a
        // C++ `!` matches up with a Rust `!`.
        OperatorMetadataEntry::unary("!", "Not", "not"),
        OperatorMetadataEntry::binary("+", "Add", "add"),
        OperatorMetadataEntry::binary("-", "Sub", "sub"),
        OperatorMetadataEntry::binary("*", "Mul", "mul"),
        OperatorMetadataEntry::binary("/", "Div", "div"),
        OperatorMetadataEntry::binary("%", "Rem", "rem"),
        OperatorMetadataEntry::binary("&", "BitAnd", "bitand"),
        OperatorMetadataEntry::binary("|", "BitOr", "bitor"),
        OperatorMetadataEntry::binary("^", "BitXor", "bitxor"),
        OperatorMetadataEntry::binary("<<", "Shl", "shl"),
        OperatorMetadataEntry::binary(">>", "Shr", "shr"),
        OperatorMetadataEntry::assign("+=", "AddAssign", "add_assign"),
        OperatorMetadataEntry::assign("-=", "SubAssign", "sub_assign"),
        OperatorMetadataEntry::assign("*=", "MulAssign", "mul_assign"),
        OperatorMetadataEntry::assign("/=", "DivAssign", "div_assign"),
        OperatorMetadataEntry::assign("%=", "RemAssign", "rem_assign"),
        OperatorMetadataEntry::assign("&=", "BitAndAssign", "bitand_assign"),
        OperatorMetadataEntry::assign("|=", "BitOrAssign", "bitor_assign"),
        OperatorMetadataEntry::assign("^=", "BitXorAssign", "bitxor_assign"),
        OperatorMetadataEntry::assign("<<=", "ShlAssign", "shl_assign"),
        OperatorMetadataEntry::assign(">>=", "ShrAssign", "shr_assign"),
        // NOTE: if adding an entry here, consider whether `func_should_infer_lifetimes_of_references`
        // is appropriate. If not, amend this structure to include an `infer_lifetimes` field.
    ];
    OperatorMetadata {
        by_cc_name_and_params: ENTRIES.iter().map(|e| ((e.cc_name, e.cc_params), *e)).collect(),
    }
});

/// Whether the function is a friend of a record, but the record is not visible
/// by ADL.
///
/// This is a heuristic to avoid generating bindings for functions that are not
/// visible by ADL.
///
/// This is necessary because ADL is needed in order to find friend functions.
fn is_friend_of_record_not_visible_by_adl(
    db: &dyn BindingsGenerator,
    func: &Func,
    param_types: &[RsTypeKind],
) -> bool {
    let Some(decl_id) = func.adl_enclosing_record else { return false };
    let ir = db.ir();
    let adl_enclosing_record = ir
        .find_decl::<Rc<Record>>(decl_id)
        .with_context(|| format!("Failed to look up `adl_enclosing_record` of {:?}", func))
        .unwrap();
    !is_visible_by_adl(adl_enclosing_record, param_types)
}

/// Ensures that `kind` is a record or a const reference to a record.
///
/// Returns the `RsTypeKind` and `Record` of the underlying record type.
fn type_by_value_or_under_const_ref<'a>(
    db: &dyn BindingsGenerator,
    kind: &'a mut RsTypeKind,
    value_desc: &str,
    errors: &Errors,
) -> ErrorsOr<(&'a RsTypeKind, &'a Rc<Record>)> {
    // Pre-record `kind_string` before any adjustments occur.
    let kind_string = kind.display(db).to_string();
    match *kind {
        RsTypeKind::Reference { referent: ref lhs, ref mut mutability, .. } => {
            if !mutability.is_const() {
                // NOTE: bindings will never actually rely on this immutability, as we
                // the resulting generated code will always result in a compiler error if used
                // because of the `errors.add`.
                errors.add(anyhow!("Expected {value_desc} reference to be immutable, but found mutable reference: {kind_string}"));
                *mutability = Mutability::Const;
            }
            if let RsTypeKind::Record { record: lhs_record, .. } = &**lhs {
                Ok((lhs, lhs_record))
            } else {
                bail_to_errors!(errors, "Expected {value_desc} to be a record or a const reference to a record, found a reference that doesn't refer to a record: {kind_string}");
            }
        }
        RsTypeKind::Record { record: ref lhs_record, .. } => Ok((kind, lhs_record)),
        _ => bail_to_errors!(
            errors,
            "Expected {value_desc} to be a record or const reference to a record, found {kind_string}"
        ),
    }
}

fn api_func_shape_for_operator_ne(
    db: &dyn BindingsGenerator,
    func: &Func,
    param_types: &mut [RsTypeKind],
    errors: &Errors,
) -> ErrorsOr<(Ident, ImplKind)> {
    // If operator== is present, don't generate ne, rely on rust's default ne.
    let eq_binding = db.get_binding(
        UnqualifiedIdentifier::Operator(Operator { name: Rc::from("==") }),
        param_types.to_vec(),
    );
    if let Some((_, ImplKind::Trait { trait_name: TraitName::PartialEq { .. }, .. })) = eq_binding {
        bail_to_errors!(errors, "operator== is present, skipping bindings for operator!=");
    }
    // C++ requires that operator!= is binary.
    let [param_1, param_2] = param_types else {
        panic!("Expected operator!= to have exactly two parameters. Found: {func:?}");
    };
    let lhs_ty = type_by_value_or_under_const_ref(db, param_1, "first operator!= param", errors);
    let rhs_ty = type_by_value_or_under_const_ref(db, param_2, "second operator!= param", errors);
    let ((_, lhs_record), (param, _)) = (lhs_ty?, rhs_ty?);
    let param = Rc::new(param.clone());
    // We generate eq instead of ne by negating the call to thunk.
    let func_name = make_rs_ident("eq");
    let impl_kind = ImplKind::new_trait(
        TraitName::PartialEq { param, negate_thunk_result: true },
        lhs_record.clone(),
        /* format_first_param_as_self= */ true,
        /* force_const_reference_params= */ true,
    );
    Ok((func_name, impl_kind))
}

fn api_func_shape_for_operator_eq(
    db: &dyn BindingsGenerator,
    func: &Func,
    param_types: &mut [RsTypeKind],
    errors: &Errors,
) -> ErrorsOr<(Ident, ImplKind)> {
    // C++ requires that operator== is binary.
    let [param_1, param_2] = param_types else {
        panic!("Expected operator== to have exactly two parameters. Found: {func:?}");
    };

    let lhs_ty = type_by_value_or_under_const_ref(db, param_1, "first operator== param", errors);
    let rhs_ty = type_by_value_or_under_const_ref(db, param_2, "second operator== param", errors);
    let ((_, lhs_record), (param, _)) = (lhs_ty?, rhs_ty?);
    let param = Rc::new(param.clone());
    let func_name = make_rs_ident("eq");
    let impl_kind = ImplKind::new_trait(
        TraitName::PartialEq { param, negate_thunk_result: false },
        lhs_record.clone(),
        /* format_first_param_as_self= */ true,
        /* force_const_reference_params= */ true,
    );
    Ok((func_name, impl_kind))
}

fn api_func_shape_for_operator_lt(
    db: &dyn BindingsGenerator,
    func: &Func,
    param_types: &mut [RsTypeKind],
    errors: &Errors,
) -> ErrorsOr<(Ident, ImplKind)> {
    let [param_1, param_2] = param_types else {
        panic!("Expected operator< to have exactly two parameters. Found: {func:?}")
    };
    let lhs_ty = type_by_value_or_under_const_ref(db, param_1, "first operator< param", errors);
    let rhs_ty = type_by_value_or_under_const_ref(db, param_2, "second operator< param", errors);
    let ((_, lhs_record), (param, rhs_record)) = (lhs_ty?, rhs_ty?);
    // Even though Rust and C++ allow operator< to be implemented on different
    // types, we don't generate bindings for them at this moment. The
    // issue is that our canonical implementation of partial_cmp relies
    // on transitivity. This would require checking that both lt(&T1,
    // &T2) and lt(&T2, &T1) are implemented. In other words, both lt
    // implementations would need to query for the existence of the other, which
    // would create a cyclic dependency.
    if lhs_record != rhs_record {
        bail_to_errors!(
            errors,
            "operator< where lhs and rhs are not the same type. This is not yet supported."
        );
    }
    let param = param.clone();
    let lhs_record = lhs_record.clone();
    // PartialOrd requires PartialEq, so we need to make sure operator== is
    // implemented for this Record type.
    let partialeq_binding = db.get_binding(
        UnqualifiedIdentifier::Operator(Operator { name: Rc::from("==") }),
        param_types.to_vec(),
    );
    match partialeq_binding {
        Some((_, ImplKind::Trait { trait_name: TraitName::PartialEq { .. }, .. })) => {}
        _ => errors.add(anyhow!("operator< where operator== is missing.")),
    }
    let func_name = make_rs_ident("lt");
    let impl_kind = ImplKind::new_trait(
        TraitName::PartialOrd { param: Rc::new(param) },
        lhs_record.clone(),
        /* format_first_param_as_self= */ true,
        /* force_const_reference_params= */ true,
    );
    Ok((func_name, impl_kind))
}

fn api_func_shape_for_operator_assign(
    func: &Func,
    maybe_record: Option<&Rc<Record>>,
    param_types: &mut [RsTypeKind],
    errors: &Errors,
) -> Option<(Ident, ImplKind)> {
    assert_eq!(param_types.len(), 2, "Unexpected number of parameters in operator=: {func:?}");
    let Some(record) = maybe_record else {
        errors.add(anyhow!("operator= must be a member function"));
        return None;
    };
    materialize_ctor_in_caller(func, param_types);

    let rhs = &param_types[1];

    //  TODO(b/219963671): consolidate UnpinAssign and Assign in ctor.rs
    let trait_name;
    let func_name;
    if record.is_unpin() {
        if rhs.is_ref_to(record) && record.should_derive_copy() {
            // `MoveAndAssignViaCopy` is derived for `Copy` types, so we don't need to generate
            // `UnpinAssign` explicitly.
            return None;
        }

        trait_name = Rc::from("::ctor::UnpinAssign");
        func_name = make_rs_ident("unpin_assign");
    } else {
        trait_name = Rc::from("::ctor::Assign");
        func_name = make_rs_ident("assign")
    };

    let impl_kind = ImplKind::Trait {
        record: record.clone(),
        trait_name: TraitName::Other {
            name: trait_name,
            params: Rc::new([rhs.clone()]),
            is_unsafe_fn: false,
        },
        impl_for: ImplFor::T,
        trait_generic_params: Rc::new([]),
        format_first_param_as_self: true,
        drop_return: true,
        associated_return_type: None,
        force_const_reference_params: false,
        // OK to always be public: the only type in the param list
        // depends on self, so it's only usable within the current
        // crate. Also, the trait is owned by Crubit, and callers
        // have no reason to define their own impl given that it's
        // not usable outside this crate.
        always_public: true,
    };
    Some((func_name, impl_kind))
}

fn api_func_shape_for_operator_unary_plus(
    db: &dyn BindingsGenerator,
    param_type: &RsTypeKind,
    errors: &Errors,
) -> ErrorsOr<(Ident, ImplKind)> {
    let (record, _) = extract_first_operator_parameter(db, param_type, errors)?;
    Ok((
        make_rs_ident("unary_plus"),
        ImplKind::Struct {
            is_unsafe: false,
            record,
            format_first_param_as_self: true,
            is_renamed_unpin_constructor: false,
        },
    ))
}

fn extract_first_operator_parameter(
    db: &dyn BindingsGenerator,
    param_types: &RsTypeKind,
    errors: &Errors,
) -> ErrorsOr<(Rc<Record>, ImplFor)> {
    match param_types {
        RsTypeKind::Record { record, .. } => Ok((record.clone(), ImplFor::T)),
        RsTypeKind::IncompleteRecord { incomplete_record, .. } => {
            bail_to_errors!(
                            errors,
                            "Incomplete record types are not yet supported as first parameter of operator, found {cc_name}", cc_name=incomplete_record.cc_name,
                        )
        }
        RsTypeKind::Reference { referent, .. } => Ok((
            expect_possibly_incomplete_record(db, referent, "first operator parameter", errors)?
                .clone(),
            ImplFor::RefT,
        )),
        RsTypeKind::RvalueReference { .. } => {
            bail_to_errors!(
                            errors,
                            "Rvalue reference types are not yet supported as first parameter of operators (b/219826128)",
                        )
        }
        _ => bail_to_errors!(
            errors,
            "Non-record-nor-reference operator parameters are not yet supported, found {}",
            param_types.display(db)
        ),
    }
}

fn expect_possibly_incomplete_record<'a>(
    db: &dyn BindingsGenerator,
    type_kind: &'a RsTypeKind,
    value_desc: &str,
    errors: &Errors,
) -> ErrorsOr<&'a Rc<Record>> {
    match type_kind {
        RsTypeKind::Record { record, .. } => Ok(record),
        RsTypeKind::IncompleteRecord { .. } => bail_to_errors!(
            errors,
            "Incomplete record types are not yet supported as {value_desc}, found {}",
            type_kind.display(db),
        ),
        _ => bail_to_errors!(
            errors,
            "Expected {value_desc} to be a record or incomplete record, found {}",
            type_kind.display(db),
        ),
    }
}

fn record_type_of_compound_assignment<'a>(
    db: &dyn BindingsGenerator,
    lhs_type: &'a mut RsTypeKind,
    errors: &Errors,
) -> ErrorsOr<&'a Rc<Record>> {
    let lhs_str = lhs_type.display(db).to_string();
    let fix_mutability = |mutability: &mut Mutability| {
        if mutability.is_const() {
            errors.add(anyhow!(
                "Compound assignment with const left-hand side is not supported, found {lhs_str}"
            ));
            *mutability = Mutability::Mut;
        }
    };

    let lhs_record_type: &Rc<Record> = match lhs_type {
        RsTypeKind::Record { record, .. } => {
            errors.add(anyhow!("Compound assignment with by-value left-hand side is not yet supported, found {lhs_str}"));
            record
        }
        RsTypeKind::IncompleteRecord { .. } => {
            bail_to_errors!(errors, "Compound assignment with incomplete record left-hand side is not yet supported, found {lhs_str}")
        }
        RsTypeKind::Reference { referent, mutability, .. } => {
            fix_mutability(mutability);
            expect_possibly_incomplete_record(db, referent, "compound assignment first parameter", errors)?
        }
        RsTypeKind::RvalueReference { referent, mutability, .. } => {
            errors.add(anyhow!("Compound assignment with rvalue reference is not yet supported (b/219826128), found {lhs_str}"));
            fix_mutability(mutability);
            expect_possibly_incomplete_record(db, referent, "compound assignment first parameter", errors)?
        }
        RsTypeKind::Pointer { pointee, mutability, .. } => {
            errors.add(anyhow!("Compound assignment operators are not yet supported for pointers with unknown lifetime (b/219826128), found {lhs_str}"));
            fix_mutability(mutability);
            expect_possibly_incomplete_record(db, pointee, "compound assignment first parameter", errors)?
        }
        _ => panic!("Compound assignment operator defined, but first parameter is not a record or reference: {lhs_str}"),
    };
    if !lhs_record_type.is_unpin() {
        // Note: we bail here to avoid generating an impl with `self: Pin<&mut Self>`
        // which will fail to compile. This could be relaxed in the future.
        bail_to_errors!(
            errors,
            "Compound assignment operators are not supported for non-Unpin types, found {lhs_str}"
        );
    }
    Ok(lhs_record_type)
}

/// Reports a fatal error generating bindings for a function.
/// Fatal errors should only be reported
fn report_fatal_func_error(db: &dyn BindingsGenerator, func: &Func, msg: &str) {
    db.fatal_errors().report(&format!("{}: {}", func.source_loc, msg));
}

fn api_func_shape_for_operator(
    db: &dyn BindingsGenerator,
    func: &Func,
    maybe_record: Option<&Rc<Record>>,
    param_types: &mut [RsTypeKind],
    op: &Operator,
    errors: &Errors,
) -> Option<(Ident, ImplKind)> {
    if let SafetyAnnotation::Unsafe = func.safety_annotation {
        report_fatal_func_error(db, func, "Unsafe annotations on operators are not supported");
    }
    match op.name.as_ref() {
        "==" => api_func_shape_for_operator_eq(db, func, param_types, errors).ok(),
        "!=" => api_func_shape_for_operator_ne(db, func, param_types, errors).ok(),
        "<=>" => {
            errors.add(anyhow!("Three-way comparison operator not yet supported (b/219827738)"));
            None
        }
        "<" => api_func_shape_for_operator_lt(db, func, param_types, errors).ok(),
        "=" => api_func_shape_for_operator_assign(func, maybe_record, param_types, errors),
        "+" if param_types.len() == 1 => {
            api_func_shape_for_operator_unary_plus(db, &param_types[0], errors).ok()
        }
        _ => {
            let Some(op_metadata) =
                OPERATOR_METADATA.by_cc_name_and_params.get(&(&op.name, param_types.len()))
            else {
                errors.add(anyhow!(
                    "Bindings for this kind of operator (operator {op} with {n} parameter(s)) are not supported",
                    op = &op.name,
                    n = param_types.len(),
                ));
                return None;
            };
            materialize_ctor_in_caller(func, param_types);
            let trait_name = op_metadata.trait_name;
            if op_metadata.is_compound_assignment {
                let record =
                    record_type_of_compound_assignment(db, &mut param_types[0], errors).ok()?;
                let func_name = make_rs_ident(op_metadata.method_name);
                let impl_kind = ImplKind::Trait {
                    record: record.clone(),
                    trait_name: TraitName::Other {
                        name: Rc::from(format!("::core::ops::{trait_name}")),
                        params: Rc::from(&param_types[1..]),
                        is_unsafe_fn: false,
                    },
                    impl_for: ImplFor::T,
                    trait_generic_params: Rc::new([]),
                    format_first_param_as_self: true,
                    drop_return: true,
                    associated_return_type: None,
                    force_const_reference_params: false,
                    always_public: false,
                };
                Some((func_name, impl_kind))
            } else {
                let (record, impl_for) =
                    extract_first_operator_parameter(db, &param_types[0], errors).ok()?;
                let func_name = make_rs_ident(op_metadata.method_name);
                let impl_kind = ImplKind::Trait {
                    record,
                    trait_name: TraitName::Other {
                        name: Rc::from(format!("::core::ops::{trait_name}")),
                        params: Rc::from(&param_types[1..]),
                        is_unsafe_fn: false,
                    },
                    impl_for,
                    trait_generic_params: Rc::new([]),
                    format_first_param_as_self: true,
                    drop_return: false,
                    associated_return_type: Some(make_rs_ident("Output")),
                    force_const_reference_params: false,
                    always_public: false,
                };
                Some((func_name, impl_kind))
            }
        }
    }
}

fn api_func_shape_for_identifier(
    db: &dyn BindingsGenerator,
    func: &Func,
    maybe_record: Option<&Rc<Record>>,
    param_types: &mut [RsTypeKind],
    id: &Identifier,
) -> (Ident, ImplKind) {
    let is_unsafe = match func.safety_annotation {
        SafetyAnnotation::Unannotated => {
            let mut param_type_iter = param_types.iter();
            if func.cc_name.is_constructor() {
                // This is a renamed constructor.
                //
                // Discard the `this` parameter, as constructors of unsafe types are not
                // automatically considered unsafe. Similarly to Rust's raw pointer types, creating
                // an unsafe type is safe, but using one is not.
                let _ = param_type_iter.next();
            }
            param_type_iter.any(|p| db.rs_type_kind_safety(p.clone()).is_unsafe())
        }
        SafetyAnnotation::Unsafe => true,
        SafetyAnnotation::DisableUnsafe => false,
    };

    let func_name = make_rs_ident(&id.identifier);
    let Some(record) = maybe_record else { return (func_name, ImplKind::None { is_unsafe }) };
    let is_renamed_unpin_constructor = func.cc_name.is_constructor() && record.is_unpin();
    let format_first_param_as_self = if func.is_instance_method() {
        let Some(first_param) = param_types.first() else {
            panic!("Missing `__this` parameter in an instance method: {:?}", func);
        };
        first_param.is_ref_to(record)
    } else {
        false
    };
    (
        func_name,
        ImplKind::Struct {
            record: record.clone(),
            format_first_param_as_self,
            is_renamed_unpin_constructor,
            is_unsafe,
        },
    )
}

fn api_func_shape_for_destructor(
    db: &dyn BindingsGenerator,
    func: &Func,
    maybe_record: Option<&Rc<Record>>,
    param_types: &mut [RsTypeKind],
) -> Option<(Ident, ImplKind)> {
    if let SafetyAnnotation::Unsafe = func.safety_annotation {
        report_fatal_func_error(db, func, "Unsafe annotations on destructors are not supported");
    }
    let Some(record) = maybe_record else {
        panic!("Destructors are always member functions, but found: {func:?}");
    };
    // Note: to avoid double-destruction of the fields, they are all wrapped in
    // ManuallyDrop in this case. See `generate_record`.
    if !record.should_implement_drop() {
        return None;
    }
    if record.is_unpin() {
        let func_name = make_rs_ident("drop");
        let impl_kind = ImplKind::Trait {
            record: record.clone(),
            trait_name: TraitName::Other {
                name: Rc::from("Drop"),
                params: Rc::from([]),
                is_unsafe_fn: false,
            },
            impl_for: ImplFor::T,
            trait_generic_params: Rc::new([]),
            format_first_param_as_self: true,
            drop_return: false,
            associated_return_type: None,
            force_const_reference_params: false,
            // OK to be always_public: canonical case.
            always_public: true,
        };
        Some((func_name, impl_kind))
    } else {
        materialize_ctor_in_caller(func, param_types);
        let func_name = make_rs_ident("pinned_drop");
        let impl_kind = ImplKind::Trait {
            record: record.clone(),
            trait_name: TraitName::Other {
                name: Rc::from("::ctor::PinnedDrop"),
                params: Rc::from([]),
                is_unsafe_fn: true,
            },
            impl_for: ImplFor::T,
            trait_generic_params: Rc::new([]),
            format_first_param_as_self: true,
            drop_return: false,
            associated_return_type: None,
            force_const_reference_params: false,
            // OK to be always_public: canonical case.
            always_public: true,
        };
        Some((func_name, impl_kind))
    }
}

/// Issue any errors related to unsafe constructors being unsupported.
fn issue_unsafe_constructor_errors(
    db: &dyn BindingsGenerator,
    func: &Func,
    record: &Record,
    param_types: &[RsTypeKind],
    errors: &Errors,
) {
    match func.safety_annotation {
        SafetyAnnotation::DisableUnsafe => {}
        SafetyAnnotation::Unsafe => {
            errors.add(anyhow!(
                "Constructors cannot be `unsafe`, but an explicit unsafe annotation was provided. See b/216648347."));
        }
        SafetyAnnotation::Unannotated => {
            // Move and copy constructors are excepted from this check, as Google C++ style
            // disallows move and copy constructors which require invariants to hold on public
            // fields of the source object.
            let is_move_or_copy_ctor = matches!(param_types, [_this, arg] if arg.is_ref_to(record));
            if is_move_or_copy_ctor {
                return;
            }

            // We skip the first parameter because it's the implicit `this` parameter.
            // Constructors of unsafe types are not automatically considered unsafe.
            let param_names = func.params.iter().map(|p| &p.identifier);
            let unsafe_params = param_names
                .zip(param_types)
                .skip(1)
                .filter_map(|(param_name, param_type)| {
                    let reason = db.rs_type_kind_safety((*param_type).clone()).unsafe_reason()?;
                    Some(format!("\n    `{param_name}`: {reason}"))
                })
                .collect::<Vec<String>>()
                .join("");
            if !unsafe_params.is_empty() {
                errors.add(anyhow!(
                    "Constructors cannot be `unsafe`, but this constructor accepts:{unsafe_params}"
                ));
            }
        }
    }
}

fn api_func_shape_for_constructor(
    db: &dyn BindingsGenerator,
    func: &Func,
    maybe_record: Option<&Rc<Record>>,
    param_types: &mut [RsTypeKind],
    errors: &Errors,
) -> Option<(Ident, ImplKind)> {
    let Some(record) = maybe_record else {
        panic!("Constructors must be associated with a record.");
    };
    if let Err(err) = record.check_by_value() {
        errors.add(err);
    }
    materialize_ctor_in_caller(func, param_types);
    issue_unsafe_constructor_errors(db, func, record, param_types, errors);

    if !record.is_unpin() {
        let func_name = make_rs_ident("ctor_new");
        let [_this, params @ ..] = param_types else {
            panic!("Missing `__this` parameter in a constructor: {:?}", func)
        };
        // Elided lifetimes won't "just work" when split across the `Ctor` trait impl, so we replace
        // them with a single named lifetime.
        for param in &mut params[..] {
            if let RsTypeKind::Reference { lifetime, .. }
            | RsTypeKind::RvalueReference { lifetime, .. } = param
            {
                if lifetime.is_elided() {
                    *lifetime = Lifetime::new("__unelided");
                }
            }
        }
        let impl_kind = ImplKind::Trait {
            record: record.clone(),
            trait_name: TraitName::CtorNew(params.iter().cloned().collect()),
            impl_for: ImplFor::T,
            trait_generic_params: Rc::new([]),
            format_first_param_as_self: false,
            drop_return: false,
            associated_return_type: Some(make_rs_ident("CtorType")),
            force_const_reference_params: false,
            // OK to be always public: the trait only has a static method,
            // meaning it's only usable within the current target anyway.
            always_public: true,
        };
        return Some((func_name, impl_kind));
    }
    match func.params.len() {
        0 => panic!("Missing `__this` parameter in a constructor: {:?}", func),
        1 => {
            let func_name = make_rs_ident("default");
            let impl_kind = ImplKind::new_trait(
                TraitName::UnpinConstructor { name: Rc::from("Default"), params: Rc::from([]) },
                record.clone(),
                /* format_first_param_as_self= */ false,
                /* force_const_reference_params= */ false,
            );
            Some((func_name, impl_kind))
        }
        2 if param_types[1].is_shared_ref_to(record) => {
            // Copy constructor.
            if should_derive_clone(record) {
                // `Clone` is derived, so we don't need to generate it explicitly.
                return None;
            }
            let func_name = make_rs_ident("clone");
            let impl_kind = ImplKind::new_trait(
                TraitName::Clone,
                record.clone(),
                /* format_first_param_as_self= */ true,
                /* force_const_reference_params= */ false,
            );
            Some((func_name, impl_kind))
        }
        2 => {
            if param_types[1].is_rvalue_ref_to(record) && record.should_derive_copy() {
                // `MoveAndAssignViaCopy` is derived for `Copy` types, so we don't need to
                // generate move constructor bindings explicitly.
                return None;
            }
            let param_type = &param_types[1];
            let func_name = make_rs_ident("from");
            let impl_kind = ImplKind::new_trait(
                TraitName::UnpinConstructor {
                    name: Rc::from("From"),
                    params: Rc::from([param_type.clone()]),
                },
                record.clone(),
                /* format_first_param_as_self= */ false,
                /* force_const_reference_params= */
                false,
            );
            Some((func_name, impl_kind))
        }
        _ => {
            // TODO(b/216648347): Support bindings for other constructors.
            errors.add(anyhow!(
                "Constructors with more than one parameter are not yet supported. See b/216648347."
            ));
            None
        }
    }
}

/// Returns the shape of the generated Rust API for a given function definition.
///
/// If the shape is a trait, this also mutates the parameter types to be
/// trait-compatible. In particular, types which would be `Ctor![T]`
/// become a `RvalueReference<'_, T>`.
///
/// Returns:
///  * `None`: the function imported as "nothing". (For example, a defaulted
///    destructor might be mapped to no `Drop` impl at all.)
///  * `(func_name, impl_kind)`: The function name and ImplKind.
fn api_func_shape(
    db: &dyn BindingsGenerator,
    func: &Func,
    param_types: &mut [RsTypeKind],
    errors: &Errors,
) -> Option<(Ident, ImplKind)> {
    let ir = db.ir();
    let maybe_record = match func.enclosing_item_id.map(|id| ir.find_untyped_decl(id)) {
        None => None,
        Some(ir::Item::Namespace(_)) => None,
        Some(ir::Item::Record(record)) => Some(record),
        // If the record was replaced by an existing Rust type using `crubit_internal_rust_type`,
        // don't generate any bindings for its functions. (That can't work!)
        Some(ir::Item::ExistingRustType(_)) => return None,
        // (This case should be impossible.)
        // TODO(jeanpierreda): Add an error here.
        Some(_) => return None,
    };

    if is_friend_of_record_not_visible_by_adl(db, func, param_types) {
        return None;
    }

    match &func.rs_name {
        UnqualifiedIdentifier::Operator(op) => {
            api_func_shape_for_operator(db, func, maybe_record, param_types, op, errors)
        }
        UnqualifiedIdentifier::Identifier(id) => {
            Some(api_func_shape_for_identifier(db, func, maybe_record, param_types, id))
        }
        UnqualifiedIdentifier::Destructor => {
            api_func_shape_for_destructor(db, func, maybe_record, param_types)
        }
        UnqualifiedIdentifier::Constructor => {
            api_func_shape_for_constructor(db, func, maybe_record, param_types, errors)
        }
    }
}

/// Returns the shape of the generated Rust API for a given function definition
/// or `None` if no function will be generated.
fn api_func_shape_if_some(
    db: &dyn BindingsGenerator,
    func: &Func,
    param_types: &mut [RsTypeKind],
) -> Option<(Ident, ImplKind)> {
    let errors = Errors::new();
    let shape = api_func_shape(db, func, param_types, &errors);
    if !errors.is_empty() {
        errors.discard();
        return None;
    }
    shape
}

/// Implementation of `BindingsGenerator::get_binding`.
pub fn get_binding(
    db: &dyn BindingsGenerator,
    expected_function_name: UnqualifiedIdentifier,
    expected_param_types: Vec<RsTypeKind>,
) -> Option<(Ident, ImplKind)> {
    db.ir()
        .get_functions_by_name(&expected_function_name)
        .filter(|function| {
            generate_function(db, (*function).clone(), None).ok().flatten().is_some()
        })
        .find_map(|function| {
            let Ok((mut function_param_types, _)) = rs_type_kinds_for_func(db, function) else {
                return None;
            };
            if !function_param_types.iter().eq(expected_param_types.iter()) {
                return None;
            }
            api_func_shape_if_some(db, function, &mut function_param_types)
        })
}

/// Implementation of `BindingsGenerator::is_record_clonable`.
pub fn is_record_clonable(db: &dyn BindingsGenerator, record: Rc<Record>) -> bool {
    if !record.is_unpin() {
        return false;
    }
    should_derive_clone(&record)
        || db
            .ir()
            .get_functions_by_name(&UnqualifiedIdentifier::Constructor)
            .filter(|function| {
                // __this is always the first parameter of constructors
                function.params.len() == 2
            })
            .any(|function| {
                let Ok((mut function_param_types, _)) = rs_type_kinds_for_func(db, function) else {
                    return false;
                };
                if function.params.len() != 2 || !function_param_types[1].is_shared_ref_to(&record)
                {
                    return false;
                }
                api_func_shape_if_some(db, function, &mut function_param_types)
                    .is_some_and(|(func_name, _)| func_name == *"clone")
            })
}

/// Mutates the provided parameters so that nontrivial by-value parameters are,
/// instead, materialized in the caller and passed by rvalue reference.
fn materialize_ctor_in_caller(func: &Func, params: &mut [RsTypeKind]) {
    let mut existing_lifetime_params: HashSet<Rc<str>> =
        params.iter().flat_map(|param| param.lifetimes().map(|lifetime| lifetime.0)).collect();
    let mut new_lifetime_param = |mut lifetime_name: String| {
        let suffix_start = lifetime_name.len();
        let mut next_suffix = 2;
        loop {
            if !existing_lifetime_params.contains(&*lifetime_name) {
                let lifetime_name = <Rc<str>>::from(lifetime_name);
                existing_lifetime_params.insert(lifetime_name.clone());
                return Lifetime(lifetime_name);
            }
            lifetime_name.truncate(suffix_start);
            write!(lifetime_name, "_{next_suffix}").unwrap();
            next_suffix += 1;
        }
    };
    for (func_param, param) in func.params.iter().zip(params.iter_mut()) {
        if param.is_unpin() {
            continue;
        }
        let value = std::mem::replace(param, RsTypeKind::Primitive(Primitive::Void)); // Temporarily swap in a garbage value.
        *param = RsTypeKind::RvalueReference {
            referent: Rc::new(value),
            mutability: Mutability::Mut,
            lifetime: new_lifetime_param(func_param.identifier.identifier.to_string()),
        };
    }
}

/// A series of adjustments to apply to parameter values to make them compatible
/// with the Rust trait signature (e.g. adding `&mut ...` or `.clone()`).
struct ParamValueAdjustments {
    clone_prefixes: Vec<TokenStream>,
    clone_suffixes: Vec<TokenStream>,
}

/// Applies adjustments to the `param_types` of a function to make it compatible
/// with the Rust trait signature.
///
/// If the Rust trait requires a function to take the params by const reference
/// and the thunk takes some of its params by value then we should add a const
/// reference around these Rust func params and clone the records when calling
/// the thunk.
///
/// This function mutates `param_types` in-place and returns a
/// `param_types`-length list containing any necessary adjustments to the
/// parameter values.
fn adjust_param_types_for_trait_impl(
    db: &dyn BindingsGenerator,
    impl_kind: &ImplKind,
    param_types: &mut [RsTypeKind],
    errors: &Errors,
) -> ParamValueAdjustments {
    let ImplKind::Trait { trait_name, force_const_reference_params: true, .. } = impl_kind else {
        return ParamValueAdjustments {
            clone_prefixes: vec![TokenStream::new(); param_types.len()],
            clone_suffixes: vec![TokenStream::new(); param_types.len()],
        };
    };
    let (clone_prefixes, clone_suffixes) = param_types
        .iter_mut()
        .enumerate()
        .map(|(i, param_type)| {
            let RsTypeKind::Record { record: param_record, .. } = &param_type else {
                return Default::default();
            };
            if !is_record_clonable(db, param_record.clone()) {
                errors.add(anyhow!(
                    "Argument {i} of Rust trait `{trait_name:?}` requires a const reference, but the C++ implementation takes non-cloneable record `{}` by value",
                    param_type.display(db),
                ));
                return Default::default();
            }
            *param_type = RsTypeKind::Reference {
                referent: Rc::new(param_type.clone()),
                mutability: Mutability::Const,
                lifetime: Lifetime::new("_"),
            };
            (quote! {&mut }, quote! {.clone()})
        })
        .unzip();
    ParamValueAdjustments { clone_prefixes, clone_suffixes }
}

#[allow(clippy::too_many_arguments)]
fn generate_func_body(
    db: &dyn BindingsGenerator,
    impl_kind: &ImplKind,
    crate_root_path: TokenStream,
    return_type: &RsTypeKind,
    param_value_adjustments: &ParamValueAdjustments,
    thunk_ident: Ident,
    thunk_prepare: TokenStream,
    thunk_args: Vec<TokenStream>,
) -> Result<TokenStream> {
    let ParamValueAdjustments { clone_prefixes, clone_suffixes } = param_value_adjustments;

    match &impl_kind {
        ImplKind::Trait { trait_name: TraitName::UnpinConstructor { .. }, .. }
        | ImplKind::Trait { trait_name: TraitName::Clone, .. }
        | ImplKind::Struct { is_renamed_unpin_constructor: true, .. } => {
            // SAFETY: A user-defined constructor is not guaranteed to
            // initialize all the fields. To make the `assume_init()` call
            // below safe, the memory is zero-initialized first. This is a
            // bit safer, because zero-initialized memory represents a valid
            // value for the currently supported field types (this may
            // change once the bindings generator starts supporting
            // reference fields). TODO(b/213243309): Double-check if
            // zero-initialization is desirable here.
            Ok(quote! {
                #thunk_prepare
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    #crate_root_path::detail::#thunk_ident( &raw mut tmp as *mut _ #( , #thunk_args )* );
                    tmp.assume_init()
                }
            })
        }
        _ => {
            // Note: for the time being, all !Unpin values are treated as if they were not
            // trivially relocatable. We could, in the special case of trivial !Unpin types,
            // not generate the thunk at all, but this would be a bit of extra work.
            //
            // TODO(jeanpierreda): separately handle non-Unpin and non-trivial types.
            let return_type_or_self = {
                let record = match impl_kind {
                    ImplKind::Struct { record, .. }
                    | ImplKind::Trait { record, impl_for: ImplFor::T, .. } => Some(&**record),
                    _ => None,
                };
                return_type.to_token_stream_replacing_by_self(db, record)
            };
            let mut body = match return_type.passing_convention() {
                PassingConvention::AbiCompatible | PassingConvention::Void => {
                    let negate_symbol = if let ImplKind::Trait {
                        trait_name: TraitName::PartialEq { negate_thunk_result: true, .. },
                        ..
                    } = &impl_kind
                    {
                        Some(quote! { ! })
                    } else {
                        None
                    };
                    quote! {
                        #negate_symbol #crate_root_path::detail::#thunk_ident(
                            #( #clone_prefixes #thunk_args #clone_suffixes ),*
                        )
                    }
                }
                PassingConvention::LayoutCompatible => {
                    quote! {
                        let mut __return = ::core::mem::MaybeUninit::<#return_type_or_self>::uninit();
                        #crate_root_path::detail::#thunk_ident(
                            &raw mut __return as *mut ::core::ffi::c_void
                            #( , #clone_prefixes #thunk_args #clone_suffixes )*
                        );
                        __return.assume_init()
                    }
                }
                PassingConvention::ComposablyBridged => {
                    let crubit_abi_type = db.crubit_abi_type(return_type.clone())?;
                    let crubit_abi_type_tokens = CrubitAbiTypeToRustTokens(&crubit_abi_type);
                    let crubit_abi_type_expr_tokens =
                        CrubitAbiTypeToRustExprTokens(&crubit_abi_type);
                    quote! {
                        ::bridge_rust::unstable_return!(@ #crubit_abi_type_expr_tokens, #crubit_abi_type_tokens, |__return_abi_buffer| {
                            #crate_root_path::detail::#thunk_ident(
                                __return_abi_buffer,
                                #(#clone_prefixes #thunk_args #clone_suffixes ),*
                            );
                        })
                    }
                }
                PassingConvention::Ctor => {
                    quote! {
                        ::ctor::FnCtor::new(
                            move |dest: *mut #return_type_or_self| {
                                #crate_root_path::detail::#thunk_ident(
                                    dest as *mut ::core::ffi::c_void
                                    #( , #thunk_args )*
                                );
                            }
                        )
                    }
                }
                PassingConvention::OwnedPtr => {
                    quote! {
                        ::core::mem::transmute(
                            #crate_root_path::detail::#thunk_ident(
                                #( #clone_prefixes #thunk_args #clone_suffixes ),*
                            )
                        )
                    }
                }
            };
            // Discard the return value if requested (for example, when calling a C++
            // operator that returns a value from a Rust trait that returns
            // unit).
            if let ImplKind::Trait { drop_return: true, .. } = impl_kind {
                if return_type.is_unpin() {
                    // If it's unpin, just discard it:
                    body = quote! { #body; };
                } else {
                    // Otherwise, in order to discard the return value and return void, we
                    // need to run the constructor.
                    body = quote! {let _ = ::ctor::emplace!(#body);};
                }

                // We would need to do this, but it's no longer used:
                //    return_type = RsTypeKind::Primitive(PrimitiveType::Unit);
                let _ = return_type; // proof that we don't need to update it.
            }
            // Only need to wrap everything in an `unsafe { ... }` block if
            // the *whole* api function is safe.
            if !impl_kind.is_unsafe() {
                body = quote! { unsafe { #body } };
            }
            Ok(quote! {
                #thunk_prepare
                #body
            })
        }
    }
}

/// A structure describing how to represent any errors that occured as an
/// unsatisfied trait bound. See `errors_as_unsatisfied_trait_bound` for more
/// details.
///
/// Note: a default value of this structure represents no errors.
#[derive(Default)]
struct ErrorsAsUnsatisfiedTraitBound {
    // like `'error`
    lifetime_param: Option<Lifetime>,
    // like "where &'error (): BindingGenerationFailure"
    unsatisfied_where_clause: TokenStream,
    // like `#[diagnostic::on_unimplemented(...)] trait BindingGenerationFailure {}`
    unimplemented_trait_def: TokenStream,
}

/// Represents `reportable_errors` as an unsatisfied trait bound which will
/// report the errors when a user attempts to compile a usage of the generated
/// function or trait that the `unsatisfied_where_clause` is attached to.
///
/// This generates code like:
///
/// ```
/// #[diagnostic::on_unimplemented(message = "binding generation for function failed\n...")]
/// pub trait BindingFailedFor{unique_id} {}
///
/// fn generated_api_func<'a>() where &'error (): BindingFailedFor{unique_id} { unreachable!() }
/// ```
///
/// Note: the `lifetime_param` `'error` is only needed until the
/// `trivial_bounds` feature is stable, see: https://github.com/rust-lang/rust/issues/48214#issuecomment-2557829956
fn errors_as_unsatisfied_trait_bound(
    reportable_errors: &Result<(), ErrorList>,
    unique_id: &str,
) -> ErrorsAsUnsatisfiedTraitBound {
    let Err(reportable_errors) = reportable_errors else {
        return ErrorsAsUnsatisfiedTraitBound::default();
    };
    let lt = Lifetime::new("error");
    let trait_name = format_ident!("BindingFailedFor{}", unique_id);
    let unsatisfied_where_clause = quote! { where & #lt (): #trait_name };
    let message = format!("binding generation for function failed\n{reportable_errors}");
    let unimplemented_trait_def = quote! {
        #[diagnostic::on_unimplemented(message = #message)]
        pub trait #trait_name {}
    };
    ErrorsAsUnsatisfiedTraitBound {
        lifetime_param: Some(lt),
        unsatisfied_where_clause,
        unimplemented_trait_def,
    }
}

/// Returns whether or not the given function should treat reference parameters with unannotated
/// lifetimes as having default/elision-provided lifetimes.
///
/// This is enabled for:
/// - Default constructors
/// - Copy constructors
/// - Move constructors
/// - Operators with default / basic lifetime behavior.
fn func_should_infer_lifetimes_of_references(func: &Func) -> bool {
    use ir::UnqualifiedIdentifier::*;
    match &func.rs_name {
        Destructor | Identifier(_) => false,
        Constructor => true,
        Operator(op_name) => {
            match &*op_name.name {
                "==" | "!=" | "<=>" | "<" | "=" => true,
                // TODO(b/333759161): Temporarily disable inference for `<<` and `>>`, as they
                // creates conflicting libc++ impls for `long` and `long long`.
                "<<" | ">>" => false,
                name => {
                    // Today, all entries in `OPERATOR_METADATA` are "simple" operators (i.e.
                    // they don't have any special lifetime behavior).
                    //
                    // Consider making this check more comprehensive if we ever need to support
                    // non-trivial lifetime behavior for some operator.
                    OPERATOR_METADATA
                        .by_cc_name_and_params
                        .keys()
                        .any(|(meta_name, _num_params)| *meta_name == name)
                }
            }
        }
    }
}

fn rs_type_kinds_for_func(
    db: &dyn BindingsGenerator,
    func: &Func,
) -> Result<(Vec<RsTypeKind>, RsTypeKind)> {

    let errors = Errors::new();
    let infer_lifetimes = func_should_infer_lifetimes_of_references(func);
    let assume_lifetimes = db
        .ir()
        .target_crubit_features(&func.owning_target)
        .contains(crubit_feature::CrubitFeature::AssumeLifetimes);

    // TODO(b/454627672): is it worth caching this?
    let func = if assume_lifetimes { &lifetime_defaults_transform_func(db, func) } else { func };
    let param_types: Vec<RsTypeKind> = func
        .params
        .iter()
        .enumerate()
        .filter_map(|(i, param)| {
            let mut param_type = param.type_.clone();
            if i == 0 && func.is_instance_method() {
                // `param_type` is a `this` pointer, but its semantics are really that of
                // references. That is, `this` in these operators is non-null.
                let CcTypeVariant::Pointer(PointerType { kind, lifetime, pointee_type: _ }) =
                    &mut param_type.variant
                else {
                    panic!(
                        "Expected first parameter of member function:\n`{func:?}`\n\
                        to be a `this` pointer, got:\n{param_type:?}",
                    )
                };
                if infer_lifetimes || lifetime.is_some() {
                    match kind {
                        PointerTypeKind::LValueRef | PointerTypeKind::RValueRef => {}
                        // Fixup pointer-like `this` values to instead be reference-like.
                        PointerTypeKind::Nullable | PointerTypeKind::NonNull => {
                            *kind = PointerTypeKind::LValueRef;
                        }
                        PointerTypeKind::Owned => unreachable!("owned pointers require an annotation on the pointer, but there's nowhere to put an annotation for the `this` pointer")

                    }
                }
            }
            errors.consume_error(
                db.rs_type_kind_with_lifetime_elision(
                    param_type,
                    LifetimeOptions {
                        infer_lifetimes,
                        is_return_type: false,

                        // Only interesting for the return type.
                        have_reference_param: false,
                        assume_lifetimes,
                    },
                )
                .map_err(|err| anyhow!("Failed to format type of parameter {i}: {err}")),
            )
        })
        .collect();

    let return_type = errors.consume_error(
        db.rs_type_kind_with_lifetime_elision(
            func.return_type.clone(),
            LifetimeOptions {
                infer_lifetimes,
                is_return_type: true,

                have_reference_param: param_types.iter().any(|pt| {
                    matches!(pt, RsTypeKind::Reference { .. } | RsTypeKind::RvalueReference { .. })
                }),
                assume_lifetimes,
            },
        )
        .map_err(|err| anyhow!("Failed to format return type: {err}")),
    );

    errors.consolidate()?;
    Ok((param_types, return_type.unwrap()))
}

/// Generate the safety documentation for an unsafe function.
///
/// Returns `None` if the function is not unsafe.
fn generate_func_safety_doc(
    db: &dyn BindingsGenerator,
    func: &Func,
    impl_kind: &ImplKind,
    mut param_idents: &[Ident],
    param_types: &[RsTypeKind],
) -> Option<String> {
    // The first param may have been removed from `param_types` due to
    // being a `this` pointer. Update `param_idents` to match.
    if param_idents.len() == param_types.len() + 1 {
        param_idents = &param_idents[1..];
    }
    assert_eq!(param_idents.len(), param_types.len());

    let mut param_unsafe_reasons = String::new();
    for (ident, param_type) in param_idents.iter().zip(param_types.iter()) {
        if let Some(reason) = db.rs_type_kind_safety(param_type.clone()).unsafe_reason() {
            writeln!(&mut param_unsafe_reasons, "* `{ident}`: {reason}").unwrap();
        }
    }

    if impl_kind.is_unsafe() &&
        // Skip safety doc for trait impls, since the trait itself
        // should document its safety requirements.
        !matches!(impl_kind, ImplKind::Trait { .. })
    {
        let mut doc = String::new();
        if let SafetyAnnotation::Unsafe = func.safety_annotation {
            // TODO(nicholasbishop): allow C++ annotations to provide a specific reason.
            doc += "The C++ function is explicitly annotated as unsafe. Ensure that its safety requirements are upheld.\n\n";
        }
        if !param_unsafe_reasons.is_empty() {
            write!(doc, "The caller must ensure that the following unsafe arguments are not misused by the function:\n{param_unsafe_reasons}").unwrap();
        }
        // Verify that we didn't generate an empty safety doc.
        assert!(!doc.is_empty());
        Some(doc)
    } else {
        None
    }
}

/// Implementation of `BindingsGenerator::generate_function`.
pub fn generate_function(
    db: &dyn BindingsGenerator,
    func: Rc<Func>,
    derived_record: Option<Rc<Record>>,
) -> Result<Option<GeneratedFunction>> {
    let _scope = ir::Item::Func(func.clone()).error_scope(db.ir(), db.errors());
    db.errors().add_category(error_report::Category::Function);
    let ir = db.ir();
    let crate_root_path = ir.crate_root_path_tokens();
    let mut features = FlagSet::empty();
    let (mut param_types, mut return_type) = rs_type_kinds_for_func(db, &func)?;

    let errors = Errors::new();
    let (func_name, mut impl_kind) =
        if let Some(values) = api_func_shape(db, &func, &mut param_types, &errors) {
            values
        } else {
            errors.consolidate()?;
            return Ok(None);
        };
    let namespace_qualifier = ir.namespace_qualifier(&func).format_for_rs();

    if let Err(err) = return_type.check_by_value() {
        // If the return type is not valid, we can't generate even a fake thunk, so we must return
        // immediately.
        errors.add(err);
        errors.consolidate()?;
    }
    let param_idents =
        func.params.iter().map(|p| make_rs_ident(&p.identifier.identifier)).collect_vec();
    let thunk: Option<Thunk> = if derived_record.is_some() {
        None
    } else {
        errors.consume_error(generate_function_thunk(
            db,
            &func,
            &param_idents,
            &param_types,
            &return_type,
        ))
    };

    let param_value_adjustments =
        adjust_param_types_for_trait_impl(db, &impl_kind, &mut param_types, &errors);

    // TODO(b/454627672): Possibly amend the logic around lifetime binding here for
    // assume_lifetimes.

    let BindingsSignature {
        mut lifetimes,
        params: api_params,
        return_type_fragment: mut quoted_return_type,
        thunk_prepare,
        thunk_args,
    } = errors.consolidate_on_err(function_signature(
        db,
        &mut features,
        &func,
        &impl_kind,
        &param_idents,
        &mut param_types,
        &mut return_type,
        derived_record.clone(),
        &errors,
    ))?;

    if let ImplKind::Trait { drop_return: true, .. } = impl_kind {
        quoted_return_type = quote! {};
    }

    if !errors.is_empty() {
        if let ImplKind::Trait { trait_name: TraitName::CtorNew(_), .. } = impl_kind {
            // Generated CtorNew functions return an `impl Trait` type which can't use
            // the `errors_as_unsatisfied_trait_bound` reporting system because
            // the `'error` lifetime causes an error when combined with `impl Trait due to
            // https://github.com/rust-lang/rust/issues/134804
            errors.consolidate()?;
        }
    }

    let reportable_status: Result<(), ErrorList> = errors.consolidate();
    let failed = reportable_status.is_err();

    let (derived_class_prefix, sep) = if let Some(ref derived_record) = derived_record {
        (derived_record.mangled_cc_name.as_ref(), "_")
    } else {
        ("", "")
    };
    let ErrorsAsUnsatisfiedTraitBound {
        lifetime_param: error_lifetime_param,
        mut unsatisfied_where_clause,
        unimplemented_trait_def,
    } = errors_as_unsatisfied_trait_bound(
        &reportable_status,
        &format!("{sep}{derived_class_prefix}{sep}{}", &func.mangled_name),
    );

    let api_func_def = {
        let thunk_ident = thunk_ident(&func);

        let func_body = if reportable_status.is_ok() {
            generate_func_body(
                db,
                &impl_kind,
                crate_root_path,
                &return_type,
                &param_value_adjustments,
                thunk_ident,
                thunk_prepare,
                thunk_args,
            )?
        } else {
            let mut result = quote! {
                #![allow(unused_variables)]
                unreachable!(
                    "This impl can never be instantiated. \
                    If this message appears at runtime, please report a crubit.rs-bug."
                )
            };
            if !return_type.is_unpin() {
                result.extend(quote! {
                    ; #[allow(unreachable_code)]
                    ::ctor::UnreachableCtor::new()
                });
            }
            result
        };

        // If there are no bindings, use `Public` for the sake of "keeping on going" when
        // collecting errors for items that will not actually be generated.
        let visibility =
            db.has_bindings(ir::Item::Func(func.clone())).unwrap_or_default().visibility;
        let pub_ = match &impl_kind {
            ImplKind::Trait { trait_name, always_public: false, .. }
                if visibility == Visibility::PubCrate =>
            {
                bail!("Implementation of {trait_name} cannot be restricted to wrappers with pub(crate)")
            }
            ImplKind::Trait { .. } => quote! {},
            _ => quote! {#visibility},
        };
        let unsafe_ = if impl_kind.is_unsafe() {
            quote! { unsafe }
        } else {
            quote! {}
        };

        // If we are generating a trait impl, its `where` clause will be on the `impl` item.
        // Otherwise, it must be on the `fn` item.
        let where_clause_on_impl = match impl_kind {
            ImplKind::Trait { .. } => true,
            // Free functions have no impl.
            ImplKind::None { .. } => false,
            // Struct functions are placed in an inherent impl block later by `generate_record`, but
            // inherent impls cannot take where clauses in the same way that trait impls can, so the
            // where clause must be on the function.
            ImplKind::Struct { .. } => false,
        };
        let where_clause = if where_clause_on_impl {
            None
        } else {
            if let Some(lt) = &error_lifetime_param {
                lifetimes.insert(0, lt.clone());
            }
            Some(core::mem::take(&mut unsatisfied_where_clause))
        };

        let fn_generic_params: TokenStream;
        if let ImplKind::Trait { trait_name, trait_generic_params, impl_for, .. } = &mut impl_kind {
            // When the impl block is for some kind of reference to T, consider the lifetime
            // parameters on the self parameter to be trait lifetimes so they can be
            // introduced before they are used.
            let first_param_lifetimes = match (impl_for, param_types.first()) {
                (ImplFor::RefT, Some(first_param)) => Some(first_param.lifetimes()),
                _ => None,
            };

            let trait_lifetimes: HashSet<Lifetime> =
                trait_name.lifetimes().chain(first_param_lifetimes.into_iter().flatten()).collect();
            fn_generic_params = format_generic_params(
                lifetimes.iter().filter(|lifetime| !trait_lifetimes.contains(lifetime)),
                std::iter::empty::<syn::Ident>(),
            );
            *trait_generic_params = Rc::from(
                lifetimes
                    .iter()
                    .filter_map(|lifetime| {
                        if trait_lifetimes.contains(lifetime) {
                            Some(lifetime.clone())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<Lifetime>>(),
            );
        } else {
            fn_generic_params = format_generic_params(&lifetimes, std::iter::empty::<syn::Ident>());
        }

        let function_return_type = match &impl_kind {
            ImplKind::Trait { associated_return_type: Some(ident), .. } => quote! {Self::#ident},
            _ => quoted_return_type.clone(),
        };
        let arrow = if !function_return_type.is_empty() {
            quote! {->}
        } else {
            quote! {}
        };

        quote! {
            #[inline(always)]
            #pub_ #unsafe_ fn #func_name #fn_generic_params(
                    #( #api_params ),* ) #arrow #function_return_type #where_clause {
                #func_body
            }
        }
    };

    let doc_comment = generate_doc_comment(
        func.doc_comment.as_deref(),
        generate_func_safety_doc(db, &func, &impl_kind, &param_idents, &param_types).as_deref(),
        Some(&func.source_loc),
        db.environment(),
    );
    let api_func: TokenStream;
    let function_id: FunctionId;
    let mut member_functions_map = HashMap::new();
    match impl_kind {
        ImplKind::None { .. } => {
            api_func = quote! { #unimplemented_trait_def #doc_comment #api_func_def };
            function_id = FunctionId {
                self_type: None,
                function_path: syn::parse2(quote! { #namespace_qualifier #func_name }).unwrap(),
            };
        }
        ImplKind::Struct { record, .. } => {
            let record_name = make_rs_ident(
                derived_record.as_deref().unwrap_or(record.as_ref()).rs_name.identifier.as_ref(),
            );
            member_functions_map.insert(
                derived_record.as_deref().unwrap_or(record.as_ref()).id,
                vec![quote! { #unsatisfied_where_clause #doc_comment #api_func_def }],
            );
            api_func = quote! {
                #unimplemented_trait_def
            };
            function_id = FunctionId {
                self_type: None,
                function_path: syn::parse2(quote! {
                    #namespace_qualifier #record_name :: #func_name
                })
                .unwrap(),
            };
        }
        ImplKind::Trait {
            record: trait_record,
            trait_name,
            impl_for,
            trait_generic_params,
            associated_return_type,
            ..
        } => {
            let mut extra_body = if let Some(name) = associated_return_type {
                let quoted_return_type = if quoted_return_type.is_empty() {
                    quote! {()}
                } else {
                    quoted_return_type
                };
                quote! {
                    type #name = #quoted_return_type;
                }
            } else if let TraitName::PartialOrd { param } = &trait_name {
                let quoted_param_or_self = match impl_for {
                    ImplFor::T => param.to_token_stream_replacing_by_self(db, Some(&trait_record)),
                    ImplFor::RefT => param.to_token_stream(db),
                };
                quote! {
                    #[inline(always)]
                    fn partial_cmp(&self, other: & #quoted_param_or_self) -> Option<core::cmp::Ordering> {
                        if self == other {
                            return Some(core::cmp::Ordering::Equal);
                        }
                        if self < other {
                            return Some(core::cmp::Ordering::Less);
                        }
                        if other < self {
                            return Some(core::cmp::Ordering::Greater);
                        }
                        None
                    }
                }
            } else {
                quote! {}
            };
            if matches!(trait_name, TraitName::CtorNew(_)) {
                extra_body.extend(quote! {
                    type Error = ::ctor::Infallible;
                });
            }

            let record_name = make_rs_ident(trait_record.rs_name.identifier.as_ref());
            let trait_lifetime_params = error_lifetime_param.as_slice();
            // NOTE: `trait_generic_params` may include lifetimes!
            let formatted_trait_generic_params =
                format_generic_params(trait_lifetime_params, &*trait_generic_params);
            let extra_items = match &trait_name {
                TraitName::CtorNew(params) if params.len() == 1 => {
                    let single_param_ = format_tuple_except_singleton_replacing_by_self(
                        db,
                        params,
                        Some(&trait_record),
                    );
                    quote! {
                        impl #formatted_trait_generic_params ::ctor::CtorNew<(#single_param_,)> for #record_name #unsatisfied_where_clause {
                            #extra_body

                            #[inline (always)]
                            fn ctor_new(args: (#single_param_,)) -> Self::CtorType {
                                let (arg,) = args;
                                <Self as ::ctor::CtorNew<#single_param_>>::ctor_new(arg)
                            }
                        }
                    }
                }
                TraitName::UnpinConstructor { name, params }
                    if *name == Rc::from("From") && reportable_status.is_ok() =>
                {
                    let single_param_ = format_tuple_except_singleton_replacing_by_self(
                        db,
                        params,
                        Some(&trait_record),
                    );
                    quote! {
                        impl #formatted_trait_generic_params ::ctor::CtorNew<#single_param_> for #record_name #unsatisfied_where_clause {
                            type CtorType = Self;
                            type Error = ::ctor::Infallible;

                            #[inline (always)]
                            fn ctor_new(args: #single_param_) -> Self::CtorType {
                                <Self as From<#single_param_>>::from(args)
                            }
                        }
                    }
                }
                _ => {
                    quote! {}
                }
            };

            let extra_api_func_def = match &trait_name {
                // Check if the current function triggers Clone generation and if we also have a
                // copy assignment operator also generate clone_from.
                TraitName::Clone
                    if reportable_status.is_ok()
                        && has_copy_assignment_operator_from_const_reference(db, &func) =>
                {
                    Some(quote! {
                        fn clone_from(&mut self, other: &Self) {
                            use ::ctor::UnpinAssign;
                            self.unpin_assign(other);
                        }
                    })
                }
                _ => None,
            };

            let record_qualifier = ir.namespace_qualifier(&trait_record).format_for_rs();
            let full_record_qualifier = if Some(trait_record.id) == func.enclosing_item_id {
                // If the method is defined in the record, then the record qualifier is not
                // needed for better readability.
                quote! {}
            } else {
                quote! { crate :: #record_qualifier }
            };
            let (trait_name_without_trait_record, impl_for) = match impl_for {
                ImplFor::T => (
                    trait_name_to_token_stream_removing_trait_record(
                        db,
                        &trait_name,
                        Some(&trait_record),
                    ),
                    quote! { #full_record_qualifier #record_name },
                ),
                ImplFor::RefT => (
                    trait_name_to_token_stream(db, &trait_name),
                    param_types[0].to_token_stream(db),
                ),
            };
            api_func = quote! {
                #unimplemented_trait_def
                #doc_comment
                impl #formatted_trait_generic_params #trait_name_without_trait_record for #impl_for #unsatisfied_where_clause {
                    #extra_body
                    #api_func_def
                    #extra_api_func_def
                }
                #extra_items
            };
            function_id = FunctionId {
                self_type: Some(syn::parse2(quote! { #record_qualifier #record_name }).unwrap()),
                function_path: {
                    let trait_name_tokens = trait_name_to_token_stream(db, &trait_name);
                    syn::parse2(quote! { #trait_name_tokens :: #func_name }).unwrap()
                },
            };
        }
    }

    // If we are generating bindings for a derived record, we reuse the base
    // record's thunks, so we don't need to generate thunks.
    let thunk_impl = if derived_record.is_some() || failed {
        None
    } else {
        generate_function_thunk_impl(db, &func)?
    };

    let function_assertation =
        if failed { None } else { generate_function_assertation(db, &func)? };

    let cc_details = [thunk_impl, function_assertation].into_iter().flatten().collect();

    let generated_item = ApiSnippets {
        generated_items: HashMap::from([(func.id, GeneratedItem::Func(api_func))]),
        thunks: match thunk {
            Some(thunk) if !failed => vec![thunk],
            _ => vec![],
        },
        features,
        cc_details,
        member_functions: member_functions_map,
        ..Default::default()
    };

    Ok(Some(GeneratedFunction {
        snippets: Rc::new(generated_item),
        id: Rc::new(function_id),
        status: reportable_status.map_err(arc_anyhow::Error::from),
    }))
}

/// The function signature for a function's bindings.
struct BindingsSignature {
    /// The lifetime parameters for the Rust function.
    lifetimes: Vec<Lifetime>,

    /// The parameter list for the Rust function.
    ///
    /// For example, `vec![quote!{self}, quote!{x: &i32}]`.
    params: Vec<TokenStream>,

    /// The return type fragment of the Rust function, as a token stream.
    ///
    /// This is the same as the actual return type, except that () is the empty
    /// tokens, non-Unpin by-value types are `Ctor![#return_type] + ...`,
    /// and wherever the type is the type of `Self`, it gets replaced by
    /// literal `Self`.
    return_type_fragment: TokenStream,

    /// Any preparation code to define the arguments in `thunk_args`.
    thunk_prepare: TokenStream,

    /// A list of expressions that refer to arguments passed to the thunk.
    /// For example, if `params` is `vec![quote!{mut self}]`, this might be
    /// `vec![quote!{&mut self}]` since the thunk takes non-C-ABI values by reference.
    thunk_args: Vec<TokenStream>,
}

/// Reformats API parameters and return values to match Rust conventions and the
/// trait requirements.
///
/// For example:
///
/// * Use the `self` keyword for the this pointer. Upcast to base classed as
///   needed.
/// * Use `Self` for the return value of constructor traits.
/// * For C++ constructors, remove `self` from the Rust side (as it becomes the
///   return value), retaining it on the C++ side / thunk args.
/// * serialize a `()` as the empty string.
#[allow(clippy::too_many_arguments)]
fn function_signature(
    db: &dyn BindingsGenerator,
    features: &mut FlagSet<Feature>,
    func: &Func,
    impl_kind: &ImplKind,
    param_idents: &[Ident],
    param_types: &mut Vec<RsTypeKind>,
    return_type: &mut RsTypeKind,
    derived_record: Option<Rc<Record>>,
    errors: &Errors,
) -> Result<BindingsSignature> {
    if let Some(derived_record) = derived_record.as_deref() {
        ensure!(
            db.ir()
                .target_crubit_features(&derived_record.owning_target)
                .contains(crubit_feature::CrubitFeature::Experimental),
            "upcasting is currently experimental, see b/216195042"
        );
    }

    let mut api_params = Vec::with_capacity(func.params.len());
    let mut thunk_args = Vec::with_capacity(func.params.len());
    let mut thunk_prepare = quote! {};
    let impl_kind_record = match impl_kind {
        ImplKind::Struct { record, .. } | ImplKind::Trait { record, impl_for: ImplFor::T, .. } => {
            Some(record)
        }
        _ => None,
    };

    for (i, (ident, type_)) in param_idents.iter().zip(param_types.iter()).enumerate() {
        // If we are generating bindings for a derived record, parameter types should be
        // kept the same because `Self` will refer to the derived record type.
        // One exception is the first parameter, as it points to the derived
        // record.
        let should_replace_by_self = derived_record.is_none() || i == 0;
        if let Err(err) = type_.check_by_value() {
            errors.add(err);
        }
        let quoted_type_or_self =
            if let (Some(impl_record), true) = (impl_kind_record, should_replace_by_self) {
                type_.to_token_stream_replacing_by_self(db, Some(impl_record))
            } else {
                type_.to_token_stream_with_owned_ptr_type(db)
            };

        match type_.passing_convention() {
            PassingConvention::AbiCompatible => {
                api_params.push(quote! {#ident: #quoted_type_or_self});
                thunk_args.push(quote! {#ident});
            }
            PassingConvention::LayoutCompatible => {
                api_params.push(quote! {mut #ident: #quoted_type_or_self});
                thunk_args.push(quote! {&mut #ident});
            }
            PassingConvention::ComposablyBridged => {
                let crubit_abi_type = db
                    .crubit_abi_type(type_.clone())
                    .with_context(|| format!("while generating bridge param '{ident}'"))?;
                let crubit_abi_type_tokens = CrubitAbiTypeToRustTokens(&crubit_abi_type);
                let crubit_abi_type_expr_tokens = CrubitAbiTypeToRustExprTokens(&crubit_abi_type);

                api_params.push(quote! {#ident: #quoted_type_or_self});
                thunk_args.push(quote! {::bridge_rust::unstable_encode!(@ #crubit_abi_type_expr_tokens, #crubit_abi_type_tokens, #ident).as_ptr() as *const u8});
            }
            PassingConvention::Ctor => {
                // `impl Ctor` will fail to compile in a trait.
                // This will only be hit if there was a bug in api_func_shape.
                if let ImplKind::Trait { .. } = &impl_kind {
                    panic!(
                        "non-Unpin types cannot work by value in traits; this should have instead \
                        become an rvalue reference to force the caller to materialize the Ctor."
                    );
                }
                // The generated bindings require a move constructor.
                if !type_.is_move_constructible() {
                    errors.add(anyhow!("Non-movable, non-trivial_abi type '{}' is not supported by value as parameter #{i}", type_.display(db)));
                }
                *features |= Feature::impl_trait_in_assoc_type;
                api_params.push(quote! {#ident: ::ctor::Ctor![#quoted_type_or_self]});
                thunk_args.push(
                    quote! {::core::pin::Pin::into_inner_unchecked(::ctor::emplace!(#ident))},
                );
            }
            PassingConvention::OwnedPtr => {
                api_params.push(quote! {#ident: #quoted_type_or_self});
                thunk_args.push(quote! {::core::mem::transmute(#ident)});
            }
            PassingConvention::Void => {
                unreachable!("parameter types should never be void")
            }
        }
    }

    let mut lifetimes: Vec<Lifetime> = unique_lifetimes(&*param_types).collect();

    let mut quoted_return_type = None;

    if let ImplKind::Struct { is_renamed_unpin_constructor: true, .. } = impl_kind {
        move_self_from_out_param_to_return_value(
            db,
            func,
            return_type,
            &mut api_params,
            &mut thunk_args,
            param_types,
            &mut lifetimes,
        )?;
        quoted_return_type = Some(quote! { Self });
    }

    // TODO: b/389131731 - Unify adjustment of return and parameter types.
    let trait_name = match &impl_kind {
        ImplKind::Trait { trait_name, .. } => Some(trait_name),
        _ => None,
    };
    match trait_name {
        Some(TraitName::PartialOrd { .. } | TraitName::PartialEq { .. }) => {
            if *return_type != RsTypeKind::Primitive(Primitive::Bool) {
                errors.add(anyhow!(
                    "comparison operator return type must be `bool`, found: {}",
                    return_type.display(db),
                ));
                *return_type = RsTypeKind::Primitive(Primitive::Bool);
            }
        }
        Some(TraitName::UnpinConstructor { .. } | TraitName::CtorNew(..) | TraitName::Clone) => {
            move_self_from_out_param_to_return_value(
                db,
                func,
                return_type,
                &mut api_params,
                &mut thunk_args,
                param_types,
                &mut lifetimes,
            )?;
            quoted_return_type = Some(quote! { Self });

            // CtorNew groups parameters into a tuple.
            if let Some(TraitName::CtorNew(args_type)) = trait_name {
                let args_type = if let Some(impl_record) = impl_kind_record {
                    format_tuple_except_singleton_replacing_by_self(
                        db,
                        args_type,
                        Some(impl_record),
                    )
                } else {
                    format_tuple_except_singleton(
                        args_type.iter().map(|rs_type_kind| rs_type_kind.to_token_stream(db)),
                    )
                };
                api_params = vec![quote! {args: #args_type}];
                let param_idents_without_this = param_idents.iter().skip(1);
                let thunk_vars = format_tuple_except_singleton(
                    param_idents_without_this.map(|ts| quote! { mut #ts }),
                );
                // We must unpack the input args from a tuple before doing other preparation.
                thunk_prepare = {
                    let mut thunk_prepare_new = quote! {
                        let #thunk_vars = args;
                    };
                    thunk_prepare_new.extend(thunk_prepare);
                    thunk_prepare_new
                };
            }
        }
        Some(TraitName::Other { .. }) | None => {}
    }

    let return_type_fragment = if matches!(
        return_type.unalias(),
        RsTypeKind::Primitive(Primitive::Void)
    ) {
        quote! {}
    } else {
        let ty = quoted_return_type
            .unwrap_or_else(|| return_type.to_token_stream_with_owned_ptr_type(db));
        if return_type.is_unpin() {
            ty
        } else {
            // TODO(jeanpierreda): use `-> impl Ctor` instead of `-> Self::X` where `X = impl
            // Ctor`. The latter requires `impl_trait_in_assoc_type`, the former
            // was stabilized in 1.75. Directly returning an unnameable `impl
            // Ctor` is sufficient for us, and makes traits like `CtorNew` more
            // similar to top-level functions.)

            // The returned lazy FnCtor depends on all inputs.
            let extra_lifetimes = if lifetimes.is_empty() {
                quote! {}
            } else {
                quote! {+ use<#(#lifetimes),*> }
            };
            *features |= Feature::impl_trait_in_assoc_type;
            if extra_lifetimes.is_empty() {
                quote! {::ctor::Ctor![#ty]}
            } else {
                quote! {impl ::ctor::Ctor<Output=#ty, Error=::ctor::Infallible> #extra_lifetimes }
            }
        }
    };

    // Change `__this: &'a SomeStruct` into `&'a self` if needed.
    if impl_kind.format_first_param_as_self() {
        let Some(first_api_param) = param_types.first() else {
            panic!(
                "`format_first_param_self` to function with no parameter types:\n\
                Func: {func:?}\nParameter types: {param_types:?}"
            )
        };
        // If param_types[0] exists, so do api_params[0] and thunk_args[0].
        match impl_kind {
            ImplKind::None { .. } => unreachable!(),
            ImplKind::Struct { .. } | ImplKind::Trait { impl_for: ImplFor::T, .. } => {
                // In the ImplFor::T reference style (which is implied for ImplKind::Struct) the
                // impl block is for `T`. The `self` parameter has a type determined by the
                // first parameter (typically a reference of some kind) and can be passed to a
                // thunk via the expression `self`.
                if first_api_param.is_c_abi_compatible_by_value() {
                    let rs_snippet = first_api_param.format_as_self_param()?;
                    thunk_args[0] = if derived_record.is_some() {
                        quote! { oops::Upcast::<_>::upcast(self) }
                    } else {
                        quote! { self }
                    };
                    api_params[0] = rs_snippet.tokens;
                    *features |= rs_snippet.features;
                } else {
                    api_params[0] = quote! { mut self };
                    if derived_record.is_some() {
                        thunk_args[0] = quote! { oops::Upcast::<_>::upcast(&mut self) };
                    } else {
                        thunk_args[0] = quote! { &mut self };
                    }
                }
            }
            ImplKind::Trait { impl_for: ImplFor::RefT, .. } => {
                // In the ImplFor::RefT reference style the impl block is for a reference type
                // referring to T (`&T`, `&mut T`, or `Pin<&mut T>` so a bare `self` parameter
                // has that type and can be passed to a thunk via the expression `self`.
                api_params[0] = quote! { self };
                if derived_record.is_some() {
                    thunk_args[0] = quote! { oops::Upcast::<_>::upcast(self) };
                } else {
                    thunk_args[0] = quote! { self };
                }
            }
        }
    } else if derived_record.is_some()
        && !thunk_args.is_empty()
        && thunk_args[0].to_string() == "__this"
    {
        let arg_this = thunk_args[0].clone();
        thunk_args[0] = quote! { oops::UnsafeUpcast::<_>::unsafe_upcast(#arg_this) };
    }

    // Unknown attributes could affect ABI and should suppress bindings by default. Note that these
    // can be annotated around with `CRUBIT_UNSAFE_IGNORE_ATTR()`
    if let Some(unknown_attr) = func.unknown_attr.as_deref() {
        let target = &func.owning_target;
        let enabled_features = db.ir().target_crubit_features(target);
        ensure!(
            enabled_features.contains(crubit_feature::CrubitFeature::Experimental),
            "crubit.rs/errors/unknown_attribute: unknown function attributes are only supported with experimental features enabled on \
            {target}\nUnknown attribute: {unknown_attr}`",
        );
    }

    Ok(BindingsSignature {
        lifetimes,
        params: api_params,
        return_type_fragment,
        thunk_prepare,
        thunk_args,
    })
}

fn move_self_from_out_param_to_return_value(
    db: &dyn BindingsGenerator,
    func: &Func,
    return_type: &mut RsTypeKind,
    api_params: &mut Vec<TokenStream>,
    thunk_args: &mut Vec<TokenStream>,
    param_types: &mut Vec<RsTypeKind>,
    lifetimes: &mut Vec<Lifetime>,
) -> Result<()> {
    // For constructors, we move the output parameter to be the return value.
    // The return value is "really" void.
    assert!(
        func.return_type.is_unit_type(),
        "Unexpectedly non-void return type of a constructor: {func:?}"
    );

    //  Presence of element #0 is indirectly verified by a `Constructor`-related
    // `match` branch a little bit above.
    *return_type = param_types[0]
        .referent()
        .ok_or_else(|| {
            anyhow!(
                "Expected pointer/reference for `__this` parameter, found {}",
                param_types[0].display(db)
            )
        })?
        .clone();

    // Grab the `__this` lifetime to remove it from the lifetime parameters.
    let this_lifetime = param_types[0].lifetime();

    // Drop `__this` parameter from the public Rust API.
    api_params.remove(0);
    thunk_args.remove(0);
    param_types.remove(0);

    // Remove the lifetime associated with `__this`.
    if let Some(this_lifetime) = this_lifetime {
        lifetimes.retain(|l| l != &this_lifetime);
        if let Some(type_still_dependent_on_removed_lifetime) = param_types
            .iter()
            .find(|t| t.lifetimes().filter(|lt| !lt.is_elided()).any(|lt| lt == this_lifetime))
        {
            bail!(
                "The lifetime of `__this` is unexpectedly also used by another \
            parameter: {}",
                type_still_dependent_on_removed_lifetime.display(db)
            );
        }
    }
    Ok(())
}

/// Formats singletons as themselves, and collections of n!=1 items as a tuple.
///
/// In other words, this formats a collection of things as if via `#(#items),*`,
/// but without lint warnings.
///
/// For example:
///
/// * [] => ()
/// * [x] => x  // equivalent to (x), but lint-free.
/// * [x, y] => (x, y)
fn format_tuple_except_singleton(iter: impl IntoIterator<Item = TokenStream>) -> TokenStream {
    let mut items = iter.into_iter();
    let Some(first) = items.next() else {
        return quote! { () };
    };

    let Some(second) = items.next() else {
        // If there's no second item, then return the first _without_ parens.
        return first;
    };

    quote! { (#first, #second #(, #items)*) }
}

fn format_tuple_except_singleton_replacing_by_self(
    db: &dyn BindingsGenerator,
    items: &[RsTypeKind],
    trait_record: Option<&Record>,
) -> TokenStream {
    match items {
        [singleton] => singleton.to_token_stream_replacing_by_self(db, trait_record),
        items => {
            let mut elements_of_tuple = quote! {};
            for (type_index, type_) in items.iter().enumerate() {
                let quoted_type_or_self = type_.to_token_stream_replacing_by_self(db, trait_record);
                if type_index > 0 {
                    (quote! {, #quoted_type_or_self }).to_tokens(&mut elements_of_tuple);
                } else {
                    quoted_type_or_self.to_tokens(&mut elements_of_tuple);
                }
            }
            quote! { ( #elements_of_tuple ) }
        }
    }
}

/// Returns true if the copy constructor has the following form:
///
/// operator=(const SameType&);
///
/// where `SameType` is the same type as the class this function is declared in.
fn has_copy_assignment_operator_from_const_reference(
    db: &dyn BindingsGenerator,
    copy_constructor: &Func,
) -> bool {
    let [_self, first_param] = &copy_constructor.params[..] else {
        return false;
    };
    let first_param_type = db.rs_type_kind(first_param.type_.clone());
    if first_param_type.is_err() {
        return false;
    };
    let Some(parent_id) = copy_constructor.enclosing_item_id else {
        return false;
    };
    let Ok(record) = db.ir().find_decl::<Rc<Record>>(parent_id) else {
        return false;
    };
    record
        .child_item_ids
        .iter()
        .filter_map(|&child_item_id| db.ir().find_decl::<Rc<Func>>(child_item_id).ok())
        .any(|func| {
            let operator_equals = matches!(&func.cc_name,
                    UnqualifiedIdentifier::Operator(op) if op.name.as_ref() == "=");
            let same_as_self = matches!(&func.params[..],
                    [_self, other] if db.rs_type_kind(other.type_.clone()) == first_param_type);
            operator_equals && same_as_self
        })
}

/// Implementation of `BindingsGenerator::overload_sets`.
pub fn overload_sets(
    db: &dyn BindingsGenerator,
) -> Rc<HashMap<Rc<FunctionId>, Option<ir::ItemId>>> {
    #[derive(Copy, Clone)]
    struct CandidateFunction {
        item_id: ir::ItemId,
        rank: Rank,
        is_ambiguous: bool,
    }
    /// A quick and dirty total order over functions.
    ///
    /// If a single unique function in the overload set is the most canonical / highest rank, then
    /// it is unambiguous, and gets bindings.
    ///
    /// Using an enum, rather than a bool, because it is very predictable that we might
    /// add extra ranking factors. For example, a `CRUBIT_CANONICAL` attribute.
    // TODO(b/381931334): Allow explicitly marking functions as canonical,
    // adding e.g. `Canonical = 2`.
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    #[repr(u8)]
    enum Rank {
        Deprecated = 0,
        Default = 1,
    }
    // Map from function name to the most canonical candidate function in that overload set.
    let mut overload_sets = HashMap::new();

    fn rank(func: &Func) -> Rank {
        if func.deprecated.is_some() {
            Rank::Deprecated
        } else {
            Rank::Default
        }
    }

    for func in db.ir().functions() {
        // TODO(b/251045039) This check shouldn't fail so eagerly.
        // Functions that fail to receive bindings may still
        // participate in a C++ overload set, and we must still detect the
        // overload.
        let Ok(Some(generated_function)) = db.generate_function(func.clone(), None) else {
            continue;
        };
        let new = CandidateFunction { item_id: func.id, rank: rank(func), is_ambiguous: false };
        let value = overload_sets.entry(generated_function.id.clone()).or_insert(new);
        if value.item_id == new.item_id {
            continue;
        }
        if new.rank > value.rank {
            *value = new;
        } else if new.rank == value.rank {
            value.is_ambiguous = true;
        }
    }
    Rc::new(
        overload_sets
            .into_iter()
            .map(
                |(id, canonical)| {
                    if canonical.is_ambiguous {
                        (id, None)
                    } else {
                        (id, Some(canonical.item_id))
                    }
                },
            )
            .collect(),
    )
}
