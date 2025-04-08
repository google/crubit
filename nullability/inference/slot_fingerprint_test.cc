// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/slot_fingerprint.h"

#include "external/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

TEST(SlotFingerprintTest, ProducesConsistentValue) {
  EXPECT_EQ(fingerprint("usr1", 1), fingerprint("usr1", 1));
}

TEST(SlotFingerprintTest, ProducesDistinctValue) {
  EXPECT_NE(fingerprint("usr1", 1), fingerprint("usr1", 0));
  EXPECT_NE(fingerprint("usr1", 1), fingerprint("usr2", 1));
  EXPECT_NE(fingerprint("usr1", 1), fingerprint("usr2", 0));
}

}  // namespace
}  // namespace clang::tidy::nullability
