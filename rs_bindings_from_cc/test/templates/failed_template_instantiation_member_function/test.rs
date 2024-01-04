// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// Test to ensure that Crubit can import class template specialization with
/// un-instantiable member function.
#[test]
fn test_failed_template_instantiation_member_function() {
    use failed_template_instantiation_member_function::*;
    Func(B::default());
}
