// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
use arc_anyhow::{bail, Result};
use crubit_feature::CrubitFeature;
use flagset::FlagSet;

/// Location where a type is used.
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum TypeLocation {
    /// The top-level return type.
    ///
    /// The "top-level" part can be explained by looking at an example of `fn
    /// foo() -> *const T`:
    /// - The top-level return type `*const T` is in the `FnReturn` location
    /// - The nested pointee type `T` is in the `Other` location
    FnReturn,

    /// The top-level parameter type.
    ///
    /// The "top-level" part can be explained by looking at an example of:
    /// `fn foo(param: *const T)`:
    /// - The top-level parameter type `*const T` is in the `FnParam` location
    /// - The nested pointee type `T` is in the `Other` location
    // TODO(b/278141494, b/278141418): Once `const` and `static` items are supported,
    // we may want to apply parameter-like formatting to their types (e.g. have
    // `format_ty_for_cc` emit `T&` rather than `T*`).
    FnParam {
        /// Whether the parameter is a self parameter.
        /// Self parameter references are never transformed into pointers, but may instead result
        /// in a member function being ref-qualified.
        is_self_param: bool,

        /// Whether elided lifetimes correspond to an elided lifetime in the return type.
        /// This is used to determine whether to emit a pointer or a reference.
        elided_is_output: bool,
    },

    /// The type of a constant item.
    Const,

    /// Inside of a compound data type, but still bridgeable: e.g. a tuple field, where the tuple
    /// itself is in a function parameter or return value.
    NestedBridgeable,

    /// The type of a template argument when specializing a generic class template or trait (e.g.,
    /// inside `rs::Result<T, E>` or `rs::Vec<T>`).
    TemplateArg,

    /// The template argument `T` of an `rs::Option<T>` specialization.
    ///
    /// This is formatted the same way as `TemplateArg`, except that when `option_references` is
    /// enabled, a reference to a sized type is formatted as a C++ reference (`rs::Option<T
    /// const&>` / `rs::Option<T&>`, like C++26 `std::optional<T&>`) rather than as a pointer.
    /// This keeps `Option<&T>` (a nullable pointer) and `Option<*const T>` (a pointer plus a
    /// tag) from mapping to the same C++ type (b/575916406).
    OptionArg,

    /// The `Item` associated type of `core::iter::Iterator` or `core::iter::IntoIterator`.
    ///
    /// Unlike other associated types and type aliases (where Rust references are formatted as C++
    /// pointers to stay consistent with generic specializations like `rs::Option<const T*>`
    /// returned by `Iterator::next`), `Iterator::Item` and `IntoIterator::Item` are formatted as
    /// C++ references (`const T&` / `T&`) when `iterator_item_references` is enabled so that
    /// `rs::IteratorAdapter` yields C++ references when iterating in C++.
    IteratorItem,

    /// Other location (e.g. pointee type, etc.).
    Other,

    /// The return type of a closure / callable argument, or the output type `T` of a
    /// `::crubit::DynErasedFuture<T>` returned by an async function.
    ///
    /// In this context, unit `()` and never `!` are formatted as `void`, references and function
    /// pointers are formatted as pointers (so `DynErasedFuture::Poll` can take `T*`), and neither
    /// bridging nor `::rs::Movable` wrapping is applied (types must be layout-compatible or
    /// unbridged).
    ClosureReturn,

    /// The type of a field of a struct or union.
    Field,
}

impl TypeLocation {
    /// Returns a descriptive error if a bridge type cannot be used in this location.
    pub fn check_bridgeable(self) -> Result<()> {
        // This match is exhaustive to force us to think about new variants when adding them.
        match self {
            TypeLocation::FnReturn => Ok(()),
            TypeLocation::FnParam { .. } => Ok(()),
            TypeLocation::Const => Ok(()),
            TypeLocation::NestedBridgeable => Ok(()),
            TypeLocation::TemplateArg | TypeLocation::OptionArg => bail!("Template arguments do not support bridgeable types"),
            TypeLocation::ClosureReturn | TypeLocation::IteratorItem | TypeLocation::Other => bail!("crubit.rs/errors/bridge_compound_type: Non-bridgeable compound data types containing bridge types cannot receive bindings"),
            TypeLocation::Field => bail!("crubit.rs/errors/bridge_field: Fields containing bridge types cannot receive bindings"),
        }
    }

    pub fn is_bridgeable(self) -> bool {
        self.check_bridgeable().is_ok()
    }

    /// Returns whether a type in this location can be wrapped in `::rs::Movable`.
    ///
    /// Top-level return values are only wrapped under the `movable_return_values` feature.
    pub fn admits_movable(self, crate_features: FlagSet<CrubitFeature>) -> bool {
        match self {
            TypeLocation::FnParam { .. } | TypeLocation::NestedBridgeable => true,
            TypeLocation::FnReturn => crate_features.contains(CrubitFeature::MovableReturnValues),
            TypeLocation::Const
            | TypeLocation::TemplateArg
            | TypeLocation::OptionArg
            | TypeLocation::IteratorItem
            | TypeLocation::ClosureReturn
            | TypeLocation::Other
            | TypeLocation::Field => false,
        }
    }
}
