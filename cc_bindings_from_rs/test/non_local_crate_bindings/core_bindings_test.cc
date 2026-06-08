// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "gtest/gtest.h"
#include "support/rs_std/rs_core.h"

namespace {

TEST(CoreBindingsTest, DurationExists) { rs::core::time::Duration unused; }

}  // namespace
