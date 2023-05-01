// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for nullability annotations on fields.

#include "nullability/test/check_diagnostics.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang {
namespace tidy {
namespace nullability {
namespace {

TEST(PointerNullabilityTest, NonNullFieldsOfPointerType) {
  // dereference field of pointer type
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      Foo* _Nonnull ptr;
    };
    void target(Foo foo) { *foo.ptr; }
  )cc"));

  // dereference field of pointer type in member function
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      Foo* _Nonnull ptr;
      void target() { *ptr; }
    };
  )cc"));
}

TEST(PointerNullabilityTest, NullableFieldsOfPointerType) {
  // dereference field of pointer type
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      Foo* _Nullable ptr;
    };
    void target(Foo foo) {
      *foo.ptr;  // [[unsafe]]
      if (foo.ptr) {
        *foo.ptr;
      } else {
        *foo.ptr;  // [[unsafe]]
      }
      *foo.ptr;  // [[unsafe]]
    }
  )cc"));

  // dereference field of pointer type in member function
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      Foo* _Nullable ptr;
      void target() {
        *ptr;  // [[unsafe]]
        if (ptr) {
          *ptr;
        } else {
          *ptr;  // [[unsafe]]
        }
        *ptr;  // [[unsafe]]
      }
    };
  )cc"));
}

TEST(PointerNullabilityTest, UnknownFieldsOfPointerType) {
  // dereference field of pointer type
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      Foo *ptr;
    };
    void target(Foo foo) { *foo.ptr; }
  )cc"));

  // dereference field of pointer type in member function
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      Foo *ptr;
      void target() { *ptr; }
    };
  )cc"));
}

// TODO: fix false positives due to unsupported PointerValues in the framework.
TEST(PointerNullabilityTest, ChainedFieldDeref) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct S {
      S* _Nonnull nonnull;
      S* _Nullable nullable;
      S* unknown;
    };
    void target(S& s) {
      *(*s.nonnull).nonnull;   // [[unsafe]] TODO: fix false positive
      *(*s.nonnull).nullable;  // [[unsafe]]
      *(*s.nonnull).unknown;   // [[unsafe]] TODO: fix false positive

      s.nonnull->nonnull->nonnull;   // [[unsafe]] TODO: fix false positive
      s.nonnull->nonnull->nullable;  // [[unsafe]] TODO: fix false positive
      s.nonnull->nullable->nonnull;  // [[unsafe]]
      s.nonnull->unknown->nonnull;   // [[unsafe]] TODO: fix false positive

      *&s;
    }
  )cc"));
}

}  // namespace
}  // namespace nullability
}  // namespace tidy
}  // namespace clang
