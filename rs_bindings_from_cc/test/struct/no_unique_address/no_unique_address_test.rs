// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[cfg(test)]
mod tests {
    use googletest::prelude::*;
    use no_unique_address::*;

    use ctor::CtorNew as _;

    #[gtest]
    fn test_get() {
        let s = Struct::Make(1, 2);
        assert_eq!(s.field1(), &1);
        assert_eq!(s.field2(), &2);
    }

    #[gtest]
    fn test_padding_between_fields() {
        let s = PaddingBetweenFields::Make(1, 2);
        assert_eq!(s.field1, 1);
        assert_eq!(s.field2(), &2);
    }

    #[gtest]
    fn test_field_in_tail_padding() {
        ctor::emplace! {
            let s = FieldInTailPadding::ctor_new((1, 2, 3));
        }
        assert_eq!(s.inner_struct().inner_int_field, 1);
        assert_eq!(s.inner_struct().inner_char_field, 2);
        assert_eq!(s.char_in_tail_padding_of_prev_field, 3);
    }

    #[gtest]
    fn test_struct_with_fields_written_before_empty_no_unique_address_field() {
        ctor::emplace! {
          let mut s = StructWithFieldsWrittenBeforeEmptyNoUniqueAddressField::Make(1);
        }
        assert_eq!(s.field1, 1);
        assert_eq!(s.no_unique_address_empty_field().method(), 12345);
    }

    #[gtest]
    fn test_class_with_fields_written_before_empty_no_unique_address_field_compile() {
        let _ = ClassWithFieldsWrittenBeforeEmptyNoUniqueAddressField::default();
    }

    #[gtest]
    fn test_struct_with_bit_fields_and_no_unique_address_fields() {
        let s = StructWithBitFieldsAndNoUniqueAddressField::default();
        assert_eq!(s.field2, 54321);
        assert_eq!(*s.no_unique_address_int_field(), 67890);
        assert_eq!(s.no_unique_address_empty_field1().method(), 12345);
        assert_eq!(s.no_unique_address_empty_field2().method(), 12345);
    }
}
