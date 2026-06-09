// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/enums/result.h"

#include "gtest/gtest.h"

namespace {

TEST(ResultTest, GetsResult) {
  auto res = result::GetsResult::new_(42);
  EXPECT_TRUE(res.value.has_value());
}

TEST(ResultTest, NestedResult) {
  auto res = result::NestedResult::new_(42);
  EXPECT_TRUE(res.in_ok.has_value());
  EXPECT_FALSE(res.in_ok.value().has_value());
  EXPECT_EQ(res.in_ok.value().err(), 42);
  EXPECT_FALSE(res.in_err.has_value());
  EXPECT_TRUE(res.in_err.err().has_value());
  EXPECT_EQ(res.in_err.err().value(), 42);
}

TEST(ResultTest, CopyNoDefaultResult) {
  auto res = result::CopyNoDefaultResult::new_(42);
  EXPECT_TRUE(res.in_ok.has_value());
  EXPECT_EQ(res.in_ok.value().val, 42);
  EXPECT_FALSE(res.in_err.has_value());
  EXPECT_EQ(res.in_err.err().val, 42);
}

TEST(ResultTest, CloneNoDefaultResult) {
  auto res = result::CloneNoDefaultResult::new_(42);
  EXPECT_TRUE(res.in_ok.has_value());
  EXPECT_EQ(res.in_ok.value().val, 42);
  EXPECT_FALSE(res.in_err.has_value());
  EXPECT_EQ(res.in_err.err().val, 42);
}

TEST(ResultTest, HasDefaultResult) {
  auto res = result::HasDefaultResult::new_("hello");
  EXPECT_TRUE(res.in_ok.has_value());
  EXPECT_EQ(res.in_ok.value().val().to_string_view(), "hello");
  EXPECT_FALSE(res.in_err.has_value());
  EXPECT_EQ(res.in_err.err().val().to_string_view(), "hello");
}

TEST(ResultTest, HasNoDefaultResult) {
  auto res = result::HasNoDefaultResult::new_("good bye");
  EXPECT_TRUE(res.in_ok.has_value());
  EXPECT_EQ(res.in_ok.value().val().to_string_view(), "good bye");
  EXPECT_FALSE(res.in_err.has_value());
  EXPECT_EQ(res.in_err.err().val().to_string_view(), "good bye");
  EXPECT_EQ(res.in_ok.value().val().to_string_view(),
            res.in_err.err().val().to_string_view());
}

TEST(ResultTest, ConstructAndPassOkResult) {
  auto copy_no_default = result::CopyNoDefaultResult::new_(42);
  rs_std::Result<result::CopyNoDefault, std::uint8_t> res(
      std::move(copy_no_default.in_ok));
  EXPECT_EQ(result::take_result_copy_no_default_ok(res), 42);
  res = rs_std::unexpected<std::uint8_t>(36);
  EXPECT_EQ(result::take_result_copy_no_default_ok(res), 36);
}

TEST(ResultTest, ConstructAndPassErrResult) {
  auto clone_no_default = result::CloneNoDefaultResult::new_(42);
  rs_std::Result<std::uint8_t, result::CloneNoDefault> err(
      std::move(clone_no_default.in_err));
  EXPECT_EQ(result::take_result_clone_no_default_err(err), 42);
  err = 122;
  EXPECT_EQ(result::take_result_clone_no_default_err(err), 122);
}

TEST(ResultTest, ConstructAndPassResultHasDefault) {
  rs_std::Result<result::HasDefault, std::uint8_t> res(
      result::HasDefault::new_("halo strategy"));
  EXPECT_EQ(result::take_result_has_default(&res), "halo strategy");
  auto has_default = std::move(res.value());
  EXPECT_EQ(has_default.val().to_string_view(), "halo strategy");
  // Moving leaves the default value behind.
  EXPECT_EQ(res.value().val().to_string_view(), "");
  res = rs_std::unexpected<std::uint8_t>(122);
  EXPECT_EQ(result::take_result_has_default(&res), "a number");
}

TEST(ResultTest, ConstructAndAccessResultHasDefault) {
  rs_std::Result<result::HasDefault, std::uint8_t> res(
      result::HasDefault::new_("accessible value"));
  EXPECT_EQ(res->val(), "accessible value");
}

TEST(ResultTest, ConstructAndPassResultByValue) {
  rs_std::Result<std::uint8_t, std::uint8_t> res(123);
  EXPECT_EQ(result::take_result_by_value(res), 123);
  rs_std::Result<std::uint8_t, std::uint8_t> err(
      rs_std::unexpected<std::uint8_t>(34));
  EXPECT_EQ(result::take_result_by_value(err), 34);
}

TEST(ResultTest, ReturnResultByValue) {
  rs_std::Result<std::uint8_t, std::uint8_t> res =
      result::return_result_by_value();
  EXPECT_TRUE(res.has_value());
  EXPECT_EQ(res.value(), 1);
}

}  // namespace
