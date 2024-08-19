// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {
    use googletest::prelude::*;

    #[gtest]
    fn test_member_function_of_class_template_defined_out_of_line_in_h_file() {
        let s = out_of_line_definition::MyTypeAlias::Create(123);
        assert_eq!(123, *s.value());
    }
}
