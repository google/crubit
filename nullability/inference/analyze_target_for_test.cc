// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/analyze_target_for_test.h"

#include <functional>
#include <utility>

#include "absl/log/check.h"
#include "clang/AST/Decl.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/ASTMatchers/ASTMatchers.h"
#include "clang/Testing/TestAST.h"
#include "llvm/ADT/STLFunctionalExtras.h"
#include "llvm/ADT/StringRef.h"

namespace clang::tidy::nullability {
namespace {
using ::clang::ast_matchers::compoundStmt;
using ::clang::ast_matchers::functionDecl;
using ::clang::ast_matchers::hasBody;
using ::clang::ast_matchers::hasName;
using ::clang::ast_matchers::MatchFinder;

// Simplifies creation of MatchCallbacks.
class CallbackAdapter : public clang::ast_matchers::MatchFinder::MatchCallback {
 public:
  explicit CallbackAdapter(
      std::function<void(const clang::ast_matchers::MatchFinder::MatchResult &)>
          Callback)
      : StoredCallback(std::move(Callback)) {
    CHECK(StoredCallback);
  }

  void run(
      const clang::ast_matchers::MatchFinder::MatchResult &result) override {
    StoredCallback(result);
  };

 private:
  std::function<void(const clang::ast_matchers::MatchFinder::MatchResult &)>
      StoredCallback;
};
}  // namespace

void analyzeTargetForTest(
    llvm::StringRef Source,
    llvm::function_ref<void(const clang::FunctionDecl &,
                            const MatchFinder::MatchResult &)>
        AnalysisCallback) {
  int MatchesForFunctionDeclarationsNamedTarget = 0;
  CallbackAdapter adapter([&, AnalysisCallback](
                              const MatchFinder::MatchResult &result) mutable {
    ++MatchesForFunctionDeclarationsNamedTarget;
    const auto *MaybeFunc = result.Nodes.getNodeAs<clang::FunctionDecl>("func");
    CHECK(MaybeFunc);
    AnalysisCallback(*MaybeFunc, result);
  });
  MatchFinder Finder;
  Finder.addMatcher(
      functionDecl(hasName("Target"), hasBody(compoundStmt())).bind("func"),
      &adapter);
  clang::TestInputs Inputs(Source);
  Inputs.FileName = "main.cc";
  clang::TestAST Ast(Inputs);
  Finder.matchAST(Ast.context());
  QCHECK_EQ(MatchesForFunctionDeclarationsNamedTarget, 1)
      << "Source must match exactly 1 function named 'Target' that has a "
         "definition";
}

}  // namespace clang::tidy::nullability
