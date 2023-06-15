// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/functions/functions_cc_api.h"
#include "support/rs_std/rs_char.h"

namespace crubit {
namespace {

using testing::DoubleEq;

namespace fn_abi_tests = functions::fn_abi_tests;
namespace fn_param_ty_tests = functions::fn_param_ty_tests;

TEST(FnAbiTests, ExternCNoMangle) {
  EXPECT_THAT(fn_abi_tests::get_42_as_f64_via_no_mangle_extern_c(),
              DoubleEq(42.0));
}

TEST(FnAbiTests, ExternCWithExportName) {
  EXPECT_EQ(12 + 34,
            fn_abi_tests::add_i32_via_extern_c_with_export_name(12, 34));
}

TEST(FnAbiTests, ExternCWithMangling) {
  // TODO(b/262904507): Uncomment the test assertion below after ensuring that
  // the `genrule` in `test/functions/BUILD` invokes `cc_bindings_from_rs` with
  // the same rustc cmdline flags as when `rustc` is used to build
  // `functions.rs` for `rust_library`.  Otherwise, the mangled name will be
  // slightly different - e.g.:
  // _ZN9functions34add_i32_via_extern_c_with_mangling17h9cf06f3d70bfe03aE vs
  // _ZN9functions34add_i32_via_extern_c_with_mangling17hc48a5cd0f6e44291E
  //
  // EXPECT_EQ(12 + 34,
  //           fn_abi_tests::add_i32_via_extern_c_with_mangling(12, 34));
}

TEST(FnAbiTests, Rust) {
  EXPECT_EQ(12 + 34, fn_abi_tests::add_i32_via_rust_abi(12, 34));
}

TEST(FnParamTyTests, Float64) {
  EXPECT_THAT(fn_param_ty_tests::add_f64(12.0, 34.0), DoubleEq(12.0 + 34.0));
}

TEST(FnParamTyTests, Int32) {
  EXPECT_EQ(12 + 34, fn_param_ty_tests::add_i32(12, 34));
}

TEST(FnParamTyTests, rs_char) {
  std::optional<const rs_std::rs_char> input = rs_std::rs_char::from_u32(U'A');
  ASSERT_TRUE(input.has_value());
  rs_std::rs_char output = fn_param_ty_tests::char_to_ascii_lowercase(*input);
  EXPECT_EQ(std::uint32_t{U'a'}, std::uint32_t{output});
}

TEST(FnParamTyTests, Int32Ptr) {
  std::int32_t x = 12;
  std::int32_t y = 34;
  std::int32_t sum;  // uninitialized
  fn_param_ty_tests::add_i32_via_ptr(&x, &y, &sum);
  EXPECT_EQ(12, x);
  EXPECT_EQ(34, y);
  EXPECT_EQ(12 + 34, sum);
}

TEST(FnParamTyTests, Int32Ref) {
  std::int32_t x = 123;
  std::int32_t y = 456;
  const std::int32_t& result = fn_param_ty_tests::get_ref_to_smaller_int(x, y);
  EXPECT_EQ(&result, &x);
}

TEST(FnParamTyTests, Int32RefWithInferredLifetime) {
  std::int32_t x = 123;
  const std::int32_t& result =
      fn_param_ty_tests::get_identical_ref_with_inferred_lifetime(x);
  EXPECT_EQ(&result, &x);
}

TEST(FnParamTyTests, Int32MutRef) {
  std::int32_t sum = -123;
  fn_param_ty_tests::set_mut_ref_to_sum_of_ints(sum, 456, 789);
  EXPECT_EQ(sum, 456 + 789);
}

std::int32_t AddInt32(std::int32_t x, std::int32_t y) { return x + y; }

std::int32_t MultiplyInt32(std::int32_t x, std::int32_t y) { return x * y; }

TEST(FnParamTyTests, FnPtr) {
  std::int32_t sum = fn_param_ty_tests::apply_binary_i32_op(12, 34, AddInt32);
  EXPECT_EQ(sum, 12 + 34);

  std::int32_t product =
      fn_param_ty_tests::apply_binary_i32_op(56, 78, MultiplyInt32);
  EXPECT_EQ(product, 56 * 78);
}

TEST(OtherFnTests, VoidReturningFunction) {
  namespace tests = functions::unit_ret_ty_tests;
  tests::set_global_i32_via_extern_c_with_export_name(123);
  EXPECT_EQ(123, tests::get_global_i32_via_extern_c_with_export_name());

  tests::set_global_i32_via_extern_c_with_export_name(456);
  EXPECT_EQ(456, tests::get_global_i32_via_extern_c_with_export_name());
}

TEST(OtherFnTests, DuplicatedParamNames) {
  namespace tests = functions::other_fn_param_tests;
  EXPECT_EQ(12 + 34, tests::add_i32_via_rust_abi_with_duplicated_param_names(
                         12, 34, 56, 78));
}

}  // namespace
}  // namespace crubit
