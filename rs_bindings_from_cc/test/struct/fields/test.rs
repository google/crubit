// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {
    use fields::*;
    use googletest::prelude::*;

    #[gtest]
    fn test_simple_struct() {
        // This test doesn't really do a whole lot beyond verifying that the
        // struct and its fields are imported correctly and that the generated
        // Rust code compiles.
        let s = SomeStruct { char_var: 1, int_var: 2 };
        assert_eq!(s.char_var, 1);
        assert_eq!(s.int_var, 2);
    }
}
