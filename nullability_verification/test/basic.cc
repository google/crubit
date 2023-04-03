// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for basic functionality (simple dereferences without control flow).

#include "nullability_verification/test/check_diagnostics.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang {
namespace tidy {
namespace nullability {
namespace {

TEST(PointerNullabilityTest, NoPointerOperations) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target() { 1 + 2; }
  )cc"));
}

TEST(PointerNullabilityTest, DerefNullPtr) {
  // nullptr
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target() {
      int *x = nullptr;
      *x;  // [[unsafe]]
    }
  )cc"));

  // 0
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target() {
      int *x = 0;
      *x;  // [[unsafe]]
    }
  )cc"));
}

TEST(PointerNullabilityTest, DerefAddrOf) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target() {
      int i;
      int *x = &i;
      *x;
    }
  )cc"));

  // transitive
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target() {
      int i;
      int *x = &i;
      int *y = x;
      *y;
    }
  )cc"));
}

TEST(PointerNullabilityTest, DerefPtrAnnotatedNonNullWithoutACheck) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nonnull x) { *x; }
  )cc"));

  // transitive
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nonnull x) {
      int *y = x;
      *y;
    }
  )cc"));
}

TEST(PointerNullabilityTest, DerefPtrAnnotatedNullableWithoutACheck) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable x) {
      *x;  // [[unsafe]]
    }
  )cc"));

  // transitive
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable x) {
      int *y = x;
      *y;  // [[unsafe]]
    }
  )cc"));
}

TEST(PointerNullabilityTest, DerefUnknownPtrWithoutACheck) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *x) { *x; }
  )cc"));

  // transitive
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *x) {
      int *y = x;
      *y;
    }
  )cc"));
}

TEST(PointerNullabilityTest, ArrowOperatorOnNonNullPtr) {
  // (->) member field
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      Foo *foo;
    };
    void target(Foo *_Nonnull foo) { foo->foo; }
  )cc"));

  // (->) member function
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      Foo *foo();
    };
    void target(Foo *_Nonnull foo) { foo->foo(); }
  )cc"));
}

TEST(PointerNullabilityTest, ArrowOperatorOnNullablePtr) {
  // (->) member field
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      Foo *foo;
    };
    void target(Foo *_Nullable foo) {
      foo->foo;  // [[unsafe]]
      if (foo) {
        foo->foo;
      } else {
        foo->foo;  // [[unsafe]]
      }
      foo->foo;  // [[unsafe]]
    }
  )cc"));

  // (->) member function
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      Foo *foo();
    };
    void target(Foo *_Nullable foo) {
      foo->foo();  // [[unsafe]]
      if (foo) {
        foo->foo();
      } else {
        foo->foo();  // [[unsafe]]
      }
      foo->foo();  // [[unsafe]]
    }
  )cc"));
}

TEST(PointerNullabilityTest, ArrowOperatorOnUnknownPtr) {
  // (->) member field
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      Foo *foo;
    };
    void target(Foo *foo) { foo->foo; }
  )cc"));

  // (->) member function
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      Foo *foo();
    };
    void target(Foo *foo) { foo->foo(); }
  )cc"));
}

}  // namespace
}  // namespace nullability
}  // namespace tidy
}  // namespace clang
