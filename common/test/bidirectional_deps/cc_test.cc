// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <type_traits>

#include "gtest/gtest.h"
#include "common/test/bidirectional_deps/leaf_cc_lib.h"
#include "common/test/bidirectional_deps/middle_rs_lib.h"

namespace crubit {
namespace {

TEST(BidirectionalDepsTest, RoundTrip) {
  LeafCcType value_from_cc = Wrap(4);
  ASSERT_EQ(middle_rs_lib::unwrap(value_from_cc), 4);

  LeafCcType value_from_rs = middle_rs_lib::wrap(2);
  ASSERT_EQ(Unwrap(value_from_rs), 2);
}

TEST(BidirectionalDepsTest, EnumRoundTrip) {
  LeafCcEnum value_from_cc = WrapEnum(2);
  ASSERT_EQ(middle_rs_lib::unwrap_enum(value_from_cc), 2);

  LeafCcEnum value_from_rs = middle_rs_lib::wrap_enum(1);
  ASSERT_EQ(UnwrapEnum(value_from_rs), 1);
}

TEST(BidirectionalDepsTest, TypeAliasRoundTrip) {
  bool res = std::is_same_v<LeafCcTypeAlias, middle_rs_lib::LeafCcTypeAlias2>;
  ASSERT_TRUE(res);
  res = std::is_same_v<LeafCcTypeAlias, middle_rs_lib::LeafCcTypeAlias3>;
  ASSERT_TRUE(res);
};
}  // namespace
}  // namespace crubit
