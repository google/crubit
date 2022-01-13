// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TYPES_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TYPES_H_

#include <cstddef>
#include <cstdint>

struct SomeStruct final {};

struct FieldTypeTestStruct final {
  bool bool_field;
  char char_field;

  unsigned char unsigned_char_field;
  signed char signed_char_field;
  char16_t char16_t_field;
  char32_t char32_t_field;
  wchar_t wchar_t_field;

  short short_field;
  int int_field;
  long long_field;
  long long long_long_field;

  unsigned short unsigned_short_field;
  unsigned int unsigned_int_field;
  unsigned long unsigned_long_field;
  unsigned long long unsigned_long_long_field;

  signed short signed_short_field;
  signed int signed_int_field;
  signed long signed_long_field;
  signed long long signed_long_long_field;

  int8_t int8_t_field;
  int16_t int16_t_field;
  int32_t int32_t_field;
  int64_t int64_t_field;
  std::int8_t std_int8_t_field;
  std::int16_t std_int16_t_field;
  std::int32_t std_int32_t_field;
  std::int64_t std_int64_t_field;

  uint8_t uint8_t_field;
  uint16_t uint16_t_field;
  uint32_t uint32_t_field;
  uint64_t uint64_t_field;
  std::uint8_t std_uint8_t_field;
  std::uint16_t std_uint16_t_field;
  std::uint32_t std_uint32_t_field;
  std::uint64_t std_uint64_t_field;

  ptrdiff_t ptrdiff_t_field;
  size_t size_t_field;
  intptr_t intptr_t_field;
  uintptr_t uintptr_t_field;
  std::ptrdiff_t std_ptrdiff_t_field;
  std::size_t std_size_t_field;
  std::intptr_t std_intptr_t_field;
  std::uintptr_t std_uintptr_t_field;

  float float_field;
  double double_field;

  int* ptr_field;

  SomeStruct struct_field;
  SomeStruct* struct_ptr_field;
  const SomeStruct* const_struct_ptr_field;
  SomeStruct& struct_ref_field;
  const SomeStruct& const_struct_ref_field;
};

inline void VoidReturningFunction() {}

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TYPES_H_
