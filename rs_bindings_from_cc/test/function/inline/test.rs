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
}
