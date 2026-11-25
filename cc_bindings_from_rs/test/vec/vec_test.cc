// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/vec/vec.h"

#include <array>
#include <cstddef>
#include <cstdint>
#include <forward_list>
#include <initializer_list>
#include <iterator>
#include <list>
#include <numeric>
#include <ranges>  // NOLINT(build/c++20)
#include <span>
#include <sstream>
#include <string_view>
#include <type_traits>
#include <utility>
#include <vector>

#include "gtest/gtest.h"
#include "absl/types/span.h"
#include "support/rs_std/rs_alloc.h"
#include "support/rs_std/slice_ref.h"

namespace crubit {
namespace {

TEST(VecTest, ReturnVec) {
  rs::Vec<int32_t> v = vec::return_vec();
  EXPECT_EQ(v.size(), 3);
  ASSERT_NE(v.data(), nullptr);
  EXPECT_EQ(v.data()[0], 1);
  EXPECT_EQ(v.data()[1], 2);
  EXPECT_EQ(v.data()[2], 3);

  // Check coercion to std::span
  std::span<const int32_t> s = v;
  EXPECT_EQ(s.size(), 3);
  EXPECT_EQ(s[0], 1);

  // Check coercion to rs::SliceRef
  rs::SliceRef<const int32_t> sr = v;
  EXPECT_EQ(sr.size(), 3);
  EXPECT_EQ(sr.data()[0], 1);
}

TEST(VecTest, ShorthandAlias) {
  static_assert(std::is_same_v<rs::Vec<int32_t>, rs_std::Vec<int32_t>>);
  rs::Vec<int32_t> v = vec::return_vec();
  EXPECT_EQ(v.size(), 3);
}

TEST(VecTest, StructWithVec) {
  vec::StructWithVec s = vec::StructWithVec::new_(10);
  EXPECT_EQ(s.v.size(), 3);
  EXPECT_EQ(s.v.data()[0], 10);
  EXPECT_EQ(s.v.data()[1], 20);
  EXPECT_EQ(s.v.data()[2], 30);
}

TEST(VecTest, Indexing) {
  rs::Vec<int32_t> v = vec::return_vec();
  EXPECT_EQ(v[0], 1);
  EXPECT_EQ(v[1], 2);
  EXPECT_EQ(v[2], 3);

  // Const indexing
  const rs::Vec<int32_t>& const_v = v;
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
  rs::Vec<uint8_t> u8_vec = vec::return_u8_vec();
  EXPECT_EQ(u8_vec.size(), 5);

  rs::Vec<uint8_t> u8_vec_copy = u8_vec;

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
  rs::Vec<int32_t> v;
  v.push_back(1);
  v.push_back(2);
  v.push_back(3);
  int32_t sum = vec::take_vec(std::move(v));
  EXPECT_EQ(sum, 6);
}

TEST(VecTest, CreateInRustPassToCppByValue) {
  rs::Vec<int32_t> v = vec::return_vec();
  EXPECT_EQ(v.size(), 3);
}

TEST(VecTest, CppGrowsPassToRust) {
  rs::Vec<int32_t> v;
  for (int i = 0; i < 100; ++i) {
    v.push_back(i);
  }
  EXPECT_EQ(v.size(), 100);
  EXPECT_GE(v.capacity(), 100);
  vec::drop_vec(std::move(v));
}

TEST(VecTest, RustGrowsPassToCpp) {
  rs::Vec<int32_t> v = vec::return_grown_vec();
  EXPECT_EQ(v.size(), 3);
  EXPECT_GE(v.capacity(), 10);
}

TEST(VecTest, CppAddsToMutRefToRustVec) {
  vec::RustVecOwner owner = vec::RustVecOwner::new_();
  {
    rs::Vec<int32_t>& v_ref = owner.vec_mut();
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
  rs::Vec<int32_t> v;
  v.push_back(1);
  vec::rust_add_elements(v);
  EXPECT_EQ(v.size(), 3);
  EXPECT_EQ(v[0], 1);
  EXPECT_EQ(v[1], 100);
  EXPECT_EQ(v[2], 200);
}

TEST(VecTest, IterationAndAccessors) {
  rs::Vec<int32_t> v;
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
    rs::Vec<int32_t> a;
    a.push_back(10);
    a.push_back(20);

    rs::Vec<int32_t> b = std::move(a);
    EXPECT_EQ(b.size(), 2);
    EXPECT_EQ(b[0], 10);
    EXPECT_EQ(b[1], 20);

    EXPECT_TRUE(a.empty());
    EXPECT_EQ(a.size(), 0);
  }

  // Move assignment destroys LHS and empties RHS
  {
    rs::Vec<int32_t> a;
    a.push_back(1);
    a.push_back(2);

    rs::Vec<int32_t> b;
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
    rs::Vec<int32_t> a;
    a.push_back(42);
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wself-move"
    a = std::move(a);
#pragma clang diagnostic pop
    EXPECT_EQ(a.size(), 1);
    EXPECT_EQ(a[0], 42);
  }
}

TEST(VecTest, ConstructFromStdVectorCopy) {
  static_assert(std::is_nothrow_constructible_v<rs::Vec<int32_t>,
                                                const std::vector<int32_t>&>);
  static_assert(
      !std::is_convertible_v<const std::vector<int32_t>&, rs::Vec<int32_t>>);
  static_assert(!std::is_convertible_v<std::vector<int32_t>, rs::Vec<int32_t>>);

  const std::vector<int32_t> src = {1, 2, 3, 4, 5};
  rs::Vec<int32_t> v(src);
  EXPECT_EQ(v.size(), 5);
  EXPECT_GE(v.capacity(), 5);
  EXPECT_EQ(src.size(), 5);
  for (size_t i = 0; i < src.size(); ++i) {
    EXPECT_EQ(v[i], src[i]);
  }
  EXPECT_EQ(vec::take_vec(std::move(v)), 15);

  const std::vector<int32_t> empty_src;
  rs::Vec<int32_t> empty_v(empty_src);
  EXPECT_TRUE(empty_v.empty());
  EXPECT_EQ(vec::take_vec(std::move(empty_v)), 0);
}

TEST(VecTest, ConstructFromStdVectorMove) {
  static_assert(std::is_nothrow_constructible_v<rs::Vec<int32_t>,
                                                std::vector<int32_t>&&>);

  std::vector<int32_t> src = {7, 8, 9};
  rs::Vec<int32_t> v(std::move(src));
  EXPECT_EQ(v.size(), 3);
  EXPECT_GE(v.capacity(), 3);
  EXPECT_EQ(v[0], 7);
  EXPECT_EQ(v[1], 8);
  EXPECT_EQ(v[2], 9);
  EXPECT_EQ(vec::take_vec(std::move(v)), 24);

  std::vector<int32_t> empty_src;
  rs::Vec<int32_t> empty_v(std::move(empty_src));
  EXPECT_TRUE(empty_v.empty());
  EXPECT_EQ(vec::take_vec(std::move(empty_v)), 0);
}

TEST(VecTest, ConstructFromInitializerList) {
  static_assert(
      std::is_nothrow_constructible_v<rs::Vec<int32_t>,
                                      std::initializer_list<int32_t>>);
  static_assert(
      std::is_convertible_v<std::initializer_list<int32_t>, rs::Vec<int32_t>>);

  rs::Vec<int32_t> v = {1, 2, 3, 4};
  EXPECT_EQ(v.size(), 4);
  EXPECT_GE(v.capacity(), 4);
  EXPECT_EQ(v[0], 1);
  EXPECT_EQ(v[1], 2);
  EXPECT_EQ(v[2], 3);
  EXPECT_EQ(v[3], 4);
  EXPECT_EQ(vec::take_vec(std::move(v)), 10);

  rs::Vec<int32_t> direct{5, 6};
  EXPECT_EQ(direct.size(), 2);
  EXPECT_EQ(direct[0], 5);
  EXPECT_EQ(direct[1], 6);

  direct = {7, 8, 9};
  EXPECT_EQ(direct.size(), 3);
  EXPECT_EQ(vec::take_vec(std::move(direct)), 24);

  rs::Vec<int32_t> empty_v = {};
  EXPECT_TRUE(empty_v.empty());
  EXPECT_EQ(empty_v.capacity(), 0);
  EXPECT_EQ(vec::take_vec(std::move(empty_v)), 0);

  rs::Vec<int32_t> empty_init_list(std::initializer_list<int32_t>{});
  EXPECT_TRUE(empty_init_list.empty());
  EXPECT_EQ(empty_init_list.capacity(), 0);
  EXPECT_EQ(vec::take_vec(std::move(empty_init_list)), 0);

  EXPECT_EQ(vec::take_vec({10, 20, 30}), 60);
}

TEST(VecTest, ConstructFromCountAndValue) {
  // Parenthesized (count, value) constructs `count` copies of `value`.
  rs::Vec<uint8_t> u8_fill(3, 0);
  ASSERT_EQ(u8_fill.size(), 3);
  EXPECT_EQ(u8_fill[0], 0);
  EXPECT_EQ(u8_fill[1], 0);
  EXPECT_EQ(u8_fill[2], 0);

  // Brace initialization must preserve std::initializer_list<uint8_t> meaning.
  rs::Vec<uint8_t> u8_init{3, 0};
  ASSERT_EQ(u8_init.size(), 2);
  EXPECT_EQ(u8_init[0], 3);
  EXPECT_EQ(u8_init[1], 0);

  rs::Vec<uint8_t> u8_init_eq = {3, 0};
  ASSERT_EQ(u8_init_eq.size(), 2);
  EXPECT_EQ(u8_init_eq[0], 3);
  EXPECT_EQ(u8_init_eq[1], 0);

  rs::Vec<uint8_t> zero_fill(0, 7);
  EXPECT_TRUE(zero_fill.empty());
  EXPECT_EQ(zero_fill.capacity(), 0);

  rs::Vec<int32_t> i32_fill(4, 42);
  ASSERT_EQ(i32_fill.size(), 4);
  EXPECT_EQ(vec::take_vec(std::move(i32_fill)), 168);

  rs::Vec<int32_t> i32_init{4, 42};
  ASSERT_EQ(i32_init.size(), 2);
  EXPECT_EQ(vec::take_vec(std::move(i32_init)), 46);
  EXPECT_EQ(vec::take_vec({3, 0}), 3);
}

TEST(VecTest, ConstructFromIteratorRange) {
  const std::vector<uint8_t> src = {10, 20, 30, 40};
  rs::Vec<uint8_t> from_vec(src.begin(), src.end());
  EXPECT_EQ((std::vector<uint8_t>(from_vec.begin(), from_vec.end())), src);

  rs::Vec<uint8_t> from_ptr(src.data(), src.data() + src.size());
  EXPECT_EQ((std::vector<uint8_t>(from_ptr.begin(), from_ptr.end())), src);

  rs::Vec<uint8_t> empty_range(src.begin(), src.begin());
  EXPECT_TRUE(empty_range.empty());
  EXPECT_EQ(empty_range.capacity(), 0);

  const std::list<int32_t> list_src = {1, 2, 3};
  rs::Vec<int32_t> from_list(list_src.begin(), list_src.end());
  EXPECT_EQ(from_list.size(), 3);
  EXPECT_EQ(vec::take_vec(std::move(from_list)), 6);

  std::istringstream iss("5 6 7");
  rs::Vec<int32_t> from_input{std::istream_iterator<int32_t>(iss),
                              std::istream_iterator<int32_t>()};
  EXPECT_EQ(from_input.size(), 3);
  EXPECT_EQ(vec::take_vec(std::move(from_input)), 18);
}

TEST(VecTest, ResizeTrivial) {
  rs::Vec<uint8_t> v;
  EXPECT_EQ(v.capacity(), 0);

  // Resize empty zero-capacity vector to 0 is a no-op.
  v.resize(0);
  EXPECT_TRUE(v.empty());
  EXPECT_EQ(v.capacity(), 0);

  // Resize up from empty zero-capacity vector value-initializes to 0.
  v.resize(5);
  ASSERT_EQ(v.size(), 5);
  EXPECT_GE(v.capacity(), 5);
  for (uint8_t elem : v) {
    EXPECT_EQ(elem, 0);
  }

  // Resize to same size leaves contents unchanged.
  v[1] = 7;
  v.resize(5);
  EXPECT_EQ(v.size(), 5);
  EXPECT_EQ(v[1], 7);
  v.resize(5, 0xFF);
  EXPECT_EQ(v.size(), 5);
  EXPECT_EQ(v[1], 7);

  // Resize up with fill value.
  v.resize(8, 0xAB);
  ASSERT_EQ(v.size(), 8);
  EXPECT_EQ(v[1], 7);
  EXPECT_EQ(v[4], 0);
  EXPECT_EQ(v[5], 0xAB);
  EXPECT_EQ(v[6], 0xAB);
  EXPECT_EQ(v[7], 0xAB);

  // Resize with value aliasing an element across reallocation.
  const size_t old_cap = v.capacity();
  v.resize(old_cap + 10, v[1]);
  ASSERT_EQ(v.size(), old_cap + 10);
  for (size_t i = 8; i < v.size(); ++i) {
    EXPECT_EQ(v[i], 7);
  }

  // Resize down preserves prefix and capacity.
  const size_t grown_cap = v.capacity();
  v.resize(3);
  EXPECT_EQ(v.size(), 3);
  EXPECT_EQ(v.capacity(), grown_cap);
  EXPECT_EQ(v[0], 0);
  EXPECT_EQ(v[1], 7);
  EXPECT_EQ(v[2], 0);

  // Resize to zero empties while preserving capacity.
  v.resize(0);
  EXPECT_TRUE(v.empty());
  EXPECT_EQ(v.capacity(), grown_cap);
}

TEST(VecTest, AssignTrivial) {
  rs::Vec<int32_t> v;
  // Assign over shorter/empty vector.
  v.assign(4, 10);
  EXPECT_EQ((std::vector<int32_t>(v.begin(), v.end())),
            (std::vector<int32_t>{10, 10, 10, 10}));

  // Assign over longer vector (shrinks).
  v.assign(2, 20);
  EXPECT_EQ((std::vector<int32_t>(v.begin(), v.end())),
            (std::vector<int32_t>{20, 20}));

  // Assign with value aliasing an element inside the vector (growing past cap).
  const size_t cap_before = v.capacity();
  v.assign(cap_before + 5, v[0]);
  ASSERT_EQ(v.size(), cap_before + 5);
  for (int32_t x : v) {
    EXPECT_EQ(x, 20);
  }

  // Assign from iterator range (contiguous, shorter, longer, empty).
  const std::vector<int32_t> short_src = {1, 2};
  v.assign(short_src.begin(), short_src.end());
  EXPECT_EQ((std::vector<int32_t>(v.begin(), v.end())), short_src);

  const std::vector<int32_t> long_src(cap_before + 20, 9);
  v.assign(long_src.begin(), long_src.end());
  EXPECT_EQ((std::vector<int32_t>(v.begin(), v.end())), long_src);

  const std::list<int32_t> list_src = {3, 4, 5};
  v.assign(list_src.begin(), list_src.end());
  EXPECT_EQ((std::vector<int32_t>(v.begin(), v.end())),
            (std::vector<int32_t>{3, 4, 5}));

  std::istringstream iss("8 9");
  v.assign(std::istream_iterator<int32_t>(iss),
           std::istream_iterator<int32_t>());
  EXPECT_EQ((std::vector<int32_t>(v.begin(), v.end())),
            (std::vector<int32_t>{8, 9}));

  v.assign(short_src.begin(), short_src.begin());
  EXPECT_TRUE(v.empty());
  EXPECT_EQ(vec::take_vec(std::move(v)), 0);
}

TEST(VecTest, AppendRangeTrivial) {
  rs::Vec<uint8_t> v;

  // Appending empty range to empty vector leaves capacity 0.
  const std::vector<uint8_t> empty_src;
  v.append_range(empty_src);
  EXPECT_TRUE(v.empty());
  EXPECT_EQ(v.capacity(), 0);

  const std::vector<uint8_t> vec_src = {1, 2, 3};
  v.append_range(vec_src);

  const std::array<uint8_t, 2> arr_src = {4, 5};
  v.append_range(std::span<const uint8_t>(arr_src));
  v.append_range(absl::MakeConstSpan(arr_src));

  const uint8_t c_arr[] = {6, 7};
  v.append_range(c_arr);

  v.append_range(std::string_view("AB"));

  rs::Vec<uint8_t> other_vec = {8, 9};
  v.append_range(other_vec);
  v.append_range(rs::SliceRef<const uint8_t>(other_vec));

  // Appending empty range to non-empty vector is a no-op.
  v.append_range(std::span<const uint8_t>());

  EXPECT_EQ(
      (std::vector<uint8_t>(v.begin(), v.end())),
      (std::vector<uint8_t>{1, 2, 3, 4, 5, 4, 5, 6, 7, 'A', 'B', 8, 9, 8, 9}));

  // Non-contiguous forward, sized, and single-pass input ranges.
  rs::Vec<int32_t> iv;
  const std::list<int32_t> list_src = {10, 20};
  iv.append_range(list_src);
  const std::forward_list<int32_t> flist_src = {30, 40};
  iv.append_range(flist_src);
  iv.append_range(std::views::iota(50, 53));
  std::istringstream iss("60 70");
  // NOLINTNEXTLINE(build/c++20)
  iv.append_range(std::ranges::istream_view<int32_t>(iss));
  EXPECT_EQ((std::vector<int32_t>(iv.begin(), iv.end())),
            (std::vector<int32_t>{10, 20, 30, 40, 50, 51, 52, 60, 70}));
  EXPECT_EQ(vec::take_vec(std::move(iv)), 383);
}

TEST(VecTest, ConstructFromCount) {
  rs::Vec<int32_t> v(3);
  EXPECT_EQ((std::vector<int32_t>(v.begin(), v.end())),
            (std::vector<int32_t>{0, 0, 0}));

  rs::Vec<int32_t> empty(0);
  EXPECT_TRUE(empty.empty());
  EXPECT_EQ(empty.capacity(), 0);

  // Braces still select the `std::initializer_list` constructor.
  rs::Vec<int32_t> one{3};
  EXPECT_EQ((std::vector<int32_t>(one.begin(), one.end())),
            (std::vector<int32_t>{3}));

  // The constructor is explicit, like `std::vector`'s.
  static_assert(!std::is_convertible_v<size_t, rs::Vec<int32_t>>);
  static_assert(std::is_constructible_v<rs::Vec<int32_t>, size_t>);
}

TEST(VecTest, GrowthPastMaxSizeTerminates) {
  rs::Vec<int32_t> v = {1, 2, 3};
  EXPECT_EQ(v.max_size(), static_cast<size_t>(PTRDIFF_MAX) / sizeof(int32_t));

  // `size() + additional` would wrap around `size_t`.
  EXPECT_DEATH_IF_SUPPORTED(v.reserve_additional_capacity(SIZE_MAX), "");
  // These do not wrap, but ask for more than `max_size()` elements.
  EXPECT_DEATH_IF_SUPPORTED(v.reserve(v.max_size() + 1), "");
  EXPECT_DEATH_IF_SUPPORTED(v.resize(v.max_size() + 1), "");
  EXPECT_DEATH_IF_SUPPORTED(v.resize(v.max_size() + 1, 0), "");
  EXPECT_DEATH_IF_SUPPORTED(v.assign(v.max_size() + 1, 0), "");

  EXPECT_EQ((std::vector<int32_t>(v.begin(), v.end())),
            (std::vector<int32_t>{1, 2, 3}));
}

TEST(VecTest, NonTrivialElementLifecycle) {
  static_assert(!std::is_trivially_copyable_v<vec::Tracked>);
  static_assert(!std::is_trivially_destructible_v<vec::Tracked>);

  vec::Tracked proto = vec::Tracked::new_();
  EXPECT_EQ(proto.strong_count(), 1);

  // Constructors: (count, value) and (first, last).
  {
    rs::Vec<vec::Tracked> zero_v(0, proto);
    EXPECT_TRUE(zero_v.empty());
    EXPECT_EQ(zero_v.capacity(), 0);
    EXPECT_EQ(proto.strong_count(), 1);

    rs::Vec<vec::Tracked> fill_v(4, proto);
    EXPECT_EQ(fill_v.size(), 4);
    EXPECT_EQ(proto.strong_count(), 5);

    std::vector<vec::Tracked> std_src = {proto, proto};
    EXPECT_EQ(proto.strong_count(), 7);
    rs::Vec<vec::Tracked> iter_v(std_src.begin(), std_src.end());
    EXPECT_EQ(iter_v.size(), 2);
    EXPECT_EQ(proto.strong_count(), 9);
  }
  EXPECT_EQ(proto.strong_count(), 1);

  // Resize up/down/same/zero from empty zero-capacity.
  {
    rs::Vec<vec::Tracked> v;
    v.resize(0);
    EXPECT_EQ(v.capacity(), 0);

    // Value-initializing resize up, then shrink to verify destruction.
    v.resize(3);
    ASSERT_EQ(v.size(), 3);
    vec::Tracked probe = v[2];
    EXPECT_EQ(probe.strong_count(), 2);
    v.resize(1);
    EXPECT_EQ(v.size(), 1);
    EXPECT_EQ(probe.strong_count(), 1);

    // Resize up with proto, same size, resize up past capacity, shrink, to 0.
    v.resize(4, proto);
    EXPECT_EQ(v.size(), 4);
    EXPECT_EQ(proto.strong_count(), 4);

    v.resize(4, proto);
    EXPECT_EQ(proto.strong_count(), 4);

    const size_t cap_before = v.capacity();
    v.resize(cap_before + 6, proto);
    EXPECT_EQ(v.size(), cap_before + 6);
    EXPECT_EQ(proto.strong_count(), 1 + cap_before + 5);

    // Resize with value aliasing an element inside the vector across realloc.
    const size_t cap_mid = v.capacity();
    v.resize(cap_mid + 4, v[1]);
    EXPECT_EQ(v.size(), cap_mid + 4);
    EXPECT_EQ(proto.strong_count(), 1 + cap_mid + 3);

    v.resize(2);
    EXPECT_EQ(v.size(), 2);
    EXPECT_EQ(proto.strong_count(), 2);

    v.resize(0);
    EXPECT_TRUE(v.empty());
    EXPECT_EQ(proto.strong_count(), 1);
  }
  EXPECT_EQ(proto.strong_count(), 1);

  // Assign (count, value) and (first, last) over shorter and longer vectors.
  {
    vec::Tracked other = vec::Tracked::new_();
    rs::Vec<vec::Tracked> v;
    v.assign(5, proto);
    EXPECT_EQ(v.size(), 5);
    EXPECT_EQ(proto.strong_count(), 6);

    // Assign shorter with `other` (must destroy excess elements).
    v.assign(2, other);
    EXPECT_EQ(v.size(), 2);
    EXPECT_EQ(proto.strong_count(), 1);
    EXPECT_EQ(other.strong_count(), 3);

    // Assign longer past capacity with `proto`.
    const size_t cap_before = v.capacity();
    v.assign(cap_before + 5, proto);
    EXPECT_EQ(v.size(), cap_before + 5);
    EXPECT_EQ(other.strong_count(), 1);
    EXPECT_EQ(proto.strong_count(), 1 + cap_before + 5);

    // Assign over longer with iterator range (shorter than current size).
    const std::vector<vec::Tracked> two_others = {other, other};
    v.assign(two_others.begin(), two_others.end());
    EXPECT_EQ(v.size(), 2);
    EXPECT_EQ(proto.strong_count(), 1);
    EXPECT_EQ(other.strong_count(), 5);

    // Assign 0 copies destroys all elements while keeping capacity.
    const size_t cap_zero = v.capacity();
    v.assign(0, proto);
    EXPECT_TRUE(v.empty());
    EXPECT_EQ(v.capacity(), cap_zero);
    EXPECT_EQ(other.strong_count(), 3);
    EXPECT_EQ(proto.strong_count(), 1);

    // Assign with value aliasing an element inside the vector (growing past
    // capacity).
    v.assign(2, proto);
    EXPECT_EQ(proto.strong_count(), 3);
    v.assign(v.capacity() + 4, v[0]);
    EXPECT_EQ(proto.strong_count(), 1 + v.size());

    // Assign over shorter with iterator range (growing past capacity).
    const std::vector<vec::Tracked> many_protos(v.capacity() + 8, proto);
    v.assign(many_protos.begin(), many_protos.end());
    EXPECT_EQ(v.size(), many_protos.size());
    EXPECT_EQ(other.strong_count(), 3);
    EXPECT_EQ(proto.strong_count(), 1 + 2 * many_protos.size());

    // Pass to Rust and let Rust drop all elements.
    EXPECT_EQ(vec::take_tracked_vec(std::move(v)), many_protos.size());
  }
  EXPECT_EQ(proto.strong_count(), 1);

  // Append range on non-trivial type (including empty range).
  {
    rs::Vec<vec::Tracked> v;
    const std::vector<vec::Tracked> empty_src;
    v.append_range(empty_src);
    EXPECT_TRUE(v.empty());
    EXPECT_EQ(v.capacity(), 0);

    const std::vector<vec::Tracked> vec_src = {proto, proto};
    const std::list<vec::Tracked> list_src = {proto, proto, proto};
    v.append_range(vec_src);
    EXPECT_EQ(v.size(), 2);
    EXPECT_EQ(proto.strong_count(), 8);
    v.append_range(list_src);
    EXPECT_EQ(v.size(), 5);
    EXPECT_EQ(proto.strong_count(), 11);
    v.append_range(empty_src);
    EXPECT_EQ(v.size(), 5);
    EXPECT_EQ(proto.strong_count(), 11);
  }
  EXPECT_EQ(proto.strong_count(), 1);
}

template <typename V>
concept HasResizeCount = requires(V& v) { v.resize(size_t{1}); };

template <typename V, typename Elem>
concept HasResizeFill =
    requires(V& v, const Elem& e) { v.resize(size_t{1}, e); };

template <typename V, typename Elem>
concept HasAssignFill =
    requires(V& v, const Elem& e) { v.assign(size_t{1}, e); };

TEST(VecTest, NonDefaultAndMoveOnlyConstraints) {
  static_assert(!std::is_default_constructible_v<vec::NonDefault>);
  static_assert(!HasResizeCount<rs::Vec<vec::NonDefault>>);
  static_assert(HasResizeFill<rs::Vec<vec::NonDefault>, vec::NonDefault>);
  static_assert(HasAssignFill<rs::Vec<vec::NonDefault>, vec::NonDefault>);

  {
    rs::Vec<vec::NonDefault> v(2, vec::NonDefault::new_(7));
    v.resize(4, vec::NonDefault::new_(9));
    v.assign(3, vec::NonDefault::new_(11));
    const std::vector<vec::NonDefault> extra = {vec::NonDefault::new_(12)};
    v.append_range(extra);
    ASSERT_EQ(v.size(), 4);
    EXPECT_EQ(v[0].value(), 11);
    EXPECT_EQ(v[3].value(), 12);
    EXPECT_EQ(vec::take_non_default_vec(std::move(v)), 4);
  }

  static_assert(!std::is_copy_constructible_v<vec::MoveOnly>);
  static_assert(!std::is_constructible_v<rs::Vec<vec::MoveOnly>, size_t,
                                         const vec::MoveOnly&>);
  static_assert(HasResizeCount<rs::Vec<vec::MoveOnly>>);
  static_assert(!HasResizeFill<rs::Vec<vec::MoveOnly>, vec::MoveOnly>);
  static_assert(!HasAssignFill<rs::Vec<vec::MoveOnly>, vec::MoveOnly>);

  {
    vec::MoveOnly m1 = vec::MoveOnly::new_();
    vec::MoveOnly m2 = vec::MoveOnly::new_();
    vec::MoveOnly m3 = vec::MoveOnly::new_();
    rs::Vec<vec::MoveOnly> v;
    v.resize(2);
    EXPECT_EQ(v.size(), 2);

    std::vector<vec::MoveOnly> src;
    src.push_back(std::move(m1));
    src.push_back(std::move(m2));
    v.assign(std::make_move_iterator(src.begin()),
             std::make_move_iterator(src.end()));
    EXPECT_EQ(v.size(), 2);
    EXPECT_EQ(v[0].strong_count(), 1);
    EXPECT_EQ(v[1].strong_count(), 1);

    std::vector<vec::MoveOnly> src2;
    src2.push_back(std::move(m3));
    // NOLINTNEXTLINE(build/c++20)
    v.append_range(std::ranges::subrange(std::make_move_iterator(src2.begin()),
                                         std::make_move_iterator(src2.end())));
    EXPECT_EQ(v.size(), 3);
    EXPECT_EQ(v[2].strong_count(), 1);

    rs::Vec<vec::MoveOnly> v2(std::make_move_iterator(v.begin()),
                              std::make_move_iterator(v.end()));
    EXPECT_EQ(v2[0].strong_count(), 1);
    EXPECT_EQ(vec::take_move_only_vec(std::move(v2)), 3);
  }
}

}  // namespace
}  // namespace crubit
