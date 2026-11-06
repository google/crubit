// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#include "cc_bindings_from_rs/test/enums/option_specialization_collision.h"

#include <cstdint>
#include <type_traits>
#include <utility>

#include "gtest/gtest.h"

namespace {

using ::option_specialization_collision::Value;

// `Option<&i32>` is bound, with the layout of the Rust type. `Option<*const
// i32>` would be the same C++ type, so it isn't bound (b/575916406).
using OptionRef = decltype(option_specialization_collision::by_ref(
    std::declval<const std::int32_t*>()));
static_assert(std::is_same_v<OptionRef, rs::Option<const std::int32_t*>>);
static_assert(sizeof(OptionRef) == sizeof(const std::int32_t*));

TEST(OptionSpecializationCollisionTest, Function) {
  std::int32_t x = 42;
  OptionRef option = option_specialization_collision::by_ref(&x);
  ASSERT_TRUE(option.has_value());
  EXPECT_EQ(*option, &x);
}

TEST(OptionSpecializationCollisionTest, Method) {
  Value value;
  value.value = 42;
  OptionRef option = value.get();
  ASSERT_TRUE(option.has_value());
  EXPECT_EQ(**option, 42);
}

}  // namespace
