// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "common/annotation_reader.h"

#include <optional>
#include <string>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "absl/log/check.h"
#include "absl/status/status.h"
#include "absl/strings/string_view.h"
#include "absl/types/span.h"
#include "common/status_test_matchers.h"
#include "common/string_view_conversion.h"
#include "clang/AST/Attr.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/TypeBase.h"
#include "clang/Testing/TestAST.h"

namespace crubit {
namespace {

using ::testing::AllOf;
using ::testing::Each;
using ::testing::ElementsAre;
using ::testing::Eq;
using ::testing::HasSubstr;
using ::testing::NotNull;
using ::testing::Optional;
using ::testing::Pointee;
using ::testing::Property;
using ::testing::ResultOf;
using ::testing::SizeIs;

template <class T>
T& LookupDecl(clang::ASTContext& context, absl::string_view name) {
  clang::DeclContextLookupResult result =
      context.getTranslationUnitDecl()->lookup(
          &context.Idents.get(StringRefFromStringView(name)));
  CHECK(result.isSingleResult());
  return *cast<T>(result.front());
}

TEST(AnnotationReaderTest, GetAnnotateAttrArgsSuccess) {
  clang::TestAST ast(R"cc(
    [[clang::annotate("foo")]] int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast.context(), "i");

  EXPECT_THAT(GetAnnotateAttrArgs(var, "foo"),
              IsOkAndHolds(Optional(ElementsAre())));
  EXPECT_THAT(GetAnnotateAttrArgs(var, "bar"), IsOkAndHolds(std::nullopt));
}

TEST(AnnotationReaderTest, GetAnnotateAttrArgsFailureDoubleAnnotation) {
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

  ASSERT_THAT(
      GetAnnotateAttrArgs(var, "foo"),
      StatusIs(absl::StatusCode::kInvalidArgument,
               HasSubstr("Arguments of `foo` annotation must be of integral "
                         "type, string literal, or constructor")));
}

TEST(AnnotationReaderTest, GetAnnotateAttrSuccessConsistentAnnotations) {
  clang::TestAST ast(R"cc(
    [[clang::annotate("foo", "arg1", 1)]] extern int i;
    [[clang::annotate("foo", "arg1", 1)]] extern int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast.context(), "i");

  ASSERT_THAT(GetAnnotateAttrArgs(var, "foo"),
              IsOkAndHolds(Optional(ElementsAre(NotNull(), NotNull()))));
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

  ASSERT_THAT(GetAnnotateAttrArgs(var, "foo"),
              IsOkAndHolds(Optional(ElementsAre())));
}

TEST(AnnotationReaderTest,
     GetAnnotateAttrSuccessAnnotationMissingFromForwardDeclaration) {
  clang::TestAST ast(R"cc(
    extern int i;
    [[clang::annotate("foo")]] int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast.context(), "i");

  ASSERT_THAT(GetAnnotateAttrArgs(var, "foo"),
              IsOkAndHolds(Optional(ElementsAre())));
}

TEST(AnnotationReaderTest, GetAnnotationWithStringArgsSuccess) {
  clang::TestAST ast(R"cc(
    [[clang::annotate("foo", "arg1", "arg2")]] int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast.context(), "i");

  EXPECT_THAT(GetAnnotationWithStringArgs(var, "foo"),
              IsOkAndHolds(Optional(ElementsAre("arg1", "arg2"))));
}

TEST(AnnotationReaderTest, GetAnnotationWithStringArgsNone) {
  clang::TestAST ast(R"cc(
    int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast.context(), "i");

  EXPECT_THAT(GetAnnotationWithStringArgs(var, "foo"),
              IsOkAndHolds(std::nullopt));
}

TEST(AnnotationReaderTest, GetAnnotationWithStringArgsFailureNonString) {
  clang::TestAST ast(R"cc(
    [[clang::annotate("foo", "arg1", 42)]] int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast.context(), "i");

  EXPECT_THAT(
      GetAnnotationWithStringArgs(var, "foo"),
      StatusIs(absl::StatusCode::kInvalidArgument,
               HasSubstr("Annotation foo arguments must be string literals.")));
}

TEST(AnnotationReaderTest, GetAnnotationWithStringArgSuccess) {
  clang::TestAST ast(R"cc(
    [[clang::annotate("foo", "bar")]] int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast.context(), "i");

  EXPECT_THAT(GetAnnotationWithStringArg(var, "foo"),
              IsOkAndHolds(Optional(Eq("bar"))));
  EXPECT_THAT(GetAnnotationWithStringArg(var, "missing"),
              IsOkAndHolds(std::nullopt));
}

TEST(AnnotationReaderTest, GetAnnotationWithStringArgFailureNoArgs) {
  clang::TestAST ast(R"cc(
    [[clang::annotate("foo")]] int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast.context(), "i");

  EXPECT_THAT(
      GetAnnotationWithStringArg(var, "foo"),
      StatusIs(
          absl::StatusCode::kInvalidArgument,
          HasSubstr("Annotation foo must have a single string argument.")));
}

TEST(AnnotationReaderTest, GetAnnotationWithStringArgFailureMultipleArgs) {
  clang::TestAST ast(R"cc(
    [[clang::annotate("foo", "a", "b")]] int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast.context(), "i");

  EXPECT_THAT(
      GetAnnotationWithStringArg(var, "foo"),
      StatusIs(
          absl::StatusCode::kInvalidArgument,
          HasSubstr("Annotation foo must have a single string argument.")));
}

TEST(AnnotationReaderTest, GetAnnotationWithStringArgFailureNonStringArg) {
  clang::TestAST ast(R"cc(
    [[clang::annotate("foo", 123)]] int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast.context(), "i");

  EXPECT_THAT(
      GetAnnotationWithStringArg(var, "foo"),
      StatusIs(absl::StatusCode::kInvalidArgument,
               HasSubstr("Annotation foo arguments must be string literals.")));
}

TEST(AnnotationReaderTest, HasAnnotationWithoutArgsCases) {
  clang::TestAST ast(R"cc(
    [[clang::annotate("no_args")]] [[clang::annotate("with_args", 1)]] int i;
    int j;
  )cc");

  auto& var_i = LookupDecl<clang::VarDecl>(ast.context(), "i");
  auto& var_j = LookupDecl<clang::VarDecl>(ast.context(), "j");

  EXPECT_THAT(HasAnnotationWithoutArgs(var_i, "no_args"), IsOkAndHolds(true));
  EXPECT_THAT(HasAnnotationWithoutArgs(var_j, "no_args"), IsOkAndHolds(false));
  EXPECT_THAT(
      HasAnnotationWithoutArgs(var_i, "with_args"),
      StatusIs(absl::StatusCode::kInvalidArgument,
               HasSubstr("Annotation with_args does not expect arguments.")));
}

TEST(AnnotationReaderTest, GetExprAsBoolSuccess) {
  clang::TestAST ast(R"cc(
    [[clang::annotate("foo", true, false)]] int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast.context(), "i");
  auto as_bool = [&](const clang::Expr* expr) {
    return GetExprAsBool(*expr, ast.context());
  };

  EXPECT_THAT(GetAnnotateAttrArgs(var, "foo"),
              IsOkAndHolds(Optional(
                  ElementsAre(ResultOf(as_bool, IsOkAndHolds(true)),
                              ResultOf(as_bool, IsOkAndHolds(false))))));
}

TEST(AnnotationReaderTest, GetExprAsBoolRejectsNonBooleanTypes) {
  clang::TestAST ast(R"cc(
    [[clang::annotate("foo", 1, 0, 'a', 1.5)]] int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast.context(), "i");
  auto* attr = var.getAttr<clang::AnnotateAttr>();
  ASSERT_NE(attr, nullptr);

  auto as_bool = [&](const clang::Expr* expr) {
    return GetExprAsBool(*expr, ast.context());
  };

  EXPECT_THAT(
      absl::MakeSpan(attr->args_begin(), attr->args_end()),
      AllOf(SizeIs(4),
            Each(ResultOf(
                as_bool,
                StatusIs(
                    absl::StatusCode::kInvalidArgument,
                    HasSubstr(
                        "annotation expression must evaluate to a bool"))))));
}

TEST(AnnotationReaderTest, ConsistentAnnotationsWithTypedefType) {
  clang::TestAST ast(R"cc(
    using MyInt = int;
    [[clang::annotate("foo", (MyInt)1)]] extern int i;
    [[clang::annotate("foo", (int)1)]] extern int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast.context(), "i");
  EXPECT_THAT(GetAnnotateAttrArgs(var, "foo"),
              IsOkAndHolds(Optional(ElementsAre(NotNull()))));
}

TEST(AnnotationReaderTest, GetTypeAnnotationSingleDeclBasic) {
  clang::TestAST ast(R"cc(
    int [[clang::annotate_type("my_type_annot")]] x;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast.context(), "x");
  const clang::Type* type = var.getType().getTypePtr();

  EXPECT_THAT(GetTypeAnnotationSingleDecl(type, "my_type_annot"),
              IsOkAndHolds(Pointee(Property(
                  &clang::AnnotateTypeAttr::getAnnotation, "my_type_annot"))));
  EXPECT_THAT(GetTypeAnnotationSingleDecl(type, "nonexistent"),
              IsOkAndHolds(nullptr));
}

TEST(AnnotationReaderTest,
     GetTypeAnnotationSingleDeclMultipleAnnotationsOnType) {
  clang::TestAST ast(R"cc(
    int [[clang::annotate_type("inner")]] [[clang::annotate_type("outer")]] x;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast.context(), "x");
  const clang::Type* type = var.getType().getTypePtr();

  EXPECT_THAT(GetTypeAnnotationSingleDecl(type, "outer"),
              IsOkAndHolds(Pointee(
                  Property(&clang::AnnotateTypeAttr::getAnnotation, "outer"))));
  EXPECT_THAT(GetTypeAnnotationSingleDecl(type, "inner"),
              IsOkAndHolds(Pointee(
                  Property(&clang::AnnotateTypeAttr::getAnnotation, "inner"))));
}

TEST(AnnotationReaderTest,
     GetTypeAnnotationSingleDeclDuplicateAnnotationError) {
  clang::TestAST ast(R"cc(
    int [[clang::annotate_type("duplicate")]] [[clang::annotate_type(
        "duplicate")]] x;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast.context(), "x");
  const clang::Type* type = var.getType().getTypePtr();

  EXPECT_THAT(
      GetTypeAnnotationSingleDecl(type, "duplicate"),
      StatusIs(
          absl::StatusCode::kInvalidArgument,
          HasSubstr(
              "Only one `duplicate` annotation may be placed on a type.")));
}

}  // namespace
}  // namespace crubit
