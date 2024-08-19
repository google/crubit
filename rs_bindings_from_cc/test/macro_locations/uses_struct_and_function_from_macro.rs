// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {
    use googletest::prelude::*;

    #[gtest]
    fn test_uses_struct_and_function_from_macro() {
        let my_struct = uses_macro::StructFromMacro { val: 3 };
        assert_eq!(my_struct.val, uses_macro::functionFromMacro(3));
    }
}
