// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/enums/result.h"

#include <cstdint>
#include <utility>

#include "gtest/gtest.h"

namespace {

TEST(ResultTest, GetsResult) {
  auto res = result::GetsResult::new_(42);
  EXPECT_TRUE(res.value.has_value());
}

TEST(ResultTest, NestedResult) {
  auto res = result::NestedResult::new_(42);
  ASSERT_TRUE(res.in_ok.has_value());
  EXPECT_FALSE(res.in_ok.value().has_value());
  EXPECT_EQ(res.in_ok.value().err(), 42);
  EXPECT_FALSE(res.in_err.has_value());
  ASSERT_TRUE(res.in_err.err().has_value());
  EXPECT_EQ(res.in_err.err().value(), 42);
}

TEST(ResultTest, CopyNoDefaultResult) {
  auto res = result::CopyNoDefaultResult::new_(42);
  ASSERT_TRUE(res.in_ok.has_value());
  EXPECT_EQ(res.in_ok.value().val, 42);
  EXPECT_FALSE(res.in_err.has_value());
  EXPECT_EQ(res.in_err.err().val, 42);
}

TEST(ResultTest, CloneNoDefaultResult) {
  auto res = result::CloneNoDefaultResult::new_(42);
  ASSERT_TRUE(res.in_ok.has_value());
  EXPECT_EQ(res.in_ok.value().val, 42);
  EXPECT_FALSE(res.in_err.has_value());
  EXPECT_EQ(res.in_err.err().val, 42);
}

TEST(ResultTest, HasDefaultResult) {
  auto res = result::HasDefaultResult::new_("hello");
  ASSERT_TRUE(res.in_ok.has_value());
  EXPECT_EQ(res.in_ok.value().val().to_string_view(), "hello");
  EXPECT_FALSE(res.in_err.has_value());
  EXPECT_EQ(res.in_err.err().val().to_string_view(), "hello");
}

TEST(ResultTest, HasNoDefaultResult) {
  auto res = result::HasNoDefaultResult::new_("good bye");
  ASSERT_TRUE(res.in_ok.has_value());
  EXPECT_EQ(res.in_ok.value().val().to_string_view(), "good bye");
  EXPECT_FALSE(res.in_err.has_value());
  EXPECT_EQ(res.in_err.err().val().to_string_view(), "good bye");
  EXPECT_EQ(res.in_ok.value().val().to_string_view(),
            res.in_err.err().val().to_string_view());
}

TEST(ResultTest, ConstructAndPassOkResult) {
  auto copy_no_default = result::CopyNoDefaultResult::new_(42);
  rs_std::Result<result::CopyNoDefault, uint8_t> res(
      std::move(copy_no_default.in_ok));
  EXPECT_EQ(result::take_result_copy_no_default_ok(res), 42);
  res = rs_std::unexpected<uint8_t>(36);
  EXPECT_EQ(result::take_result_copy_no_default_ok(res), 36);
}

TEST(ResultTest, ConstructAndPassErrResult) {
  auto clone_no_default = result::CloneNoDefaultResult::new_(42);
  rs_std::Result<uint8_t, result::CloneNoDefault> err(
      std::move(clone_no_default.in_err));
  EXPECT_EQ(result::take_result_clone_no_default_err(err), 42);
  err = 122;
  EXPECT_EQ(result::take_result_clone_no_default_err(err), 122);
}

TEST(ResultTest, ConstructAndPassResultHasDefault) {
  rs_std::Result<result::HasDefault, uint8_t> res(
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
  rs_std::Result<result::HasDefault, uint8_t> res(
      result::HasDefault::new_("accessible value"));
  EXPECT_EQ(res->val(), "accessible value");
}

TEST(ResultTest, ConstructAndPassResultByValue) {
  rs_std::Result<uint8_t, uint8_t> res(123);
  EXPECT_EQ(result::take_result_by_value(res), 123);
  rs_std::Result<uint8_t, uint8_t> err(rs_std::unexpected<uint8_t>(34));
  EXPECT_EQ(result::take_result_by_value(err), 34);
}

TEST(ResultTest, ReturnResultByValue) {
  rs_std::Result<uint8_t, uint8_t> res = result::return_result_by_value();
  ASSERT_TRUE(res.has_value());
  EXPECT_EQ(res.value(), 1);
}

template <typename R>
concept HasValue = requires(R& r) { r.value(); };

template <typename R>
concept HasDeref = requires(R& r) { *r; };

template <typename R>
concept HasError = requires(R& r) { r.error(); };

template <typename R>
concept HasErr = requires(R& r) { r.err(); };

TEST(ResultTest, ResultUnitOk) {
  rs_std::Result<rs_std::unit_t, uint8_t> res_ok =
      result::return_result_unit_ok(true);
  ASSERT_TRUE(res_ok.has_value());
  EXPECT_TRUE(result::take_result_unit_ok(res_ok));
  static_assert(!HasValue<rs_std::Result<rs_std::unit_t, uint8_t>>,
                "Result<unit_t, E>::value() must not be available");
  static_assert(!HasDeref<rs_std::Result<rs_std::unit_t, uint8_t>>,
                "Result<unit_t, E>::operator*() must not be available");

  rs_std::Result<rs_std::unit_t, uint8_t> res_err =
      result::return_result_unit_ok(false);
  EXPECT_FALSE(res_err.has_value());
  EXPECT_EQ(res_err.err(), 42);
  EXPECT_FALSE(result::take_result_unit_ok(res_err));

  rs_std::Result<rs_std::unit_t, uint8_t> res_in_place(std::in_place,
                                                       rs_std::unit);
  EXPECT_TRUE(res_in_place.has_value());
}

TEST(ResultTest, ResultUnitErr) {
  rs_std::Result<uint8_t, rs_std::unit_t> res_ok =
      result::return_result_unit_err(false);
  ASSERT_TRUE(res_ok.has_value());
  EXPECT_EQ(res_ok.value(), 100);
  EXPECT_FALSE(result::take_result_unit_err(res_ok));

  rs_std::Result<uint8_t, rs_std::unit_t> res_err =
      result::return_result_unit_err(true);
  EXPECT_FALSE(res_err.has_value());
  EXPECT_TRUE(result::take_result_unit_err(res_err));
  static_assert(!HasError<rs_std::Result<uint8_t, rs_std::unit_t>>,
                "Result<T, unit_t>::error() must not be available");
  static_assert(!HasErr<rs_std::Result<uint8_t, rs_std::unit_t>>,
                "Result<T, unit_t>::err() must not be available");

  rs_std::Result<uint8_t, rs_std::unit_t> res_unexpect(rs_std::unexpect,
                                                       rs_std::unit);
  EXPECT_FALSE(res_unexpect.has_value());
}

}  // namespace
