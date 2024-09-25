// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/bazel/crate_features/needs_feature.h"

#include "gtest/gtest.h"

namespace crubit {
namespace {

TEST(NeedsFeatureTest, CallFoo) { ASSERT_EQ(needs_feature::foo(), 42); }

}  // namespace
}  // namespace crubit
