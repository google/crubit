// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {
    #[test]
    fn test_two_template_parameters() {
        let s =
            two_template_parameters::AliasToTemplateWithTwoParams { value1: 123, value2: 456.789 };
        assert_eq!(123, s.value1);
        assert_eq!(456.789, s.value2);
    }
}
