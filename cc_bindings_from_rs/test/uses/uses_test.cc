// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstdint>
#include <type_traits>
#include <utility>

#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/uses/uses_cc_api.h"

namespace crubit {
namespace {

TEST(UsesTest, UsesExportsAsUsing) { EXPECT_EQ(uses::f(), 42); }

}  // namespace
}  // namespace crubit