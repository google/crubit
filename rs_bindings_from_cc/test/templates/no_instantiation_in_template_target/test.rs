// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {
    use googletest::prelude::*;

    #[gtest]
    fn test_alias_to_template_without_instantiation_in_different_target() {
        let s = type_alias_in_different_target::TypeAliasInDifferentTarget::Create(321);
        assert_eq!(321, *s.value());
    }
}
