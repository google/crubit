// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/known_traits/index/rs_index.h"

#include "gmock/gmock.h"
#include "gtest/gtest.h"

namespace crubit {
namespace {

TEST(IndexTest, IndexByUsize) {
  auto pair = rs_index::IntPair::new_(10, 20);

  EXPECT_EQ(pair[0], 10);
  EXPECT_EQ(pair[1], 20);
}

TEST(IndexTest, IndexByCustomType) {
  auto pair = rs_index::IntPair::new_(10, 20);
  auto index0 = rs_index::CustomIndex::new_(0);
  auto index1 = rs_index::CustomIndex::new_(1);
  auto index40 = rs_index::CustomIndex::new_(40);

  EXPECT_EQ(pair[index0], 10);
  EXPECT_EQ(pair[index1], 20);
  EXPECT_EQ(pair[index40], 20);
}

TEST(IndexTest, IndexByTuple) {
  auto map = rs_index::Map::new_(2, 2);
  auto index = std::make_tuple(0, 0);
  EXPECT_EQ(map[index].to_string_view(), "tile(0, 0)");
  EXPECT_EQ(map[std::make_tuple(0, 1)].to_string_view(), "tile(0, 1)");
  EXPECT_EQ(map[std::make_tuple(1, 0)].to_string_view(), "tile(1, 0)");
  EXPECT_EQ(map[std::make_tuple(1, 1)].to_string_view(), "tile(1, 1)");
}

}  // namespace
}  // namespace crubit
