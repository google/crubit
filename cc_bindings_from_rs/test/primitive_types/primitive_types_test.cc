// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <string>
#include <type_traits>

#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/primitive_types/primitive_types_cc_api.h"

namespace crubit {
namespace {

TEST(PrimitiveTypesTest, CVoidPtr) {
  std::string string;
  auto s =
      primitive_types::test_c_void_ptr::new_struct_with_c_void_pointer_member(
          &string, &string);
  EXPECT_EQ(s.ptr_mut, &string);
  EXPECT_EQ(s.ptr_const, &string);
  // We need an extra set of parentheses to avoid "too many arguments provided
  // to function-like macro invocation" when `EXPECT_TRUE` is used together with
  // `std::is_same_v<T, U>`.
  EXPECT_TRUE((std::is_same_v<void*, decltype(s.ptr_mut)>));
  EXPECT_TRUE((std::is_same_v<const void*, decltype(s.ptr_const)>));

  const void* string_const_ptr =
      primitive_types::test_c_void_ptr::identity_const_c_void_ptr(&string);
  EXPECT_EQ(string_const_ptr, &string);
  EXPECT_TRUE((std::is_same_v<const void*, decltype(string_const_ptr)>));
  EXPECT_TRUE((std::is_same_v<const void*(const void*),
                              decltype(primitive_types::test_c_void_ptr::
                                           identity_const_c_void_ptr)>));

  void* string_mut_ptr =
      primitive_types::test_c_void_ptr::identity_mut_c_void_ptr(&string);
  EXPECT_EQ(string_mut_ptr, &string);
  EXPECT_TRUE((std::is_same_v<void*, decltype(string_mut_ptr)>));
  EXPECT_TRUE(
      (std::is_same_v<void*(void*), decltype(primitive_types::test_c_void_ptr::
                                                 identity_mut_c_void_ptr)>));
}

}  // namespace
}  // namespace crubit
