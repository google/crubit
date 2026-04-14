// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:depends_on_nested_types_cc

#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![deny(warnings)]

/// This should have bindings because Bar is a nested item of Foo, and the module
/// "foo" can be generated because it wouldn't conflict with anything else.
pub type FooBar = ::nested_types_cc::foo::Bar;
pub use ::nested_types_cc::foo::bar as foo_bar;

/// This should not have bindings because Bar is a nested item of Foo, and the
/// module "conflicting_snake_case_names" cannot be generated because it
/// conflicts with the child module of ConflictingSnakeCaseNames_.
pub type ConflictingSnakeCaseNamesInner = ::nested_types_cc::conflicting_snake_case_names::Inner;
