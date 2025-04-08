// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "common/annotation_reader.h"

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "absl/strings/string_view.h"
#include "common/status_test_matchers.h"
#include "clang/include/clang/Testing/TestAST.h"

namespace crubit {
namespace {

using testing::Eq;
using testing::HasSubstr;
using testing::Ne;

template <class T>
T& LookupDecl(clang::ASTContext& context, absl::string_view name) {
  clang::DeclContextLookupResult result =
      context.getTranslationUnitDecl()->lookup(&context.Idents.get(name));
  CHECK(result.isSingleResult());
  return *cast<T>(result.front());
}

TEST(GetAnnotateAttrArgsTest, Success) {
  clang::TestAST ast(R"cc(
    [[clang::annotate("foo")]] int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast.context(), "i");

  EXPECT_THAT(GetAnnotateAttrArgs(var, "foo"), IsOkAndHolds(Ne(std::nullopt)));
  EXPECT_THAT(GetAnnotateAttrArgs(var, "bar"), IsOkAndHolds(Eq(std::nullopt)));
}

TEST(GetAnnotateAttrArgsTest, FailureDoubleAnnotation) {
  clang::TestAST ast(R"cc(
    [[clang::annotate("foo")]] [[clang::annotate("foo")]] int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast.context(), "i");

  EXPECT_THAT(
      GetAnnotateAttrArgs(var, "foo"),
      StatusIs(
          absl::StatusCode::kInvalidArgument,
          HasSubstr(
              "Only one `foo` annotation may be placed on a declaration.")));
}

TEST(AnnotationReaderTest, GetAnnotateAttrFailureArgNotIntegralOrString) {
  clang::TestAST ast(R"cc(
    [[clang::annotate("foo", 1.0)]] extern int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast.context(), "i");

  ASSERT_THAT(GetAnnotateAttrArgs(var, "foo"),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       HasSubstr("Arguments of `foo` annotation must be of "
                                 "integral type or string literals")));
}

TEST(AnnotationReaderTest, GetAnnotateAttrSuccessConsistentAnnotations) {
  clang::TestAST ast(R"cc(
    [[clang::annotate("foo", "arg1", 1)]] extern int i;
    [[clang::annotate("foo", "arg1", 1)]] extern int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast.context(), "i");

  ASSERT_THAT(GetAnnotateAttrArgs(var, "foo"), IsOkAndHolds(Ne(std::nullopt)));
}

TEST(AnnotationReaderTest, GetAnnotateAttrFailureConflictingIntArgs) {
  clang::TestAST ast(R"cc(
    [[clang::annotate("foo", 1)]] extern int i;
    [[clang::annotate("foo", 2)]] extern int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast.context(), "i");

  ASSERT_THAT(
      GetAnnotateAttrArgs(var, "foo"),
      StatusIs(
          absl::StatusCode::kInvalidArgument,
          HasSubstr(
              "Different declarations have inconsistent `foo` annotations.")));
}

TEST(AnnotationReaderTest, GetAnnotateAttrFailureConflictingStringArgs) {
  clang::TestAST ast(R"cc(
    [[clang::annotate("foo", "1")]] extern int i;
    [[clang::annotate("foo", "2")]] extern int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast.context(), "i");

  ASSERT_THAT(
      GetAnnotateAttrArgs(var, "foo"),
      StatusIs(
          absl::StatusCode::kInvalidArgument,
          HasSubstr(
              "Different declarations have inconsistent `foo` annotations.")));
}

TEST(AnnotationReaderTest, GetAnnotateAttrFailureConflictingArgCounts) {
  clang::TestAST ast(R"cc(
    [[clang::annotate("foo")]] extern int i;
    [[clang::annotate("foo", 1)]] extern int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast.context(), "i");

  ASSERT_THAT(
      GetAnnotateAttrArgs(var, "foo"),
      StatusIs(
          absl::StatusCode::kInvalidArgument,
          HasSubstr(
              "Different declarations have inconsistent `foo` annotations.")));
}

TEST(AnnotationReaderTest,
     GetAnnotateAttrSuccessAnnotationMissingFromDefinition) {
  clang::TestAST ast(R"cc(
    [[clang::annotate("foo")]] extern int i;
    int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast.context(), "i");

  ASSERT_THAT(GetAnnotateAttrArgs(var, "foo"), IsOkAndHolds(Ne(std::nullopt)));
}

TEST(AnnotationReaderTest,
     GetAnnotateAttrSuccessAnnotationMissingFromForwardDeclaration) {
  clang::TestAST ast(R"cc(
    extern int i;
    [[clang::annotate("foo")]] int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast.context(), "i");

  ASSERT_THAT(GetAnnotateAttrArgs(var, "foo"), IsOkAndHolds(Ne(std::nullopt)));
}

}  // namespace
}  // namespace crubit
