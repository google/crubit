// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {
    use googletest::prelude::*;

    #[gtest]
    fn test_rs_char_parameter_type_and_return_type() {
        use roundtrip::rs_char_test::*;
        let roundtrip = NextChar('a');
        assert_eq!(roundtrip, 'b');
    }

    #[gtest]
    fn test_rs_char_field_type() {
        use roundtrip::rs_char_test::*;
        let s = SomeStruct { c: 'x' };
        assert_eq!('x', s.GetChar());
    }

    #[gtest]
    fn test_rs_char_via_type_alias() {
        use roundtrip::rs_char_test::*;
        let roundtrip = NextCharViaTypeAlias('a');
        assert_eq!(roundtrip, 'b');
    }

    #[gtest]
    fn test_rs_char_via_import() {
        use roundtrip::rs_char_test::*;
        let roundtrip = NextCharViaImport('a');
        assert_eq!(roundtrip, 'b');
    }
}
