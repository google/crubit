// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {
    use googletest::prelude::*;
    use methods::*;

    #[gtest]
    fn test_instance_const_method() {
        let s = InstanceMethods { int_field: 124 };
        assert_eq!(124, s.get_int_field());
    }

    #[gtest]
    fn test_instance_nonconst_method() {
        let mut s = InstanceMethods { int_field: 123 };
        s.set_int_field(457);
        assert_eq!(457, s.int_field);
    }

    #[gtest]
    fn test_inline_instance_const_method() {
        let s = InstanceMethods { int_field: 124 };
        assert_eq!(124, s.inline_get_int_field());
    }

    #[gtest]
    fn test_inline_instance_nonconst_method() {
        let mut s = InstanceMethods { int_field: 123 };
        s.inline_set_int_field(457);
        assert_eq!(457, s.int_field);
    }

    #[gtest]
    fn test_static_factory_method() {
        let s: SomeClass = SomeClass::static_factory_method(123);
        assert_eq!(123, s.int_var);
    }

    #[gtest]
    fn test_static_method_that_multiplies_its_args() {
        assert_eq!(42 * 789, SomeClass::static_method_that_multiplies_its_args(42, 789));
    }

    #[gtest]
    fn test_static_inline_method() {
        assert_eq!(42 * 456, SomeClass::static_inline_method(456));
    }
}
