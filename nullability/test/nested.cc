// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for pointers-to-pointers.

#include "nullability/test/check_diagnostics.h"
#include "clang/include/clang/Basic/LLVM.h"
#include "external/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

TEST(PointerNullabilityTest, DoubleDereference) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int **p) {
      *p;
      **p;
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int **_Nonnull p) {
      *p;
      **p;
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nonnull *p) {
      *p;
      **p;
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nonnull *_Nonnull p) {
      *p;
      **p;
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int **_Nullable p) {
      *p;   // [[unsafe]]
      **p;  // [[unsafe]]
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable *p) {
      *p;
      **p;  // [[unsafe]]
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable *_Nullable p) {
      *p;   // [[unsafe]]
      **p;  // [[unsafe]]
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable *_Nonnull p) {
      *p;
      **p;  // [[unsafe]]
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nonnull *_Nullable p) {
      *p;   // [[unsafe]]
      **p;  // [[unsafe]]
    }
  )cc"));
}

TEST(PointerNullabilityTest, AssignmentsFromNullptr) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int **p) {
      *p = nullptr;
      p = nullptr;
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int **_Nonnull p) {
      *p = nullptr;
      p = nullptr;  // [[unsafe]]
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nonnull *p) {
      *p = nullptr;  // [[unsafe]]
      p = nullptr;
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nonnull *_Nonnull p) {
      *p = nullptr;  // [[unsafe]]
      p = nullptr;   // [[unsafe]]
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int **_Nullable p) {
      if (p) {
        *p = nullptr;
      }
      p = nullptr;
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable *p) {
      *p = nullptr;
      p = nullptr;
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable *_Nullable p) {
      if (p) {
        *p = nullptr;
      }
      p = nullptr;
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable *_Nonnull p) {
      *p = nullptr;
      p = nullptr;  // [[unsafe]]
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nonnull *_Nullable p) {
      if (p) {
        *p = nullptr;  // [[unsafe]]
      }
      p = nullptr;
    }
  )cc"));
}

