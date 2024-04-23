// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/bazel/crate_flags/crate_flags_cc_api.h"

namespace crubit {
namespace {

TEST(NeedsFeatureTest, CallFoo) { ASSERT_EQ(crate_flags::foo(), 42); }

}  // namespace
}  // namespace crubit
