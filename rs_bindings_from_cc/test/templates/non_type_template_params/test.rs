// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {
    use googletest::prelude::*;
    use non_type_template_params::*;

    #[gtest]
    fn test_non_type_template_params() {
        assert_eq!(123 * 100, MyMultiplierX100::Multiply(123));
        assert_eq!(123 * 1000, MyMultiplierX1000::Multiply(123));
    }

    #[gtest]
    fn test_big_const() {
        assert_eq!(18446744073709551615, BigNumericConst::GetValue());
    }
}
