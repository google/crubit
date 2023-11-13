// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests that involve smart pointers.

// TODO(b/304963199): We do not actually check smart pointers, so these tests
// are full of false negatives. For now, they are mainly intended to check that
// the check does not crash or assert-fail on smart pointers.

#include "nullability/test/check_diagnostics.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

TEST(SmartPointerTest, DefaultConstructedSmartPointerIsNull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    void target() {
      std::unique_ptr<int> p;
      *p;  // TODO(b/304963199): False negative.
    }
  )cc"));
}

TEST(SmartPointerTest, MakeUniqueReturnsNonNull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    void target() {
      auto p = std::make_unique<int>(0);
      *p;
    }
  )cc"));
}

TEST(SmartPointerTest, ParameterAnnotations) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    void target(Nonnull<std::unique_ptr<int>> nonnull,
                Nullable<std::unique_ptr<int>> nullable,
                std::unique_ptr<int> unknown) {
      *nonnull;
      *nullable;  // TODO(b/304963199): False negative.
      *unknown;
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
