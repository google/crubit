// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "common/annotation_reader.h"

#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "common/check.h"
#include "common/status.h"
#include "common/status_test_matchers.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Attr.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/TypeBase.h"
#include "clang/Basic/Diagnostic.h"
#include "clang/Frontend/ASTUnit.h"
#include "clang/Tooling/Tooling.h"
#include "llvm/ADT/ArrayRef.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/Error.h"

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

// Parses `code` as C++20 and returns the resulting `ASTUnit`, crashing if
// parsing fails or emits errors.
std::unique_ptr<clang::ASTUnit> BuildAst(llvm::StringRef code) {
  std::unique_ptr<clang::ASTUnit> ast =
      clang::tooling::buildASTFromCodeWithArgs(
          code, {"-std=c++20", "-target", "x86_64-unknown-linux-gnu"});
  CRUBIT_CHECK(ast != nullptr);
  CRUBIT_CHECK(!ast->getDiagnostics().hasErrorOccurred());
  return ast;
}

template <class T>
T& LookupDecl(clang::ASTContext& context, llvm::StringRef name) {
  clang::DeclContextLookupResult result =
      context.getTranslationUnitDecl()->lookup(&context.Idents.get(name));
  CRUBIT_CHECK(result.isSingleResult());
  return *cast<T>(result.front());
}

// Returns a matcher that can be used with `EXPECT_THAT` to check the result of
// `GetExprAsBool` for a given matcher.
inline auto MakeExprAsBoolMatcher(const clang::ASTContext& ast_context) {
  return [&ast_context](auto matcher) {
    return ResultOf(
        [&ast_context](const clang::Expr& expr) {
          return GetExprAsBool(expr, ast_context);
        },
        matcher);
  };
}

TEST(AnnotationReaderTest, GetAnnotateAttrArgsSuccess) {
  std::unique_ptr<clang::ASTUnit> ast = BuildAst(R"cc(
    [[clang::annotate("foo")]] int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast->getASTContext(), "i");

  EXPECT_THAT(GetAnnotateAttrArgs(var, "foo"),
              IsOkAndHolds(Optional(ElementsAre())));
  EXPECT_THAT(GetAnnotateAttrArgs(var, "bar"), IsOkAndHolds(std::nullopt));
}

TEST(AnnotationReaderTest, GetAnnotateAttrArgsFailureDoubleAnnotation) {
  std::unique_ptr<clang::ASTUnit> ast = BuildAst(R"cc(
    [[clang::annotate("foo")]] [[clang::annotate("foo")]] int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast->getASTContext(), "i");

  EXPECT_THAT(
      GetAnnotateAttrArgs(var, "foo"),
      StatusIs(
          crubit::StatusCode::kInvalidArgument,
          HasSubstr(
              "Only one `foo` annotation may be placed on a declaration.")));
}

TEST(AnnotationReaderTest, GetAnnotateAttrFailureArgNotIntegralOrString) {
  std::unique_ptr<clang::ASTUnit> ast = BuildAst(R"cc(
    [[clang::annotate("foo", 1.0)]] extern int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast->getASTContext(), "i");

  ASSERT_THAT(
      GetAnnotateAttrArgs(var, "foo"),
      StatusIs(crubit::StatusCode::kInvalidArgument,
               HasSubstr("Arguments of `foo` annotation must be of integral "
                         "type, string literal, or constructor")));
}

TEST(AnnotationReaderTest, GetAnnotateAttrSuccessConsistentAnnotations) {
  std::unique_ptr<clang::ASTUnit> ast = BuildAst(R"cc(
    [[clang::annotate("foo", "arg1", 1)]] extern int i;
    [[clang::annotate("foo", "arg1", 1)]] extern int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast->getASTContext(), "i");

  ASSERT_THAT(GetAnnotateAttrArgs(var, "foo"),
              IsOkAndHolds(Optional(ElementsAre(NotNull(), NotNull()))));
}

TEST(AnnotationReaderTest, GetAnnotateAttrFailureConflictingIntArgs) {
  std::unique_ptr<clang::ASTUnit> ast = BuildAst(R"cc(
    [[clang::annotate("foo", 1)]] extern int i;
    [[clang::annotate("foo", 2)]] extern int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast->getASTContext(), "i");

  ASSERT_THAT(
      GetAnnotateAttrArgs(var, "foo"),
      StatusIs(
          crubit::StatusCode::kInvalidArgument,
          HasSubstr(
              "Different declarations have inconsistent `foo` annotations.")));
}

TEST(AnnotationReaderTest, GetAnnotateAttrFailureConflictingStringArgs) {
  std::unique_ptr<clang::ASTUnit> ast = BuildAst(R"cc(
    [[clang::annotate("foo", "1")]] extern int i;
    [[clang::annotate("foo", "2")]] extern int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast->getASTContext(), "i");

  ASSERT_THAT(
      GetAnnotateAttrArgs(var, "foo"),
      StatusIs(
          crubit::StatusCode::kInvalidArgument,
          HasSubstr(
              "Different declarations have inconsistent `foo` annotations.")));
}

