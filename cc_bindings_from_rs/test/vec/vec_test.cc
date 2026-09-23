// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/vec/vec.h"

#include <numeric>
#include <span>
#include <vector>

#include "gtest/gtest.h"
#include "absl/types/span.h"
#include "support/rs_std/rs_alloc.h"
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
  EXPECT_DEATH_IF_SUPPORTED(v[3], "");
  EXPECT_DEATH_IF_SUPPORTED(const_v[3], "");
}

TEST(VecTest, CStringCallability) {
  rs_std::Vec<uint8_t> u8_vec = vec::return_u8_vec();
  EXPECT_EQ(u8_vec.size(), 5);

  rs_std::Vec<uint8_t> u8_vec_copy = u8_vec;

  auto result = ::rs::alloc::ffi::CString::new_(std::move(u8_vec_copy));
  ASSERT_TRUE(result.has_value());
  ::rs::alloc::ffi::CString c_str = std::move(result).value();
  EXPECT_EQ(c_str.as_bytes().size(), 5);
  EXPECT_EQ(c_str.as_bytes().data()[0], 'H');

  ::rs::alloc::ffi::CString c_str2 =
      ::rs::alloc::ffi::CString::from_vec_unchecked(std::move(u8_vec));
  EXPECT_EQ(c_str2.as_bytes().size(), 5);
  EXPECT_EQ(c_str2.as_bytes().data()[0], 'H');
}

TEST(VecTest, CreateInCppPassToRustByValue) {
  rs_std::Vec<int32_t> v;
  v.push_back(1);
  v.push_back(2);
  v.push_back(3);
  int32_t sum = vec::take_vec(std::move(v));
  EXPECT_EQ(sum, 6);
}

TEST(VecTest, CreateInRustPassToCppByValue) {
  rs_std::Vec<int32_t> v = vec::return_vec();
  EXPECT_EQ(v.size(), 3);
}

TEST(VecTest, CppGrowsPassToRust) {
  rs_std::Vec<int32_t> v;
  for (int i = 0; i < 100; ++i) {
    v.push_back(i);
  }
  EXPECT_EQ(v.size(), 100);
  EXPECT_GE(v.capacity(), 100);
  vec::drop_vec(std::move(v));
}

TEST(VecTest, RustGrowsPassToCpp) {
  rs_std::Vec<int32_t> v = vec::return_grown_vec();
  EXPECT_EQ(v.size(), 3);
  EXPECT_GE(v.capacity(), 10);
}

TEST(VecTest, CppAddsToMutRefToRustVec) {
  vec::RustVecOwner owner = vec::RustVecOwner::new_();
  {
    rs_std::Vec<int32_t>& v_ref = owner.vec_mut();
    EXPECT_EQ(v_ref.size(), 0);
    v_ref.push_back(10);
    v_ref.push_back(20);
    v_ref.emplace_back(30);
  }
  EXPECT_EQ(owner.len(), 3);
  EXPECT_EQ(owner.element(0), 10);
  EXPECT_EQ(owner.element(1), 20);
  EXPECT_EQ(owner.element(2), 30);
}

TEST(VecTest, RustAddsToMutRefToCppVec) {
  rs_std::Vec<int32_t> v;
  v.push_back(1);
  vec::rust_add_elements(v);
  EXPECT_EQ(v.size(), 3);
  EXPECT_EQ(v[0], 1);
  EXPECT_EQ(v[1], 100);
  EXPECT_EQ(v[2], 200);
}

TEST(VecTest, IterationAndAccessors) {
  rs_std::Vec<int32_t> v;
  EXPECT_TRUE(v.empty());

  v.push_back(10);
  v.push_back(20);
  v.push_back(30);
  EXPECT_FALSE(v.empty());
  EXPECT_EQ(v.front(), 10);
  EXPECT_EQ(v.back(), 30);

  // Range-based for loop (non-const)
  for (int32_t& x : v) {
    x += 1;
  }
  EXPECT_EQ(v[0], 11);
  EXPECT_EQ(v[1], 21);
  EXPECT_EQ(v[2], 31);

  // Range-based for loop (const)
  const auto& const_v = v;
  EXPECT_EQ(const_v.front(), 11);
  EXPECT_EQ(const_v.back(), 31);
  std::vector<int32_t> collected;
  for (const int32_t& x : const_v) {
    collected.push_back(x);
  }
  EXPECT_EQ(collected, (std::vector<int32_t>{11, 21, 31}));

  // Standard algorithms: std::accumulate
  int32_t sum = std::accumulate(v.begin(), v.end(), 0);
  EXPECT_EQ(sum, 63);

  // Reverse iteration
  std::vector<int32_t> reversed;
  for (auto it = v.rbegin(); it != v.rend(); ++it) {
    reversed.push_back(*it);
  }
  EXPECT_EQ(reversed, (std::vector<int32_t>{31, 21, 11}));

  // Const iterators: cbegin / cend
  std::vector<int32_t> from_const;
  for (auto it = v.cbegin(); it != v.cend(); ++it) {
    from_const.push_back(*it);
  }
  EXPECT_EQ(from_const, (std::vector<int32_t>{11, 21, 31}));
}

TEST(VecTest, MoveOperations) {
  // Move constructor
  {
    rs_std::Vec<int32_t> a;
    a.push_back(10);
    a.push_back(20);

    rs_std::Vec<int32_t> b = std::move(a);
    EXPECT_EQ(b.size(), 2);
    EXPECT_EQ(b[0], 10);
    EXPECT_EQ(b[1], 20);

    EXPECT_TRUE(a.empty());
    EXPECT_EQ(a.size(), 0);
  }

  // Move assignment destroys LHS and empties RHS
  {
    rs_std::Vec<int32_t> a;
    a.push_back(1);
    a.push_back(2);

    rs_std::Vec<int32_t> b;
    b.push_back(100);
    b.push_back(200);
    b.push_back(300);

    a = std::move(b);

    EXPECT_EQ(a.size(), 3);
    EXPECT_EQ(a[0], 100);
    EXPECT_EQ(a[1], 200);
    EXPECT_EQ(a[2], 300);

    EXPECT_TRUE(b.empty());
    EXPECT_EQ(b.size(), 0);
  }

  // Self-move assignment is a safe no-op
  {
    rs_std::Vec<int32_t> a;
    a.push_back(42);
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wself-move"
    a = std::move(a);
#pragma clang diagnostic pop
    EXPECT_EQ(a.size(), 1);
    EXPECT_EQ(a[0], 42);
  }
}

}  // namespace
}  // namespace crubit
