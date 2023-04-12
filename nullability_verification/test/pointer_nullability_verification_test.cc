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

TEST(PointerNullabilityTest, NonFlowSensitiveMaterializeTemporaryExpr) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int *_Nonnull makeNonnull();
    int *_Nullable makeNullable();
    int *makeUnannotated();

    template <typename T>
    T identity(const T &);

    void target() {
      {
        *identity<int *_Nonnull>(makeNonnull());
        int *const &p = makeNonnull();
        *p;
      }
      {
        *identity<int *_Nullable>(makeNullable());  // [[unsafe]]
        int *const &p = makeNullable();
        *p;  // [[unsafe]]
      }
      {
        *identity<int *>(makeUnannotated());
        int *const &p = makeUnannotated();
        *p;
      }
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <typename T0, typename T1>
    struct Struct2Arg {
      T0 getT0();
      T1 getT1();
    };

    template <typename T>
    T make();

    template <typename T>
    T identity(const T &);

    void target(Struct2Arg<int *, int *_Nullable> &p) {
      *identity<Struct2Arg<int *, int *_Nullable>>(p).getT0();
      *identity<Struct2Arg<int *, int *_Nullable>>(
           make<Struct2Arg<int *, int *_Nullable>>())
           .getT0();
      *identity<Struct2Arg<int *, int *_Nullable>>(
           Struct2Arg<int *, int *_Nullable>(p))
           .getT0();
      *identity<int *>(p.getT0());
      *identity<Struct2Arg<int *, int *_Nullable>>(p).getT1();  // [[unsafe]]
      *identity<Struct2Arg<int *, int *_Nullable>>(             // [[unsafe]]
           make<Struct2Arg<int *, int *_Nullable>>())
           .getT1();
      *identity<Struct2Arg<int *, int *_Nullable>>(  // [[unsafe]]
           Struct2Arg<int *, int *_Nullable>(p))
           .getT1();
      *identity<int *_Nullable>(p.getT1());  // [[unsafe]]
    }
  )cc"));
}

TEST(PointerNullabilityTest, FunctionTemplates) {
  // Call expression that returns the first of two type parameters.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <typename T0, typename T1>
    T0 returnFirst();

    void target() {
      *returnFirst<int *_Nonnull, int *_Nullable>();
      *returnFirst<int *, int *_Nullable>();
      *returnFirst<int *_Nullable, int *_Nonnull>();  // [[unsafe]]
    }
  )cc"));

  // Call expression that returns the second of two type parameters.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <typename T0, typename T1>
    T1 returnSecond();

    void target() {
      *returnSecond<int *_Nullable, int *_Nonnull>();
      *returnSecond<int *_Nullable, int *>();
      *returnSecond<int *, int *_Nullable>();  // [[unsafe]]
    }
  )cc"));

  // Call expression that has an int parameter and two type parameters,
  // returning the first type parameter.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <int I0, typename T1, typename T2>
    T1 fn3ArgWithInt();

    void target() {
      *fn3ArgWithInt<1, int *_Nullable, int *>();  // [[unsafe]]
      *fn3ArgWithInt<1, int *, int *_Nullable>();
    }
  )cc"));

  // Call expression with template parameter substituted with a concrete struct.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct StructUnknownNullable {
      int *var0;
      int *_Nullable var1;

      int *getVar0();
      int *_Nullable getVar1();
    };

    template <typename T0, typename T1>
    T1 returnSecond();

    void target() {
      *returnSecond<StructUnknownNullable, int *_Nullable>();  // [[unsafe]]
      *returnSecond<int *_Nonnull, StructUnknownNullable *>();
      // TODO: The following line is a false positive. We correctly compute the
      // nullability of the expression, as confirmed by the call to
      // `assert_nullability`. However, the dataflow framework currently does
      // not model pointer values for this expression, which results in a (in
      // this case incorrect) nullptr value.
      *returnSecond<int *_Nonnull, StructUnknownNullable>()  // [[unsafe]]
           .var0;  // TODO: Fix false positive.
      __assert_nullability<NK_unspecified>(
          returnSecond<int *_Nonnull, StructUnknownNullable>().var0);
      *returnSecond<int *_Nonnull, StructUnknownNullable>().var1;  // [[unsafe]]
      *returnSecond<int *_Nonnull, StructUnknownNullable>().getVar0();
      *returnSecond<int *_Nonnull, StructUnknownNullable>()  // [[unsafe]]
           .getVar1();
    }
  )cc"));
}

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
