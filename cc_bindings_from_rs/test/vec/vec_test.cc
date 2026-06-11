// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/vec/vec.h"

#include <span>

#include "gtest/gtest.h"
#include "absl/types/span.h"
#include "support/rs_std/slice_ref.h"

namespace crubit {
namespace {

TEST(VecTest, ReturnVec) {
  rs_std::Vec<int32_t> v = vec::return_vec();
  EXPECT_EQ(v.size(), 3);
  ASSERT_NE(v.data(), nullptr);
  EXPECT_EQ(v.data()[0], 1);
  EXPECT_EQ(v.data()[1], 2);
  EXPECT_EQ(v.data()[2], 3);

  // Check coercion to std::span
  std::span<const int32_t> s = v;
  EXPECT_EQ(s.size(), 3);
  EXPECT_EQ(s[0], 1);

  // Check coercion to rs_std::SliceRef
  rs_std::SliceRef<const int32_t> sr = v;
  EXPECT_EQ(sr.size(), 3);
  EXPECT_EQ(sr.data()[0], 1);
}

TEST(VecTest, StructWithVec) {
  vec::StructWithVec s = vec::StructWithVec::new_(10);
  EXPECT_EQ(s.v.size(), 3);
  EXPECT_EQ(s.v.data()[0], 10);
  EXPECT_EQ(s.v.data()[1], 20);
  EXPECT_EQ(s.v.data()[2], 30);
}

TEST(VecTest, Indexing) {
  rs_std::Vec<int32_t> v = vec::return_vec();
  EXPECT_EQ(v[0], 1);
  EXPECT_EQ(v[1], 2);
  EXPECT_EQ(v[2], 3);

  // Const indexing
  const rs_std::Vec<int32_t>& const_v = v;
  EXPECT_EQ(const_v[0], 1);
  EXPECT_EQ(const_v[1], 2);
  EXPECT_EQ(const_v[2], 3);

  // Non-const modification
  v[1] = 42;
  EXPECT_EQ(v[1], 42);
  EXPECT_EQ(const_v[1], 42);

  // Bounds check (death test)
  EXPECT_DEATH(v[3], "");
  EXPECT_DEATH(const_v[3], "");
}

}  // namespace
}  // namespace crubit
