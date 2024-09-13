// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/restrict_to_main_walker.h"

#include <string>
#include <vector>

#include "absl/base/nullability.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/Basic/SourceManager.h"
#include "clang/Testing/TestAST.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {
using ::testing::UnorderedElementsAre;

class TestWalker : public RestrictToMainFileOrHeaderWalker<TestWalker> {
 public:
  std::vector<std::string> &EligibleFunctionNames;

  TestWalker(std::vector<std::string> &Out, const SourceManager &SourceManager,
             bool ShouldRestrictToMainFileOrHeader)
      : RestrictToMainFileOrHeaderWalker(SourceManager,
                                         ShouldRestrictToMainFileOrHeader),
        EligibleFunctionNames(Out) {}

  bool VisitFunctionDecl(absl::Nonnull<const FunctionDecl *> FD) {
    if (!RestrictToMainFileOrHeader || inMainFileOrHeader(FD->getBeginLoc()))
      EligibleFunctionNames.push_back(FD->getNameAsString());
    return true;
  }
};

TEST(RestrictToMainFileOrHeaderWalkerTest, NoRestrict) {
  TestInputs Inputs;
  Inputs.Code = R"cc(
#include "input.h"
#include "not_input.h"
    void func();
  )cc";
  Inputs.ExtraFiles = {{"not_input.h", R"cc(
                          void not_input_header_func();
                        )cc"},
                       {"input.h", R"cc(
                          void input_header_func();)cc"}};
  TestAST AST(Inputs);
  std::vector<std::string> EligibleFunctionNames;
  TestWalker W(EligibleFunctionNames, AST.context().getSourceManager(),
               /*ShouldRestrictToMainFileOrHeader=*/false);
  W.TraverseAST(AST.context());
  EXPECT_THAT(EligibleFunctionNames,
              UnorderedElementsAre("func", "input_header_func",
                                   "not_input_header_func"));
}

TEST(RestrictToMainFileOrHeaderWalkerTest, Restrict) {
  TestInputs Inputs;
  Inputs.Code = R"cc(
#include "input.h"
#include "not_input.h"
    void func();
  )cc";
  Inputs.ExtraFiles = {{"not_input.h", R"cc(
                          void not_input_header_func();
                        )cc"},
                       {"input.h", R"cc(
                          void input_header_func();)cc"}};
  TestAST AST(Inputs);
  std::vector<std::string> EligibleFunctionNames;
  TestWalker W(EligibleFunctionNames, AST.context().getSourceManager(),
               /*ShouldRestrictToMainFileOrHeader=*/true);
  W.TraverseAST(AST.context());
  EXPECT_THAT(EligibleFunctionNames,
              UnorderedElementsAre("func", "input_header_func"));
}

}  // namespace
}  // namespace clang::tidy::nullability
