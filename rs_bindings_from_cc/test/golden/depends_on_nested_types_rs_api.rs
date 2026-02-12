// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:depends_on_nested_types_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

/// This should have bindings because Bar is a nested item of Foo, and the module
/// "foo" can be generated because it wouldn't conflict with anything else.
pub type FooBar = ::nested_types_cc::foo::Bar;
pub use ::nested_types_cc::foo::bar as foo_bar;

// Error while generating bindings for type alias 'ConflictingSnakeCaseNamesInner':
// Can't generate bindings for ConflictingSnakeCaseNamesInner, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:depends_on_nested_types_cc needs [//features:wrapper] for ConflictingSnakeCaseNamesInner (error: crubit.rs/errors/nested_type: records ["ConflictingSnakeCaseNames", "ConflictingSnakeCaseNames_"] all have nested items, but all map to the same nested module name: `conflicting_snake_case_names`)
