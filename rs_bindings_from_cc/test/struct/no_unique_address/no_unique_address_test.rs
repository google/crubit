// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {
    use no_unique_address::*;

    use ctor::CtorNew as _;

    #[test]
    fn test_get() {
        let s = Struct::Make(1, 2);
        assert_eq!(s.field1(), &1);
        assert_eq!(s.field2(), &2);
    }

    #[test]
    fn test_padding_between_fields() {
        let s = PaddingBetweenFields::Make(1, 2);
        assert_eq!(s.field1, 1);
        assert_eq!(s.field2(), &2);
    }

    #[test]
    fn test_field_in_tail_padding() {
        ctor::emplace! {
            let s = FieldInTailPadding::ctor_new((1, 2, 3));
        }
        assert_eq!(s.inner_struct().inner_int_field, 1);
        assert_eq!(s.inner_struct().inner_char_field, 2);
        assert_eq!(s.char_in_tail_padding_of_prev_field, 3);
    }
}