TEST(PointerNullabilityTest, AssignmentFromMutableVariables) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int **p, int *_Nonnull nonnull, int *_Nullable nullable,
                int **unknown_unknown, int **_Nonnull unknown_nonnull,
                int *_Nonnull *nonnull_unknown,
                int *_Nonnull *_Nonnull nonnull_nonnull,
                int **_Nullable unknown_nullable,
                int *_Nullable *nullable_unknown,
                int *_Nullable *_Nullable nullable_nullable,
                int *_Nullable *_Nonnull nullable_nonnull,
                int *_Nonnull *_Nullable nonnull_nullable) {
      *p = nonnull;
      *p = nullable;
      p = unknown_unknown;
      p = unknown_nonnull;
      p = nonnull_unknown;
      p = nonnull_nonnull;
      p = unknown_nullable;
      p = nullable_unknown;
      p = nullable_nullable;
      p = nullable_nonnull;
      p = nonnull_nullable;
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int **_Nonnull p, int *_Nonnull nonnull,
                int *_Nullable nullable, int **unknown_unknown,
                int **_Nonnull unknown_nonnull, int *_Nonnull *nonnull_unknown,
                int *_Nonnull *_Nonnull nonnull_nonnull,
                int **_Nullable unknown_nullable,
                int *_Nullable *nullable_unknown,
                int *_Nullable *_Nullable nullable_nullable,
                int *_Nullable *_Nonnull nullable_nonnull,
                int *_Nonnull *_Nullable nonnull_nullable) {
      *p = nonnull;
      *p = nullable;
      p = unknown_unknown;
      p = unknown_nonnull;
      p = nonnull_unknown;
      p = nonnull_nonnull;
      p = unknown_nullable;  // [[unsafe]]
      p = nullable_unknown;
      p = nullable_nullable;  // [[unsafe]]
      p = nullable_nonnull;
      p = nonnull_nullable;  // [[unsafe]]
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nonnull *p, int *_Nonnull nonnull,
                int *_Nullable nullable, int **unknown_unknown,
                int **_Nonnull unknown_nonnull, int *_Nonnull *nonnull_unknown,
                int *_Nonnull *_Nonnull nonnull_nonnull,
                int **_Nullable unknown_nullable,
                int *_Nullable *nullable_unknown,
                int *_Nullable *_Nullable nullable_nullable,
                int *_Nullable *_Nonnull nullable_nonnull,
                int *_Nonnull *_Nullable nonnull_nullable) {
      *p = nonnull;
      *p = nullable;  // [[unsafe]]
      p = unknown_unknown;
      p = unknown_nonnull;
      p = nonnull_unknown;
      p = nonnull_nonnull;
      p = unknown_nullable;
      p = nullable_unknown;   // TODO: b/343960612 - FALSE NEGATIVE
      p = nullable_nullable;  // TODO: b/343960612 - FALSE NEGATIVE
      p = nullable_nonnull;   // TODO: b/343960612 - FALSE NEGATIVE
      p = nonnull_nullable;
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nonnull *_Nonnull p, int *_Nonnull nonnull,
                int *_Nullable nullable, int **unknown_unknown,
                int **_Nonnull unknown_nonnull, int *_Nonnull *nonnull_unknown,
                int *_Nonnull *_Nonnull nonnull_nonnull,
                int **_Nullable unknown_nullable,
                int *_Nullable *nullable_unknown,
                int *_Nullable *_Nullable nullable_nullable,
                int *_Nullable *_Nonnull nullable_nonnull,
                int *_Nonnull *_Nullable nonnull_nullable) {
      *p = nonnull;
      *p = nullable;  // [[unsafe]]
      p = unknown_unknown;
      p = unknown_nonnull;
      p = nonnull_unknown;
      p = nonnull_nonnull;
      p = unknown_nullable;   // [[unsafe]]
      p = nullable_unknown;   // TODO: b/343960612 - FALSE NEGATIVE
      p = nullable_nullable;  // [[unsafe]]
      p = nullable_nonnull;   // TODO: b/343960612 - FALSE NEGATIVE
      p = nonnull_nullable;   // [[unsafe]]
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int **_Nullable p, int *_Nonnull nonnull,
                int *_Nullable nullable, int **unknown_unknown,
                int **_Nonnull unknown_nonnull, int *_Nonnull *nonnull_unknown,
                int *_Nonnull *_Nonnull nonnull_nonnull,
                int **_Nullable unknown_nullable,
                int *_Nullable *nullable_unknown,
                int *_Nullable *_Nullable nullable_nullable,
                int *_Nullable *_Nonnull nullable_nonnull,
                int *_Nonnull *_Nullable nonnull_nullable) {
      if (p) {
        *p = nonnull;
        *p = nullable;
      }
      p = unknown_unknown;
      p = unknown_nonnull;
      p = nonnull_unknown;
      p = nonnull_nonnull;
      p = unknown_nullable;
      p = nullable_unknown;
      p = nullable_nullable;
      p = nullable_nonnull;
      p = nonnull_nullable;
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable *p, int *_Nonnull nonnull,
                int *_Nullable nullable, int **unknown_unknown,
                int **_Nonnull unknown_nonnull, int *_Nonnull *nonnull_unknown,
                int *_Nonnull *_Nonnull nonnull_nonnull,
                int **_Nullable unknown_nullable,
                int *_Nullable *nullable_unknown,
                int *_Nullable *_Nullable nullable_nullable,
                int *_Nullable *_Nonnull nullable_nonnull,
                int *_Nonnull *_Nullable nonnull_nullable) {
      *p = nonnull;  // TODO: b/343960612 - FALSE NEGATIVE
      *p = nullable;
      p = unknown_unknown;
      p = unknown_nonnull;
      p = nonnull_unknown;  // TODO: b/343960612 - FALSE NEGATIVE
      p = nonnull_nonnull;  // TODO: b/343960612 - FALSE NEGATIVE
      p = unknown_nullable;
      p = nullable_unknown;
      p = nullable_nullable;
      p = nullable_nonnull;
      p = nonnull_nullable;  // TODO: b/343960612 - FALSE NEGATIVE
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable *_Nullable p, int *_Nonnull nonnull,
                int *_Nullable nullable, int **unknown_unknown,
                int **_Nonnull unknown_nonnull, int *_Nonnull *nonnull_unknown,
                int *_Nonnull *_Nonnull nonnull_nonnull,
                int **_Nullable unknown_nullable,
                int *_Nullable *nullable_unknown,
                int *_Nullable *_Nullable nullable_nullable,
                int *_Nullable *_Nonnull nullable_nonnull,
                int *_Nonnull *_Nullable nonnull_nullable) {
      if (p) {
        *p = nonnull;
        *p = nullable;
      }
      p = unknown_unknown;
      p = unknown_nonnull;
      p = nonnull_unknown;  // TODO: b/343960612 - FALSE NEGATIVE
      p = nonnull_nonnull;  // TODO: b/343960612 - FALSE NEGATIVE
      p = unknown_nullable;
      p = nullable_unknown;
      p = nullable_nullable;
      p = nullable_nonnull;
      p = nonnull_nullable;  // TODO: b/343960612 - FALSE NEGATIVE
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable *_Nonnull p, int *_Nonnull nonnull,
                int *_Nullable nullable, int **unknown_unknown,
                int **_Nonnull unknown_nonnull, int *_Nonnull *nonnull_unknown,
                int *_Nonnull *_Nonnull nonnull_nonnull,
                int **_Nullable unknown_nullable,
                int *_Nullable *nullable_unknown,
                int *_Nullable *_Nullable nullable_nullable,
                int *_Nullable *_Nonnull nullable_nonnull,
                int *_Nonnull *_Nullable nonnull_nullable) {
      *p = nonnull;
      *p = nullable;
      p = unknown_unknown;
      p = unknown_nonnull;
      p = nonnull_unknown;   // TODO: b/343960612 - FALSE NEGATIVE
      p = nonnull_nonnull;   // TODO: b/343960612 - FALSE NEGATIVE
      p = unknown_nullable;  // [[unsafe]]
      p = nullable_unknown;
      p = nullable_nullable;  // [[unsafe]]
      p = nullable_nonnull;
      p = nonnull_nullable;  // [[unsafe]]
    }
  )cc"));

  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nonnull *_Nullable p, int *_Nonnull nonnull,
                int *_Nullable nullable, int **unknown_unknown,
                int **_Nonnull unknown_nonnull, int *_Nonnull *nonnull_unknown,
                int *_Nonnull *_Nonnull nonnull_nonnull,
                int **_Nullable unknown_nullable,
                int *_Nullable *nullable_unknown,
                int *_Nullable *_Nullable nullable_nullable,
                int *_Nullable *_Nonnull nullable_nonnull,
                int *_Nonnull *_Nullable nonnull_nullable) {
      if (p) {
        *p = nonnull;
        *p = nullable;  // [[unsafe]]
      }
      p = unknown_unknown;
      p = unknown_nonnull;
      p = nonnull_unknown;
      p = nonnull_nonnull;
      p = unknown_nullable;
      p = nullable_unknown;   // TODO: b/343960612 - FALSE NEGATIVE
      p = nullable_nullable;  // TODO: b/343960612 - FALSE NEGATIVE
      p = nullable_nonnull;   // TODO: b/343960612 - FALSE NEGATIVE
      p = nonnull_nullable;
    }
  )cc"));
}

