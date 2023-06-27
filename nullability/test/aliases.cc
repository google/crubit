// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for nullability information hidden behind aliases.

#include "nullability/test/check_diagnostics.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

TEST(PointerNullabilityTest, Aliases) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <typename T>
    struct Factory {
      T get();
    };
    using NeverNull = int *_Nonnull;
    using MaybeNull = int *_Nullable;

    void target(Factory<NeverNull> never, Factory<MaybeNull> maybe) {
      *never.get();
      *maybe.get();  // [[unsafe]]
    }
  )cc"));
}

}  // namespace
}  // namespace clang::tidy::nullability