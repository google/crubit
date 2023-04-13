// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for casts of types containing nullability annotations.

#include "nullability_verification/test/check_diagnostics.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang {
namespace tidy {
namespace nullability {
namespace {

// TODO(b/233582219): Implement diagnosis of unreachable program points
TEST(PointerNullabilityTest, NonNullPtrImplicitCastToBool) {
  // x
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nonnull x) {
      *x;
      if (x) {
        *x;
      } else {
        *x;  // unreachable
      }
      *x;
    }
  )cc"));

  // !x
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nonnull x) {
      *x;
      if (!x) {
        *x;  // unreachable
      } else {
        *x;
      }
      *x;
    }
  )cc"));
}

TEST(PointerNullabilityTest, NullablePtrImplicitCastToBool) {
  // x
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable x) {
      *x;  // [[unsafe]]
      if (x) {
        *x;
      } else {
        *x;  // [[unsafe]]
      }
      *x;  // [[unsafe]]
    }
  )cc"));

  // !x
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable x) {
      *x;  // [[unsafe]]
      if (!x) {
        *x;  // [[unsafe]]
      } else {
        *x;
      }
      *x;  // [[unsafe]]
    }
  )cc"));
}

// TODO(b/233582219): Fix false negatives. Casting the pointer to boolean is
// evidence of the author considering null a possibility, hence the unnannotated
// pointer should be considered nullable and emit warnings where it fails or is
// not null checked.
TEST(PointerNullabilityTest, UnknownPtrImplicitCastToBool) {
  // x
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *x) {
      *x;  // false-negative
      if (x) {
        *x;
      } else {
        *x;  // false-negative
      }
      *x;  // false-negative
    }
  )cc"));

  // !x
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *x) {
      *x;  // false-negative
      if (!x) {
        *x;  // false-negative
      } else {
        *x;
      }
      *x;  // false-negative
    }
  )cc"));
}

TEST(PointerNullabilityTest, CastExpression) {
  // TODO: We currently do not warn on local variables
  // whose annotations conflict with the initializer. Decide whether to do so,
  // and then treat static casts in an equivalent manner.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable p) {
      static_cast<int *_Nonnull>(p);  // TODO: To warn, or not to warn, that is
                                      // the question.
      static_cast<int *>(p);
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <int I0, typename T1, typename T2>
    struct Struct3Arg {
      T1 arg1;
      T2 arg2;
    };

    void target(Struct3Arg<1, int *_Nullable, int *> &p) {
      *static_cast<const Struct3Arg<1, int *, int *> &>(p).arg1;  // [[unsafe]]
      *static_cast<const Struct3Arg<1, int *, int *> &>(p).arg2;
      *static_cast<int *>(p.arg1);  // [[unsafe]]
      *static_cast<int *>(p.arg2);
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct Base {};
    struct Derived : public Base {};

    void target(Derived *_Nullable x, Derived *_Nonnull y) {
      *static_cast<Base *>(x);  // [[unsafe]]
      *static_cast<Base *>(y);  // [[unsafe]] TODO: Fix false positive.
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <int I0, typename T1, typename T2>
    struct Struct3Arg {
      T1 arg1;
      T2 arg2;
    };

    void target(Struct3Arg<1, int *_Nullable, int *> &p) {
      *((const Struct3Arg<1, int *, int *> &)p).arg1;  // [[unsafe]]
      *((const Struct3Arg<1, int *, int *> &)p)        // [[unsafe]]
           .arg2;        // TODO: Fix false positive.
      *(int *)p.arg1;    // [[unsafe]]
      *(int *)p.arg2;    // [[unsafe]] TODO: Fix false positive.
      *(float *)p.arg1;  // [[unsafe]]
      *(char *)p.arg2;   // [[unsafe]] TODO: Fix false positive.
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <typename T0, typename T1>
    struct Struct2Arg {
      T0 arg0;
      T1 arg1;
    };

    void target(Struct2Arg<const int *, const int *_Nullable> &p) {
      *const_cast<int *>(p.arg0);  // [[unsafe]] TODO: Fix false positive.
      *const_cast<int *>(p.arg1);  // [[unsafe]]
    }
  )cc"));
}

}  // namespace
}  // namespace nullability
}  // namespace tidy
}  // namespace clang
