// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/primitive_types/primitive_types.h"

#include <stdint.h>

#include <string>
#include <type_traits>

#include "gtest/gtest.h"

namespace crubit {
namespace {

TEST(PrimitiveTypesTest, CVoidPtr) {
  std::string string;
  auto s =
      primitive_types::test_c_void_ptr::new_struct_with_c_void_pointer_member(
          &string, &string);
  EXPECT_EQ(s.ptr_mut, &string);
  EXPECT_EQ(s.ptr_const, &string);
  static_assert(std::is_same_v<void*, decltype(s.ptr_mut)>);
  static_assert(std::is_same_v<const void*, decltype(s.ptr_const)>);

  const void* string_const_ptr =
      primitive_types::test_c_void_ptr::identity_const_c_void_ptr(&string);
  EXPECT_EQ(string_const_ptr, &string);
  static_assert(std::is_same_v<const void*, decltype(string_const_ptr)>);
  static_assert(std::is_same_v<const void*(const void*),
                               decltype(primitive_types::test_c_void_ptr::
                                            identity_const_c_void_ptr)>);

  void* string_mut_ptr =
      primitive_types::test_c_void_ptr::identity_mut_c_void_ptr(&string);
  EXPECT_EQ(string_mut_ptr, &string);
  static_assert(std::is_same_v<void*, decltype(string_mut_ptr)>);
  static_assert(
      std::is_same_v<
          void*(void*),
          decltype(primitive_types::test_c_void_ptr::identity_mut_c_void_ptr)>);
}

TEST(PrimitiveTypesTest, MaybeUninitTest) {
  namespace maybe_uninit = primitive_types::test_maybe_uninit;

  int32_t val = 0;
  const int32_t const_val = 0;

  auto& ref = maybe_uninit::maybe_uninit_ref(const_val);
  static_assert(std::is_same_v<const int32_t&, decltype(ref)>);

  auto& ref_mut = maybe_uninit::maybe_uninit_ref_mut(val);
  static_assert(std::is_same_v<int32_t&, decltype(ref_mut)>);

  auto ptr = maybe_uninit::maybe_uninit_ptr(&const_val);
  static_assert(std::is_same_v<const int32_t*, decltype(ptr)>);

  auto ptr_mut = maybe_uninit::maybe_uninit_ptr_mut(&val);
  static_assert(std::is_same_v<int32_t*, decltype(ptr_mut)>);
}

TEST(PrimitiveTypesTest, ArgumentTypes) {
  namespace types = primitive_types::argument_types;

  char* c_char_ptr = nullptr;
  types::c_char_ptr_arg(c_char_ptr);

  char* c_char_mut_ptr = nullptr;
  types::c_char_mut_ptr_arg(c_char_mut_ptr);
}

TEST(PrimitiveTypesTest, ReturnTypes) {
  namespace types = primitive_types::return_types;

  static_assert(std::is_same_v<decltype(types::c_void()), void>);
  static_assert(std::is_same_v<decltype(types::c_void_mut_ptr()), void*>);
  EXPECT_EQ(types::c_void_mut_ptr(), nullptr);

  static_assert(
      std::is_same_v<decltype(types::c_void_const_ptr()), const void*>);
  EXPECT_EQ(types::c_void_const_ptr(), nullptr);

  static_assert(std::is_same_v<decltype(types::c_char()), char>);
  EXPECT_EQ(types::c_char(), 0);

  static_assert(std::is_same_v<decltype(types::c_char_mut_ptr()), char*>);
  EXPECT_EQ(types::c_char_mut_ptr(), nullptr);

  static_assert(
      std::is_same_v<decltype(types::c_char_const_ptr()), const char*>);
  EXPECT_EQ(types::c_char_const_ptr(), nullptr);

  static_assert(std::is_same_v<decltype(types::c_schar()), signed char>);
  EXPECT_EQ(types::c_schar(), 0);

  static_assert(std::is_same_v<decltype(types::c_uchar()), unsigned char>);
  EXPECT_EQ(types::c_uchar(), 0);

  static_assert(std::is_same_v<decltype(types::c_short()), short>);
  EXPECT_EQ(types::c_short(), 0);

  static_assert(std::is_same_v<decltype(types::c_ushort()), unsigned short>);
  EXPECT_EQ(types::c_ushort(), 0);

  static_assert(std::is_same_v<decltype(types::c_int()), int>);
  EXPECT_EQ(types::c_int(), 0);

  static_assert(std::is_same_v<decltype(types::c_uint()), unsigned int>);
  EXPECT_EQ(types::c_uint(), 0);

  static_assert(std::is_same_v<decltype(types::c_long()), long>);
  EXPECT_EQ(types::c_long(), 0);

  static_assert(std::is_same_v<decltype(types::c_ulong()), unsigned long>);
  EXPECT_EQ(types::c_ulong(), 0);

  static_assert(std::is_same_v<decltype(types::c_longlong()), long long>);
  EXPECT_EQ(types::c_longlong(), 0);

  static_assert(
      std::is_same_v<decltype(types::c_ulonglong()), unsigned long long>);
  EXPECT_EQ(types::c_ulonglong(), 0);

  static_assert(std::is_same_v<decltype(types::c_float()), float>);
  EXPECT_EQ(types::c_float(), 0);

  static_assert(std::is_same_v<decltype(types::c_double()), double>);
  EXPECT_EQ(types::c_double(), 0);

  static_assert(std::is_same_v<decltype(types::i8()), int8_t>);
  EXPECT_EQ(types::i8(), 0);

  static_assert(std::is_same_v<decltype(types::u8()), uint8_t>);
  EXPECT_EQ(types::u8(), 0);

  static_assert(std::is_same_v<decltype(types::i16()), int16_t>);
  EXPECT_EQ(types::i16(), 0);

  static_assert(std::is_same_v<decltype(types::u16()), uint16_t>);
  EXPECT_EQ(types::u16(), 0);

  static_assert(std::is_same_v<decltype(types::i32()), int32_t>);
  EXPECT_EQ(types::i32(), 0);

  static_assert(std::is_same_v<decltype(types::u32()), uint32_t>);
  EXPECT_EQ(types::u32(), 0);

  static_assert(std::is_same_v<decltype(types::i64()), int64_t>);
  EXPECT_EQ(types::i64(), 0);

  static_assert(std::is_same_v<decltype(types::u64()), uint64_t>);
  EXPECT_EQ(types::u64(), 0);

  static_assert(std::is_same_v<decltype(types::isize()), intptr_t>);
  EXPECT_EQ(types::isize(), 0);

  static_assert(std::is_same_v<decltype(types::usize()), uintptr_t>);
  EXPECT_EQ(types::usize(), 0);

  static_assert(std::is_same_v<decltype(types::f32()), float>);
  EXPECT_EQ(types::f32(), 0);

  static_assert(std::is_same_v<decltype(types::f64()), double>);
  EXPECT_EQ(types::f64(), 0);
}

TEST(PrimitiveTypesTest, FieldTypes) {
  using primitive_types::field_types::Types;

  static_assert(std::is_same_v<decltype(Types::c_void_mut_ptr), void*>);
  static_assert(std::is_same_v<decltype(Types::c_void_const_ptr), const void*>);

  static_assert(std::is_same_v<decltype(Types::c_char), char>);

  static_assert(std::is_same_v<decltype(Types::c_schar), signed char>);
  static_assert(std::is_same_v<decltype(Types::c_uchar), unsigned char>);
  static_assert(std::is_same_v<decltype(Types::c_short), short>);
  static_assert(std::is_same_v<decltype(Types::c_ushort), unsigned short>);
  static_assert(std::is_same_v<decltype(Types::c_int), int>);
  static_assert(std::is_same_v<decltype(Types::c_uint), unsigned int>);
  static_assert(std::is_same_v<decltype(Types::c_long), long>);
  static_assert(std::is_same_v<decltype(Types::c_ulong), unsigned long>);
  static_assert(std::is_same_v<decltype(Types::c_longlong), long long>);
  static_assert(
      std::is_same_v<decltype(Types::c_ulonglong), unsigned long long>);
  static_assert(std::is_same_v<decltype(Types::c_float), float>);
  static_assert(std::is_same_v<decltype(Types::c_double), double>);

  static_assert(std::is_same_v<decltype(Types::i8), int8_t>);
  static_assert(std::is_same_v<decltype(Types::u8), uint8_t>);
  static_assert(std::is_same_v<decltype(Types::i16), int16_t>);
  static_assert(std::is_same_v<decltype(Types::u16), uint16_t>);
  static_assert(std::is_same_v<decltype(Types::i32), int32_t>);
  static_assert(std::is_same_v<decltype(Types::u32), uint32_t>);
  static_assert(std::is_same_v<decltype(Types::i64), int64_t>);
  static_assert(std::is_same_v<decltype(Types::u64), uint64_t>);
  static_assert(std::is_same_v<decltype(Types::isize), intptr_t>);
  static_assert(std::is_same_v<decltype(Types::usize), uintptr_t>);
  static_assert(std::is_same_v<decltype(Types::f32), float>);
  static_assert(std::is_same_v<decltype(Types::f64), double>);

  static_assert(std::is_same_v<decltype(Types::i8_func),
                               std::type_identity_t<void(int8_t)>*>);

  static_assert(std::is_same_v<decltype(Types::c_char_func),
                               std::type_identity_t<void(char)>*>);
}
}  // namespace
}  // namespace crubit
