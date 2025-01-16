// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::db::BindingsGenerator;
use crate::generate_function_thunk::{
    generate_function_thunk, generate_function_thunk_impl, thunk_ident,
    thunk_ident_for_derived_member_function,
};
use crate::ApiSnippets;

use crate::rs_snippet::{
    check_by_value, format_generic_params, format_generic_params_replacing_by_self,
    should_derive_clone, unique_lifetimes, Lifetime, Mutability, PrimitiveType, RsTypeKind,
};
use arc_anyhow::{Context, Result};
use code_gen_utils::make_rs_ident;
use error_report::{anyhow, bail, ErrorList};
use errors::{bail_to_errors, Errors, ErrorsOr};
use ir::*;
use itertools::Itertools;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::Write as _;
use std::ptr;
use std::rc::Rc;
use std::sync::LazyLock;

#[derive(Clone)]
pub struct GeneratedFunction {
    /// The generated Rust function.
    pub snippets: Rc<ApiSnippets>,
    /// The function's ID.
    pub id: Rc<FunctionId>,
    /// The status of function generation.
    /// If this is `Err`, the function or trait impl exists, but is not
    /// callable.
    pub status: Result<()>,
}

/// Uniquely identifies a generated Rust function.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FunctionId {
    // If the function is on a trait impl, contains the name of the Self type for
    // which the trait is being implemented.
    self_type: Option<syn::Path>,
    // Fully qualified path of the function. For functions in impl blocks, this
    // includes the name of the type or trait on which the function is being
    // implemented, e.g. `Default::default`.
    pub function_path: syn::Path,
}

/// The name of a one-function trait, with extra entries for
/// specially-understood traits and families of traits.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TraitName {
    /// The constructor trait for !Unpin types, with a list of parameter types.
    /// For example, `CtorNew(vec![])` is the default constructor.
    CtorNew(Rc<[RsTypeKind]>),
    /// An Unpin constructor trait, e.g. From or Clone, with a list of parameter
    /// types.
    UnpinConstructor {
        name: Rc<str>,
        // /// Clonable, comparable token stream, which can be copied into a new TokenStream.
        // #[repr(transparent)]
        // struct TokenArray(Rc<[TokenTree]>);
        // // impl From<TokenStream> for TokenArray, From<TokenArray> for TokenStream, PartialEq,
        // Eq, Hash, etc.

        // This avoids deferred parsing.

        // I just can't figure out how to make the equality check not prohibitively ugly:

        // impl PartialEq for TokenArray {
        //   fn eq(&self, other: &TokenArray) {
        //     struct EqTokenTree<'a>(&'a TokenTree);
        //     impl PartialEq for EqTokenTree {
        //       fn eq(&self, other: &EqTokenTree) {
        //         match (&self.0, &other.0) {
        //           (Group(g1), Group(g2)) => g1.delimiter() == g2.delimiter(),
        //           (Ident(i1), Ident(i2)) => i1 == i2,
        //           (Punct(p1), Punct(p2)) => p1.as_char() == p2.as_char(),
        //           (Literal(l1), Literal(l2)) => /* can't find a better way to do this */
        // l1.to_string() == l2.to_string(),           _ => False,
        //         }
        //       }
        //     }
        //     self.0.iter().map(EqTokenTree).eq(other.0.iter().map(EqTokenTree))
        //   }
        // }
        params: Rc<[RsTypeKind]>,
    },
    /// The PartialEq trait.
    PartialEq { param: Rc<RsTypeKind> },
    /// The PartialOrd trait.
    PartialOrd { param: Rc<RsTypeKind> },
    /// Any other trait, e.g. Eq.
    Other { name: Rc<str>, params: Rc<[RsTypeKind]>, is_unsafe_fn: bool },
}

impl TraitName {
    /// Returns the generic parameters in this trait name.
    fn params(&self) -> &[RsTypeKind] {
        match self {
            Self::CtorNew(params)
            | Self::UnpinConstructor { params, .. }
            | Self::Other { params, .. } => params,
            Self::PartialEq { param } | Self::PartialOrd { param } => core::slice::from_ref(param),
        }
    }

