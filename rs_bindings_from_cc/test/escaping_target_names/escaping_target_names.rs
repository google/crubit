// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {

    #[test]
    fn test_dashes_in_crate_name() {
        use has_dashes_in_name::SomeStruct;
        use uses_target_with_dashes::SomeFunc;
        let s = SomeStruct { value: 42 };
        assert_eq!(SomeFunc(s), 42);
    }
}
