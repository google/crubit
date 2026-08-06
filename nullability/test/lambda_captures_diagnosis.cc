// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for preserving flow-narrowed nullability across by-value lambda
// captures. A pointer proven nonnull before a by-value capture should be
// treated as nonnull inside the lambda body; by-reference and this/*this
// captures must remain modeled as nullable.

#include "nullability/test/check_diagnostics.h"
#include "external/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

// --- Raw pointers ---------------------------------------------------------

// POSITIVE (the core fix): a nullable pointer, null-checked nonnull, captured
// by value, is treated as nonnull inside the lambda body.
TEST(LambdaCaptureTest, ByValueAfterCheckIsNonnull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#define CHECK(x) \
      if (!(x)) __builtin_abort();
    void target(int* _Nullable ptr) {
      CHECK(ptr);
      auto lambda = [ptr]() { *ptr; };
      lambda();
    }
  )cc"));
}

// NEGATIVE: by-value capture with no prior check is still flagged.
TEST(LambdaCaptureTest, ByValueWithoutCheckIsFlagged) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable ptr) {
      auto lambda = [ptr]() {
        *ptr;  // [[unsafe]]
      };
      lambda();
    }
  )cc"));
}

// NEGATIVE: a by-reference capture aliases mutable outer state, so even after a
// prior check it must remain nullable.
TEST(LambdaCaptureTest, ByReferenceAfterCheckIsFlagged) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#define CHECK(x) \
      if (!(x)) __builtin_abort();
    void target(int* _Nullable ptr) {
      CHECK(ptr);
      auto lambda = [&ptr]() {
        *ptr;  // [[unsafe]]
      };
      lambda();
    }
  )cc"));
}

// NEGATIVE: a `this`-capture reading a null-checked member pointer must remain
// nullable (the enclosing object's member may be reset later).
TEST(LambdaCaptureTest, ThisCaptureMemberIsFlagged) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#define CHECK(x) \
      if (!(x)) __builtin_abort();
    struct S {
      int* _Nullable m;
      void target() {
        CHECK(m);
        auto lambda = [this]() {
          *m;  // [[unsafe]]
        };
        lambda();
      }
    };
  )cc"));
}

// POSITIVE: an init-capture `[p = ptr]` after a check is by-value and nonnull.
TEST(LambdaCaptureTest, InitCaptureAfterCheckIsNonnull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#define CHECK(x) \
      if (!(x)) __builtin_abort();
    void target(int* _Nullable ptr) {
      CHECK(ptr);
      auto lambda = [p = ptr]() { *p; };
      lambda();
    }
  )cc"));
}

// NEGATIVE: an init-capture from a possibly-null value is still flagged.
TEST(LambdaCaptureTest, InitCaptureFromNullableIsFlagged) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* _Nullable maybeNull();
    void target() {
      auto lambda = [p = maybeNull()]() {
        *p;  // [[unsafe]]
      };
      lambda();
    }
  )cc"));
}

// A nested by-value capture through two lambda levels stays nonnull.
TEST(LambdaCaptureTest, NestedByValueAfterCheckIsNonnull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#define CHECK(x) \
      if (!(x)) __builtin_abort();
    void target(int* _Nullable ptr) {
      CHECK(ptr);
      auto outer = [ptr]() {
        auto inner = [ptr]() { *ptr; };
        inner();
      };
      outer();
    }
  )cc"));
}

// --- Smart pointers -------------------------------------------------------

// POSITIVE: a nullable smart pointer, null-checked nonnull, captured by value,
// is treated as nonnull inside the lambda body.
TEST(LambdaCaptureTest, SmartPointerByValueAfterCheckIsNonnull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
#define CHECK(x) \
      if (!(x)) __builtin_abort();
    void target(std::unique_ptr<int> _Nullable ptr) {
      CHECK(ptr);
      auto lambda = [ptr = std::move(ptr)]() { *ptr; };
      lambda();
    }
  )cc"));
}

// NEGATIVE: a by-value smart pointer capture with no prior check is flagged.
TEST(LambdaCaptureTest, SmartPointerByValueWithoutCheckIsFlagged) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    void target(std::shared_ptr<int> _Nullable ptr) {
      auto lambda = [ptr]() {
        *ptr;  // [[unsafe]]
      };
      lambda();
    }
  )cc"));
}

// --- Init-capture with std::move (the false-negative fix) ----------------

// POSITIVE (the core fix): a nullable move-only smart pointer, unchecked,
// moved into an init-capture is treated as nullable inside the lambda body.
TEST(LambdaCaptureTest, InitCaptureMoveSmartFromNullableIsFlagged) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
    std::unique_ptr<int> _Nullable maybeNull();
    void target() {
      auto p = maybeNull();
      auto lambda = [p = std::move(p)]() {
        *p;  // [[unsafe]]
      };
      lambda();
    }
  )cc"));
}

// POSITIVE: a nullable raw pointer, unchecked, moved into an init-capture is
// treated as nullable inside the lambda body.
TEST(LambdaCaptureTest, InitCaptureMoveRawFromNullableIsFlagged) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <utility>
    int* _Nullable maybeNull();
    void target() {
      int* _Nullable q = maybeNull();
      auto lambda = [q = std::move(q)]() {
        *q;  // [[unsafe]]
      };
      lambda();
    }
  )cc"));
}

// NEGATIVE: a move-only smart pointer proven nonnull before an init-capture
// move stays nonnull inside the lambda body (parent-CL behavior preserved).
TEST(LambdaCaptureTest, InitCaptureMoveSmartAfterCheckIsNonnull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#include <memory>
#define CHECK(x) \
      if (!(x)) __builtin_abort();
    std::unique_ptr<int> _Nullable maybeNull();
    void target() {
      auto p = maybeNull();
      CHECK(p);
      auto lambda = [p = std::move(p)]() { *p; };
      lambda();
    }
  )cc"));
}

}  // namespace
}  // namespace clang::tidy::nullability
