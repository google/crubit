// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/functions/functions.h"

#include <cstdint>
#include <optional>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "support/rs_std/rs_char.h"

namespace crubit {
namespace {

using testing::DoubleEq;

namespace fn_abi_tests = functions::fn_abi_tests;
namespace fn_param_ty_tests = functions::fn_param_ty_tests;

TEST(FnAbiTest, ExternCNoMangle) {
  EXPECT_THAT(fn_abi_tests::get_42_as_f64_via_no_mangle_extern_c(),
              DoubleEq(42.0));
}

TEST(FnAbiTest, ExternCWithExportName) {
  EXPECT_EQ(12 + 34,
            fn_abi_tests::add_i32_via_extern_c_with_export_name(12, 34));
}

TEST(FnAbiTest, ExternCWithMangling) {
  EXPECT_EQ(12 + 34, fn_abi_tests::add_i32_via_extern_c_with_mangling(12, 34));
}

TEST(FnAbiTest, Rust) {
  EXPECT_EQ(12 + 34, fn_abi_tests::add_i32_via_rust_abi(12, 34));
}

TEST(FnParamTyTest, Float64) {
  EXPECT_THAT(fn_param_ty_tests::add_f64(12.0, 34.0), DoubleEq(12.0 + 34.0));
}

TEST(FnParamTyTest, Int32) {
  EXPECT_EQ(12 + 34, fn_param_ty_tests::add_i32(12, 34));
}

TEST(FnParamTyTest, rs_char) {
  std::optional<const rs_std::rs_char> input = rs_std::rs_char::from_u32(U'A');
  ASSERT_TRUE(input.has_value());
  rs_std::rs_char output = fn_param_ty_tests::char_to_ascii_lowercase(*input);
  EXPECT_EQ(std::uint32_t{U'a'}, std::uint32_t{output});
}

TEST(FnParamTyTest, Int32Ptr) {
  std::int32_t x = 12;
  std::int32_t y = 34;
  std::int32_t sum;  // uninitialized
  fn_param_ty_tests::add_i32_via_ptr(&x, &y, &sum);
  EXPECT_EQ(12, x);
  EXPECT_EQ(34, y);
  EXPECT_EQ(12 + 34, sum);
}

TEST(FnParamTyTest, Int32Ref) {
  std::int32_t x = 123;
  std::int32_t y = 456;
  const std::int32_t& result = fn_param_ty_tests::get_ref_to_smaller_int(x, y);
  EXPECT_EQ(&result, &x);
}

TEST(FnParamTyTest, Int32RefWithInferredLifetime) {
  std::int32_t x = 123;
  const std::int32_t& result =
      fn_param_ty_tests::get_identical_ref_with_inferred_lifetime(x);
  EXPECT_EQ(&result, &x);
}

TEST(FnParamTyTest, Int32MutRef) {
  std::int32_t sum = -123;
  fn_param_ty_tests::set_mut_ref_to_sum_of_ints(sum, 456, 789);
  EXPECT_EQ(sum, 456 + 789);
}

std::int32_t AddInt32(std::int32_t x, std::int32_t y) { return x + y; }

std::int32_t MultiplyInt32(std::int32_t x, std::int32_t y) { return x * y; }

TEST(FnParamTyTest, FnPtr) {
  std::int32_t sum = fn_param_ty_tests::apply_binary_i32_op(12, 34, AddInt32);
  EXPECT_EQ(sum, 12 + 34);

  std::int32_t product =
      fn_param_ty_tests::apply_binary_i32_op(56, 78, MultiplyInt32);
  EXPECT_EQ(product, 56 * 78);
}

TEST(OtherFnTest, VoidReturningFunction) {
  namespace tests = functions::unit_ret_ty_tests;
  tests::set_global_i32_via_extern_c_with_export_name(123);
  EXPECT_EQ(123, tests::get_global_i32_via_extern_c_with_export_name());

  tests::set_global_i32_via_extern_c_with_export_name(456);
  EXPECT_EQ(456, tests::get_global_i32_via_extern_c_with_export_name());
}

TEST(OtherFnTest, DuplicatedParamNames) {
  namespace tests = functions::other_fn_param_tests;
  EXPECT_EQ(12 + 34, tests::add_i32_via_rust_abi_with_duplicated_param_names(
                         12, 34, 56, 78));
}

// TODO(jeanpierreda): Investigate if there is a way to test that the generated
// function is actually deprecated.
TEST(FnAttributeTest, DeprecatedAttribute) {
  namespace tests = functions::fn_attribute_tests;
  EXPECT_EQ(12 + 34, tests::add_i32(12, 34));
}

TEST(UnsafeFnTest, UnsafeFunction) {
  namespace tests = functions::unsafe_fn_tests;
  EXPECT_EQ(12 + 34, tests::unsafe_add(12, 34));
}

// Right now, these tests really just verify existence.
// TODO(b/335837488): Verify compilation failure when you discard
TEST(NoDiscardTest, WithoutMessageWorks) {
  namespace tests = functions::fn_must_use_tests;
  EXPECT_EQ(5, tests::no_msg_add(2, 3));
}

TEST(NoDiscardTest, WithMessageWorks) {
  namespace tests = functions::fn_must_use_tests;
  EXPECT_EQ(5, tests::msg_add(2, 3));
}

}  // namespace
}  // namespace crubit