TEST(PointerNullabilityTest, AssignmentFromVariablesInnerConst) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target(int *_Nullable const *_Nullable inner_const_nullable,
                int *_Nonnull const *_Nullable inner_const_nonnull,
                int *_Nonnull nonnull, int *_Nullable nullable,
                int **unknown_unknown, int **_Nonnull unknown_nonnull,
                int *_Nonnull *nonnull_unknown,
                int *_Nonnull *_Nonnull nonnull_nonnull,
                int **_Nullable unknown_nullable,
                int *_Nullable *nullable_unknown,
                int *_Nullable *_Nullable nullable_nullable,
                int *_Nullable *_Nonnull nullable_nonnull,
                int *_Nonnull *_Nullable nonnull_nullable) {
      inner_const_nullable = unknown_unknown;
      inner_const_nullable = unknown_nonnull;
      inner_const_nullable = nonnull_unknown;
      inner_const_nullable = nonnull_nonnull;
      inner_const_nullable = unknown_nullable;
      inner_const_nullable = nullable_unknown;
      inner_const_nullable = nullable_nullable;
      inner_const_nullable = nullable_nonnull;
      inner_const_nullable = nonnull_nullable;
      inner_const_nonnull = unknown_unknown;
      inner_const_nonnull = unknown_nonnull;
      inner_const_nonnull = nonnull_unknown;
      inner_const_nonnull = nonnull_nonnull;
      inner_const_nonnull = unknown_nullable;
      inner_const_nonnull = nullable_unknown;  // TODO: b/343960612 - FALSE
                                               // NEGATIVE
      inner_const_nonnull =
          nullable_nullable;  // TODO: b/343960612 - FALSE NEGATIVE
      inner_const_nonnull = nullable_nonnull;  // TODO: b/343960612 - FALSE
                                               // NEGATIVE
      inner_const_nonnull = nonnull_nullable;
    }
  )cc"));
}

TEST(PointerNullabilityTest, Initialization) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target() {
      int **_Nonnull a = nullptr;  // [[unsafe]]
      int **_Nullable b = nullptr;

      int integer;
      int *_Nonnull nonnull = &integer;
      int *_Nullable nullable = &integer;

      int **_Nonnull c = &nonnull;
      int **_Nullable d = &nonnull;

      int **_Nonnull e = &nullable;
      int **_Nullable f = &nullable;

      int *_Nonnull *g = &nullable;  // TODO: b/343960612 - FALSE NEGATIVE
      int *_Nullable *h = &nullable;
      nullable = nullptr;
      int *_Nonnull *i = &nullable;  // TODO: b/343960612 - FALSE NEGATIVE
      int *_Nullable *j = &nullable;
    }
  )cc"));
}
}  // namespace
}  // namespace clang::tidy::nullability
