// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {
    #[test]
    fn test_template_template_params() {
        assert_eq!(42, template_template_params::MyTypeAlias::GetPolicy());
    }
}