TEST(AnnotationReaderTest, GetAnnotateAttrFailureConflictingArgCounts) {
  std::unique_ptr<clang::ASTUnit> ast = BuildAst(R"cc(
    [[clang::annotate("foo")]] extern int i;
    [[clang::annotate("foo", 1)]] extern int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast->getASTContext(), "i");

  ASSERT_THAT(
      GetAnnotateAttrArgs(var, "foo"),
      StatusIs(
          crubit::StatusCode::kInvalidArgument,
          HasSubstr(
              "Different declarations have inconsistent `foo` annotations.")));
}

TEST(AnnotationReaderTest,
     GetAnnotateAttrSuccessAnnotationMissingFromDefinition) {
  std::unique_ptr<clang::ASTUnit> ast = BuildAst(R"cc(
    [[clang::annotate("foo")]] extern int i;
    int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast->getASTContext(), "i");

  ASSERT_THAT(GetAnnotateAttrArgs(var, "foo"),
              IsOkAndHolds(Optional(ElementsAre())));
}

TEST(AnnotationReaderTest,
     GetAnnotateAttrSuccessAnnotationMissingFromForwardDeclaration) {
  std::unique_ptr<clang::ASTUnit> ast = BuildAst(R"cc(
    extern int i;
    [[clang::annotate("foo")]] int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast->getASTContext(), "i");

  ASSERT_THAT(GetAnnotateAttrArgs(var, "foo"),
              IsOkAndHolds(Optional(ElementsAre())));
}

TEST(AnnotationReaderTest, GetAnnotationWithStringArgsSuccess) {
  std::unique_ptr<clang::ASTUnit> ast = BuildAst(R"cc(
    [[clang::annotate("foo", "arg1", "arg2")]] int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast->getASTContext(), "i");

  EXPECT_THAT(GetAnnotationWithStringArgs(var, "foo"),
              IsOkAndHolds(Optional(ElementsAre("arg1", "arg2"))));
}

TEST(AnnotationReaderTest, GetAnnotationWithStringArgsNone) {
  std::unique_ptr<clang::ASTUnit> ast = BuildAst(R"cc(
    int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast->getASTContext(), "i");

  EXPECT_THAT(GetAnnotationWithStringArgs(var, "foo"),
              IsOkAndHolds(std::nullopt));
}

TEST(AnnotationReaderTest, GetAnnotationWithStringArgsFailureNonString) {
  std::unique_ptr<clang::ASTUnit> ast = BuildAst(R"cc(
    [[clang::annotate("foo", "arg1", 42)]] int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast->getASTContext(), "i");

  EXPECT_THAT(
      GetAnnotationWithStringArgs(var, "foo"),
      StatusIs(crubit::StatusCode::kInvalidArgument,
               HasSubstr("Annotation foo arguments must be string literals.")));
}

TEST(AnnotationReaderTest, GetAnnotationWithStringArgSuccess) {
  std::unique_ptr<clang::ASTUnit> ast = BuildAst(R"cc(
    [[clang::annotate("foo", "bar")]] int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast->getASTContext(), "i");

  EXPECT_THAT(GetAnnotationWithStringArg(var, "foo"),
              IsOkAndHolds(Optional(Eq("bar"))));
  EXPECT_THAT(GetAnnotationWithStringArg(var, "missing"),
              IsOkAndHolds(std::nullopt));
}

TEST(AnnotationReaderTest, GetAnnotationWithStringArgFailureNoArgs) {
  std::unique_ptr<clang::ASTUnit> ast = BuildAst(R"cc(
    [[clang::annotate("foo")]] int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast->getASTContext(), "i");

  EXPECT_THAT(
      GetAnnotationWithStringArg(var, "foo"),
      StatusIs(
          crubit::StatusCode::kInvalidArgument,
          HasSubstr("Annotation foo must have a single string argument.")));
}

TEST(AnnotationReaderTest, GetAnnotationWithStringArgFailureMultipleArgs) {
  std::unique_ptr<clang::ASTUnit> ast = BuildAst(R"cc(
    [[clang::annotate("foo", "a", "b")]] int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast->getASTContext(), "i");

  EXPECT_THAT(
      GetAnnotationWithStringArg(var, "foo"),
      StatusIs(
          crubit::StatusCode::kInvalidArgument,
          HasSubstr("Annotation foo must have a single string argument.")));
}

TEST(AnnotationReaderTest, GetAnnotationWithStringArgFailureNonStringArg) {
  std::unique_ptr<clang::ASTUnit> ast = BuildAst(R"cc(
    [[clang::annotate("foo", 123)]] int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast->getASTContext(), "i");

  EXPECT_THAT(
      GetAnnotationWithStringArg(var, "foo"),
      StatusIs(crubit::StatusCode::kInvalidArgument,
               HasSubstr("Annotation foo arguments must be string literals.")));
}

