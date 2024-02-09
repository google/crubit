// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// Test to ensure that Crubit can import class template specialization with
/// un-instantiable member function that passes the type check but has an
/// incorrect function body.
#[test]
fn test_build() {
    // TODO:(b/248542210): Assert `AForInt` is generated and does not have
    // method `Call_FailMethod`, `FailMethod`, `FailStaticMethod`.
    // Currently, it fails with:
    // failed_template_instantiation_member_function_recursive_rust_api_impl.cc:
    // 13: failed_template_instantiation_member_function_recursive.h:14:37:
    // error: static assertion failed.
}
