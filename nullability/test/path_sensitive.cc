// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests that involve path-sensitivity.

#include "nullability/test/check_diagnostics.h"
#include "external/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

TEST(PointerNullabilityTest, ConditionalInitialization) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct S {
      bool b;
    };
    S return_s();
    int* _Nonnull produce_int();

    void target(int i) {
      int* p = nullptr;
      bool b = false;
      if (i < 0) {
        S s = return_s();
        b = s.b;
        if (!b) p = produce_int();
      } else {
        S s = return_s();
        b = s.b;
        if (!b) p = produce_int();
      }
      if (b) p = produce_int();

      (void)*p;
    }
  )cc"));
}

TEST(PointerNullabilityTest, ConditionalInitialization2) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    bool return_bool();
    int *_Nonnull produce_int();

    void target() {
      int *p = nullptr;
      bool b = false;
      b = return_bool();
      if (!b) p = produce_int();

      (void)*p;  // [[unsafe]]
    }
  )cc"));
}

TEST(PointerNullabilityTest, ComplexLoopCondition) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* _Nullable produce_int();
    void target() {
      int* p1;
      int* p2;
      while ((p1 = produce_int()) != nullptr && (p2 = produce_int()) != nullptr) {
        *p1;
        *p2;
      }
    }
  )cc"));
}

TEST(PointerNullabilityTest, OnePointerGuaranteedNonnull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable p1, int *_Nullable p2) {
      if (!p1 && !p2) return;
      if (p1)
        *p1;
      else
        // `p2` must be nonnull or we would have returned above.
        *p2;
    }
  )cc"));
}

// The check cannot reason about integer inequalities, so does not know that the
// loop has at least one iteration.
TEST(PointerNullabilityTest, DereferencePointerSetInLoop) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target() {
      int* _Nullable p = nullptr;
      int x = 0;
      for (int i = 0; i < 10; ++i) {
        p = &x;
      }
      *p = 1;  // [[unsafe]]
    }
  )cc"));
}

// A do-while loop makes it clear there's at least one iteration.
TEST(PointerNullabilityTest, DereferencePointerSetInDoWhileLoop) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target() {
      int* _Nullable p = nullptr;
      int x = 0;
      int i = 0;
      do {
        p = &x;
        ++i;
      } while (i < 10);
      *p = 1;
    }
  )cc"));
}

}  // namespace
}  // namespace clang::tidy::nullability
