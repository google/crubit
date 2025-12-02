// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for `checkDiagnostics()` itself.

#include "nullability/test/check_diagnostics.h"

#include <optional>
#include <vector>

#include "nullability/pointer_nullability_diagnosis.h"
#include "external/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"
#include "external/llvm-project/third-party/unittest/googletest/include/gtest/gtest-spi.h"
#include "external/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

using ::testing::AllOf;
using ::testing::ElementsAre;
using ::testing::Field;
using ::testing::IsEmpty;
using ::testing::Optional;

TEST(PointerNullabilityTest, CheckNoDiagnostics) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target() {}
  )cc"));
}

TEST(PointerNullabilityTest, GetNoDiagnostics) {
  EXPECT_THAT(checkAndGetDiagnostics(R"cc(
                void target() {}
              )cc"),
              Optional(IsEmpty()));
}

TEST(PointerNullabilityTest, CheckExpectedDiagnostic) {
  EXPECT_TRUE(checkDiagnostics(R"cc(
    void target() {
      int* p = nullptr;
      *p;  // [[unsafe]]
    }
  )cc"));
}

TEST(PointerNullabilityTest, GetExpectedDiagnostic) {
  EXPECT_THAT(
      checkAndGetDiagnostics(R"cc(
        void target() {
          int* p = nullptr;
          *p;  // [[unsafe]]
        }
      )cc"),
      Optional(ElementsAre(
          AllOf(
              Field("Code", &PointerNullabilityDiagnostic::Code,
                    PointerNullabilityDiagnostic::ErrorCode::ExpectedNonnull),
              Field("Ctx", &PointerNullabilityDiagnostic::Ctx,
                    PointerNullabilityDiagnostic::Context::NullableDereference),
              Field("NoteMessage", &PointerNullabilityDiagnostic::NoteMessage,
                    "")),
          AllOf(
              Field("Code", &PointerNullabilityDiagnostic::Code,
                    PointerNullabilityDiagnostic::ErrorCode::ExpectedNonnull),
              Field("Ctx", &PointerNullabilityDiagnostic::Ctx,
                    PointerNullabilityDiagnostic::Context::NullableDereference),
              Field("NoteMessage", &PointerNullabilityDiagnostic::NoteMessage,
                    "")))));
}

TEST(PointerNullabilityTest, CheckUnexpectedDiagnostic) {
  bool Result = true;
  EXPECT_NONFATAL_FAILURE(Result = checkDiagnostics(R"cc(
                            void target() {
                              1;  // [[unsafe]]
                            }
                          )cc"),
                          "Expected diagnostics but didn't find them");
  EXPECT_EQ(Result, false);
}

TEST(PointerNullabilityTest, GetUnexpectedDiagnostic) {
  std::optional<std::vector<PointerNullabilityDiagnostic>> Result;
  EXPECT_NONFATAL_FAILURE(Result = checkAndGetDiagnostics(R"cc(
                            void target() {
                              1;  // [[unsafe]]
                            }
                          )cc"),
                          "Expected diagnostics but didn't find them");
  EXPECT_EQ(Result, std::nullopt);
}

TEST(PointerNullabilityTest, CheckMissingDiagnostic) {
  bool Result = true;
  EXPECT_NONFATAL_FAILURE(Result = checkDiagnostics(R"cc(
                            void target() {
                              int *p = nullptr;
                              *p;  // Missing diagnostic
                            }
                          )cc"),
                          "Found diagnostics but didn't expect them");
  EXPECT_EQ(Result, false);
}

TEST(PointerNullabilityTest, GetMissingDiagnostic) {
  std::optional<std::vector<PointerNullabilityDiagnostic>> Result;
  EXPECT_NONFATAL_FAILURE(Result = checkAndGetDiagnostics(R"cc(
                            void target() {
                              int* p = nullptr;
                              *p;  // Missing diagnostic
                            }
                          )cc"),
                          "Found diagnostics but didn't expect them");
  EXPECT_EQ(Result, std::nullopt);
}

}  // namespace
}  // namespace clang::tidy::nullability
