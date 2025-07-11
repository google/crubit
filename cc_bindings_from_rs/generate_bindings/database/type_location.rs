// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

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

    /// Other location (e.g. pointee type, field type, etc.).
    Other,
}
