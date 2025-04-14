// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {
    use forward_declare::CppCast;
    use googletest::prelude::*;

    #[gtest]
    fn test_alias_to_template_instantiation() {
        let s = type_alias::MyTypeAlias::Create(123);
        assert_eq!(123, *s.value());
    }

    #[gtest]
    fn test_aliases_in_same_target_are_compatible() {
        let s: type_alias::MyTypeAlias = type_alias::MyTypeAlias::Create(456);
        let s2: type_alias::OtherTypeAliasInSameTarget = s;
        assert_eq!(456, *s2.value());
    }

    #[gtest]
    fn test_alias_in_different_target_than_template() {
        let s = type_alias_in_different_target::TypeAliasInDifferentTarget::Create(789);
        assert_eq!(789, *s.value());

        // Template instantiation from `type_alias_in_different_target` can be cast
        // (i.e. transmuted) into identical instantiation from `type_alias` crate.
        let s2: type_alias::MyTypeAlias = s.cpp_cast();
        assert_eq!(789, *s2.value());
    }
}