    /// Returns the lifetimes used in this trait name.
    pub fn lifetimes(&self) -> impl Iterator<Item = Lifetime> + '_ {
        self.params().iter().flat_map(|p| p.lifetimes())
    }
    /// Similar to to_tokens but removing a given record type from the list of
    /// generic args
    ///
    /// This is used to remove the record whose trait implementation is being
    /// generated.
    fn to_token_stream_removing_trait_record(
        &self,
        db: &dyn BindingsGenerator,
        trait_record: Option<&Record>,
    ) -> TokenStream {
        match self {
            Self::UnpinConstructor { name, params } | Self::Other { name, params, .. } => {
                let name_as_token_stream = name.parse::<TokenStream>().unwrap();
                let formatted_params =
                    format_generic_params_replacing_by_self(db, &**params, trait_record);
                quote! {#name_as_token_stream #formatted_params}
            }
            Self::PartialEq { param } => {
                if trait_record.is_some() && param.is_record(trait_record.unwrap()) {
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
            Self::PartialOrd { param } => {
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
            Self::CtorNew(arg_types) => {
                let formatted_arg_types =
                    format_tuple_except_singleton_replacing_by_self(db, arg_types, trait_record);
                quote! { ::ctor::CtorNew < #formatted_arg_types > }
            }
        }
    }
}

impl TraitName {
    fn to_token_stream(&self, db: &dyn BindingsGenerator) -> TokenStream {
        self.to_token_stream_removing_trait_record(db, None)
    }
}

/// The kind of the `impl` block the function needs to be generated in.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ImplKind {
    /// Used for free functions for which we don't want the `impl` block.
    None { is_unsafe: bool },
    /// Used for inherent methods for which we need an `impl SomeStruct { ... }`
    /// block.
    Struct {
        /// For example, `SomeStruct`.
        record: Rc<Record>,
        is_unsafe: bool,
        /// Whether to format the first parameter as "self" (e.g. `__this:
        /// &mut T` -> `&mut self`)
        format_first_param_as_self: bool,
    },
    /// Used for trait methods for which we need an `impl TraitName for
    /// SomeStruct { ... }` block.
    Trait {
        /// For example, `SomeStruct`.
        record: Rc<Record>,
        /// For example, `quote!{ From<i32> }`.
        trait_name: TraitName,
        /// Reference style for the `impl` block and self parameters.
        impl_for: ImplFor,

        /// The generic params of trait `impl` (e.g. `vec!['b]`).
        /// These start empty and only later are mutated into the
        /// correct value.
        trait_generic_params: Rc<[Lifetime]>,

        /// Whether to format the first parameter as "self" (e.g. `__this:
        /// &mut T` -> `&mut self`)
        format_first_param_as_self: bool,
        /// Whether to drop the C++ function's return value and return unit
        /// instead.
        drop_return: bool,

        /// If this trait's method returns an associated type, it has this name.
        /// For example, this is `Output` on
        /// [`Add`](https://doc.rust-lang.org/std/ops/trait.Add.html).
        associated_return_type: Option<Ident>,

        /// Whether args should always be const references in Rust, even if they
        /// are by value in C++.
        ///
        /// For example, the traits for == and < only accept const reference
        /// parameters, but C++ allows values.
        force_const_reference_params: bool,
    },
}
impl ImplKind {
    fn new_trait(
        trait_name: TraitName,
        record: Rc<Record>,
        format_first_param_as_self: bool,
        force_const_reference_params: bool,
    ) -> Self {
        ImplKind::Trait {
            record,
            trait_name,
            impl_for: ImplFor::T,
            trait_generic_params: Rc::new([]),
            format_first_param_as_self,
            drop_return: false,
            associated_return_type: None,
            force_const_reference_params,
        }
    }
    fn format_first_param_as_self(&self) -> bool {
        matches!(
            self,
            Self::Trait { format_first_param_as_self: true, .. }
                | Self::Struct { format_first_param_as_self: true, .. }
        )
    }
    /// Returns whether the function is defined as `unsafe fn ...`.
    fn is_unsafe(&self) -> bool {
        matches!(
            self,
            Self::None { is_unsafe: true, .. }
                | Self::Struct { is_unsafe: true, .. }
                | Self::Trait { trait_name: TraitName::Other { is_unsafe_fn: true, .. }, .. }
        )
    }
}

/// Whether the impl block is for T, and the receivers take self by reference,
/// or the impl block is for a reference to T, and the method receivers take
/// self by value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ImplFor {
    /// Implement the trait for `T` directly.
    ///
    /// ```
    /// impl Trait for T {
    ///     fn const_method<'a>(&'a self);
    ///     fn mut_method<'a>(&'a mut self);
    ///     fn pin_method<'a>(Pin<&'a mut self>);
    /// }
    /// ```
    T,
    /// Implement the trait for `&T`, `&mut T`, or `Pin<&mut T>`, depending on
    /// the Rust type of the self parameter.
    ///
    /// ```
    /// impl<'a> Trait for &'a T {
    ///     fn const_method(self);
    /// }
    /// impl<'a> Trait for &'a mut UnpinT {
    ///     fn mut_method(self);
    /// }
    /// impl<'a> Trait for Pin<&'a mut NonUnpinT> {
    ///     fn pin_method(self);
    /// }
    /// ```
    RefT,
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
        TraitName::PartialEq { param },
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
    let partialeq_binding = get_binding(
        db,
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
) -> ErrorsOr<(Ident, ImplKind)> {
    assert_eq!(param_types.len(), 2, "Unexpected number of parameters in operator=: {func:?}");
    let Some(record) = maybe_record else {
        bail_to_errors!(errors, "operator= must be a member function")
    };
    materialize_ctor_in_caller(func, param_types);

    let rhs = &param_types[1];

    //  TODO(b/219963671): consolidate UnpinAssign and Assign in ctor.rs
    let trait_name;
    let func_name;
    if record.is_unpin() {
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
    };
    Ok((func_name, impl_kind))
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
        RsTypeKind::Pointer { pointee, mutability } => {
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
) -> ErrorsOr<(Ident, ImplKind)> {
    if let SafetyAnnotation::Unsafe = func.safety_annotation {
        report_fatal_func_error(db, func, "Unsafe annotations on operators are not supported");
    }
    match op.name.as_ref() {
        "==" => api_func_shape_for_operator_eq(db, func, param_types, errors),
        "<=>" => {
            bail_to_errors!(errors, "Three-way comparison operator not yet supported (b/219827738)")
        }
        "<" => api_func_shape_for_operator_lt(db, func, param_types, errors),
        "=" => api_func_shape_for_operator_assign(func, maybe_record, param_types, errors),
        _ => {
            let Some(op_metadata) =
                OPERATOR_METADATA.by_cc_name_and_params.get(&(&op.name, param_types.len()))
            else {
                bail_to_errors!(
                    errors,
                    "Bindings for this kind of operator (operator {op} with {n} parameter(s)) are not supported",
                    op = &op.name,
                    n = param_types.len(),
                );
            };
            materialize_ctor_in_caller(func, param_types);
            let trait_name = op_metadata.trait_name;
            if op_metadata.is_compound_assignment {
                let record = record_type_of_compound_assignment(db, &mut param_types[0], errors)?;
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
                };
                Ok((func_name, impl_kind))
            } else {
                let (record, impl_for) = match &param_types[0] {
                    RsTypeKind::Record { record, .. } => (record, ImplFor::T),
                    RsTypeKind::IncompleteRecord { incomplete_record, .. } => {
                        bail_to_errors!(
                            errors,
                            "Incomplete record types are not yet supported as first parameter of operator, found {cc_name}", cc_name=incomplete_record.cc_name,
                        )
                    },
                    RsTypeKind::Reference { referent, .. } => (
                        expect_possibly_incomplete_record(db, referent, "first operator parameter", errors)?,
                        ImplFor::RefT,
                    ),
                    RsTypeKind::RvalueReference { .. } => {
                        bail_to_errors!(
                            errors,
                            "Rvalue reference types are not yet supported as first parameter of operators (b/219826128)",
                        )
                    }
                    _ => bail_to_errors!(errors, "Non-record-nor-reference operator parameters are not yet supported, found {}", param_types[0].display(db)),
                };

                let func_name = make_rs_ident(op_metadata.method_name);
                let impl_kind = ImplKind::Trait {
                    record: record.clone(),
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
                };
                Ok((func_name, impl_kind))
            }
        }
    }
}

fn api_func_shape_for_identifier(
    func: &Func,
    maybe_record: Option<&Rc<Record>>,
    param_types: &mut [RsTypeKind],
    id: &Identifier,
    is_unsafe: bool,
) -> (Ident, ImplKind) {
    let func_name = make_rs_ident(&id.identifier);
    let Some(record) = maybe_record else { return (func_name, ImplKind::None { is_unsafe }) };
    let format_first_param_as_self = if func.is_instance_method() {
        let Some(first_param) = param_types.first() else {
            panic!("Missing `__this` parameter in an instance method: {:?}", func);
        };
        first_param.is_ref_to(record)
    } else {
        false
    };
    (func_name, ImplKind::Struct { record: record.clone(), format_first_param_as_self, is_unsafe })
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
    if !crate::generate_struct_and_union::should_implement_drop(record) {
        return None;
    }
    if record.is_unpin() {
        let func_name = make_rs_ident("drop");
        let impl_kind = ImplKind::new_trait(
            TraitName::Other { name: Rc::from("Drop"), params: Rc::from([]), is_unsafe_fn: false },
            record.clone(),
            /* format_first_param_as_self= */ true,
            /* force_const_reference_params= */
            false,
        );
        Some((func_name, impl_kind))
    } else {
        materialize_ctor_in_caller(func, param_types);
        let func_name = make_rs_ident("pinned_drop");
        let impl_kind = ImplKind::new_trait(
            TraitName::Other {
                name: Rc::from("::ctor::PinnedDrop"),
                params: Rc::from([]),
                is_unsafe_fn: true,
            },
            record.clone(),
            /* format_first_param_as_self= */ true,
            /* force_const_reference_params= */ false,
        );
        Some((func_name, impl_kind))
    }
}

fn api_func_shape_for_constructor(
    func: &Func,
    maybe_record: Option<&Rc<Record>>,
    param_types: &mut [RsTypeKind],
    is_unsafe: bool,
    errors: &Errors,
) -> Option<(Ident, ImplKind)> {
    let Some(record) = maybe_record else {
        panic!("Constructors must be associated with a record.");
    };
    if is_unsafe {
        // TODO(b/216648347): Allow this outside of traits (e.g. after supporting
        // translating C++ constructors into static methods in Rust).
        errors.add(anyhow!(
            "Unsafe constructors (e.g. with no elided or explicit lifetimes) \
            are intentionally not supported. See b/216648347.",
        ));
    }
    if let Err(err) = check_by_value(record) {
        errors.add(err);
    }
    materialize_ctor_in_caller(func, param_types);
    if !record.is_unpin() {
        let func_name = make_rs_ident("ctor_new");
        let [_this, params @ ..] = param_types else {
            panic!("Missing `__this` parameter in a constructor: {:?}", func)
        };
        let impl_kind = ImplKind::Trait {
            record: record.clone(),
            trait_name: TraitName::CtorNew(params.iter().cloned().collect()),
            impl_for: ImplFor::T,
            trait_generic_params: Rc::new([]),
            format_first_param_as_self: false,
            drop_return: false,
            associated_return_type: Some(make_rs_ident("CtorType")),
            force_const_reference_params: false,
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
            // Copy constructor
            if should_derive_clone(record) {
                None
            } else {
                let func_name = make_rs_ident("clone");
                let impl_kind = ImplKind::new_trait(
                    TraitName::UnpinConstructor { name: Rc::from("Clone"), params: Rc::from([]) },
                    record.clone(),
                    /* format_first_param_as_self= */ true,
                    /* force_const_reference_params= */ false,
                );
                Some((func_name, impl_kind))
            }
        }
        2 => {
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
/// trait-compatible. In particular, types which would be `impl Ctor<Output=T>`
/// become a `RvalueReference<'_, T>`.
///
/// Returns:
///
///  * `Err(_)`: something went wrong importing this function.
///  * `Ok(None)`: the function imported as "nothing". (For example, a defaulted
///    destructor might be mapped to no `Drop` impl at all.)
///  * `Ok((func_name, impl_kind))`: The function name and ImplKind.
fn api_func_shape(
    db: &dyn BindingsGenerator,
    func: &Func,
    param_types: &mut [RsTypeKind],
    errors: &Errors,
) -> Option<(Ident, ImplKind)> {
    let ir = db.ir();
    let maybe_record = match ir.record_for_member_func(func).map(<&Rc<Record>>::try_from) {
        None => None,
        Some(Ok(record)) => Some(record),
        // Functions whose record was replaced with some other IR Item type are ignored.
        // This occurs for instance if you use crubit_internal_rust_type: member functions defined
        // out-of-line, such as implicitly generated constructors, will still be present in the IR,
        // but should be ignored.
        Some(Err(_)) => return None,
    };

    if is_friend_of_record_not_visible_by_adl(db, func, param_types) {
        return None;
    }

    let is_unsafe = match func.safety_annotation {
        SafetyAnnotation::Unannotated => param_types.iter().any(|p| p.is_unsafe(db)),
        SafetyAnnotation::Unsafe => true,
        SafetyAnnotation::DisableUnsafe => false,
    };

    match &func.name {
        UnqualifiedIdentifier::Operator(op) => {
            api_func_shape_for_operator(db, func, maybe_record, param_types, op, errors).ok()
        }
        UnqualifiedIdentifier::Identifier(id) => {
            Some(api_func_shape_for_identifier(func, maybe_record, param_types, id, is_unsafe))
        }
        UnqualifiedIdentifier::Destructor => {
            api_func_shape_for_destructor(db, func, maybe_record, param_types)
        }
        UnqualifiedIdentifier::Constructor => {
            api_func_shape_for_constructor(func, maybe_record, param_types, is_unsafe, errors)
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

/// Returns the generated bindings for a function with the given name and param
/// types. If none exists, returns None.
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
            let mut function_param_types = function
                .params
                .iter()
                .map(|param| db.rs_type_kind(param.type_.rs_type.clone()))
                .collect::<Result<Vec<_>>>()
                .ok()?;
            if !function_param_types.iter().eq(expected_param_types.iter()) {
                return None;
            }
            api_func_shape_if_some(db, function, &mut function_param_types)
        })
}

/// Returns whether the given record either implements or derives the Clone
/// trait.
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
                let mut function_param_types = function
                    .params
                    .iter()
                    .map(|param| db.rs_type_kind(param.type_.rs_type.clone()))
                    .collect::<Result<Vec<_>>>()
                    .unwrap_or_default();
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
        let value = std::mem::replace(param, RsTypeKind::Primitive(PrimitiveType::Unit)); // Temporarily swap in a garbage value.
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
) -> TokenStream {
    let ParamValueAdjustments { clone_prefixes, clone_suffixes } = param_value_adjustments;
    match &impl_kind {
        ImplKind::Trait { trait_name: TraitName::UnpinConstructor { .. }, .. } => {
            // SAFETY: A user-defined constructor is not guaranteed to
            // initialize all the fields. To make the `assume_init()` call
            // below safe, the memory is zero-initialized first. This is a
            // bit safer, because zero-initialized memory represents a valid
            // value for the currently supported field types (this may
            // change once the bindings generator starts supporting
            // reference fields). TODO(b/213243309): Double-check if
            // zero-initialization is desirable here.
            quote! {
                let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                unsafe {
                    #crate_root_path::detail::#thunk_ident( &mut tmp #( , #thunk_args )* );
                    tmp.assume_init()
                }
            }
        }
        _ => {
            // Note: for the time being, all !Unpin values are treated as if they were not
            // trivially relocatable. We could, in the special case of trivial !Unpin types,
            // not generate the thunk at all, but this would be a bit of extra work.
            //
            // TODO(jeanpierreda): separately handle non-Unpin and non-trivial types.
            let mut body = if return_type.is_c_abi_compatible_by_value() {
                quote! {
                    #crate_root_path::detail::#thunk_ident(
                        #( #clone_prefixes #thunk_args #clone_suffixes ),*
                    )
                }
            } else {
                let return_type_or_self = {
                    let record = match impl_kind {
                        ImplKind::Struct { ref record, .. }
                        | ImplKind::Trait { ref record, impl_for: ImplFor::T, .. } => {
                            Some(&**record)
                        }
                        _ => None,
                    };
                    return_type.to_token_stream_replacing_by_self(db, record)
                };
                if return_type.is_unpin() {
                    quote! {
                        let mut __return =
                            ::core::mem::MaybeUninit::<#return_type_or_self>::uninit();
                        #crate_root_path::detail::#thunk_ident(
                            &mut __return
                            #( , #clone_prefixes #thunk_args #clone_suffixes )*
                        );
                        __return.assume_init()
                    }
                } else {
                    // TODO(b/200067242): the Pin-wrapping code doesn't know to wrap &mut
                    // MaybeUninit<T> in Pin if T is !Unpin. It should understand
                    // 'structural pinning', so that we do not need into_inner_unchecked()
                    // here.
                    quote! {
                        ::ctor::FnCtor::new(
                            move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<
                                                                    #return_type_or_self>>| {
                            #crate_root_path::detail::#thunk_ident(
                                ::core::pin::Pin::into_inner_unchecked(dest)
                                #( , #thunk_args )*
                            );
                        })
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
            quote! {
                #thunk_prepare
                #body
            }
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
/// #[diagnostic::on_unimplemented(message = "binding genertion for function failed\n...")]
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
    let message = format!("binding genertion for function failed\n{reportable_errors}");
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

/// Generates Rust source code for a given `Func`.
/// `derived_record` is a derived class type which re-exports `func` as a
/// method on this record.`func` must be a method on a base class of
/// `derived_record`.
///
/// Returns:
///
///  * `Err(_)`: couldn't import the function, emit an `UnsupportedItem`.
///  * `Ok(None)`: the function imported as "nothing". (For example, a defaulted
///    destructor might be mapped to no `Drop` impl at all.)
///  * `Ok((rs_api, rs_thunk, function_id))`: The Rust function definition,
///    thunk FFI definition, and function ID.
pub fn generate_function(
    db: &dyn BindingsGenerator,
    func: Rc<Func>,
    derived_record: Option<Rc<Record>>,
) -> Result<Option<GeneratedFunction>> {
    let ir = db.ir();
    let crate_root_path = crate::crate_root_path_tokens(&ir);
    let mut features = BTreeSet::new();
    let param_errors = Errors::new();
    let mut param_types: Vec<RsTypeKind> = func
        .params
        .iter()
        .enumerate()
        .filter_map(|(i, p)| {
            db.rs_type_kind(p.type_.rs_type.clone())
                .map_err(|err| {
                    param_errors.add(anyhow!("Failed to format type of parameter {i}: {err}"))
                })
                .ok()
        })
        .collect();
    param_errors.consolidate()?;

    let errors = Errors::new();
    let (func_name, mut impl_kind) =
        if let Some(values) = api_func_shape(db, &func, &mut param_types, &errors) {
            values
        } else {
            errors.consolidate()?;
            return Ok(None);
        };
    let namespace_qualifier = ir.namespace_qualifier(&func).format_for_rs();

    let mut return_type = errors.consolidate_on_err(
        db.rs_type_kind(func.return_type.rs_type.clone())
            .with_context(|| "Failed to format return type"),
    )?;
    if let Err(err) = return_type.check_by_value() {
        errors.add(err);
    }
    let param_idents =
        func.params.iter().map(|p| make_rs_ident(&p.identifier.identifier)).collect_vec();
    let thunk = generate_function_thunk(
        db,
        &func,
        &param_idents,
        &param_types,
        &return_type,
        derived_record.clone(),
    )
    .unwrap_or_else(|err| {
        errors.add(err);
        TokenStream::new()
    });

    let param_value_adjustments =
        adjust_param_types_for_trait_impl(db, &impl_kind, &mut param_types, &errors);

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

    let ErrorsAsUnsatisfiedTraitBound {
        lifetime_param: error_lifetime_param,
        mut unsatisfied_where_clause,
        unimplemented_trait_def,
    } = errors_as_unsatisfied_trait_bound(&reportable_status, &func.mangled_name);

    let api_func_def = {
        let thunk_ident = if let Some(ref derived_record) = derived_record {
            thunk_ident_for_derived_member_function(&func, derived_record.clone())
        } else {
            thunk_ident(&func)
        };

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
            )
        } else {
            quote! {
                #![allow(unused_variables)]
                unreachable!(
                    "This impl can never be instantiated. \
                    If this message appears at runtime, please report a <internal link>."
                )
            }
        };

        let pub_ = match impl_kind {
            ImplKind::None { .. } | ImplKind::Struct { .. } => quote! { pub },
            ImplKind::Trait { .. } => quote! {},
        };
        let unsafe_ = if impl_kind.is_unsafe() {
            quote! { unsafe }
        } else {
            quote! {}
        };

        // If we don't have an outer `impl ... { ... }` block, we have to introduce the
        // lifetimes and bounds inside this one.
        let has_wrapping_impl = match impl_kind {
            ImplKind::None { .. } => false,
            ImplKind::Struct { .. } | ImplKind::Trait { .. } => true,
        };
        let where_clause = if has_wrapping_impl {
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

    let doc_comment = crate::generate_doc_comment(
        func.doc_comment.as_deref(),
        Some(&func.source_loc),
        db.generate_source_loc_doc_comment(),
    );
    let api_func: TokenStream;
    let function_id: FunctionId;
    match impl_kind {
        ImplKind::None { .. } => {
            api_func = quote! { #unimplemented_trait_def #doc_comment #api_func_def };
            function_id = FunctionId {
                self_type: None,
                function_path: syn::parse2(quote! { #namespace_qualifier #func_name }).unwrap(),
            };
        }
        ImplKind::Struct { record, .. } => {
            let record_name;
            if let Some(ref derived_record) = derived_record {
                // Generate the bindings for the derived record.
                record_name = make_rs_ident(derived_record.rs_name.as_ref());
            } else {
                record_name = make_rs_ident(record.rs_name.as_ref());
            };
            let error_lifetime_generic = match &error_lifetime_param {
                Some(lifetime) => quote! { <#lifetime> },
                None => quote! {},
            };
            api_func = quote! { #unimplemented_trait_def impl #error_lifetime_generic #record_name #unsatisfied_where_clause { #doc_comment #api_func_def } };
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
            let extra_body = if let Some(name) = associated_return_type {
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

            let record_name = make_rs_ident(trait_record.rs_name.as_ref());
            let extra_items;
            let trait_lifetime_params = error_lifetime_param.as_slice();
            // NOTE: `trait_generic_params` may include lifetimes!
            let formatted_trait_generic_params =
                format_generic_params(trait_lifetime_params, &*trait_generic_params);
            match &trait_name {
                TraitName::CtorNew(params) => {
                    if params.len() == 1 {
                        let single_param_ = format_tuple_except_singleton_replacing_by_self(
                            db,
                            params,
                            Some(&trait_record),
                        );
                        extra_items = quote! {
                            impl #formatted_trait_generic_params ::ctor::CtorNew<(#single_param_,)> for #record_name #unsatisfied_where_clause {
                                #extra_body

                                #[inline (always)]
                                fn ctor_new(args: (#single_param_,)) -> Self::CtorType {
                                    let (arg,) = args;
                                    <Self as ::ctor::CtorNew<#single_param_>>::ctor_new(arg)
                                }
                            }
                        }
                    } else {
                        extra_items = quote! {}
                    }
                }
                _ => {
                    extra_items = quote! {};
                }
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
                    trait_name.to_token_stream_removing_trait_record(db, Some(&trait_record)),
                    quote! { #full_record_qualifier #record_name },
                ),
                ImplFor::RefT => {
                    (trait_name.to_token_stream(db), param_types[0].to_token_stream(db))
                }
            };
            api_func = quote! {
                #unimplemented_trait_def
                #doc_comment
                impl #formatted_trait_generic_params #trait_name_without_trait_record for #impl_for #unsatisfied_where_clause {
                    #extra_body
                    #api_func_def
                }
                #extra_items
            };
            function_id = FunctionId {
                self_type: Some(syn::parse2(quote! { #record_qualifier #record_name }).unwrap()),
                function_path: {
                    let trait_name_tokens = trait_name.to_token_stream(db);
                    syn::parse2(quote! { #trait_name_tokens :: #func_name }).unwrap()
                },
            };
        }
    }

    // If we are generating bindings for a derived record, we reuse the base
    // record's thunks, so we don't need to generate thunks.
    let cc_details = if derived_record.is_some() || failed {
        quote! {}
    } else {
        generate_function_thunk_impl(db, &func)?
    };

    let generated_item = ApiSnippets {
        main_api: api_func,
        thunks: if failed { TokenStream::new() } else { thunk },
        features,
        cc_details,
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
    /// tokens, non-Unpin by-value types are `impl Ctor<Output=#return_type> +
    /// ...`, and wherever the type is the type of `Self`, it gets replaced by
    /// literal `Self`.
    return_type_fragment: TokenStream,

    /// Any preparation code to define the arguments in `thunk_args`.
    thunk_prepare: TokenStream,

    /// The arguments passed to the thunk, expressed in terms of `params`.
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
    features: &mut BTreeSet<Ident>,
    func: &Func,
    impl_kind: &ImplKind,
    param_idents: &[Ident],
    param_types: &mut Vec<RsTypeKind>,
    return_type: &mut RsTypeKind,
    derived_record: Option<Rc<Record>>,
    errors: &Errors,
) -> Result<BindingsSignature> {
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
        if !type_.is_unpin() {
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
            let quoted_type_or_self = if let Some(impl_record) = impl_kind_record {
                if should_replace_by_self {
                    type_.to_token_stream_replacing_by_self(db, Some(impl_record))
                } else {
                    type_.to_token_stream(db)
                }
            } else {
                type_.to_token_stream(db)
            };
            features.insert(make_rs_ident("impl_trait_in_assoc_type"));
            api_params.push(quote! {#ident: impl ::ctor::Ctor<Output=#quoted_type_or_self>});
            thunk_args
                .push(quote! {::core::pin::Pin::into_inner_unchecked(::ctor::emplace!(#ident))});
        } else {
            let quoted_type_or_self = if let Some(impl_record) = impl_kind_record {
                if should_replace_by_self {
                    type_.to_token_stream_replacing_by_self(db, Some(impl_record))
                } else {
                    type_.to_token_stream(db)
                }
            } else {
                type_.to_token_stream(db)
            };
            if type_.is_c_abi_compatible_by_value() {
                api_params.push(quote! {#ident: #quoted_type_or_self});
                thunk_args.push(quote! {#ident});
            } else {
                api_params.push(quote! {mut #ident: #quoted_type_or_self});
                thunk_args.push(quote! {&mut #ident});
            }
        }
    }

    let mut lifetimes: Vec<Lifetime> = unique_lifetimes(&*param_types).collect();

    let mut quoted_return_type = None;
    // TODO: b/389131731 - Unify adjustment of return and parameter types.
    let trait_name = match &impl_kind {
        ImplKind::Trait { trait_name, .. } => Some(trait_name),
        _ => None,
    };
    match trait_name {
        Some(TraitName::PartialOrd { .. } | TraitName::PartialEq { .. }) => {
            if *return_type != RsTypeKind::Primitive(PrimitiveType::bool) {
                errors.add(anyhow!(
                    "comparison operator return type must be `bool`, found: {}",
                    return_type.display(db),
                ));
                *return_type = RsTypeKind::Primitive(PrimitiveType::bool);
            }
        }
        Some(TraitName::UnpinConstructor { .. } | TraitName::CtorNew(..)) => {
            // For constructors, we move the output parameter to be the return value.
            // The return value is "really" void.
            if !func.return_type.rs_type.is_unit_type() {
                panic!("Unexpectedly non-void return type of a constructor: {func:?}");
            }

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
            quoted_return_type = Some(quote! {Self});

            // Grab the `__this` lifetime to remove it from the lifetime parameters.
            let this_lifetime = param_types[0].lifetime().ok_or_else(|| {
                anyhow!(
                    "Missing lifetime for `__this` parameter type: {}",
                    param_types[0].display(db)
                )
            })?;

            // Drop `__this` parameter from the public Rust API.
            api_params.remove(0);
            thunk_args.remove(0);
            param_types.remove(0);

            // Remove the lifetime associated with `__this`.
            lifetimes.retain(|l| l != &this_lifetime);
            if let Some(type_still_dependent_on_removed_lifetime) =
                param_types.iter().find(|t| t.lifetimes().any(|lt| lt == this_lifetime))
            {
                bail!(
                    "The lifetime of `__this` is unexpectedly also used by another \
                    parameter: {}",
                    type_still_dependent_on_removed_lifetime.display(db)
                );
            }

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
                let thunk_vars = format_tuple_except_singleton(thunk_args.iter().cloned());
                thunk_prepare.extend(quote! {let #thunk_vars = args;});
            }
        }
        Some(TraitName::Other { .. }) | None => {}
    }

    let return_type_fragment = if return_type == &RsTypeKind::Primitive(PrimitiveType::Unit) {
        quote! {}
    } else {
        let ty = quoted_return_type.unwrap_or_else(|| return_type.to_token_stream(db));
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
            features.insert(make_rs_ident("impl_trait_in_assoc_type"));
            quote! {impl ::ctor::Ctor<Output=#ty> #extra_lifetimes }
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
                    api_params[0] = rs_snippet.tokens;
                    features.extend(rs_snippet.features);
                    if derived_record.is_some() {
                        thunk_args[0] = quote! { oops::Upcast::<_>::upcast(self) };
                    } else {
                        thunk_args[0] = quote! { self };
                    }
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

    Ok(BindingsSignature {
        lifetimes,
        params: api_params,
        return_type_fragment,
        thunk_prepare,
        thunk_args,
    })
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

/// Identifies all functions having overloads that we can't import (yet).
///
/// TODO(b/213280424): Implement support for overloaded functions.
pub fn overloaded_funcs(db: &dyn BindingsGenerator) -> Rc<HashSet<Rc<FunctionId>>> {
    let mut seen_funcs = HashSet::new();
    let mut overloaded_funcs = HashSet::new();
    for func in db.ir().functions() {
        // TODO(b/251045039) This check shouldn't fail so eagerly.
        // Functions that fail to receive bindings may still
        // participate in a C++ overload set, and we must still detect the
        // overload.
        if let Ok(Some(f)) = db.generate_function(func.clone(), None) {
            let function_id = &f.id;
            if !seen_funcs.insert(function_id.clone()) {
                overloaded_funcs.insert(function_id.clone());
            }
        }
    }
    Rc::new(overloaded_funcs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use crate::BindingsTokens;
    use arc_anyhow::Result;
    use googletest::prelude::*;
    use ir_testing::{retrieve_func, with_lifetime_macros};
    use token_stream_matchers::{
        assert_cc_matches, assert_cc_not_matches, assert_rs_matches, assert_rs_not_matches,
    };
    use token_stream_printer::rs_tokens_to_formatted_string_for_tests;

    #[gtest]
    fn test_simple_function() -> Result<()> {
        let ir = ir_from_cc("int Add(int a, int b);")?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[inline(always)]
                pub fn Add(a: ::core::ffi::c_int, b: ::core::ffi::c_int) -> ::core::ffi::c_int {
                    unsafe { crate::detail::__rust_thunk___Z3Addii(a, b) }
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                mod detail {
                    #[allow(unused_imports)]
                    use super::*;
                    unsafe extern "C" {
                        #[link_name = "_Z3Addii"]
                        pub(crate) unsafe fn __rust_thunk___Z3Addii(a: ::core::ffi::c_int, b: ::core::ffi::c_int) -> ::core::ffi::c_int;
                    }
                }
            }
        );

        assert_cc_not_matches!(rs_api_impl, quote! {__rust_thunk___Z3Addii});

        Ok(())
    }

    #[gtest]
    fn test_inline_function() -> Result<()> {
        let ir = ir_from_cc("inline int Add(int a, int b);")?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[inline(always)]
                pub fn Add(a: ::core::ffi::c_int, b: ::core::ffi::c_int) -> ::core::ffi::c_int {
                    unsafe { crate::detail::__rust_thunk___Z3Addii(a, b) }
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                mod detail {
                    #[allow(unused_imports)]
                    use super::*;
                    unsafe extern "C" {
                        pub(crate) unsafe fn __rust_thunk___Z3Addii(a: ::core::ffi::c_int, b: ::core::ffi::c_int) -> ::core::ffi::c_int;
                    }
                }
            }
        );

        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" int __rust_thunk___Z3Addii(int a, int b) {
                    return Add(a, b);
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_simple_function_with_types_from_other_target() -> Result<()> {
        let ir = ir_from_cc_dependency(
            "inline ReturnStruct DoSomething(ParamStruct param);",
            "struct ReturnStruct final {}; struct ParamStruct final {};",
        )?;

        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[inline(always)]
                pub fn DoSomething(mut param: dependency::ParamStruct)
                    -> dependency::ReturnStruct {
                     unsafe {
                         let mut __return =
                             ::core::mem::MaybeUninit::<dependency::ReturnStruct>::uninit();
                         crate::detail::__rust_thunk___Z11DoSomething11ParamStruct(
                             &mut __return, &mut param);
                         __return.assume_init()
                     }
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
            mod detail {
                #[allow(unused_imports)]
                use super::*;
                unsafe extern "C" {
                    pub(crate) unsafe fn __rust_thunk___Z11DoSomething11ParamStruct(
                        __return: &mut ::core::mem::MaybeUninit<dependency::ReturnStruct>,
                        param: &mut dependency::ParamStruct
                    );
                }
            }}
        );

        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___Z11DoSomething11ParamStruct(
                        struct ReturnStruct* __return, struct ParamStruct* param) {
                    new (__return) auto(DoSomething(std::move(*param)));
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_ref_to_struct_in_thunk_impls() -> Result<()> {
        let ir = ir_from_cc("struct S{}; inline void foo(S& s) {} ")?;
        let rs_api_impl = generate_bindings_tokens(ir)?.rs_api_impl;
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___Z3fooR1S(struct S* s) {
                    foo(*s);
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_const_ref_to_struct_in_thunk_impls() -> Result<()> {
        let ir = ir_from_cc("struct S{}; inline void foo(const S& s) {} ")?;
        let rs_api_impl = generate_bindings_tokens(ir)?.rs_api_impl;
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___Z3fooRK1S(const struct S* s) {
                    foo(*s);
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_unsigned_int_in_thunk_impls() -> Result<()> {
        let ir = ir_from_cc("inline void foo(unsigned int i) {} ")?;
        let rs_api_impl = generate_bindings_tokens(ir)?.rs_api_impl;
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___Z3fooj(unsigned int i) {
                    foo(i);
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_record_static_methods_qualify_call_in_thunk() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            struct SomeStruct {
                static inline int some_func() { return 42; }
            }; "#,
        )?;

        assert_cc_matches!(
            generate_bindings_tokens(ir)?.rs_api_impl,
            quote! {
                extern "C" int __rust_thunk___ZN10SomeStruct9some_funcEv() {
                    return SomeStruct::some_func();
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_record_instance_methods_deref_this_in_thunk() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            struct SomeStruct {
                inline int some_func(int arg) const { return 42 + arg; }
            }; "#,
        )?;

        assert_cc_matches!(
            generate_bindings_tokens(ir)?.rs_api_impl,
            quote! {
                extern "C" int __rust_thunk___ZNK10SomeStruct9some_funcEi(
                        const struct SomeStruct* __this, int arg) {
                    return __this->some_func(arg);
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_ptr_func() -> Result<()> {
        let ir = ir_from_cc(r#" inline int* Deref(int*const* p); "#)?;

        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[inline(always)]
                pub unsafe fn Deref(p: *const *mut ::core::ffi::c_int) -> *mut ::core::ffi::c_int {
                    crate::detail::__rust_thunk___Z5DerefPKPi(p)
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                mod detail {
                    #[allow(unused_imports)]
                    use super::*;
                    unsafe extern "C" {
                        pub(crate) unsafe fn __rust_thunk___Z5DerefPKPi(p: *const *mut ::core::ffi::c_int) -> *mut ::core::ffi::c_int;
                    }
                }
            }
        );

        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" int* __rust_thunk___Z5DerefPKPi(int* const * p) {
                    return Deref(p);
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_const_char_ptr_func() -> Result<()> {
        // This is a regression test: We used to include the "const" in the name
        // of the CcType, which caused a panic in the code generator
        // ('"const char" is not a valid Ident').
        // It's therefore important that f() is inline so that we need to
        // generate a thunk for it (where we then process the CcType).
        let ir = ir_from_cc(r#" inline void f(const signed char *str); "#)?;

        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[inline(always)]
                pub unsafe fn f(str: *const ::core::ffi::c_schar) {
                    crate::detail::__rust_thunk___Z1fPKa(str)
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                extern "C" {
                    pub(crate) unsafe fn __rust_thunk___Z1fPKa(str: *const ::core::ffi::c_schar);
                }
            }
        );

        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___Z1fPKa(signed char const * str){ f(str); }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_func_ptr_thunk() -> Result<()> {
        // Using an `inline` keyword forces generation of a C++ thunk in
        // `rs_api_impl` (i.e. exercises `format_cpp_type` and similar code).
        let ir = ir_from_cc(
            r#"
            int multiply(int x, int y);
            inline int (*inline_get_pointer_to_function())(int, int) {
                return multiply;
            }
        "#,
        )?;
        let rs_api_impl = generate_bindings_tokens(ir)?.rs_api_impl;
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" crubit::type_identity_t<int(int , int)>*
                __rust_thunk___Z30inline_get_pointer_to_functionv() {
                    return inline_get_pointer_to_function();
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_doc_comment_func() -> Result<()> {
        let ir = ir_from_cc(
            "
        // Doc Comment
        // with two lines
        int func();",
        )?;

        assert_rs_matches!(
            generate_bindings_tokens(ir)?.rs_api,
            // leading space is intentional so there is a space between /// and the text of the
            // comment
            quote! {
                #[doc = " Doc Comment\n with two lines\n \n Generated from: google3/ir_from_cc_virtual_header.h;l=6"]
                #[inline(always)]
                pub fn func
            }
        );

        Ok(())
    }

    /// Trivial types (at least those that are mapped to Copy rust types) do not
    /// get a Drop impl.
    #[gtest]
    fn test_impl_drop_trivial() -> Result<()> {
        let ir = ir_from_cc(
            r#"struct Trivial final {
                ~Trivial() = default;
                int x;
            };"#,
        )?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_not_matches!(rs_api, quote! {impl Drop});
        assert_rs_not_matches!(rs_api, quote! {impl ::ctor::PinnedDrop});
        assert_rs_matches!(rs_api, quote! {pub x: ::core::ffi::c_int});
        assert_cc_not_matches!(rs_api_impl, quote! { std::destroy_at });
        Ok(())
    }

    #[gtest]
    fn test_impl_default_explicitly_defaulted_constructor() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct DefaultedConstructor final {
                DefaultedConstructor() = default;
            };"#,
        )?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                impl Default for DefaultedConstructor {
                    #[inline(always)]
                    fn default() -> Self {
                        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                        unsafe {
                            crate::detail::__rust_thunk___ZN20DefaultedConstructorC1Ev(&mut tmp);
                            tmp.assume_init()
                        }
                    }
                }
            }
        );
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___ZN20DefaultedConstructorC1Ev(
                        struct DefaultedConstructor* __this) {
                    crubit::construct_at(__this);
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_impl_clone_that_propagates_lifetime() -> Result<()> {
        // This test covers the case where a single lifetime applies to 1)
        // the `__this` parameter and 2) other constructor parameters. For
        // example, maybe the newly constructed object needs to have the
        // same lifetime as the constructor's parameter. (This might require
        // annotating the whole C++ struct with a lifetime, so maybe the
        // example below is not fully realistic/accurate...).
        let ir = ir_from_cc(&with_lifetime_macros(
            r#"#pragma clang lifetime_elision
            struct Foo final {
                Foo(const int& $a i) $a;
            };"#,
        ))?;
        let ctor: &Func = ir
            .items()
            .filter_map(|item| match item {
                Item::Func(func) => Some(&**func),
                _ => None,
            })
            .find(|f| {
                matches!(&f.name, UnqualifiedIdentifier::Constructor)
                    && f.params
                        .get(1)
                        .map(|p| p.identifier.identifier.as_ref() == "i")
                        .unwrap_or_default()
            })
            .unwrap();
        {
            // Double-check that the test scenario set up above uses the same lifetime
            // for both of the constructor's parameters: `__this` and `i`.
            assert_eq!(ctor.params.len(), 2);
            let this_lifetimes = ctor.params[0].type_.rs_type.lifetime_args().unwrap();
            let i_lifetimes = ctor.params[1].type_.rs_type.lifetime_args().unwrap();
            assert_eq!(i_lifetimes[0], this_lifetimes[0]);
        }

        // Before cl/423346348 the generated Rust code would incorrectly look
        // like this (note the mismatched 'a and 'b lifetimes):
        //     fn from<'b>(i: &'a i32) -> Self
        // After this CL, this scenario will result in an explicit error.
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_not_matches!(rs_api, quote! {impl From});
        assert_rs_matches!(rs_api, {
            let txt = "Generated from: google3/ir_from_cc_virtual_header.h;l=34\n\
                           Error while generating bindings for item 'Foo::Foo':\n\
                           The lifetime of `__this` is \
                               unexpectedly also used by another parameter: &'a::core::ffi::c_int";
            quote! { __COMMENT__ #txt }
        });
        Ok(())
    }

    #[gtest]
    fn test_impl_default_non_trivial_struct() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct NonTrivialStructWithConstructors final {
                NonTrivialStructWithConstructors();
                ~NonTrivialStructWithConstructors();  // Non-trivial
            };"#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_not_matches!(rs_api, quote! {impl Default});
        Ok(())
    }

    #[gtest]
    fn test_impl_from_for_1_arg_constructor() -> Result<()> {
        for explicit_qualifier in ["", "explicit"] {
            let ir = ir_from_cc(&format!(
                r#"#pragma clang lifetime_elision
                struct SomeStruct final {{
                    {explicit_qualifier} SomeStruct(int i);  // implicit - no `explicit` keyword
                }};"#,
            ))?;
            let rs_api = generate_bindings_tokens(ir)?.rs_api;
            assert_rs_matches!(
                rs_api,
                quote! {
                    impl From<::core::ffi::c_int> for SomeStruct {
                        #[inline(always)]
                        fn from(i: ::core::ffi::c_int) -> Self {
                            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                            unsafe {
                                crate::detail::__rust_thunk___ZN10SomeStructC1Ei(&mut tmp, i);
                                tmp.assume_init()
                            }
                        }
                    }
                }
            );
        }
        Ok(())
    }

    #[gtest]
    fn test_impl_from_for_implicit_conversion_from_reference() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct SomeOtherStruct final { int i; };
            struct StructUnderTest final {
                StructUnderTest(const SomeOtherStruct& other);  // implicit - no `explicit` keyword
            };"#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        // This is a regression test for b/223800038: We want to ensure that the
        // code says `impl<'b>` (instead of incorrectly declaring that lifetime
        // in `fn from<'b>`).
        assert_rs_matches!(
            rs_api,
            quote! {
                impl<'b> From<&'b crate::SomeOtherStruct> for StructUnderTest {
                    #[inline(always)]
                    fn from(other: &'b crate::SomeOtherStruct) -> Self {
                        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                        unsafe {
                            crate::detail::__rust_thunk___ZN15StructUnderTestC1ERK15SomeOtherStruct(
                                &mut tmp, other);
                            tmp.assume_init()
                        }
                    }
                }
            },
        );
        Ok(())
    }

    /// Methods with missing lifetimes for `self` should give a useful error
    /// message.
    #[gtest]
    fn test_eq_nolifetime() -> Result<()> {
        // Missing lifetimes currently only causes hard errors for trait impls,
        // not For inherent methods.
        let ir = ir_from_cc("struct SomeStruct{SomeStruct& operator=(const SomeStruct&);};")?;

        let rs_api = rs_tokens_to_formatted_string_for_tests(generate_bindings_tokens(ir)?.rs_api)?;
        assert!(rs_api.contains(
            "// Error while generating bindings for item 'SomeStruct::operator=':\n\
             // `self` has no lifetime. Use lifetime annotations or \
                `#pragma clang lifetime_elision` to create bindings for this function."
        ));
        Ok(())
    }

    #[gtest]
    fn test_impl_eq_for_member_function() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct SomeStruct final {
                inline bool operator==(const SomeStruct& other) const {
                    return i == other.i;
                }
                int i;
            };"#,
        )?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                impl PartialEq for SomeStruct {
                    #[inline(always)]
                    fn eq<'a, 'b>(&'a self, other: &'b Self) -> bool {
                        unsafe { crate::detail::__rust_thunk___ZNK10SomeStructeqERKS_(self, other) }
                    }
                }
            }
        );
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" bool __rust_thunk___ZNK10SomeStructeqERKS_(
                        const struct SomeStruct* __this, const struct SomeStruct* other) {
                    return __this->operator==(*other);
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_impl_eq_for_free_function() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            namespace ns {
                struct SomeStruct final { int i; };
            }
            bool operator==(const ns::SomeStruct& lhs, const ns::SomeStruct& rhs) {
                return lhs.i == rhs.i;
            }
            "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                impl PartialEq for crate::ns::SomeStruct {
                    #[inline(always)]
                    fn eq<'a, 'b>(&'a self, rhs: &'b Self) -> bool {
                        unsafe { crate::detail::__rust_thunk___ZeqRKN2ns10SomeStructES2_(self, rhs) }
                    }
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_impl_eq_for_free_function_different_types() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct SomeStruct final { int i; };
            struct SomeOtherStruct final { int i; };
            bool operator==(const SomeStruct& lhs, const SomeOtherStruct& rhs) {
                return lhs.i == rhs.i;
            }"#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                impl PartialEq<crate::SomeOtherStruct> for crate::SomeStruct {
                    #[inline(always)]
                    fn eq<'a, 'b>(&'a self, rhs: &'b crate::SomeOtherStruct) -> bool {
                        unsafe { crate::detail::__rust_thunk___ZeqRK10SomeStructRK15SomeOtherStruct(self, rhs) }
                    }
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_impl_eq_for_free_function_by_value() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct SomeStruct final { int i; };
            bool operator==(SomeStruct lhs, SomeStruct rhs) {
                return lhs.i == rhs.i;
            }"#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                impl PartialEq for crate::SomeStruct {
                    #[inline(always)]
                    fn eq(&self, rhs: &Self) -> bool {
                        unsafe {
                            crate::detail::__rust_thunk___Zeq10SomeStructS_(&mut self.clone(), &mut rhs.clone())
                        }
                    }
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_impl_lt_for_member_function() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct SomeStruct final {
                inline bool operator==(const SomeStruct& other) const {
                    return i == other.i;
                }
                inline bool operator<(const SomeStruct& other) const {
                    return i < other.i;
                }
                int i;
            };"#,
        )?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                impl PartialOrd for SomeStruct {
                    #[inline(always)]
                    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
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
                    #[inline(always)]
                    fn lt<'a, 'b>(&'a self, other: &'b Self) -> bool {
                        unsafe { crate::detail::__rust_thunk___ZNK10SomeStructltERKS_(self, other) }
                    }
                }
            }
        );
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" bool __rust_thunk___ZNK10SomeStructltERKS_(
                        const struct SomeStruct* __this, const struct SomeStruct* other) {
                    return __this->operator<(*other);
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_impl_lt_for_free_function() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct SomeStruct final {
                inline bool operator==(const SomeStruct& other) const {
                    return i == other.i;
                }
                int i;
            };
            bool operator<(const SomeStruct& lhs, const SomeStruct& rhs) {
                return lhs.i < rhs.i;
            }"#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                impl PartialOrd for crate::SomeStruct {
                    #[inline(always)]
                    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
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
                    #[inline(always)]
                    fn lt<'a, 'b>(&'a self, rhs: &'b Self) -> bool {
                        unsafe { crate::detail::__rust_thunk___ZltRK10SomeStructS1_(self, rhs) }
                    }
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_impl_lt_for_free_function_by_value() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct SomeStruct final { int i; };
            bool operator==(SomeStruct lhs, SomeStruct rhs) {
                return lhs.i == rhs.i;
            }
            bool operator<(SomeStruct lhs, SomeStruct rhs) {
                return lhs.i < rhs.i;
            }"#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                impl PartialOrd for crate::SomeStruct {
                    #[inline(always)]
                    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
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
                    #[inline(always)]
                    fn lt(& self, rhs: &Self) -> bool {
                        unsafe { crate::detail::__rust_thunk___Zlt10SomeStructS_(
                                &mut self.clone(), &mut rhs.clone()) }
                    }
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_assign() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            #pragma clang lifetime_elision
            struct SomeStruct {
                ~SomeStruct();
                SomeStruct& operator=(const SomeStruct& other);
            };"#,
        )?;
        let BindingsTokens { rs_api, .. } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                impl<'b> ::ctor::Assign<&'b Self> for SomeStruct {
                    #[inline(always)]
                    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, other: &'b Self) {
                        unsafe {
                            crate::detail::__rust_thunk___ZN10SomeStructaSERKS_(self, other);
                        }
                    }
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_assign_nonreference_other() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            #pragma clang lifetime_elision
            struct SomeStruct {
                ~SomeStruct();
                SomeStruct& operator=(int other);
            };"#,
        )?;
        let BindingsTokens { rs_api, .. } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                impl<'b> ::ctor::Assign<&'b Self> for SomeStruct {
                    #[inline(always)]
                    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
                        unsafe {
                            crate::detail::__rust_thunk___ZN10SomeStructaSERKS_(self, __param_0);
                        }
                    }
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_assign_nonreference_return() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            #pragma clang lifetime_elision
            struct SomeStruct {
                ~SomeStruct();
                int operator=(const SomeStruct& other);
            };"#,
        )?;
        let BindingsTokens { rs_api, .. } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                impl<'b> ::ctor::Assign<&'b Self> for SomeStruct {
                    #[inline(always)]
                    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, other: &'b Self) {
                        unsafe {
                            crate::detail::__rust_thunk___ZN10SomeStructaSERKS_(self, other);
                        }
                    }
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_impl_eq_non_const_member_function() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct SomeStruct final {
                bool operator==(const SomeStruct& other) /* no `const` here */;
            };"#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_not_matches!(rs_api, quote! {impl PartialEq});
        Ok(())
    }

    #[gtest]
    fn test_impl_lt_different_operands() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct SomeStruct1 final {
                int i;
            };
            struct SomeStruct2 final {
                inline bool operator==(const SomeStruct1& other) const {
                    return i == other.i;
                }
                inline bool operator<(const SomeStruct1& other) const {
                    return i < other.i;
                };
                int i;
            };"#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_not_matches!(rs_api, quote! {impl PartialOrd});
        Ok(())
    }

    #[gtest]
    fn test_impl_lt_non_const_member_function() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct SomeStruct final {
                inline bool operator==(const SomeStruct& other) const {
                    return i == other.i;
                }
                int i;
                bool operator<(const SomeStruct& other) /* no `const` here */;
            };"#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_not_matches!(rs_api, quote! {impl PartialOrd});
        Ok(())
    }

    #[gtest]
    fn test_impl_lt_rhs_by_value() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct SomeStruct final {
                inline bool operator==(const SomeStruct& other) const {
                    return i == other.i;
                }
                int i;
                bool operator<(SomeStruct other) const;
            };"#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_not_matches!(rs_api, quote! {impl PartialOrd});
        Ok(())
    }

    #[gtest]
    fn test_impl_lt_missing_eq_impl() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct SomeStruct final {
                inline bool operator<(const SomeStruct& other) const {
                    return i < other.i;
                }
                int i;
            };"#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_not_matches!(rs_api, quote! {impl PartialOrd});
        Ok(())
    }

    #[gtest]
    fn test_thunk_ident_function() -> Result<()> {
        let ir = ir_from_cc("inline int foo() {}")?;
        let func = retrieve_func(&ir, "foo");
        assert_eq!(thunk_ident(func), make_rs_ident("__rust_thunk___Z3foov"));
        Ok(())
    }

    #[gtest]
    fn test_thunk_ident_special_names() {
        let ir = ir_from_cc("struct Class {};").unwrap();

        let destructor =
            ir.get_functions_by_name(&UnqualifiedIdentifier::Destructor).next().unwrap();
        assert_eq!(thunk_ident(destructor), make_rs_ident("__rust_thunk___ZN5ClassD1Ev"));

        let default_constructor = ir
            .get_functions_by_name(&UnqualifiedIdentifier::Constructor)
            .find(|f| f.params.len() == 1)
            .unwrap();
        assert_eq!(thunk_ident(default_constructor), make_rs_ident("__rust_thunk___ZN5ClassC1Ev"));
    }

    #[gtest]
    fn test_elided_lifetimes() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
          struct S final {
            int& f(int& i);
          };"#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                pub fn f<'a, 'b>(&'a mut self, i: &'b mut ::core::ffi::c_int) -> &'a mut ::core::ffi::c_int { ... }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                pub(crate) unsafe fn __rust_thunk___ZN1S1fERi<'a, 'b>(__this: &'a mut crate::S, i: &'b mut ::core::ffi::c_int)
                    -> &'a mut ::core::ffi::c_int;
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_annotated_lifetimes() -> Result<()> {
        let ir = ir_from_cc(&with_lifetime_macros(
            r#"
          int& $a f(int& $a i1, int& $a i2);
          "#,
        ))?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                pub fn f<'a>(i1: &'a mut ::core::ffi::c_int, i2: &'a mut ::core::ffi::c_int) -> &'a mut ::core::ffi::c_int { ... }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                pub(crate) unsafe fn __rust_thunk___Z1fRiS_<'a>(i1: &'a mut ::core::ffi::c_int, i2: &'a mut ::core::ffi::c_int)
                    -> &'a mut ::core::ffi::c_int;
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_format_generic_params() -> Result<()> {
        assert!(format_generic_params(/* lifetimes= */ &[], std::iter::empty::<syn::Ident>())
            .is_empty(),);

        let idents = ["T1", "T2"].iter().map(|s| make_rs_ident(s));
        assert_rs_matches!(
            format_generic_params(/* lifetimes= */ &[], idents),
            quote! { < T1, T2 > }
        );

        let lifetimes = ["a", "b", "_"].iter().map(|s| Lifetime::new(s)).collect::<Vec<_>>();
        assert_rs_matches!(
            format_generic_params(&lifetimes, std::iter::empty::<syn::Ident>()),
            quote! { < 'a, 'b > }
        );

        Ok(())
    }

    #[gtest]
    fn test_format_tuple_except_singleton() {
        fn format(xs: Vec<TokenStream>) -> TokenStream {
            format_tuple_except_singleton(xs)
        }
        assert_rs_matches!(format(vec![]), quote! {()});
        assert_rs_matches!(format(vec![quote! {a}]), quote! {a});
        assert_rs_matches!(format(vec![quote! {a}, quote! {b}]), quote! {(a, b)});
    }

    #[gtest]
    fn test_overloaded_functions() -> Result<()> {
        // TODO(b/213280424): We don't support creating bindings for overloaded
        // functions yet, except in the case of overloaded constructors with a
        // single parameter.
        let ir = ir_from_cc(
            r#" #pragma clang lifetime_elision
                void f() {}
                void f(int i) {}
                struct S1 final {
                  void f() {}
                  void f(int i) {}
                };
                struct S2 final {
                  void f();
                };
                struct S3 final {
                  S3(int i);
                  S3(double d);
                };

                namespace foo { void not_overloaded(); }
                namespace bar { void not_overloaded(); }
            "#,
        )?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;

        // Cannot overload free functions.
        assert_cc_matches!(rs_api, {
            let txt = "Generated from: google3/ir_from_cc_virtual_header.h;l=4\n\
                           Error while generating bindings for item 'f':\n\
                           Cannot generate bindings for overloaded function";
            quote! { __COMMENT__ #txt }
        });
        assert_rs_not_matches!(rs_api, quote! {pub fn f()});
        assert_rs_not_matches!(rs_api, quote! {pub fn f(i: ::core::ffi::c_int)});

        assert_cc_matches!(rs_api, {
            let txt = "Generated from: google3/ir_from_cc_virtual_header.h;l=7\n\
                           Error while generating bindings for item 'S1::f':\n\
                           Cannot generate bindings for overloaded function";
            quote! { __COMMENT__ #txt }
        });
        assert_rs_not_matches!(rs_api, quote! {pub fn f(... S1 ...)});

        // And thunks aren't generated for either.
        assert_cc_not_matches!(rs_api_impl, quote! {f});

        // But we can import member functions that have the same name as a free
        // function.
        assert_rs_matches!(rs_api, quote! {pub fn f<'a>(&'a mut self)});

        // We can also import overloaded single-parameter constructors.
        assert_rs_matches!(rs_api, quote! {impl From<::core::ffi::c_int> for S3});
        assert_rs_matches!(rs_api, quote! {impl From<f64> for S3});

        // And we can import functions that have the same name + signature, but that are
        // in 2 different namespaces.
        assert_rs_matches!(rs_api, quote! { pub fn not_overloaded() });
        Ok(())
    }

    /// !Unpin references should not be pinned.
    #[gtest]
    fn test_nonunpin_ref_param() -> Result<()> {
        let rs_api = generate_bindings_tokens(ir_from_cc(
            r#"
            #pragma clang lifetime_elision
            struct S {~S();};
            void Function(const S& s);
        "#,
        )?)?
        .rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                fn Function<'a>(s: &'a crate::S) { ... }
            }
        );
        Ok(())
    }

    /// !Unpin mut references must be pinned.
    #[gtest]
    fn test_nonunpin_mut_param() -> Result<()> {
        let rs_api = generate_bindings_tokens(ir_from_cc(
            r#"
            #pragma clang lifetime_elision
            struct S {~S();};
            void Function(S& s);
        "#,
        )?)?
        .rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                fn Function<'a>(s: ::core::pin::Pin<&'a mut crate::S>) { ... }
            }
        );
        Ok(())
    }

    /// !Unpin &self should not be pinned.
    #[gtest]
    fn test_nonunpin_ref_self() -> Result<()> {
        let rs_api = generate_bindings_tokens(ir_from_cc(
            r#"
            #pragma clang lifetime_elision
            struct S {
              ~S();
              void Function() const;
            };
        "#,
        )?)?
        .rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                fn Function<'a>(&'a self) { ... }
            }
        );
        Ok(())
    }

    /// !Unpin &mut self must be pinned.
    #[gtest]
    fn test_nonunpin_mut_self() -> Result<()> {
        let rs_api = generate_bindings_tokens(ir_from_cc(
            r#"
            #pragma clang lifetime_elision
            struct S {
              ~S();
              void Function();
            };
        "#,
        )?)?
        .rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                fn Function<'a>(self: ::core::pin::Pin<&'a mut Self>) { ... }
            }
        );
        Ok(())
    }

    /// Drop::drop must not use self : Pin<...>.
    #[gtest]
    fn test_nonunpin_drop() -> Result<()> {
        let rs_api = generate_bindings_tokens(ir_from_cc(
            r#"
            struct S {~S();};
        "#,
        )?)?
        .rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) { ... }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_nonunpin_0_arg_constructor() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            // This type must be `!Unpin`.
            struct HasConstructor {
                explicit HasConstructor() {}
                ~HasConstructor();
            };"#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(rs_api, quote! {#[::ctor::recursively_pinned(PinnedDrop)]});
        assert_rs_matches!(
            rs_api,
            quote! {
                impl ::ctor::CtorNew<()> for HasConstructor {
                    type CtorType = impl ::ctor::Ctor<Output = Self>;

                    #[inline (always)]
                    fn ctor_new(args: ()) -> Self::CtorType {
                        let () = args;
                        unsafe {
                            ::ctor::FnCtor::new(move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                                crate::detail::__rust_thunk___ZN14HasConstructorC1Ev(::core::pin::Pin::into_inner_unchecked(dest));
                            })
                        }
                    }
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_nonunpin_1_arg_constructor() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            // This type must be `!Unpin`.
            struct HasConstructor {
                explicit HasConstructor(unsigned char input) {}
                ~HasConstructor();
            };"#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(rs_api, quote! {#[::ctor::recursively_pinned(PinnedDrop)]});
        assert_rs_matches!(
            rs_api,
            quote! {
                impl ::ctor::CtorNew<::core::ffi::c_uchar> for HasConstructor {
                    type CtorType = impl ::ctor::Ctor<Output = Self>;

                    #[inline (always)]
                    fn ctor_new(args: ::core::ffi::c_uchar) -> Self::CtorType {
                        let input = args;
                        unsafe {
                            ::ctor::FnCtor::new(move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                                crate::detail::__rust_thunk___ZN14HasConstructorC1Eh(::core::pin::Pin::into_inner_unchecked(dest), input);
                            })
                        }
                    }
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_nonunpin_2_arg_constructor() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            // This type must be `!Unpin`.
            struct HasConstructor {
                explicit HasConstructor(unsigned char input1, signed char input2) {}
                ~HasConstructor();
            };"#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(rs_api, quote! {#[::ctor::recursively_pinned(PinnedDrop)]});
        assert_rs_matches!(
            rs_api,
            quote! {
                impl ::ctor::CtorNew<(::core::ffi::c_uchar, ::core::ffi::c_schar)> for HasConstructor {
                    type CtorType = impl ::ctor::Ctor<Output = Self>;

                    #[inline (always)]
                    fn ctor_new(args: (::core::ffi::c_uchar, ::core::ffi::c_schar)) -> Self::CtorType {
                        let (input1, input2) = args;
                        unsafe {
                            ::ctor::FnCtor::new(move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                                crate::detail::__rust_thunk___ZN14HasConstructorC1Eha(::core::pin::Pin::into_inner_unchecked(dest), input1, input2);
                            })
                        }
                    }
                }
            }
        );
        Ok(())
    }

    /// Traits which monomorphize the `Ctor` parameter into the caller must
    /// synthesize an RvalueReference parameter, with an appropriate
    /// lifetime parameter.
    #[gtest]
    fn test_nonunpin_by_value_params() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            // This type must be `!Unpin`.
            struct HasConstructor {
                // int& x is here to create a 'b lifetime, which collides with a synthesized
                // lifetime name. But that's OK! We handle collisions!
                // (`a` would also work, but that's just because the left hand doesn't know what
                // the right is doing: the `a` lifetime is present in some places, but eventually
                // removed from the public interface.)
                explicit HasConstructor(const int& x, HasConstructor y, HasConstructor b) {}
                ~HasConstructor();
            };"#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(rs_api, quote! {#[::ctor::recursively_pinned(PinnedDrop)]});
        assert_rs_matches!(
            rs_api,
            quote! {
                impl <'b, 'y, 'b_2> ::ctor::CtorNew<(
                    &'b ::core::ffi::c_int,
                    ::ctor::RvalueReference<'y, Self>,
                    ::ctor::RvalueReference<'b_2, Self>)
                > for HasConstructor {
                    // The captures are why we need explicit lifetimes for the two rvalue reference
                    // parameters.
                    type CtorType = impl ::ctor::Ctor<Output = Self> + use<'b, 'y, 'b_2>;

                    #[inline (always)]
                    fn ctor_new(args: (
                        &'b ::core::ffi::c_int,
                        ::ctor::RvalueReference<'y, Self>,
                        ::ctor::RvalueReference<'b_2, Self>)
                    ) -> Self::CtorType {
                        let (x, y, b) = args;
                        unsafe {
                            ::ctor::FnCtor::new(move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                                crate::detail::__rust_thunk___ZN14HasConstructorC1ERKiS_S_(::core::pin::Pin::into_inner_unchecked(dest), x, y, b);
                            })
                        }
                    }
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_nonunpin_return() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            // This type must be `!Unpin`.
            struct Nontrivial {~Nontrivial();};

            Nontrivial ReturnsByValue(const int& x, const int& y);
            "#,
        )?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                pub fn ReturnsByValue<'a, 'b>(x: &'a ::core::ffi::c_int, y: &'b ::core::ffi::c_int)
                -> impl ::ctor::Ctor<Output=crate::Nontrivial> + use<'a, 'b> {
                    unsafe {
                        ::ctor::FnCtor::new(move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<crate::Nontrivial>>| {
                            crate::detail::__rust_thunk___Z14ReturnsByValueRKiS0_(::core::pin::Pin::into_inner_unchecked(dest), x, y);
                        })
                    }

                }
            }
        );

        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___Z14ReturnsByValueRKiS0_(
                        struct Nontrivial* __return, int const* x, int const* y) {
                    new(__return) auto(ReturnsByValue(*x, *y));
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_nonunpin_const_return() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            // This type must be `!Unpin`.
            struct Nontrivial {~Nontrivial();};

            const Nontrivial ReturnsByValue(const int& x, const int& y);
            "#,
        )?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                pub fn ReturnsByValue<'a, 'b>(x: &'a ::core::ffi::c_int, y: &'b ::core::ffi::c_int)
                -> impl ::ctor::Ctor<Output=crate::Nontrivial> + use<'a, 'b> {
                    unsafe {
                        ::ctor::FnCtor::new(move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<crate::Nontrivial>>| {
                            crate::detail::__rust_thunk___Z14ReturnsByValueRKiS0_(::core::pin::Pin::into_inner_unchecked(dest), x, y);
                        })
                    }

                }
            }
        );

        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___Z14ReturnsByValueRKiS0_(
                        struct Nontrivial* __return, int const* x, int const* y) {
                    new(__return) auto(ReturnsByValue(*x, *y));
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_unpin_by_value_param() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct Trivial final {
              int trivial_field;
            };

            void foo(Trivial param);
            "#,
        )?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[inline(always)]
                pub fn foo(mut param: crate::Trivial) {
                    unsafe { crate::detail::__rust_thunk___Z3foo7Trivial(&mut param) }
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                pub(crate) unsafe fn __rust_thunk___Z3foo7Trivial(param: &mut crate::Trivial);
            }
        );
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___Z3foo7Trivial(struct Trivial* param) {
                    foo(std::move(*param));
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_unpin_by_value_return() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct Trivial final {
              int trivial_field;
            };

            Trivial foo();
            "#,
        )?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[inline(always)]
                pub fn foo() -> crate::Trivial {
                    unsafe {
                        let mut __return = ::core::mem::MaybeUninit::<crate::Trivial>::uninit();
                        crate::detail::__rust_thunk___Z3foov(&mut __return);
                        __return.assume_init()
                    }
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                pub(crate) unsafe fn __rust_thunk___Z3foov(
                    __return: &mut ::core::mem::MaybeUninit<crate::Trivial>
                );
            }
        );
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___Z3foov(struct Trivial* __return) {
                    new (__return) auto(foo());
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_unpin_rvalue_ref_qualified_method() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct TrivialWithRvalueRefQualifiedMethod final {
              void rvalue_ref_qualified_method() &&;
            };
            "#,
        )?;
        let BindingsTokens { rs_api, .. } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[inline(always)]
                pub fn rvalue_ref_qualified_method<'a>(self: ::ctor::RvalueReference<'a, Self>) {
                    unsafe {
                        crate::detail::__rust_thunk___ZNO35TrivialWithRvalueRefQualifiedMethod27rvalue_ref_qualified_methodEv(self)
                    }
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                #[link_name = "_ZNO35TrivialWithRvalueRefQualifiedMethod27rvalue_ref_qualified_methodEv"]
                pub(crate) unsafe fn __rust_thunk___ZNO35TrivialWithRvalueRefQualifiedMethod27rvalue_ref_qualified_methodEv < 'a > (__this :
                    :: ctor :: RvalueReference < 'a , crate :: TrivialWithRvalueRefQualifiedMethod >) ;
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_unpin_rvalue_ref_const_qualified_method() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct TrivialWithRvalueRefConstQualifiedMethod final {
              void rvalue_ref_const_qualified_method() const &&;
            };
            "#,
        )?;
        let BindingsTokens { rs_api, .. } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[inline(always)]
                pub fn rvalue_ref_const_qualified_method<'a>(self: ::ctor::ConstRvalueReference<'a, Self>) {
                    unsafe {
                        crate::detail::__rust_thunk___ZNKO40TrivialWithRvalueRefConstQualifiedMethod33rvalue_ref_const_qualified_methodEv(self)
                    }
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                #[link_name = "_ZNKO40TrivialWithRvalueRefConstQualifiedMethod33rvalue_ref_const_qualified_methodEv"]
                pub(crate) unsafe fn __rust_thunk___ZNKO40TrivialWithRvalueRefConstQualifiedMethod33rvalue_ref_const_qualified_methodEv < 'a > (__this :
                    :: ctor :: ConstRvalueReference < 'a , crate :: TrivialWithRvalueRefConstQualifiedMethod >) ;
            }
        );
        Ok(())
    }

    /// Assignment is special in that it discards the return type.
    /// So if the return type is !Unpin, it needs to emplace!() it.
    #[gtest]
    fn test_nonunpin_return_assign() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            // This type must be `!Unpin`.
            struct Nontrivial {
                ~Nontrivial();
                Nontrivial operator=(const Nontrivial& other);
            };
            "#,
        )?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                impl<'b> ::ctor::Assign<&'b Self> for Nontrivial {
                    #[inline(always)]
                    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, other: &'b Self) {
                        unsafe {
                            let _ = ::ctor::emplace!(::ctor::FnCtor::new(
                                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                                    crate::detail::__rust_thunk___ZN10NontrivialaSERKS_(
                                        ::core::pin::Pin::into_inner_unchecked(dest),
                                        self,
                                        other
                                    );
                                }
                            ));
                        }
                    }
                }
            }
        );

        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___ZN10NontrivialaSERKS_(
                    struct Nontrivial* __return, struct Nontrivial* __this,
                    const struct Nontrivial* other
                ) {
                    new(__return) auto(__this->operator=(*other));
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_nonunpin_param() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            // This type must be `!Unpin`.
            struct Nontrivial {
                Nontrivial(Nontrivial&&);
                ~Nontrivial();
            };

            void TakesByValue(Nontrivial x);
            "#,
        )?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                pub fn TakesByValue(x: impl ::ctor::Ctor<Output=crate::Nontrivial>) {
                    unsafe {
                        crate::detail::__rust_thunk___Z12TakesByValue10Nontrivial(::core::pin::Pin::into_inner_unchecked(::ctor::emplace!(x)))
                    }
                }
            }
        );

        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___Z12TakesByValue10Nontrivial(struct Nontrivial*x) {
                    TakesByValue(std::move(*x));
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_nonunpin_trait_param() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            // This type must be `!Unpin`.
            struct Nontrivial {
                Nontrivial(Nontrivial&&);
                Nontrivial& operator=(Nontrivial) {}
                ~Nontrivial();
            };

            struct Trivial final {
                /*implicit*/ Trivial(Nontrivial) {}
            };
            "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                impl<'__param_0> From<::ctor::RvalueReference<'__param_0, crate::Nontrivial>> for Trivial {
                    #[inline(always)]
                    fn from(__param_0: ::ctor::RvalueReference<'__param_0, crate::Nontrivial>) -> Self {
                        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
                        unsafe {
                            crate::detail::__rust_thunk___ZN7TrivialC1E10Nontrivial(
                                &mut tmp,
                                __param_0
                            );
                            tmp.assume_init()
                        }
                    }
                }
            }
        );
        Ok(())
    }

    #[gtest]
    fn test_nonmovable_param() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            // This type must be `!Unpin` and non-move constructible.
            struct Nonmovable {
                Nonmovable(Nonmovable&&) = delete;
            };

            void TakesByValue(Nonmovable) {}
            "#,
        )?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        // Bindings for TakesByValue cannot be generated.
        assert_rs_matches!(rs_api, quote! {TakesByValue<'error>});
        assert_cc_not_matches!(rs_api_impl, quote! {TakesByValue});
        Ok(())
    }

    #[gtest]
    fn test_invalid_unsafe_annotation_causes_fatal_error() -> googletest::Result<()> {
        let ir = ir_from_cc(
            r#"
            struct Trivial final {
                [[clang::annotate("crubit_override_unsafe", true)]]
                ~Trivial();
            };
            "#,
        )
        .or_fail()?;
        let error_message = generate_bindings_tokens(ir).err().or_fail()?.to_string();
        assert_that!(
            error_message,
            contains_substring("Unsafe annotations on destructors are not supported")
        );
        Ok(())
    }

    #[gtest]
    fn test_function_returning_rvalue_reference() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct SomeStruct final {
                // Inline to force generation (and test coverage) of C++ thunks.
                inline SomeStruct&& GetRValueReference() {
                  return static_cast<SomeStruct&&>(*this);
                }
                int field;
            };
            "#,
        )?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                impl SomeStruct {
                    ...
                    #[inline(always)]
                    pub fn GetRValueReference<'a>(&'a mut self)
                            -> ::ctor::RvalueReference<'a, crate::SomeStruct> {
                        unsafe {
                            crate::detail::__rust_thunk___ZN10SomeStruct18GetRValueReferenceEv(self)
                        }
                    }
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                extern "C" {
                    ...
                    pub(crate) unsafe fn __rust_thunk___ZN10SomeStruct18GetRValueReferenceEv<'a>(
                            __this: &'a mut crate::SomeStruct
                       ) -> ::ctor::RvalueReference<'a, crate::SomeStruct>;
                    ...
                }
            }
        );

        // Note that you can't just convert directly from xvalue to lvalue:
        //
        //     return &static_cast<SomeStruct&>(__this->GetRValueReference());
        //
        // For the above, Clang will emit an error that "non-const lvalue reference to
        // type 'struct SomeStruct' cannot bind to a temporary of type
        // 'SomeStruct'" (This is somewhat misleading, because there are no
        // temporaries here).  We must first bind the return value to a name
        // (`lvalue` below), so that it becomes an lvalue. Only then can it be
        // converted to a pointer.
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" struct SomeStruct*
                __rust_thunk___ZN10SomeStruct18GetRValueReferenceEv(struct SomeStruct* __this) {
                    struct SomeStruct&& lvalue = __this->GetRValueReference();
                    return &lvalue;
                }
            }
        );

        Ok(())
    }
}
