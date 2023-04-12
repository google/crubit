// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <optional>
#include <set>
#include <string>

#include "nullability_verification/test/check_diagnostics.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang {
namespace tidy {
namespace nullability {
namespace {

TEST(PointerNullabilityTest, ParenthesizedExpressions) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <typename T0>
    struct Struct1Arg {
      T0 arg0;
      T0 getT0();
    };

    void target(Struct1Arg<int *_Nullable> p) {
      *(p).arg0;         // [[unsafe]]
      *((p)).arg0;       // [[unsafe]]
      *(p).getT0();      // [[unsafe]]
      *(((p))).getT0();  // [[unsafe]]
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <int I0, typename T1, typename T2>
    struct Struct3ArgWithInt {
      T1 arg1;
      T2 arg2;

      T1 getT1();
      T2 getT2();
    };

    void target(Struct3ArgWithInt<1, int *, int *_Nullable> p) {
      *(((p)).arg1);
      *(((p))).getT1();
      (*((p)).arg2);         // [[unsafe]]
      *(((((p)))).getT2());  // [[unsafe]]
    }
  )cc"));
}

// TODO: fix false positives due to unsupported PointerValues in the framework.
TEST(PointerNullabilityTest, PointerArithmetic) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable p, int *_Nonnull q, int *r) {
      *++p;  // [[unsafe]]
      *p++;  // [[unsafe]]
      *--p;  // [[unsafe]]
      *p--;  // [[unsafe]]
      *+p;   // [[unsafe]]

      *++q;  // [[unsafe]] TODO: fix false positive
      *q++;  // [[unsafe]] TODO: fix false positive
      *--q;  // [[unsafe]] TODO: fix false positive
      *q--;  // [[unsafe]] TODO: fix false positive
      *+q;   // [[unsafe]] TODO: fix false positive

      *++r;  // [[unsafe]] TODO: fix false positive
      *r++;  // [[unsafe]] TODO: fix false positive
      *--r;  // [[unsafe]] TODO: fix false positive
      *r--;  // [[unsafe]] TODO: fix false positive
      *+r;   // [[unsafe]] TODO: fix false positive
    }
  )cc"));
}

// TODO: fix false positives due to unsupported PointerValues in the framework.
TEST(PointerNullabilityTest, Deref) {
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

TEST(PointerNullabilityTest, NonPointerReturnType) {
  checkDiagnostics(R"cc(
    struct S {
      int* p;
      int*& target() { return p; }
    };
  )cc");

  checkDiagnostics(R"cc(
    struct S {
      int* _Nullable p;
      int* _Nonnull& target() {
        return p;  // TODO: Fix false negative.
      }
    };
  )cc");
}

TEST(PointerNullabilityTest, ParenTypeInTemplate) {
  checkDiagnostics(R"cc(
    template <typename T>
    struct S {
      T(a);
      T(*(b));

      T (*f)();
      T(((*g)))();
    };

    void targetNullable(S<int* _Nullable> s) {
      *s.a;   // [[unsafe]]
      **s.b;  // [[unsafe]]
      *s.f;
      *s.g;
      // TODO: Handle function pointers. The analysis currently crashes.
      // *s.f();
      // *s.g();
    }

    void targetNonnull(S<int* _Nonnull> s) {
      *s.a;
      **s.b;
      *s.f;
      *s.g;
      // TODO: Handle function pointers. The analysis currently crashes.
      // *s.f();
      // *s.g();
    }
  )cc");

  checkDiagnostics(R"cc(
    template <typename T>
    struct S {
      T arg;
    };

    void targetNullable(S<int* _Nullable>(a), S<int* _Nullable>(*(b)),
                        S<int(*_Nullable)> c, S<int*(*(*_Nullable))> d,
                        S<int* _Nullable (*)()> e) {
      *a.arg;    // [[unsafe]]
      *b->arg;   // [[unsafe]]
      *c.arg;    // [[unsafe]]
      ***d.arg;  // [[unsafe]]
      *e.arg;    // [[unsafe]]

      // TODO: Handle function pointers. The analysis currently crashes.
      // *e.arg();
    }

    void targetNonnull(S<int* _Nonnull>(a), S<int* _Nonnull>(*(b)),
                       S<int(*_Nonnull)> c, S<int*(*(*_Nonnull))> d,
                       S<int* _Nonnull (*)()> e) {
      *a.arg;
      *b->arg;
      *c.arg;
      ***d.arg;
      *e.arg;

      // TODO: Handle function pointers. The analysis currently crashes.
      // *e.arg();
    }
  )cc");
}

}  // namespace
}  // namespace nullability
}  // namespace tidy
}  // namespace clang
