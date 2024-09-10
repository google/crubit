// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/header_rename/rename_lib.h"
#include "cc_bindings_from_rs/test/header_rename/rename_lib_cc_api.h"

namespace crubit {
namespace {

TEST(RenameLibTest, UsingOldHeaderName) { EXPECT_EQ(rename_lib::f(), 42); }

}  // namespace
}  // namespace crubit
