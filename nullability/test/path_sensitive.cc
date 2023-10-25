// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests that involve path-sensitivity.

#include "nullability/test/check_diagnostics.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

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

      // TODO(b/306417052): False positive. `p` is set to a non-null pointer on
      // all possible execution paths.
      (void)*p;  // [[unsafe]]
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

      // TODO(b/307492164): False negative. This dereference is unsafe.
      // This false negative likely happens because we don't model a return
      // value for the `return_bool()` call above, so the `false` value that `b`
      // is initialized with does not get overwritten.
      (void)*p;
    }
  )cc"));
}

}  // namespace
}  // namespace clang::tidy::nullability
