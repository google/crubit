// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for parenthesized expressions.

#include "nullability/test/check_diagnostics.h"
#include "external/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
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

}  // namespace
}  // namespace clang::tidy::nullability
