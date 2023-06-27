// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for temporary materialization.

#include "nullability/test/check_diagnostics.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

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

}  // namespace
}  // namespace clang::tidy::nullability
