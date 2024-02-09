// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use failed_template_instantiation_member_function::*;

/// Test to ensure that Crubit can import class template specialization with
/// un-instantiable member function.
#[test]
fn test_failed_template_instantiation_member_function() {
    InvokeNoOp(AForNoMethod::default());
    InvokeMethodReturnAuto(AForHasMethodReturningVoid::default());
    assert_eq!(1, InvokeMethodReturnAutoAndInt(AForHasMethodReturningInt::default()));
}

#[test]
fn test_failed_template_instantiation_member_function_preserves_instantiable() {
    AForNoMethod::default().NoOp();

    AForHasMethodReturningVoid::default().NoOp();
    AForHasMethodReturningVoid::default().Call_MethodReturnAuto(HasMethodReturningVoid::default());

    AForHasMethodReturningInt::default().NoOp();
    AForHasMethodReturningInt::default().Call_MethodReturnAuto(HasMethodReturningInt::default());
    assert_eq!(
        1,
        AForHasMethodReturningInt::default().Call_MethodReturnInt(HasMethodReturningInt::default())
    );
    AForHasMethodReturningInt::default().Call_MethodReturnAuto(HasMethodReturningInt::default());
    assert_eq!(
        1,
        AForHasMethodReturningInt::default().Call_MethodReturnInt(HasMethodReturningInt::default())
    );
    assert_eq!(
        1,
        AForHasMethodReturningInt::default()
            .Call_Call_MethodReturnInt(HasMethodReturningInt::default())
    );
}
