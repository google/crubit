// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_annotations/lifetime_annotations.h"

#include <string>
#include <utility>

#include "devtools/cymbal/data_flow/testing_support.h"
#include "lifetime_annotations/test/named_func_lifetimes.h"
#include "lifetime_annotations/test/run_on_code.h"
#include "testing/base/public/gunit.h"
#include "third_party/llvm/llvm-project/clang/include/clang/ASTMatchers/ASTMatchFinder.h"
#include "third_party/llvm/llvm-project/clang/include/clang/ASTMatchers/ASTMatchers.h"

namespace devtools_rust {
namespace {

using testing::StartsWith;
using testing::status::IsOkAndHolds;
using testing::status::StatusIs;

std::string QualifiedName(const clang::FunctionDecl* func) {
  std::string str;
  llvm::raw_string_ostream ostream(str);
  func->printQualifiedName(ostream);
  ostream.flush();
  return str;
}

class LifetimeAnnotationsTest : public testing::Test {
 protected:
  absl::StatusOr<NamedFuncLifetimes> GetNamedLifetimeAnnotations(
      absl::string_view code,
      const clang::tooling::FileContentMappings& file_contents =
          clang::tooling::FileContentMappings()) {
    absl::StatusOr<NamedFuncLifetimes> result;
    runOnCodeWithLifetimeHandlers(
        code,
        [&result](clang::ASTContext& ast_context,
                  const LifetimeAnnotationContext& lifetime_context) {
          using clang::ast_matchers::findAll;
          using clang::ast_matchers::functionDecl;
          using clang::ast_matchers::match;

          NamedFuncLifetimes named_func_lifetimes;
          for (const auto& node :
               match(findAll(functionDecl().bind("func")), ast_context)) {
            if (const auto* func =
                    node.getNodeAs<clang::FunctionDecl>("func")) {
              llvm::Expected<FunctionLifetimes> func_lifetimes =
                  GetLifetimeAnnotations(func, lifetime_context);

              if (!func_lifetimes) {
                result = absl::UnknownError(
                    llvm::toString(func_lifetimes.takeError()));
                return;
              }
              named_func_lifetimes.Add(QualifiedName(func),
                                       NameLifetimes(*func_lifetimes));
            }
          }

          result = std::move(named_func_lifetimes);
        },
        {}, file_contents);

    return result;
  }
};

TEST_F(LifetimeAnnotationsTest, NoLifetimes) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        int f(int);
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "()"}})));
}

TEST_F(LifetimeAnnotationsTest, Failure_NoAnnotationsNoLifetimeElision) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        int** f(int*);
  )"),
              StatusIs(absl::StatusCode::kUnknown,
                       StartsWith("Lifetime elision not enabled")));
}

TEST_F(LifetimeAnnotationsTest, Failure_NoAnnotationsElisionPragmaInWrongFile) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        #include "header.h"
  )",
                                          {std::make_pair("header.h", R"(
        int** f(int*);
  )")}),
              StatusIs(absl::StatusCode::kUnknown,
                       StartsWith("Lifetime elision not enabled")));
}

TEST_F(LifetimeAnnotationsTest, LifetimeElision_OneInputLifetime) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        int** f(int*);
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "a -> (a, a)"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeElision_NoOutputLifetimes) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        void f(int**, int *);
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "(a, b), c"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeElision_Templates) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        template <class T> class vector {};
        int* f(vector<int *>);
        vector<int*> g(int *);
  )"),
              IsOkAndHolds(LifetimesAre({{"f", "a -> a"}, {"g", "a -> a"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeElision_Method) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        struct S {
          int** method(int *, int *);
        };
  )"),
              IsOkAndHolds(LifetimesAre({{"S::method", "c: a, b -> (c, c)"}})));
}

TEST_F(LifetimeAnnotationsTest, LifetimeElision_FailureTooFewInputLifetimes) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        int* f();
  )"),
              StatusIs(absl::StatusCode::kUnknown,
                       StartsWith("Cannot determine output lifetimes")));
}

TEST_F(LifetimeAnnotationsTest, LifetimeElision_FailureTooManyInputLifetimes) {
  EXPECT_THAT(GetNamedLifetimeAnnotations(R"(
        #pragma clang lifetime_elision
        int* f(int**);
  )"),
              StatusIs(absl::StatusCode::kUnknown,
                       StartsWith("Cannot determine output lifetimes")));
}

}  // namespace
}  // namespace devtools_rust
