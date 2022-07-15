// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {
    use operators::*;
    use static_assertions::{assert_impl_all, assert_not_impl_any};

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
        assert_not_impl_any!(TestStruct1: PartialEq<TestStruct2>);
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
        assert_not_impl_any!(OperandForFreeFuncInDifferentNamespace: PartialEq);
    }

    #[test]
    fn test_add_const_member_int() {
        let s = AddableConstMemberInt { i: 11 };
        assert_eq!(33, &s + 22);
    }

    #[test]
    fn test_add_const_member_by_ref() {
        let s1 = AddableConstMemberByRef { i: 11 };
        let s2 = AddableConstMemberByRef { i: 22 };
        assert_eq!(33, (&s1 + &s2).i);
    }

    #[test]
    fn test_add_non_const_member_by_ref() {
        let mut s1 = AddableNonConstMemberByRef { i: 11 };
        let s2 = AddableNonConstMemberByRef { i: 22 };
        assert_eq!(33, (&mut s1 + &s2).i);
    }

    #[test]
    fn test_add_const_member_by_value() {
        let s1 = AddableConstMemberByValue { i: 11 };
        let s2 = AddableConstMemberByValue { i: 22 };
        assert_eq!(33, (&s1 + s2).i);
    }

    #[test]
    fn test_add_non_const_member_by_value() {
        let mut s1 = AddableNonConstMemberByValue { i: 11 };
        let s2 = AddableNonConstMemberByValue { i: 22 };
        assert_eq!(33, (&mut s1 + s2).i);
    }

    #[test]
    fn test_add_returns_void() {
        let mut s1 = AddableReturnsVoid { i: 11 };
        let s2 = AddableReturnsVoid { i: 22 };
        &mut s1 + &s2;
        assert_eq!(s1.i, 33);
    }
}
