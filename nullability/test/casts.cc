// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for casts of types containing nullability annotations.

#include "nullability/test/check_diagnostics.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

// TODO(b/233582219): Implement diagnosis of unreachable program points
TEST(PointerNullabilityTest, NonNullPtrImplicitCastToBool) {
  // x
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nonnull x) {
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
    void target(int *_Nonnull x) {
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
    void target(int *_Nullable x) {
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
    void target(int *_Nullable x) {
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

// CK_Bitcast: Bitcasts preserve outer nullability
TEST(PointerNullabilityTest, Bitcast) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <class X>
    struct vector {};

    void target() {
      // Bitcasts preserve nullability.
      __assert_nullability<NK_nullable>((void *)value<int *_Nullable>());
      __assert_nullability<NK_nonnull>((void *)value<int *_Nonnull>());
      __assert_nullability<NK_unspecified>((void *)value<int *>());
      // Nullability of further outer pointer types is preserved in bitcasts.
      __assert_nullability<NK_nullable, NK_nullable>(
          (void **)value<int *_Nullable *_Nullable>());
      __assert_nullability<NK_nonnull, NK_nonnull>(
          (void **)value<int *_Nonnull *_Nonnull>());
      __assert_nullability<NK_unspecified, NK_unspecified>(
          (void **)value<int **>());
      // But nullability of other inner types is dropped.
      __assert_nullability<NK_nullable, NK_unspecified>(
          (void **)value<vector<int *_Nullable> *_Nullable>());
      __assert_nullability<NK_nonnull, NK_unspecified>(
          (void **)value<vector<int *_Nonnull> *_Nonnull>());

      __assert_nullability<NK_nonnull, NK_unspecified>(
          (void **)value<int *_Nonnull>);
      __assert_nullability<NK_nonnull>((void *)value<int *_Nonnull *_Nonnull>());
    }
  )cc"));
}

// CK_NoOp: No-op casts preserve deep nullability
// TODO: fix false-positives from treating untracked values as unsafe.
TEST(PointerNullabilityTest, NoOp) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <class X>
    struct vector {};

    void target() {
      // No-op casts preserve deep nullability.
      __assert_nullability  // [[unsafe]] TODO: fix false positive
          <NK_nullable, NK_nullable>(const_cast<vector<int> *>(
              (vector<int> *const)value<vector<int *_Nullable> *_Nullable>()));
    }
  )cc"));
}

// Casts between types with inheritance - only simple cases handled.
// TODO: fix false-positives from treating untracked values as unsafe.
TEST(PointerNullabilityTest, Inheritance) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <class X>
    struct base {
      virtual void ensure_polymorphic();
    };
    template <class X>
    struct derived : base<X> {};

    void target() {
      // CK_BaseToDerived: preserves outer nullability only.
      // TODO: determine that derived's type param is actually nullable here.
      __assert_nullability<NK_nullable, NK_unspecified>(
          (derived<int *> *)value<base<int *_Nullable> *_Nullable>());
      // CK_Dynamic: dynamic_cast returns a nullable pointer.
      auto b = value<base<int *_Nonnull> *_Nonnull>();
      __assert_nullability  // [[unsafe]] TODO: fix false positive
          <NK_nullable, NK_unspecified>(dynamic_cast<derived<int> *>(b));
      // ... only if casting to a pointer!
      auto c = value<base<int *>>();
      __assert_nullability<NK_unspecified>(dynamic_cast<derived<int *> &>(c));
    }
  )cc"));
}

// User-defined conversions could do anything, use declared type.
TEST(PointerNullabilityTest, UserDefinedConversions) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <class X>
    struct BuildFromPointer {
      BuildFromPointer(int *);
    };

    void target() {
      // User-defined conversions could do anything.
      // CK_ConstructorConversion
      __assert_nullability<NK_unspecified>(
          (BuildFromPointer<double *>)value<int *_Nonnull>());
    }
  )cc"));
}

TEST(PointerNullabilityTest, CastToNonPointer) {
  // Casting to non-pointer types destroyes nullability.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    using I = __INTPTR_TYPE__;

    // TODO: fix false-positives from treating untracked values as unsafe.
    void target() {
      // Casting away pointerness destroys nullability.
      // CK_PointerToIntegral
      __assert_nullability<>((I)value<int *_Nonnull>());
      // CK_PointerToBoolean
      __assert_nullability<>((bool)value<int *_Nonnull>());
      // Casting them back does not recover it.
      // CK_IntegralToPointer
      __assert_nullability  // [[unsafe]] TODO: fix false positive
          <>((int *)(I)value<int *_Nonnull>());
    }
  )cc"));
}

TEST(PointerNullabilityTest, TrivialNullability) {
  // Casts with trivial nullability
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target() {
      // Null is nullable!
      __assert_nullability<NK_nullable>((int *)nullptr);

      // Decayed objects are non-null.
      int array[2];
      __assert_nullability<NK_nonnull>((int *)array);
    }
  )cc"));
}

TEST(PointerNullabilityTest, CastNullToAlias) {
  // This used to crash!
  EXPECT_TRUE(checkDiagnostics(R"cc(
    using P = int *;
    P target() { return nullptr; }
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
      *static_cast<Base *>(y);
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
      *((const Struct3Arg<1, int *, int *> &)p).arg2;
      *(int *)p.arg1;  // [[unsafe]]
      *(int *)p.arg2;
      *(float *)p.arg1;  // [[unsafe]]
      *(char *)p.arg2;
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <typename T0, typename T1>
    struct Struct2Arg {
      T0 arg0;
      T1 arg1;
    };

    void target(Struct2Arg<const int *, const int *_Nullable> &p) {
      *const_cast<int *>(p.arg0);
      *const_cast<int *>(p.arg1);  // [[unsafe]]
    }
  )cc"));
}

TEST(PointerNullabilityTest, CastToNullptrT) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    namespace std {
    using nullptr_t = decltype(nullptr);
    }
    void target(const std::nullptr_t null) { std::nullptr_t p = null; }
  )cc"));
}

}  // namespace
}  // namespace clang::tidy::nullability
