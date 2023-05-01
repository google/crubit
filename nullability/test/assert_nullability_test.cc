// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests that check that `__assert_nullability` works correctly.
//
// Note that this file should not contain all tests that use
// `__assert_nullability`, but only tests to verify that `__assert_nullability`
// itself works.
//
// TODO(mboehme): Because this test is doing something different than the
// other tests, we would ideally want to place it in a different directory.
// For the time being, the `_test` suffix at the end of the filename is
// intended as a weak way of indicating this.

#include "nullability/test/check_diagnostics.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang {
namespace tidy {
namespace nullability {
namespace {

TEST(PointerNullabilityTest, AssertNullability) {
  // Concrete struct.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct StructNonnullNullable {
      int* _Nonnull nonnull;
      int* _Nullable nullable;
    };

    void target(StructNonnullNullable p) {
      __assert_nullability<>(p);
      __assert_nullability<NK_nonnull>(p);                   // [[unsafe]]
      __assert_nullability<NK_nullable>(p);                  // [[unsafe]]
      __assert_nullability<NK_nonnull, NK_nullable>(p);      // [[unsafe]]
      __assert_nullability<NK_nonnull, NK_unspecified>(p);   // [[unsafe]]
      __assert_nullability<NK_nonnull, NK_nonnull>(p);       // [[unsafe]]
      __assert_nullability<NK_nullable, NK_nullable>(p);     // [[unsafe]]
      __assert_nullability<NK_unspecified, NK_nullable>(p);  // [[unsafe]]
    }
  )cc"));

  // Struct with two template type parameters.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <typename T0, typename T1>
    struct Struct2Arg {};

    void target(Struct2Arg<int *, int *_Nullable> p) {
      __assert_nullability<NK_unspecified>(p);  // [[unsafe]]
      __assert_nullability<NK_nullable>(p);     // [[unsafe]]

      __assert_nullability<NK_unspecified, NK_nonnull>(p);  // [[unsafe]]
      __assert_nullability<NK_unspecified, NK_nullable>(p);
      __assert_nullability<NK_unspecified, NK_unspecified>(p);  // [[unsafe]]
      __assert_nullability<NK_nonnull, NK_nullable>(p);         // [[unsafe]]
      __assert_nullability<NK_nullable, NK_nullable>(p);        // [[unsafe]]

      __assert_nullability  // [[unsafe]]
          <NK_unspecified, NK_nullable, NK_unspecified>(p);
      __assert_nullability  // [[unsafe]]
          <NK_unspecified, NK_nullable, NK_nullable>(p);
    }
  )cc"));

  // Struct with one type and non-type template parameters.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <int I0, typename T1, typename T2>
    struct Struct3ArgWithInt {};

