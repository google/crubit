// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <type_traits>
#include <utility>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/known_traits/default/rs_default_cc_api.h"

namespace crubit {
namespace {

TEST(DefaultTest, ExplicitImpl) {
  namespace tests = rs_default::explicit_impl;
  static_assert(std::is_default_constructible_v<tests::SomeStruct>);
  static_assert(!std::is_trivially_default_constructible_v<tests::SomeStruct>);

  // The next linke invokes the default C++ constructor, which calls into the
  // `Default` impl on Rust side (which happens to initialize the struct with
  // 42).
  tests::SomeStruct s{};
  EXPECT_EQ(42, tests::extract_int(std::move(s)));
}

TEST(DefaultTest, DerivedImpl) {
  namespace tests = rs_default::derived_impl;
  static_assert(std::is_default_constructible_v<tests::SomeStruct>);
  static_assert(!std::is_trivially_default_constructible_v<tests::SomeStruct>);

  // The next linke invokes the default C++ constructor, which calls into the
  // `Default` impl on Rust side (the derived impl happens to initialize the
  // struct with 0).
  tests::SomeStruct s{};
  EXPECT_EQ(0, tests::extract_int(std::move(s)));
}

TEST(DefaultTest, NoImpl) {
  namespace tests = rs_default::no_impl;
  static_assert(!std::is_default_constructible_v<tests::SomeStruct>);
}

}  // namespace
}  // namespace crubit
