// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/loc_filter.h"

#include <memory>
#include <string>

#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/ASTMatchers/ASTMatchers.h"
#include "clang/Testing/TestAST.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {
using ::clang::ast_matchers::functionDecl;
using ::clang::ast_matchers::hasName;
using ::clang::ast_matchers::match;
using ::clang::ast_matchers::selectFirst;

TEST(getLocFilterTest, NoRestrict) {
  TestInputs Inputs;
  Inputs.Code = R"cc(
#include "input.h"
#include "not_input.h"
    void func();
  )cc";
  Inputs.ExtraFiles = {{"input.h", R"cc(
                          void input_header_func();)cc"},
                       {"not_input.h", R"cc(
                          void not_input_header_func();
                        )cc"}};
  TestAST AST(Inputs);

  std::unique_ptr<LocFilter> Filter =
      getLocFilter(AST.context().getSourceManager(), LocFilterKind::kAllowAll);
  EXPECT_TRUE(Filter->check(
      selectFirst<FunctionDecl>(
          "f", match(functionDecl(hasName("func")).bind("f"), AST.context()))
          ->getBeginLoc()));
  EXPECT_TRUE(Filter->check(
      selectFirst<FunctionDecl>(
          "f", match(functionDecl(hasName("input_header_func")).bind("f"),
                     AST.context()))
          ->getBeginLoc()));
  EXPECT_TRUE(Filter->check(
      selectFirst<FunctionDecl>(
          "f", match(functionDecl(hasName("not_input_header_func")).bind("f"),
                     AST.context()))
          ->getBeginLoc()));
}

TEST(getLocFilterTest, RestrictMainFileOrHeader) {
  TestInputs Inputs;
  Inputs.Code = R"cc(
#include "input.h"
#include "not_input.h"
    void func();
  )cc";
  Inputs.ExtraFiles = {{"input.h", R"cc(
                          void input_header_func();)cc"},
                       {"not_input.h", R"cc(
                          void not_input_header_func();
                        )cc"}};
  TestAST AST(Inputs);

  std::unique_ptr<LocFilter> Filter = getLocFilter(
      AST.context().getSourceManager(), LocFilterKind::kMainFileOrHeader);
  EXPECT_TRUE(Filter->check(
      selectFirst<FunctionDecl>(
          "f", match(functionDecl(hasName("func")).bind("f"), AST.context()))
          ->getBeginLoc()));
  EXPECT_TRUE(Filter->check(
      selectFirst<FunctionDecl>(
          "f", match(functionDecl(hasName("input_header_func")).bind("f"),
                     AST.context()))
          ->getBeginLoc()));
  EXPECT_FALSE(Filter->check(
      selectFirst<FunctionDecl>(
          "f", match(functionDecl(hasName("not_input_header_func")).bind("f"),
                     AST.context()))
          ->getBeginLoc()));
}

TEST(getLocFilterTest, RestrictAllowAllButNotMainFile) {
  TestInputs Inputs;
  Inputs.Code = R"cc(
#include "input.h"
#include "not_input.h"
    void func();
  )cc";
  Inputs.ExtraFiles = {{"input.h", R"cc(
                          void input_header_func();)cc"},
                       {"not_input.h", R"cc(
                          void not_input_header_func();
                        )cc"}};
  TestAST AST(Inputs);

  std::unique_ptr<LocFilter> Filter = getLocFilter(
      AST.context().getSourceManager(), LocFilterKind::kAllowAllButNotMainFile);
  EXPECT_FALSE(Filter->check(
      selectFirst<FunctionDecl>(
          "f", match(functionDecl(hasName("func")).bind("f"), AST.context()))
          ->getBeginLoc()));
  EXPECT_TRUE(Filter->check(
      selectFirst<FunctionDecl>(
          "f", match(functionDecl(hasName("input_header_func")).bind("f"),
                     AST.context()))
          ->getBeginLoc()));
  EXPECT_TRUE(Filter->check(
      selectFirst<FunctionDecl>(
          "f", match(functionDecl(hasName("not_input_header_func")).bind("f"),
                     AST.context()))
          ->getBeginLoc()));
}

TEST(getLocFilterTest, RestrictMainHeaderButNotMainFile) {
  TestInputs Inputs;
  Inputs.Code = R"cc(
#include "input.h"
#include "not_input.h"
    void func();
  )cc";
  Inputs.ExtraFiles = {{"input.h", R"cc(
                          void input_header_func();)cc"},
                       {"not_input.h", R"cc(
                          void not_input_header_func();
                        )cc"}};
  TestAST AST(Inputs);

  std::unique_ptr<LocFilter> Filter =
      getLocFilter(AST.context().getSourceManager(),
                   LocFilterKind::kMainHeaderButNotMainFile);
  EXPECT_FALSE(Filter->check(
      selectFirst<FunctionDecl>(
          "f", match(functionDecl(hasName("func")).bind("f"), AST.context()))
          ->getBeginLoc()));
  EXPECT_TRUE(Filter->check(
      selectFirst<FunctionDecl>(
          "f", match(functionDecl(hasName("input_header_func")).bind("f"),
                     AST.context()))
          ->getBeginLoc()));
  EXPECT_FALSE(Filter->check(
      selectFirst<FunctionDecl>(
          "f", match(functionDecl(hasName("not_input_header_func")).bind("f"),
                     AST.context()))
          ->getBeginLoc()));
}

}  // namespace
}  // namespace clang::tidy::nullability
