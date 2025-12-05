// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for default member initializers for class members with nullability
// annotations.

#include "nullability/test/check_diagnostics.h"
#include "external/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

TEST(PointerNullabilityTest,
     WarnsWhenNonNullStructMemberDefaultInitializerIsNull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct S {
      int* _Nonnull target /* [[unsafe]] */ = nullptr;
    };
  )cc"));
}

// TODO: b/376638797 - Warn on initializer for `target` below.
TEST(PointerNullabilityTest,
     WarnsWhenNonNullStructMemberDefaultInitializerIsNullable) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* _Nullable f();
    struct S {
      int* _Nonnull target = f();
    };
  )cc"));
}

// TODO: b/376638797 - Warn on initializer for `target` below.
TEST(PointerNullabilityTest,
     WarnsWhenNonNullStructMemberDefaultInitializerIsNullableFromOtherMember) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct S {
      int* _Nullable other = nullptr;
      int* _Nonnull target = other;
    };
  )cc"));
}

TEST(PointerNullabilityTest,
     NoWarningWhenNullableStructMemberDefaultInitializerIsNull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct S {
      int* _Nullable target = nullptr;
    };
  )cc"));
}

TEST(PointerNullabilityTest,
     WarnsWhenNonNullClassMemberDefaultInitializerIsNull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    class C {
      int* _Nonnull target /* [[unsafe]] */ = nullptr;
    };
  )cc"));
}

// TODO: b/376638797 - Warn on initializer for `target` below.
TEST(PointerNullabilityTest,
     WarnsWhenNonNullClassMemberDefaultInitializerIsNullable) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    int* _Nullable f();
    class C {
      int* _Nonnull target = f();
    };
  )cc"));
}

// TODO: b/376638797 - Warn on initializer for `target` below.
TEST(PointerNullabilityTest,
     WarnsWhenNonNullClassMemberDefaultInitializerIsNullableFromOtherMember) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    class C {
      int* _Nullable other = nullptr;
      int* _Nonnull target = other;
    };
  )cc"));
}

TEST(PointerNullabilityTest,
     NoWarningWhenNullableClassMemberDefaultInitializerIsNull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    class C {
      int* _Nullable target = nullptr;
    };
  )cc"));
}

TEST(PointerNullabilityTest,
     NoWarningWhenNonNullUnionMemberDefaultInitializerIsNull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    union U {
      int* _Nonnull target = nullptr;
    };
  )cc"));
}

}  // namespace
}  // namespace clang::tidy::nullability
