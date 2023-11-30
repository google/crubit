// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "gtest/gtest.h"
#include "common/test/bidirectional_deps/leaf_cc_lib.h"
#include "common/test/bidirectional_deps/middle_rs_lib_cc_api.h"

namespace crubit {
namespace {

TEST(BidirectionalDepsTest, RoundTrip) {
  LeafCcType value_from_cc = Wrap(4);
  ASSERT_EQ(middle_rs_lib::unwrap(value_from_cc), 4);

  LeafCcType value_from_rs = middle_rs_lib::wrap(2);
  ASSERT_EQ(Unwrap(value_from_rs), 2);
}

}  // namespace
}  // namespace crubit
