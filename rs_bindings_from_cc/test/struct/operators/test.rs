// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {
    use operators::*;

    #[test]
    fn test_eq_member_func_same_operands() {
        let s1 = TestStruct2 { i: 1005 };
        let s2 = TestStruct2 { i: 2005 };
        let s3 = TestStruct2 { i: 3000 };
        assert!(s1 == s2);
        assert!(s1 != s3);
    }

    #[test]
    fn test_eq_member_func_different_operands() {
        let s1 = TestStruct2 { i: 1005 };
        let s2 = TestStruct1 { i: 2005 };
        let s3 = TestStruct1 { i: 3000 };
        assert!(s1 == s2);
        assert!(s1 != s3);
    }

    #[test]
    fn test_eq_free_func() {
        // TODO(lukasza): Cover TestStruct3 equality.
    }

    #[test]
    fn test_eq_out_of_line_definition() {
        let s1 = OperandForOutOfLineDefinition { i: 1005 };
        let s2 = OperandForOutOfLineDefinition { i: 2005 };
        let s3 = OperandForOutOfLineDefinition { i: 3000 };
        assert!(s1 == s2);
        assert!(s1 != s3);
    }
}
