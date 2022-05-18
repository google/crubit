// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_NO_UNIQUE_ADDRESS_NO_UNIQUE_ADDRESS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_NO_UNIQUE_ADDRESS_NO_UNIQUE_ADDRESS_H_
#pragma clang lifetime_elision

// The no_unique_address.h header is present both in
// rs_bindings_from_cc/test/struct/no_unique_address/ and in
// rs_bindings_from_cc/test/golden/ because the format provides end-to-end
// coverage for working accessor functions, while the latter helps manually
// inspect and verify the expected layout of the generated Rust struct.

struct Struct final {
  static Struct Make(int f1, char f2) { return Struct{f1, f2}; }
  // Nobody would ever use a no_unique_address int/char field, this is just
  // enough to test that the transmute is correct.
  [[no_unique_address]] int field1 = 1;
  [[no_unique_address]] char field2 = 2;
};

// Regression test for b/232418721.  This tests that the offset of `field2` is
// correct (given its alignment requirements there need to be 3 bytes of padding
// between `field1` and `field2`).  The verification is mostly done through
// compile-time assertions of field offsets in the generated Rust code.  Before
// cl/448287893 `field2` would be incorrectly placed at offset 1.
struct PaddingBetweenFields final {
  static PaddingBetweenFields Make(char f1, int f2) {
    return PaddingBetweenFields{f1, f2};
  }

  char field1 = 1;                       // size: 1, alignment: 1 => offset: 0
  [[no_unique_address]] int field2 = 2;  // size: 4, alignment: 4 => offset: 4
};

// Layout properties of FieldInTailPadding_InnerStruct look as follows:
// - alignment: 4 (because of `inner_int_field`)
// - dsize (size without padding): 5
//   (4 bytes for `inner_int_field`, 1 byte for `inner_char_field`)
// - size: 8 (dsize adjusted up to account for alignment)
struct FieldInTailPadding_InnerStruct {
  int inner_int_field;    // size: 4, alignment: 4 => offset: 0
  char inner_char_field;  // size: 1, alignment: 1 => offset: 4

  // User-defined destructor to make this struct non-POD for the purposes of
  // layout.
  ~FieldInTailPadding_InnerStruct() {}
};

// Regression test against b/232418721#comment7.  This tests that the offset of
// `char_in_tail_padding_of_prev_field`` is correct - because of
// `no_unique_address` this field should be laid out inside the tail padding of
// `inner_struct` (i.e. offset of `char_in_tail_padding_of_prev_field`` should
// be 5 = dsize of `s` rather than 8 = size of `s`).  The verification is mostly
// done through compile-time assertions of field offsets in the generated Rust
// code.  The initial alignment-based fix idea for b/232418721 would incorrectly
// put `char_in_tail_padding_of_prev_field` at offset 8.
struct FieldInTailPadding {
  FieldInTailPadding(int inner_int, char inner_char, char outer_char) {
    inner_struct.inner_int_field = inner_int;
    inner_struct.inner_char_field = inner_char;
    char_in_tail_padding_of_prev_field = outer_char;
  }

  [[no_unique_address]] FieldInTailPadding_InnerStruct inner_struct;
  char char_in_tail_padding_of_prev_field;  // offset: 5 (dsize of `s`).
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_NO_UNIQUE_ADDRESS_NO_UNIQUE_ADDRESS_H_
