// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![cfg(test)]

use cc_template::cc_template;
use googletest::prelude::*;

/// Test the `cc_template!` macro with:
/// * the JSON file `__cc_template_instantiations.json` (the environment
///   variable is configured in the BUILD file)
/// * `__cc_template_instantiations_rs_api.rs` as the "generated" Rust bindings
///   file

#[allow(non_camel_case_types)]
mod __cc_template_instantiations_rs_api;

#[gtest]
fn test_in_mocked_context() {
    let x = <cc_template!(my_namespace::MyTemplate<MyArg>)>::new(42);
    assert!(x.value == 42);
}
