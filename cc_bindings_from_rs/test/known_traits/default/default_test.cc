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

template <typename TypeUnderTest>
void MainTestBody(std::int32_t expected_default_value) {
  static_assert(std::is_default_constructible_v<TypeUnderTest>);
  static_assert(!std::is_trivially_default_constructible_v<TypeUnderTest>);

  // The next line invokes the default C++ constructor, which calls into the
  // `Default::default()` static method on Rust side.
  TypeUnderTest s{};
  EXPECT_EQ(expected_default_value, TypeUnderTest::extract_int(std::move(s)));
}

TEST(DefaultTest, ExplicitImpl) {
  MainTestBody<rs_default::explicit_impl::SomeStruct>(42);
}

TEST(DefaultTest, DerivedImpl) {
  MainTestBody<rs_default::derived_impl::SomeStruct>(0);
}

TEST(DefaultTest, NoImpl) {
  namespace tests = rs_default::no_impl;
  static_assert(!std::is_default_constructible_v<tests::SomeStruct>);
}

}  // namespace
}  // namespace crubit
