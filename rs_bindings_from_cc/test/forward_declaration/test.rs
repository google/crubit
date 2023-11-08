// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {
    use item_exists::type_exists;

    #[test]
    fn test_same_library() {
        assert!(type_exists!(definition_and_forward_declaration_same_cc_library::A));
        assert!(type_exists!(definition_and_forward_declaration_same_cc_library::my_namespace::B));
    }

    // TODO(b/309614052): Currently Crubit cannot generate bindings if the forward
    // declaration and definition are from different targets.
    #[test]
    fn test_separate_library() {
        assert!(!type_exists!(definition_and_forward_declaration_separate_cc_library::A));
        assert!(!type_exists!(
            definition_and_forward_declaration_separate_cc_library::my_namespace::B
        ));
    }
}
