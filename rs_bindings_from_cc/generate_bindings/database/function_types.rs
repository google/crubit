// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! The types used to represent generated Rust functions.

use crate::code_snippet::ApiSnippets;
use crate::rs_snippet::{Lifetime, RsTypeKind};
use arc_anyhow::Result;
use ir::Record;
use std::rc::Rc;
use syn::Ident;

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
    pub self_type: Option<syn::Path>,
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
    /// The std::clone::Clone trait.
    Clone,
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
    PartialEq { param: Rc<RsTypeKind>, negate_thunk_result: bool },
    /// The PartialOrd trait.
    PartialOrd { param: Rc<RsTypeKind> },
    /// Any other trait, e.g. Eq.
    Other { name: Rc<str>, params: Rc<[RsTypeKind]>, is_unsafe_fn: bool },
}

impl std::fmt::Display for TraitName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TraitName::CtorNew { .. } => {
                write!(f, "CtorNew")
            }
            TraitName::UnpinConstructor { name, .. } => {
                write!(f, "{name}")
            }
            TraitName::PartialEq { .. } => {
                write!(f, "PartialEq")
            }
            TraitName::PartialOrd { .. } => {
                write!(f, "PartialOrd")
            }
            TraitName::Other { name, .. } => {
                write!(f, "{name}")
            }
            TraitName::Clone => {
                write!(f, "Clone")
            }
        }
    }
}

impl TraitName {
    /// Returns the generic parameters in this trait name.
    fn params(&self) -> &[RsTypeKind] {
        match self {
            Self::CtorNew(params)
            | Self::UnpinConstructor { params, .. }
            | Self::Other { params, .. } => params,
            Self::PartialEq { param, .. } | Self::PartialOrd { param } => {
                core::slice::from_ref(param)
            }
            Self::Clone => &[],
        }
    }

    /// Returns the lifetimes used in this trait name.
    pub fn lifetimes(&self) -> impl Iterator<Item = Lifetime> + use<'_> {
        self.params().iter().flat_map(|p| p.lifetimes())
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
        /// Whether this function wraps a C++ constructor for an `Unpin` type, so it has an implicit
        /// `out: *mut Self` parameter.
        is_renamed_unpin_constructor: bool,
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

        /// Whether this trait impl should be globally visible, even when the type is `pub(crate)`
        /// in a `:wrapper` target.
        ///
        /// Set this to true when:
        ///
        /// * The implementation will not change in backwards-incompatible ways when features are
        ///   added to Crubit.
        /// * The implementation is well-known and not subject to being overwritten by the
        ///   `:wrapper` library owner.
        ///
        /// For example, `Drop` and `PinnedDrop` are perfect instances of this: there is only one
        /// logical implementation, which won't change over time, and it's not permitted for library
        /// owners to change it to something else.
        always_public: bool,
    },
}
impl ImplKind {
    pub fn new_trait(
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
            always_public: false,
        }
    }
    pub fn format_first_param_as_self(&self) -> bool {
        matches!(
            self,
            Self::Trait { format_first_param_as_self: true, .. }
                | Self::Struct { format_first_param_as_self: true, .. }
        )
    }
    /// Returns whether the function is defined as `unsafe fn ...`.
    pub fn is_unsafe(&self) -> bool {
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
