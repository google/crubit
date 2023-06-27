// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for constructors.

#include "nullability/test/check_diagnostics.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

TEST(PointerNullabilityTest, ConstructExpr) {
  // Constructor call assigned to local variable.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct TakeNonnull {
      explicit TakeNonnull(int *_Nonnull) {}
    };
    struct TakeNullable {
      explicit TakeNullable(int *_Nullable) {}
    };
    struct TakeUnannotated {
      explicit TakeUnannotated(int *) {}
    };
    int *_Nonnull makeNonnull();
    int *_Nullable makeNullable();
    int *makeUnannotated();
    void target() {
      auto NN1 = TakeNonnull(makeNonnull());
      auto NN2 = TakeNonnull(makeNullable());  // [[unsafe]]
      auto NN3 = TakeNonnull(makeUnannotated());

      auto NB1 = TakeNullable(makeNonnull());
      auto NB2 = TakeNullable(makeNullable());
      auto NB3 = TakeNullable(makeUnannotated());

      auto UN1 = TakeUnannotated(makeNonnull());
      auto UN2 = TakeUnannotated(makeNullable());
      auto UN3 = TakeUnannotated(makeUnannotated());
    }
  )cc"));

  // Constructor call in a base initializer.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct TakeNonnull {
      explicit TakeNonnull(int *_Nonnull);
    };
    struct target : TakeNonnull {
      target(int *_Nullable ptr_nullable)  // Forced line break.
          : TakeNonnull(ptr_nullable)      // [[unsafe]]
      {}
    };
  )cc"));

  // Call to a delegating constructor.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nullable makeNullable();
    struct target {
      target(int *_Nonnull);
      target()                      // Forced line break.
          : target(makeNullable())  // [[unsafe]]
      {}
    };
  )cc"));
}

TEST(PointerNullabilityTest, ConstructorMemberInitializer) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nullable makeNullable();
    struct target {
      int *_Nonnull ptr_nonnull;
      int *_Nullable ptr_nullable;
      int *ptr_unannotated;
      target()
          : ptr_nonnull(makeNullable()),  // [[unsafe]]
            ptr_nullable(makeNullable()),
            ptr_unannotated(makeNullable()) {}
    };
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nonnull makeNonnull();
    struct target {
      int *_Nonnull ptr_nonnull;
      int *_Nullable ptr_nullable;
      int *ptr_unannotated;
      target()
          : ptr_nonnull(makeNonnull()),
            ptr_nullable(makeNonnull()),
            ptr_unannotated(makeNonnull()) {}
    };
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *makeUnannotated();
    struct target {
      int *_Nonnull ptr_nonnull;
      int *_Nullable ptr_nullable;
      int *ptr_unannotated;
      target()
          : ptr_nonnull(makeUnannotated()),
            ptr_nullable(makeUnannotated()),
            ptr_unannotated(makeUnannotated()) {}
    };
  )cc"));
}

}  // namespace
}  // namespace clang::tidy::nullability
