// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ast_util.h"

#include <memory>
#include <string>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "absl/status/status.h"
#include "absl/status/status_matchers.h"
#include "absl/status/statusor.h"
#include "absl/strings/str_cat.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclTemplate.h"
#include "clang/AST/Type.h"
#include "clang/Frontend/ASTUnit.h"
#include "clang/Tooling/Tooling.h"
#include "llvm/Support/Casting.h"

namespace {

using ::absl_testing::StatusIs;

constexpr char kStdTemplates[] = R"(
  namespace std {
  inline namespace __u {
  template <typename T> struct char_traits {};
  template <typename T> struct allocator {};
  template <typename T, typename Traits = char_traits<T>>
  struct basic_string_view {};
  }  // namespace __u
  }  // namespace std
  template <typename T> struct char_traits {};
)";

// Returns the class template specialization that is the type of the global
// variable `x` in `ast_unit`, or null if there is none.
const clang::ClassTemplateSpecializationDecl* GetTypeOfX(
    clang::ASTUnit& ast_unit) {
  clang::ASTContext& ctx = ast_unit.getASTContext();
  for (const clang::Decl* decl : ctx.getTranslationUnitDecl()->decls()) {
    const auto* var = llvm::dyn_cast<clang::VarDecl>(decl);
    if (var == nullptr || var->getName() != "x") continue;
    return llvm::dyn_cast_or_null<clang::ClassTemplateSpecializationDecl>(
        var->getType()->getAsCXXRecordDecl());
  }
  return nullptr;
}

// Parses `kStdTemplates` followed by `code`.
std::unique_ptr<clang::ASTUnit> Parse(const std::string& code) {
  return clang::tooling::buildASTFromCodeWithArgs(
      absl::StrCat(kStdTemplates, code), {"-std=c++17"});
}

TEST(ParameterizedByTAndStdTraitTTest, StdStringView) {
  std::unique_ptr<clang::ASTUnit> ast_unit =
      Parse("std::basic_string_view<char, std::char_traits<char>> x;");
  ASSERT_NE(ast_unit, nullptr);
  const clang::ClassTemplateSpecializationDecl* spec = GetTypeOfX(*ast_unit);
  ASSERT_NE(spec, nullptr);

  absl::StatusOr<clang::QualType> t = crubit::ParameterizedByTAndStdTraitT(
      ast_unit->getASTContext(), spec, "char_traits");
  ASSERT_TRUE(t.ok()) << t.status();
  EXPECT_TRUE(ast_unit->getASTContext().hasSameType(
      *t, ast_unit->getASTContext().CharTy));
}

TEST(ParameterizedByTAndStdTraitTTest, WrongTraitName) {
  std::unique_ptr<clang::ASTUnit> ast_unit =
      Parse("std::basic_string_view<char, std::allocator<char>> x;");
  ASSERT_NE(ast_unit, nullptr);
  const clang::ClassTemplateSpecializationDecl* spec = GetTypeOfX(*ast_unit);
  ASSERT_NE(spec, nullptr);

  EXPECT_THAT(crubit::ParameterizedByTAndStdTraitT(ast_unit->getASTContext(),
                                                   spec, "char_traits"),
              StatusIs(absl::StatusCode::kInvalidArgument));
}

TEST(ParameterizedByTAndStdTraitTTest, TraitNotInStd) {
  std::unique_ptr<clang::ASTUnit> ast_unit =
      Parse("std::basic_string_view<char, ::char_traits<char>> x;");
  ASSERT_NE(ast_unit, nullptr);
  const clang::ClassTemplateSpecializationDecl* spec = GetTypeOfX(*ast_unit);
  ASSERT_NE(spec, nullptr);

  EXPECT_THAT(crubit::ParameterizedByTAndStdTraitT(ast_unit->getASTContext(),
                                                   spec, "char_traits"),
              StatusIs(absl::StatusCode::kInvalidArgument));
}

TEST(ParameterizedByTAndStdTraitTTest, MismatchedTraitArgument) {
  std::unique_ptr<clang::ASTUnit> ast_unit =
      Parse("std::basic_string_view<char, std::char_traits<wchar_t>> x;");
  ASSERT_NE(ast_unit, nullptr);
  const clang::ClassTemplateSpecializationDecl* spec = GetTypeOfX(*ast_unit);
  ASSERT_NE(spec, nullptr);

  EXPECT_THAT(crubit::ParameterizedByTAndStdTraitT(ast_unit->getASTContext(),
                                                   spec, "char_traits"),
              StatusIs(absl::StatusCode::kInvalidArgument));
}

}  // namespace
