// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for function calls.

#include "nullability_verification/test/check_diagnostics.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang {
namespace tidy {
namespace nullability {
namespace {

TEST(PointerNullabilityTest, CallExprWithPointerReturnType) {
  // free function
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nonnull makeNonnull();
    int *_Nullable makeNullable();
    int *makeUnannotated();
    void target() {
      *makeNonnull();
      *makeNullable();  // [[unsafe]]
      *makeUnannotated();
    }
  )cc"));

  // member function
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      int *_Nonnull makeNonnull();
      int *_Nullable makeNullable();
      int *makeUnannotated();
    };
    void target(Foo foo) {
      *foo.makeNonnull();
      *foo.makeNullable();  // [[unsafe]]
      *foo.makeUnannotated();
    }
  )cc"));

  // overloaded operator call
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct MakeNonnull {
      int *_Nonnull operator()();
    };
    struct MakeNullable {
      int *_Nullable operator()();
    };
    struct MakeUnannotated {
      int *operator()();
    };
    void target() {
      MakeNonnull makeNonnull;
      *makeNonnull();

      MakeNullable makeNullable;
      *makeNullable();  // [[unsafe]]

      MakeUnannotated makeUnannotated;
      *makeUnannotated();
    }
  )cc"));

  // function pointer
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nonnull (*makeNonnull)(),
                int* _Nullable (*makeNullable)(), int* (*makeUnannotated)()) {
      *makeNonnull();
      *makeNullable();  // [[unsafe]]
      *makeUnannotated();
    }
  )cc"));

  // pointer to function pointer
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nonnull (**makeNonnull)(),
                int* _Nullable (**makeNullable)(), int* (**makeUnannotated)()) {
      *(*makeNonnull)();
      *(*makeNullable)();  // [[unsafe]]
      *(*makeUnannotated)();
    }
  )cc"));

  // function returning a function pointer which returns a pointer
  EXPECT_TRUE(checkDiagnostics(R"cc(
    typedef int* _Nonnull (*MakeNonnullT)();
    typedef int* _Nullable (*MakeNullableT)();
    typedef int* (*MakeUnannotatedT)();
    void target(MakeNonnullT (*makeNonnull)(), MakeNullableT (*makeNullable)(),
                MakeUnannotatedT (*makeUnannotated)()) {
      *(*makeNonnull)()();
      *(*makeNullable)()();  // [[unsafe]]
      *(*makeUnannotated)()();
    }
  )cc"));

  // free function returns reference to pointer
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nonnull &makeNonnull();
    int *_Nullable &makeNullable();
    int *&makeUnannotated();
    void target() {
      *makeNonnull();
      *makeNullable();  // [[unsafe]]
      *makeUnannotated();
    }
  )cc"));

  // function called in loop
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nullable makeNullable();
    bool makeBool();
    void target() {
      bool first = true;
      while (true) {
        int *x = makeNullable();
        if (first && x == nullptr) return;
        first = false;
        *x;  // [[unsafe]]
      }
    }
  )cc"));
}

}  // namespace
}  // namespace nullability
}  // namespace tidy
}  // namespace clang
