// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use cc_std::std::shared_ptr;
use destructors::{DestructionOrderRecorder, FieldDestructionOrderTester};
use googletest::{expect_eq, gtest};

#[gtest]
fn test_field_destruction_order() {
    let field1_value = 1;
    let field2_value = 2;
    let field3_value = 3;

    let expected_destruction_order_in_cpp = 321;
    let expected_destruction_order_in_rust = 321;

    // The 3 statements below just confirm what C++ documentation says in
    // https://en.cppreference.com/w/cpp/language/destructor: For both
    // user-defined or implicitly-defined destructors, after the body of the
    // destructor is executed, the compiler calls the destructors for all
    // non-static non-variant members of the class, in **reverse order of
    // declaration**.
    let record = shared_ptr::new(0);
    FieldDestructionOrderTester::DestructFromCpp(
        field1_value,
        field2_value,
        field3_value,
        record.clone(),
    );
    expect_eq!(expected_destruction_order_in_cpp, *record);

    // The main test is below - it tries to confirm that the destruction
    // order of the C++ object is preserved when the destruction is
    // triggered from Rust, via `impl Drop` emitted by the bindings
    // generator.
    let record = shared_ptr::new(0);
    {
        // The code below constructs the same FieldDestructionOrderTester object
        // as done internally in DestructFromCpp above.
        let tester = FieldDestructionOrderTester::from((
            DestructionOrderRecorder::from((field1_value, record.clone())),
            DestructionOrderRecorder::from((field2_value, record.clone())),
            DestructionOrderRecorder::from((field3_value, record.clone())),
        ));
        // Dropping the `tester` should invoke destructors of field1/2/3 in the
        // same order as C++ (e.g. by calling into the C++ destructor of
        // FieldDestructionOrderTester).  Note that Rust uses a different order
        // as explains in https://doc.rust-lang.org/reference/destructors.html:
        // The fields of a struct are dropped in **declaration order**.
        drop(tester);
    }
    expect_eq!(expected_destruction_order_in_rust, *record);
}

#[gtest]
fn test_private_operator_delete() {
    let token = shared_ptr::new(0);
    {
        let tester = destructors::PrivateOperatorDelete::from(token.clone());
        expect_eq!(shared_ptr::use_count(&token), 2);
        drop(tester);
    }
    expect_eq!(shared_ptr::use_count(&token), 1);
}
