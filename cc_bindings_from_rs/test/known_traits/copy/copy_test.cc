// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <type_traits>
#include <utility>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/known_traits/copy/copy_cc_api.h"

namespace crubit {
namespace {

TEST(CopyTest, ExplicitImpl) {
  namespace tests = copy::explicit_impl;
  tests::SomeStruct s = tests::create_struct(123);

  // The next line invokes the copy C++ constructor.
  tests::SomeStruct copy(s);
  static_assert(std::is_trivially_copy_constructible_v<tests::SomeStruct>);

  // Minimal verification that the copy constructor worked as expected.
  EXPECT_EQ(123, tests::extract_int(std::move(copy)));
}

TEST(CopyTest, DerivedImpl) {
  namespace tests = copy::derived_impl;
  tests::SomeStruct s = tests::create_struct(123);

  // The next line invokes the copy C++ constructor.
  tests::SomeStruct copy(s);
  static_assert(std::is_trivially_copy_constructible_v<tests::SomeStruct>);

  // Minimal verification that the copy constructor worked as expected.
  EXPECT_EQ(123, tests::extract_int(std::move(copy)));
}

TEST(CopyTest, NoImpl) {
  namespace tests = copy::no_impl;
  static_assert(!std::is_copy_constructible_v<tests::SomeStruct>);
}

}  // namespace
}  // namespace crubit
