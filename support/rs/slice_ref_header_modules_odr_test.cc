// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <array>

#include "gtest/gtest.h"
#include "support/rs/slice_ref_header_modules_depot_path.h"
#include "support/rs/slice_ref_header_modules_virtual_include.h"

namespace {

TEST(SliceRefHeaderModulesOdrTest, DualIncludePathsDoNotViolateOdr) {
  constexpr std::array<int, 2> kData = {42, 7};
  EXPECT_EQ(SliceRefFirstViaVirtualInclude(kData), 42);
  EXPECT_EQ(SliceRefFirstViaDepotPath(kData), 42);
}

}  // namespace
