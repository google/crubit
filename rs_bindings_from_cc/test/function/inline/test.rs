// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {
    use hello_world::*;

    #[test]
    fn test_hello_world() {
        assert_eq!(hello_world_inline(), 42);
    }

    #[test]
    fn test_take_struct_by_const_ref() {
        let s = SomeStruct { int_field: 789 };
        assert_eq!(789, take_struct_by_const_ref(&s));
    }

    #[test]
    fn test_double_unsigned_int() {
        assert_eq!(double_unsigned_int(123), 246);
    }

    #[test]
    fn test_forward_declared_doubler() {
        assert_eq!(foo::forward_declared_doubler(124), 248);
    }
}
