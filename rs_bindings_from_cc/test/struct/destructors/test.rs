// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {
    use field_destruction_order::*;
    use std::mem::ManuallyDrop;

    #[test]
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
        DestructionOrderRecorder::ClearDestructionRecord();
        FieldDestructionOrderTester::DestructFromCpp(field1_value, field2_value, field3_value);
        assert_eq!(
            expected_destruction_order_in_cpp,
            DestructionOrderRecorder::GetDestructionRecord()
        );

        // The main test is below - it tries to confirm that the destruction
        // order of the C++ object is preserved when the destruction is
        // triggered from Rust, via `impl Drop` emitted by the bindings
        // generator.
        DestructionOrderRecorder::ClearDestructionRecord();
        {
            // The code below constructs the same FieldDestructionOrderTester object
            // as done internally in DestructFromCpp above.
            let tester = FieldDestructionOrderTester {
                field1: ManuallyDrop::new(DestructionOrderRecorder {
                    int_field: ManuallyDrop::new(field1_value),
                }),
                field2: ManuallyDrop::new(DestructionOrderRecorder {
                    int_field: ManuallyDrop::new(field2_value),
                }),
                field3: ManuallyDrop::new(DestructionOrderRecorder {
                    int_field: ManuallyDrop::new(field3_value),
                }),
            };
            // Dropping the `tester` should invoke destructors of field1/2/3 in the
            // same order as C++ (e.g. by calling into the C++ destructor of
            // FieldDestructionOrderTester).  Note that Rust uses a different order
            // as explains in https://doc.rust-lang.org/reference/destructors.html:
            // The fields of a struct are dropped in **declaration order**.
            drop(tester);
        }
        assert_eq!(
            expected_destruction_order_in_rust,
            DestructionOrderRecorder::GetDestructionRecord()
        );
    }
}
