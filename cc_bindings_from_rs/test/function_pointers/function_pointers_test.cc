// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/function_pointers/function_pointers.h"

#include <cstdint>
#include <tuple>
#include <type_traits>
#include <utility>

#include "gtest/gtest.h"

namespace {

using ::function_pointers::call_fn_ptr_no_args_or_return;
using ::function_pointers::call_fn_ptr_with_five;
using ::function_pointers::call_fn_ptr_with_repr_c_struct_ptr_containing_seven;
using ::function_pointers::CStruct;
using ::function_pointers::HasFnPtrField;

TEST(FunctionPointersTest, TestFnPtrFields) {
  HasFnPtrField has_fn_ptr_field = HasFnPtrField::with_add_ten();
  EXPECT_EQ((has_fn_ptr_field.ptr)(5), 15);
}

extern "C" void DoNothing() {}

TEST(FunctionPointersTest, TestCallFnPtrNoArgsOrReturn) {
  call_fn_ptr_no_args_or_return(DoNothing);
}

extern "C" int32_t AddTen(int32_t x) { return x + 10; }

TEST(FunctionPointersTest, TestRustCallToCFnPtr) {
  EXPECT_EQ(call_fn_ptr_with_five(AddTen), 15);
}

extern "C" int32_t GetFieldValue(const CStruct* c_struct) {
  return c_struct->field;
}

TEST(FunctionPointersTest, TestRustCallToCFnPtrWithReprCStructPtr) {
  EXPECT_EQ(call_fn_ptr_with_repr_c_struct_ptr_containing_seven(GetFieldValue),
            7);
}

}  // namespace