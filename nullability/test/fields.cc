// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for nullability annotations on fields.

#include "nullability/test/check_diagnostics.h"
#include "external/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

TEST(PointerNullabilityTest, NonNullFieldsOfPointerType) {
  // dereference field of pointer type
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      Foo *_Nonnull ptr;
    };
    void target(Foo foo) { *foo.ptr; }
  )cc"));

  // dereference field of pointer type in member function
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      Foo *_Nonnull ptr;
      void target() { *ptr; }
    };
  )cc"));
}

TEST(PointerNullabilityTest, NullableFieldsOfPointerType) {
  // dereference field of pointer type
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Foo {
      Foo *_Nullable ptr;
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
      Foo *_Nullable ptr;
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

TEST(PointerNullabilityTest, ChainedFieldDeref) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct S {
      S *_Nonnull nonnull;
      S *_Nullable nullable;
      S *unknown;
    };
    void target(S &s) {
      *(*s.nonnull).nonnull;
      *(*s.nonnull).nullable;  // [[unsafe]]
      *(*s.nonnull).unknown;

      s.nonnull->nonnull->nonnull;
      s.nonnull->nonnull->nullable;
      s.nonnull->nullable->nonnull;  // [[unsafe]]
      s.nonnull->unknown->nonnull;

      *&s;
    }
  )cc"));
}

// This is a crash repro. It sets up a situation where we're merging pointers
// that don't have a null state to check that we don't crash in this case.
TEST(PointerNullabilityTest, MergePointersWithoutNullState) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct S {
      void *p;
    };
    void target(bool cond) {
      S src, dst;
      if (cond) dst = src;

      // `dst` has different values in the two branches that merge here, so we
      // will attempt to merge the values of `dst.p` from the two branches.

      // These lines are only here to ensure that `p` is modeled.
      S unrelated;
      unrelated.p;
    }
  )cc"));
}

TEST(PointerNullabilityTest, CreatesConsistentPointerValueForField) {
  // This is a repro for a false positive.
  // The call to `some_func()` clears the value of `p_`.
  // Our logic for creating new pointer values used to work only on prvalues,
  // so it would create two independent pointer values for the two accesses of
  // `p_`, and hence we would not be able to conclude that `p_` was null in the
  // `p_->target()` call.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct S {
      Nullable<S*> const p_;
      void target() {
        some_func();

        if (p_ != nullptr)
          p_->target();  // p_ needs to be a member variable to repro.
      }
      void some_func();
    };
  )cc"));
}

}  // namespace
}  // namespace clang::tidy::nullability
