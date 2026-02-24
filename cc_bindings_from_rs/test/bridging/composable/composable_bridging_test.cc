// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/bridging/composable/composable_bridging.h"

#include <optional>

#include "gtest/gtest.h"
#include "support/rs_std/slice_ref.h"

namespace crubit {
namespace {

using ::rs_std::SliceRef;

TEST(ComposableBridging, MaybeInt) {
  std::optional<int> maybe_int = composable_bridging::maybe_int();
  ASSERT_TRUE(maybe_int.has_value());
  EXPECT_EQ(maybe_int.value(), 4);
}

TEST(ComposableBridging, MaybeStringView) {
  std::optional<std::string_view> maybe_string_view =
      composable_bridging::maybe_string_view();
  EXPECT_FALSE(maybe_string_view.has_value());
}

TEST(ComposableBridging, MaybeIntSlice) {
  std::optional<rs_std::SliceRef<const int>> maybe_int_slice =
      composable_bridging::maybe_int_slice();
  ASSERT_TRUE(maybe_int_slice.has_value());
  rs_std::SliceRef<const int> int_slice = maybe_int_slice.value();
  auto int_slice_span = int_slice.to_span();
  EXPECT_EQ(int_slice_span.size(), 3);
  EXPECT_EQ(int_slice_span[0], 1);
  EXPECT_EQ(int_slice_span[1], 2);
  EXPECT_EQ(int_slice_span[2], 3);
}

TEST(ComposableBridging, Parameters) {
  composable_bridging::assert_none(std::nullopt);
  composable_bridging::assert_some_5(std::make_optional(5));
  composable_bridging::assert_some_some_5(
      std::make_optional(std::make_optional(5)));
}

TEST(ComposableBridging, NestedSlices) {
  using ::composable_bridging::option_slice_without_first;
  EXPECT_EQ(option_slice_without_first(std::nullopt), std::nullopt);
  EXPECT_EQ(option_slice_without_first(
                std::make_optional<SliceRef<const int>>({1, 2, 3})),
            std::make_optional<SliceRef<const int>>({2, 3}));
}

TEST(ComposableBridging, NestedRefs) {
  using ::composable_bridging::option_adds_one_to_ref;
  EXPECT_EQ(option_adds_one_to_ref(std::nullopt), std::nullopt);
  int x = 5;
  std::optional<int* crubit_nonnull> y =
      option_adds_one_to_ref(std::make_optional(&x));
  ASSERT_TRUE(y.has_value());
  EXPECT_EQ(*y.value(), 6);
  EXPECT_EQ(x, 6);
}

}  // namespace
}  // namespace crubit
