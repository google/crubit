// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for diagnostics on lambda captures.

#include "nullability/test/check_diagnostics.h"
#include "external/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

TEST(LambdaCaptureTest, ByValueAfterCheckIsNonnull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable ptr) {
      if (!ptr) return;
      auto lambda = [ptr]() { *ptr; };
    }
  )cc"));
}

TEST(LambdaCaptureTest, ByValueWithoutCheckIsFlagged) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable ptr) {
      auto lambda = [ptr]() {
        *ptr;  // [[unsafe]]
      };
    }
  )cc"));
}

TEST(LambdaCaptureTest, ByReferenceAfterCheckIsFlagged) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable ptr) {
      if (!ptr) return;
      auto lambda = [&ptr]() {
        *ptr;  // [[unsafe]]
      };
    }
  )cc"));
}

TEST(LambdaCaptureTest, ThisCaptureMemberIsFlagged) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct S {
      int* _Nullable m;
      void target() {
        if (!m) return;
        auto lambda = [this]() {
          *m;  // [[unsafe]]
        };
      }
    };
  )cc"));
}

TEST(LambdaCaptureTest, InitCaptureAfterCheckIsNonnull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable ptr) {
      if (!ptr) return;
      auto lambda = [p = ptr]() { *p; };
    }
  )cc"));
}

TEST(LambdaCaptureTest, InitCaptureFromNullableIsFlagged) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* _Nullable maybeNull();
    void target() {
      auto lambda = [p = maybeNull()]() {
        *p;  // [[unsafe]]
      };
    }
  )cc"));
}

TEST(LambdaCaptureTest, NestedByValueAfterCheckIsNonnull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable ptr) {
      if (!ptr) return;
      auto outer = [ptr]() { auto inner = [ptr]() { *ptr; }; };
    }
  )cc"));
}

TEST(LambdaCaptureTest, SmartPointerByValueAfterCheckIsNonnull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    void target(std::unique_ptr<int> _Nullable ptr) {
      if (!ptr) return;
      auto lambda = [ptr = std::move(ptr)]() { *ptr; };
    }
  )cc"));
}

TEST(LambdaCaptureTest, SmartPointerByValueWithoutCheckIsFlagged) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    void target(std::shared_ptr<int> _Nullable ptr) {
      auto lambda = [ptr]() {
        *ptr;  // [[unsafe]]
      };
    }
  )cc"));
}

TEST(LambdaCaptureTest, InitCaptureMoveSmartFromNullableIsFlagged) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    std::unique_ptr<int> _Nullable maybeNull();
    void target() {
      auto p = maybeNull();
      auto lambda = [p = std::move(p)]() {
        *p;  // [[unsafe]]
      };
    }
  )cc"));
}

TEST(LambdaCaptureTest, InitCaptureMoveRawFromNullableIsFlagged) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <utility>
    int* _Nullable maybeNull();
    void target() {
      int* _Nullable q = maybeNull();
      auto lambda = [q = std::move(q)]() {
        *q;  // [[unsafe]]
      };
    }
  )cc"));
}

TEST(LambdaCaptureTest, InitCaptureMoveSmartAfterCheckIsNonnull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    std::unique_ptr<int> _Nullable maybeNull();
    void target() {
      auto p = maybeNull();
      if (!p) return;
      auto lambda = [p = std::move(p)]() { *p; };
    }
  )cc"));
}

}  // namespace
}  // namespace clang::tidy::nullability
