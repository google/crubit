// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "common/annotation_reader.h"

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "absl/strings/string_view.h"
#include "common/status_test_matchers.h"
#include "clang/Testing/TestAST.h"

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

}  // namespace
}  // namespace crubit
