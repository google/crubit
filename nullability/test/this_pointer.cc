// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for the treatment of the `this` pointer (which is always nonnull).
#include <optional>
#include <set>
#include <string>

#include "nullability/test/check_diagnostics.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang {
namespace tidy {
namespace nullability {
namespace {

TEST(PointerNullabilityTest, ThisPointer) {
  // (->) implicit `this`
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      void foo();
      void target() { foo(); }
    };
  )cc"));

  // (->) explicit `this`
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      void foo();
      void target() { this->foo(); }
    };
  )cc"));
}

}  // namespace
}  // namespace nullability
}  // namespace tidy
}  // namespace clang
