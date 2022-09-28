// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
use cc_import::cc_import;

cc_import! {
    "//rs_bindings_from_cc/test/cc_import:two" as lib_two;
    "//rs_bindings_from_cc/test/cc_import:three";
}

pub fn add_two_and_three() -> i32 {
    simple_math::get_two() + simple_math::get_three()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_math() {
        assert_eq!(add_two_and_three(), 5);
    }

    #[test]
    fn test_not_reopened_namespaces() {
        assert_eq!(complex_math::two_only::get_square(), 4);
        assert_eq!(complex_math::three_only::get_square(), 9);
    }
}
