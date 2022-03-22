// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {
    use operators::*;
    use static_assertions::{assert_impl_all, assert_not_impl_all};

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

        // The relation is not necessarily symmetrical:
        assert_impl_all!(TestStruct2: PartialEq<TestStruct1>);
        assert_not_impl_all!(TestStruct1: PartialEq<TestStruct2>);
    }

    #[test]
    fn test_non_operator_method_name() {
        let s2 = TestStruct2 { i: 2005 };
        assert_eq!(2005, s2.operator1());
    }

    #[test]
    fn test_eq_out_of_line_definition() {
        let s1 = OperandForOutOfLineDefinition { i: 1005 };
        let s2 = OperandForOutOfLineDefinition { i: 2005 };
        let s3 = OperandForOutOfLineDefinition { i: 3000 };
        assert!(s1 == s2);
        assert!(s1 != s3);
    }

    #[test]
    fn test_eq_free_func() {
        let s1 = OperandForFreeFunc { i: 1005 };
        let s2 = OperandForFreeFunc { i: 2005 };
        assert!(s1 == s2);
    }

    #[test]
    fn test_eq_free_func_different_namespace() {
        // We probably should try to mimic "argument-dependent lookup" (ADL) and
        // only generate bindings for PartialEq if `operator==` free function is
        // defined in the same namespace as the lhs. See also
        // https://en.cppreference.com/w/cpp/language/adl
        assert_not_impl_all!(OperandForFreeFuncInDifferentNamespace: PartialEq);
    }
}
