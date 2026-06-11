// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/traits/stdlib/stdlib.h"

#include <algorithm>
#include <cstdint>
#include <iterator>
#include <ranges>
#include <type_traits>
#include <vector>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "support/rs_std/iterator_adapter.h"
#include "support/rs_std/rs_core.h"
#include "support/rs_std/rs_std.h"

namespace crubit {
namespace {

TEST(StdlibTraitTest, Default) {
  auto my_struct = stdlib::MyStruct::new_(42);
  auto str =
      rs::std::string::ToString::impl<stdlib::MyStruct>::to_string(my_struct);
  EXPECT_EQ(str.as_str(), "MyStruct(42)");
}

TEST(StdlibTraitTest, IteratorNext) {
  using impl = rs::core::iter::Iterator::impl<stdlib::MyStruct>;
  auto s = stdlib::MyStruct::new_(3);
  EXPECT_EQ(std::optional(impl::next(s)), std::make_optional(2));
  EXPECT_EQ(std::optional(impl::next(s)), std::make_optional(1));
  EXPECT_EQ(std::optional(impl::next(s)), std::make_optional(0));
  EXPECT_EQ(std::optional(impl::next(s)), std::nullopt);
}

TEST(StdlibTraitTest, IteratorAdapter_ModernRangesApi) {
  auto s = stdlib::MyStruct::new_(3);
  std::vector<int32_t> v;
  std::ranges::copy(rs::IteratorAdapter(std::move(s)), rs::IteratorEnd{},
                    std::back_inserter(v));
  EXPECT_THAT(v, testing::ElementsAre(2, 1, 0));
}

TEST(StdlibTraitTest, IteratorAdapter_LegacyIteratorApi) {
  // Legacy C++ APIs require that each of the pair of begin/end iterators
  // has exactly the same type.  `std::common_iterator` is one way to achieve
  // this when the underlying implementation uses a separate sentinel type.
  //
  // This is somewhat limited - it requires that the underlying iterator is
  // copyable.  `MyStruct` is indeed copyable, but this requirement is not
  // necessarily true for all Rust iterators.
  auto s = stdlib::MyStruct::new_(3);
  using Iterator = rs::IteratorAdapter<stdlib::MyStruct>;
  using Sentinel = rs::IteratorEnd;
  using LegacyIterator = std::common_iterator<Iterator, Sentinel>;
  std::vector<int32_t> v(LegacyIterator(rs::IteratorAdapter(std::move(s))),
                         LegacyIterator(rs::IteratorEnd{}));
  EXPECT_THAT(v, testing::ElementsAre(2, 1, 0));
}

static_assert(std::input_iterator<rs::IteratorAdapter<stdlib::MyStruct>>);
static_assert(
    std::sentinel_for<rs::IteratorEnd, rs::IteratorAdapter<stdlib::MyStruct>>);

using MyIterator = rs::IteratorAdapter<stdlib::MyStruct>;
using Traits = std::iterator_traits<MyIterator>;

// Check if traits are defined
static_assert(requires { typename Traits::value_type; });
static_assert(requires { typename Traits::difference_type; });
static_assert(requires { typename Traits::reference; });
static_assert(requires { typename Traits::iterator_category; });
static_assert(std::is_same_v<typename Traits::iterator_category,
                             std::input_iterator_tag>);
static_assert(requires { typename MyIterator::iterator_concept; });
static_assert(std::is_same_v<typename MyIterator::iterator_concept,
                             std::input_iterator_tag>);

static_assert(std::input_iterator<rs::IteratorAdapter<stdlib::RefIterator>>);

using RefIterator = rs::IteratorAdapter<stdlib::RefIterator>;
using RefTraits = std::iterator_traits<RefIterator>;
static_assert(std::is_same_v<typename RefTraits::value_type, const int32_t>);
static_assert(std::is_same_v<typename RefTraits::reference, const int32_t&>);

TEST(StdlibTraitTest, RefIteratorAdapter) {
  std::vector<int32_t> data = {10, 20, 30};
  auto s = stdlib::RefIterator::new_(rs_std::SliceRef<const int32_t>(data));
  std::vector<int32_t> v;
  std::ranges::copy(rs::IteratorAdapter(std::move(s)), rs::IteratorEnd{},
                    std::back_inserter(v));
  EXPECT_THAT(v, testing::ElementsAre(10, 20, 30));
}

TEST(StdlibTraitTest, NonCloneableIteratorAdapter) {
  auto s = stdlib::NonCloneableIterator::new_(3);
  std::vector<stdlib::NonCloneableValue> v;
  std::ranges::move(rs::IteratorAdapter(std::move(s)), rs::IteratorEnd{},
                    std::back_inserter(v));
  EXPECT_EQ(v.size(), 3);
  EXPECT_EQ(v[0].x, 2);
  EXPECT_EQ(v[1].x, 1);
  EXPECT_EQ(v[2].x, 0);
}

}  // namespace
}  // namespace crubit
