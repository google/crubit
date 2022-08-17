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
    fn test_many_operators_neg() {
        let s = ManyOperators { i: 7 };
        assert_eq!(-7, (-&s).i);
    }

    #[test]
    fn test_many_operators_not() {
        let s = ManyOperators { i: 7 };
        assert_eq!(0, (!&s).i);
    }

    #[test]
    fn test_many_operators_add() {
        let s1 = ManyOperators { i: 7 };
        let s2 = ManyOperators { i: 3 };
        assert_eq!(10, (&s1 + s2).i);
    }

    #[test]
    fn test_many_operators_sub() {
        let s1 = ManyOperators { i: 7 };
        let s2 = ManyOperators { i: 3 };
        assert_eq!(4, (&s1 - s2).i);
    }

    #[test]
    fn test_many_operators_mul() {
        let s1 = ManyOperators { i: 7 };
        let s2 = ManyOperators { i: 3 };
        assert_eq!(21, (&s1 * s2).i);
    }

    #[test]
    fn test_many_operators_div() {
        let s1 = ManyOperators { i: 7 };
        let s2 = ManyOperators { i: 3 };
        assert_eq!(2, (&s1 / s2).i);
    }

    #[test]
    fn test_many_operators_rem() {
        let s1 = ManyOperators { i: 7 };
        let s2 = ManyOperators { i: 3 };
        assert_eq!(1, (&s1 % s2).i);
    }

    #[test]
    fn test_many_operators_bit_and() {
        let s1 = ManyOperators { i: 7 };
        let s2 = ManyOperators { i: 3 };
        assert_eq!(3, (&s1 & s2).i);
    }

    #[test]
    fn test_many_operators_bit_or() {
        let s1 = ManyOperators { i: 7 };
        let s2 = ManyOperators { i: 3 };
        assert_eq!(7, (&s1 | s2).i);
    }

    #[test]
    fn test_many_operators_bit_xor() {
        let s1 = ManyOperators { i: 7 };
        let s2 = ManyOperators { i: 3 };
        assert_eq!(4, (&s1 ^ s2).i);
    }

    #[test]
    fn test_many_operators_shl() {
        let s1 = ManyOperators { i: 7 };
        let s2 = ManyOperators { i: 3 };
        assert_eq!(56, (&s1 << s2).i);
    }

    #[test]
    fn test_many_operators_shr() {
        let s1 = ManyOperators { i: 7 };
        let s2 = ManyOperators { i: 3 };
        assert_eq!(0, (&s1 >> s2).i);
    }

    #[test]
    fn test_many_operators_add_assign() {
        let mut s1 = ManyOperators { i: 7 };
        let s2 = ManyOperators { i: 3 };
        s1 += s2;
        assert_eq!(10, s1.i);
    }

    #[test]
    fn test_many_operators_sub_assign() {
        let mut s1 = ManyOperators { i: 7 };
        let s2 = ManyOperators { i: 3 };
        s1 -= s2;
        assert_eq!(4, s1.i);
    }

    #[test]
    fn test_many_operators_mul_assign() {
        let mut s1 = ManyOperators { i: 7 };
        let s2 = ManyOperators { i: 3 };
        s1 *= s2;
        assert_eq!(21, s1.i);
    }

    #[test]
    fn test_many_operators_div_assign() {
        let mut s1 = ManyOperators { i: 7 };
        let s2 = ManyOperators { i: 3 };
        s1 /= s2;
        assert_eq!(2, s1.i);
    }

    #[test]
    fn test_many_operators_rem_assign() {
        let mut s1 = ManyOperators { i: 7 };
        let s2 = ManyOperators { i: 3 };
        s1 %= s2;
        assert_eq!(1, s1.i);
    }

    #[test]
    fn test_many_operators_bit_and_assign() {
        let mut s1 = ManyOperators { i: 7 };
        let s2 = ManyOperators { i: 3 };
        s1 &= s2;
        assert_eq!(3, s1.i);
    }

    #[test]
    fn test_many_operators_bit_or_assign() {
        let mut s1 = ManyOperators { i: 7 };
        let s2 = ManyOperators { i: 3 };
        s1 |= s2;
        assert_eq!(7, s1.i);
    }

    #[test]
    fn test_many_operators_bit_xor_assign() {
        let mut s1 = ManyOperators { i: 7 };
        let s2 = ManyOperators { i: 3 };
        s1 ^= s2;
        assert_eq!(4, s1.i);
    }

    #[test]
    fn test_many_operators_shl_assign() {
        let mut s1 = ManyOperators { i: 7 };
        let s2 = ManyOperators { i: 3 };
        s1 <<= s2;
        assert_eq!(56, s1.i);
    }

    #[test]
    fn test_many_operators_shr_assign() {
        let mut s1 = ManyOperators { i: 7 };
        let s2 = ManyOperators { i: 3 };
        s1 >>= s2;
        assert_eq!(0, s1.i);
    }
}
