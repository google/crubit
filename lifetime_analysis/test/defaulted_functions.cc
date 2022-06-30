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
  // To check that we synthesize defaulted default constructors correctly,
  // we create a class `T` with a field of a type whose default constructor
  // takes a `this` pointer with static lifetime. We verify that this causes
  // the default constructor of `T` to also take a `this` pointer with static
  // lifetime.

  GetLifetimesOptions options;
  options.include_implicit_methods = true;
  EXPECT_THAT(
      GetLifetimes(R"(
    struct S {
      S() {
        static S* last_constructed = nullptr;
        last_constructed = this;
      }
    };
    struct T {
      S s;
    };
    void f() {
      static T t;
    }
  )",
                   options),
      LifetimesAre({{"S::S", "static:"}, {"T::T", "static:"}, {"f", ""}}));
}

TEST_F(DefaultedFunctions, DefaultConstrutor_BaseClass) {
  // See DefaultConstrutor_RecordTypeField for an exaplanation of hwo this
  // test works.

  GetLifetimesOptions options;
  options.include_implicit_methods = true;
  EXPECT_THAT(
      GetLifetimes(R"(
    struct S {
      S() {
        static S* last_constructed = nullptr;
        last_constructed = this;
      }
    };
    struct T : public S {};
    void f() {
      static T t;
    }
  )",
                   options),
      LifetimesAre({{"S::S", "static:"}, {"T::T", "static:"}, {"f", ""}}));
}

}  // namespace
}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
