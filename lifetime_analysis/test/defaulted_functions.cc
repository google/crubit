// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests that defaulted functions are analyzed correctly.

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "lifetime_analysis/test/lifetime_analysis_test.h"

namespace clang {
namespace tidy {
namespace lifetimes {
namespace {

class DefaultedFunctions : public LifetimeAnalysisTest {};

TEST_F(DefaultedFunctions, DefaultConstrutor_NoRecordTypeFieldsNoBases) {
  GetLifetimesOptions options;
  options.include_implicit_methods = true;
  EXPECT_THAT(GetLifetimes(R"(
    struct S {
      int i;
    };
    void target() {
      S();
    }

  )",
                           options),
              // Test is successful if we can call the default constructor.
              LifetimesAre({{"S::S", "a:"}, {"target", ""}}));
}

TEST_F(DefaultedFunctions, DefaultConstrutor_LifetimeParam) {
  GetLifetimesOptions options;
  options.include_implicit_methods = true;
  EXPECT_THAT(GetLifetimes(R"(
    struct [[clang::annotate("lifetime_params", "a")]] S {
      [[clang::annotate("member_lifetimes", "a")]]
      int* p;
    };
    void target() {
      S();
    }

  )",
                           options),
              LifetimesAre({{"S::S", "(a, b):"}, {"target", ""}}));
}

TEST_F(DefaultedFunctions, DefaultConstrutor_RecordTypeFields) {
  EXPECT_THAT(
      GetLifetimes(R"(
    struct S {};
    struct T {
      S s;
    };
    void f() {
      T();
    }
  )"),
      // TODO(b/230693710): This documents that defaulted default
      // constructors on classes with record-type fields are currently
      // not supported.
      LifetimesAre({{"T::T", "ERROR: unsupported type of defaulted function"},
                    {"f", "ERROR: No lifetimes for constructor T"}}));
}

TEST_F(DefaultedFunctions, DefaultConstrutor_BaseClass) {
  EXPECT_THAT(
      GetLifetimes(R"(
    struct S {};
    struct T : public S {};
    void f() {
      T();
    }
  )"),
      // TODO(b/230693710): This documents that defaulted default
      // constructors on derived classes are currently not supported.
      LifetimesAre({{"T::T", "ERROR: unsupported type of defaulted function"},
                    {"f", "ERROR: No lifetimes for constructor T"}}));
}

}  // namespace
}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