TEST(AnnotationReaderTest, HasAnnotationWithoutArgsCases) {
  std::unique_ptr<clang::ASTUnit> ast = BuildAst(R"cc(
    [[clang::annotate("no_args")]] [[clang::annotate("with_args", 1)]] int i;
    int j;
  )cc");

  auto& var_i = LookupDecl<clang::VarDecl>(ast->getASTContext(), "i");
  auto& var_j = LookupDecl<clang::VarDecl>(ast->getASTContext(), "j");

  EXPECT_THAT(HasAnnotationWithoutArgs(var_i, "no_args"), IsOkAndHolds(true));
  EXPECT_THAT(HasAnnotationWithoutArgs(var_j, "no_args"), IsOkAndHolds(false));
  EXPECT_THAT(
      HasAnnotationWithoutArgs(var_i, "with_args"),
      StatusIs(crubit::StatusCode::kInvalidArgument,
               HasSubstr("Annotation with_args does not expect arguments.")));
}

TEST(AnnotationReaderTest, GetExprAsBoolSuccess) {
  std::unique_ptr<clang::ASTUnit> ast = BuildAst(R"cc(
    [[clang::annotate("foo", true, false)]] int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast->getASTContext(), "i");
  auto expr_as_bool = MakeExprAsBoolMatcher(ast->getASTContext());

  EXPECT_THAT(GetAnnotateAttrArgs(var, "foo"),
              IsOkAndHolds(Optional(
                  ElementsAre(Pointee(expr_as_bool(IsOkAndHolds(true))),
                              Pointee(expr_as_bool(IsOkAndHolds(false)))))));
}

TEST(AnnotationReaderTest, GetExprAsBoolRejectsNonBooleanTypes) {
  std::unique_ptr<clang::ASTUnit> ast = BuildAst(R"cc(
    [[clang::annotate("foo", 1, 0, 'a', 1.5)]] int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast->getASTContext(), "i");
  auto* attr = var.getAttr<clang::AnnotateAttr>();
  ASSERT_THAT(attr, NotNull());

  auto expr_as_bool = MakeExprAsBoolMatcher(ast->getASTContext());

  EXPECT_THAT(
      llvm::ArrayRef(attr->args_begin(), attr->args_end()),
      AllOf(
          SizeIs(4),
          Each(Pointee(expr_as_bool(StatusIs(
              crubit::StatusCode::kInvalidArgument,
              HasSubstr("annotation expression must evaluate to a bool")))))));
}

TEST(AnnotationReaderTest, ConsistentAnnotationsWithTypedefType) {
  std::unique_ptr<clang::ASTUnit> ast = BuildAst(R"cc(
    using MyInt = int;
    [[clang::annotate("foo", (MyInt)1)]] extern int i;
    [[clang::annotate("foo", (int)1)]] extern int i;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast->getASTContext(), "i");
  EXPECT_THAT(GetAnnotateAttrArgs(var, "foo"),
              IsOkAndHolds(Optional(ElementsAre(NotNull()))));
}

TEST(AnnotationReaderTest, GetTypeAnnotationSingleDeclBasic) {
  std::unique_ptr<clang::ASTUnit> ast = BuildAst(R"cc(
    int [[clang::annotate_type("my_type_annot")]] x;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast->getASTContext(), "x");
  const clang::Type* type = var.getType().getTypePtr();

  EXPECT_THAT(GetTypeAnnotationSingleDecl(type, "my_type_annot"),
              IsOkAndHolds(Pointee(Property(
                  &clang::AnnotateTypeAttr::getAnnotation, "my_type_annot"))));
  EXPECT_THAT(GetTypeAnnotationSingleDecl(type, "nonexistent"),
              IsOkAndHolds(nullptr));
}

TEST(AnnotationReaderTest,
     GetTypeAnnotationSingleDeclMultipleAnnotationsOnType) {
  std::unique_ptr<clang::ASTUnit> ast = BuildAst(R"cc(
    int [[clang::annotate_type("inner")]] [[clang::annotate_type("outer")]] x;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast->getASTContext(), "x");
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
  std::unique_ptr<clang::ASTUnit> ast = BuildAst(R"cc(
    int [[clang::annotate_type("duplicate")]] [[clang::annotate_type(
        "duplicate")]] x;
  )cc");

  auto& var = LookupDecl<clang::VarDecl>(ast->getASTContext(), "x");
  const clang::Type* type = var.getType().getTypePtr();

  EXPECT_THAT(
      GetTypeAnnotationSingleDecl(type, "duplicate"),
      StatusIs(
          crubit::StatusCode::kInvalidArgument,
          HasSubstr(
              "Only one `duplicate` annotation may be placed on a type.")));
}

TEST(AnnotationReaderTest, AnnotationWithNonStringLiteralArg) {
  std::unique_ptr<clang::ASTUnit> ast = BuildAst(R"cc(
    [[clang::annotate("my_annot", 123)]] void foo();
  )cc");
  auto& decl = LookupDecl<clang::FunctionDecl>(ast->getASTContext(), "foo");

  llvm::Expected<std::optional<std::vector<std::string>>> result =
      GetAnnotationWithStringArgs(decl, "my_annot");
  EXPECT_FALSE(result);
  llvm::consumeError(result.takeError());
}

}  // namespace
}  // namespace crubit
