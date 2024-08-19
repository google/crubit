// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {
    use calling_conventions::*;
    use googletest::prelude::*;

    #[gtest]
    fn test_default_cc() {
        let s = UnusualSwiftcallStruct { x0: 0x1111_1111, x1: 0x2222_2222, x2: 0x4444_4444 };
        let func_differentiator = 0xffff_0000;
        assert_eq!(function_with_default_cc(s), 0x7777_7777 + func_differentiator);
    }

    #[gtest]
    fn test_swiftcall_cc() {
        let s = UnusualSwiftcallStruct { x0: 0x1111_1111, x1: 0x2222_2222, x2: 0x4444_4444 };
        let func_differentiator = 0x0000_ffff;
        assert_eq!(function_with_swiftcall_cc(s), 0x7777_7777 + func_differentiator);
    }
}
