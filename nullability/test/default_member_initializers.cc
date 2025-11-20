// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for default member initializers for class members with nullability
// annotations.

#include "nullability/pointer_nullability_diagnosis.h"
#include "nullability/test/check_diagnostics.h"
#include "external/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"
#include "external/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

using ::testing::AllOf;
using ::testing::Each;
using ::testing::Field;
using ::testing::Optional;

TEST(PointerNullabilityTest,
     WarnsWhenNonNullStructMemberDefaultInitializedToNull) {
  EXPECT_THAT(
      checkAndGetDiagnostics(R"cc(
        struct S {
          int* _Nonnull target /* [[unsafe]] */ = nullptr;
        };
      )cc"),
      Optional(Each(
          AllOf(Field("Code", &PointerNullabilityDiagnostic::Code,
                      PointerNullabilityDiagnostic::ErrorCode::ExpectedNonnull),
                Field("Ctx", &PointerNullabilityDiagnostic::Ctx,
                      PointerNullabilityDiagnostic::Context::Initializer),
                Field("NoteMessage", &PointerNullabilityDiagnostic::NoteMessage,
                      "consider declaring the aggregate class member with "
                      "ABSL_REQUIRE_EXPLICIT_INIT")))));
}

// TODO: b/376638797 - Warn on initializer for `target` below.
TEST(PointerNullabilityTest,
     WarnsWhenNonNullStructMemberDefaultInitializedToNullable) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct S {
      int* _Nullable other = nullptr;
      int* _Nonnull target = other;
    };
  )cc"));
}

TEST(PointerNullabilityTest,
     NoWarningWhenNullableStructMemberDefaultInitializedToNull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    struct S {
      int* _Nullable target = nullptr;
    };
  )cc"));
}

TEST(PointerNullabilityTest,
     WarnsWhenNonNullClassMemberDefaultInitializedToNull) {
  EXPECT_THAT(
      checkAndGetDiagnostics(R"cc(
        class C {
          int* _Nonnull target /* [[unsafe]] */ = nullptr;
        };
      )cc"),
      Optional(Each(
          AllOf(Field("Code", &PointerNullabilityDiagnostic::Code,
                      PointerNullabilityDiagnostic::ErrorCode::ExpectedNonnull),
                Field("Ctx", &PointerNullabilityDiagnostic::Ctx,
                      PointerNullabilityDiagnostic::Context::Initializer),
                Field("NoteMessage", &PointerNullabilityDiagnostic::NoteMessage,
                      "")))));
}

// TODO: b/376638797 - Warn on initializer for `target` below.
TEST(PointerNullabilityTest,
     WarnsWhenNonNullClassMemberDefaultInitializedToNullable) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    class C {
      int* _Nullable other = nullptr;
      int* _Nonnull target = other;
    };
  )cc"));
}

TEST(PointerNullabilityTest,
     NoWarningWhenNullableClassMemberDefaultInitializedToNull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    class C {
      int* _Nullable target = nullptr;
    };
  )cc"));
}

TEST(PointerNullabilityTest,
     NoWarningWhenNonNullUnionMemberDefaultInitializedToNull) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    union U {
      int* _Nonnull target = nullptr;
    };
  )cc"));
}

}  // namespace
}  // namespace clang::tidy::nullability
