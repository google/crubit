// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for diagnostics on smart pointers.

#include "nullability/test/check_diagnostics.h"
#include "nullability/type_nullability.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

// Static initializer turns on support for smart pointers.
test::EnableSmartPointers Enable;

TEST(SmartPointerTest, Dereference) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    void target() {
      *std::unique_ptr<int>();  // [[unsafe]]
      *std::make_unique<int>();
    }
  )cc"));
}

TEST(SmartPointerTest, ArrowOp) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    struct S {
      int i = 0;
    };
    void target() {
      std::unique_ptr<S>()->i;  // [[unsafe]]
      std::make_unique<S>()->i;
    }
  )cc"));
}

TEST(SmartPointerTest, Subscript) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    void target() {
      std::unique_ptr<int[]>()[0];  // [[unsafe]]
      std::make_unique<int[]>(1)[0];
    }
  )cc"));
}

TEST(SmartPointerTest, ReturnValue_Nonnull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    bool cond();
    Nonnull<std::unique_ptr<int>> target() {
      if (cond())
        return std::make_unique<int>(0);
      else
        return std::unique_ptr<int>();  // TODO(b/304963199): False negative.
    }
  )cc"));
}

TEST(SmartPointerTest, ReturnValue_Nullable) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    bool cond();
    Nullable<std::unique_ptr<int>> target() {
      if (cond())
        return std::make_unique<int>(0);
      else
        return std::unique_ptr<int>();
    }
  )cc"));
}

TEST(SmartPointerTest, ReturnValue_Unknown) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    bool cond();
    std::unique_ptr<int> target() {
      if (cond())
        return std::make_unique<int>(0);
      else
        return std::unique_ptr<int>();
    }
  )cc"));
}

}  // namespace
}  // namespace clang::tidy::nullability
