// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {
    use elided_lifetimes::*;
    use methods::*;

    #[test]
    fn test_instance_const_method() {
        let s = ElidedLifetimes { int_field: 124 };
        assert_eq!(124, s.get_int_field());
    }

    #[test]
    fn test_instance_nonconst_method() {
        let mut s = ElidedLifetimes { int_field: 123 };
        s.set_int_field(457);
        assert_eq!(457, s.int_field);
    }

    #[test]
    fn test_inline_instance_const_method() {
        let s = ElidedLifetimes { int_field: 124 };
        assert_eq!(124, s.inline_get_int_field());
    }

    #[test]
    fn test_inline_instance_nonconst_method() {
        let mut s = ElidedLifetimes { int_field: 123 };
        s.inline_set_int_field(457);
        assert_eq!(457, s.int_field);
    }

    #[test]
    fn test_static_factory_method() {
        let s: SomeClass = SomeClass::static_factory_method(123);
        assert_eq!(123, s.int_var);
    }

    #[test]
    fn test_static_method_that_multiplies_its_args() {
        assert_eq!(42 * 789, SomeClass::static_method_that_multiplies_its_args(42, 789));
    }

    #[test]
    fn test_static_inline_method() {
        assert_eq!(42 * 456, SomeClass::static_inline_method(456));
    }
}
