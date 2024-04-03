// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests of diagnoser-specific pragma support.
// (Detailed tests of the shared analysis part are in pragma.cc)

#include "nullability/test/check_diagnostics.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

TEST(PragmaDiagnosisTest, SharedAnalysisSmoke) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#pragma nullability file_default nullable

    void target(int *p) {
      *p;  // [[unsafe]]
    }
  )cc"));
}

TEST(PragmaDiagnosisTest, ReturnType) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#pragma nullability file_default nonnull
    bool cond();
    int *target(int *_Nonnull nn, int *_Nullable n, int *u) {
      if (cond()) return nn;
      if (cond()) return n;  // TODO(b/332569977): unsafe
      return u;
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
#pragma nullability file_default nullable
    bool cond();
    int *target(int *_Nonnull nn, int *_Nullable n, int *u) {
      if (cond()) return nn;
      if (cond()) return n;
      return u;
    }
  )cc"));
}

TEST(PragmaDiagnosisTest, Assignment) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#pragma nullability file_default nonnull
    int *v;
    void target(int *_Nonnull nn, int *_Nullable n, int *u) {
      v = nn;
      v = n;  // TODO(b/307797224): unsafe
      v = u;
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
#pragma nullability file_default nullable
    int *v;
    int *target(int *_Nonnull nn, int *_Nullable n, int *u) {
      v = nn;
      v = n;
      v = u;
    }
  )cc"));
}

TEST(PragmaDiagnosisTest, FunctionCall) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
#pragma nullability file_default nonnull
    void consume(int *x);
    void target(int *_Nonnull nn, int *_Nullable n, int *u) {
      consume(nn);
      consume(n);  // TODO(b/332569977): unsafe
      consume(u);
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
#pragma nullability file_default nullable
    void consume(int *x);
    void target(int *_Nonnull nn, int *_Nullable n, int *u) {
      consume(nn);
      consume(n);
      consume(u);
    }
  )cc"));
}

}  // namespace
}  // namespace clang::tidy::nullability