    void target(Struct3ArgWithInt<2147483647, int* _Nullable, int* _Nonnull> p) {
      __assert_nullability<>(p);             // [[unsafe]]
      __assert_nullability<NK_nonnull>(p);   // [[unsafe]]
      __assert_nullability<NK_nullable>(p);  // [[unsafe]]

      __assert_nullability<NK_unspecified, NK_nonnull>(p);  // [[unsafe]]
      __assert_nullability<NK_nonnull, NK_nonnull>(p);      // [[unsafe]]
      __assert_nullability<NK_nonnull, NK_nullable>(p);     // [[unsafe]]
      __assert_nullability<NK_nullable, NK_nonnull>(p);
      __assert_nullability<NK_nullable, NK_nullable>(p);     // [[unsafe]]
      __assert_nullability<NK_nullable, NK_unspecified>(p);  // [[unsafe]]

      __assert_nullability  // [[unsafe]]
          <NK_unspecified, NK_nullable, NK_nonnull>(p);
    }
  )cc"));

  // Nested template arguments.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <typename T0, typename T1>
    struct Struct2Arg {};

    void target(
        Struct2Arg<Struct2Arg<int *, int *_Nullable>,
                   Struct2Arg<Struct2Arg<int *_Nullable, int *_Nonnull>,
                              Struct2Arg<int *_Nullable, int *_Nullable>>>
            p) {
      __assert_nullability<>(p);  // [[unsafe]]

      __assert_nullability<NK_unspecified, NK_nullable, NK_nullable, NK_nonnull,
                           NK_nullable, NK_nullable>(p);
      __assert_nullability  // [[unsafe]]
          <NK_unspecified, NK_nullable, NK_nullable, NK_nonnull, NK_nullable,
           NK_nullable, NK_nullable>(p);
      __assert_nullability  // [[unsafe]]
          <NK_unspecified, NK_nullable, NK_nullable, NK_nonnull, NK_nullable>(
              p);
    }
  )cc"));

  // Struct with two template parameters substituted with concrete structs.
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct StructUnknownNullable {
      int* unknown;
      int* _Nullable nullable;
    };

    struct StructNullableNonnull {
      int* _Nullable nullable;
      int* _Nonnull nonnull;
    };

    template <typename T1, typename T2>
    struct Struct2Arg {};

    void target(Struct2Arg<StructUnknownNullable, StructNullableNonnull> p) {
      __assert_nullability<>(p);

      __assert_nullability<NK_unspecified, NK_nullable>(p);  // [[unsafe]]

      __assert_nullability  // [[unsafe]]
          <NK_unspecified, NK_nullable, NK_nullable>(p);
      __assert_nullability  // [[unsafe]]
          <NK_unspecified, NK_nullable, NK_nullable, NK_nonnull>(p);
      __assert_nullability  // [[unsafe]]
          <NK_nonnull, NK_nullable, NK_nullable, NK_nonnull>(p);
      __assert_nullability  // [[unsafe]]
          <NK_unspecified, NK_nullable, NK_nullable, NK_nonnull,
           NK_unspecified>(p);
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    template <typename T0, typename T1>
    struct Struct2Arg {
      T0 arg0;
      T1 arg1;

      T0 getT0();
      T1 getT1();
    };

    void target(
        Struct2Arg<Struct2Arg<int *, int *_Nullable>,
                   Struct2Arg<Struct2Arg<int *_Nullable, int *_Nonnull>,
                              Struct2Arg<int *_Nullable, int *_Nullable>>>
            p) {
      __assert_nullability<NK_unspecified, NK_nullable, NK_nullable, NK_nonnull,
                           NK_nullable, NK_nullable>(p);
      __assert_nullability<NK_unspecified, NK_nullable>(p.arg0);
      __assert_nullability<NK_unspecified>(p.arg0.arg0);
      __assert_nullability<NK_nullable>(p.arg0.arg1);
      __assert_nullability<NK_nullable, NK_nonnull, NK_nullable, NK_nullable>(
          p.arg1);
      __assert_nullability<NK_nullable, NK_nonnull>(p.arg1.arg0);
      __assert_nullability<NK_nullable>(p.arg1.arg0.arg0);
      __assert_nullability<NK_nonnull>(p.arg1.arg0.arg1);
      __assert_nullability<NK_nullable, NK_nullable>(p.arg1.arg1);
      __assert_nullability<NK_nullable>(p.arg1.arg1.arg0);
      __assert_nullability<NK_nullable>(p.arg1.arg1.arg1);

      __assert_nullability<>(p.arg0.arg0);           // [[unsafe]]
      __assert_nullability<NK_unspecified>(p.arg0);  // [[unsafe]]
      __assert_nullability                           // [[unsafe]]
          <NK_unspecified, NK_nullable, NK_nonnull, NK_nullable, NK_nullable>(
              p.arg1);

      __assert_nullability<NK_unspecified, NK_nullable>(p.getT0());
      __assert_nullability<NK_nonnull>(p.getT1().getT0().getT1());

      __assert_nullability  // [[unsafe]]
          <NK_unspecified, NK_nullable, NK_unspecified>(p.getT0());
      __assert_nullability  // [[unsafe]]
          <NK_unspecified>(p.getT0());

      __assert_nullability<NK_nonnull>(p.getT1().arg0.getT1());
      __assert_nullability<NK_nonnull>(p.arg1.getT0().arg1);
      __assert_nullability<NK_nonnull>(p.arg1.arg0.arg1);

      __assert_nullability  // [[unsafe]]
          <>(p.getT1().getT0().getT1());
      __assert_nullability  // [[unsafe]]
          <NK_nonnull, NK_nonnull>(p.arg1.getT0().arg1);
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int* _Nullable p, int* _Nonnull q, int* r) {
      __assert_nullability<NK_nonnull, NK_nullable>(&p);
      __assert_nullability<NK_nonnull, NK_nonnull>(&q);
      __assert_nullability<NK_nonnull>(&*p);  // [[unsafe]]
      __assert_nullability<NK_nonnull>(&*q);
      __assert_nullability<NK_nonnull>(&*r);
    }
  )cc"));
}

}  // namespace
}  // namespace nullability
}  // namespace tidy
}  // namespace clang
