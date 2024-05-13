// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/test/check_diagnostics.h"

#include <algorithm>
#include <iterator>
#include <memory>
#include <vector>

#include "nullability/pointer_nullability_diagnosis.h"
#include "nullability/pragma.h"
#include "nullability/test/test_headers.h"
#include "clang/AST/ASTConsumer.h"
#include "clang/ASTMatchers/ASTMatchers.h"
#include "clang/Analysis/CFG.h"
#include "clang/Analysis/FlowSensitive/Solver.h"
#include "clang/Frontend/FrontendActions.h"
#include "clang/Testing/CommandLineArgs.h"
#include "clang/Testing/TestAST.h"
#include "clang/Tooling/Tooling.h"
#include "third_party/llvm/llvm-project/clang/unittests/Analysis/FlowSensitive/TestingSupport.h"
#include "llvm/ADT/ArrayRef.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/raw_ostream.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {

static bool checkDiagnostics(llvm::StringRef SourceCode, TestLanguage Lang) {
  using ast_matchers::BoundNodes;
  using ast_matchers::functionDecl;
  using ast_matchers::hasName;
  using ast_matchers::match;
  using ast_matchers::stmt;

  llvm::Annotations AnnotatedCode(SourceCode);
  clang::TestInputs Inputs(AnnotatedCode.code());
  Inputs.Language = Lang;
  Inputs.ExtraArgs = {
      "-fsyntax-only",
      "-Wno-unused-value",
      "-Wno-nonnull",
      "-include",
      "check_diagnostics_preamble.h",
      "-I.",
  };
  for (const auto &Entry :
       llvm::ArrayRef(test_headers_create(), test_headers_size()))
    Inputs.ExtraFiles.try_emplace(Entry.name, Entry.data);
  NullabilityPragmas Pragmas;
  Inputs.MakeAction = [&] {
    struct Action : public SyntaxOnlyAction {
      NullabilityPragmas &Pragmas;
      Action(NullabilityPragmas &Pragmas) : Pragmas(Pragmas) {}

      std::unique_ptr<ASTConsumer> CreateASTConsumer(
          CompilerInstance &CI, llvm::StringRef File) override {
        registerPragmaHandler(CI.getPreprocessor(), Pragmas);
        return SyntaxOnlyAction::CreateASTConsumer(CI, File);
      }
    };
    return std::make_unique<Action>(Pragmas);
  };
  clang::TestAST AST(Inputs);

  SmallVector<BoundNodes, 1> MatchResult =
      match(functionDecl(hasName("target")).bind("target"), AST.context());
  if (MatchResult.empty()) {
    ADD_FAILURE() << "didn't find target function";
    return false;
  }

  std::unique_ptr<dataflow::Solver> Solver = makeDefaultSolver();

  bool Success = true;
  for (const ast_matchers::BoundNodes &BN : MatchResult) {
    const FunctionDecl *Target = BN.getNodeAs<FunctionDecl>("target");

    llvm::DenseMap<unsigned, std::string> Annotations =
        dataflow::test::buildLineToAnnotationMapping(
            AST.sourceManager(), AST.context().getLangOpts(),
            Target->getSourceRange(), AnnotatedCode);

    llvm::SmallVector<PointerNullabilityDiagnostic> Diagnostics;
    if (llvm::Error Err = diagnosePointerNullability(Target, Pragmas, *Solver)
                              .moveInto(Diagnostics)) {
      ADD_FAILURE() << Err;
      return false;
    }

    // Note: use sorted sets for expected and actual lines to improve
    // readability of the error output in case the test fails.
    std::set<unsigned> ExpectedLines, ActualLines;
    for (const auto &[Line, _] : Annotations) {
      ExpectedLines.insert(Line);
    }
    for (const auto &Diag : Diagnostics)
      ActualLines.insert(
          AST.sourceManager().getPresumedLineNumber(Diag.Range.getBegin()));
    if (ActualLines != ExpectedLines) {
      Success = false;

      // Clang's line numbers are one-based, so add an extra empty line at the
      // beginning.
      llvm::SmallVector<llvm::StringRef> Lines = {""};
      SourceCode.split(Lines, '\n');

      std::vector<unsigned> ExpectedButNotFound;
      std::set_difference(ExpectedLines.begin(), ExpectedLines.end(),
                          ActualLines.begin(), ActualLines.end(),
                          std::back_inserter(ExpectedButNotFound));
      std::vector<unsigned> FoundButNotExpected;
      std::set_difference(ActualLines.begin(), ActualLines.end(),
                          ExpectedLines.begin(), ExpectedLines.end(),
                          std::back_inserter(FoundButNotExpected));

      std::string ErrorMessage;
      llvm::raw_string_ostream OS(ErrorMessage);

      if (!ExpectedButNotFound.empty()) {
        OS << "Expected diagnostics but didn't find them:\n";
        for (unsigned Line : ExpectedButNotFound)
          OS << Line << ": " << Lines[Line] << "\n";
      }
      if (!FoundButNotExpected.empty()) {
        OS << "Found diagnostics but didn't expect them:\n";
        for (unsigned Line : FoundButNotExpected)
          OS << Line << ": " << Lines[Line] << "\n";
      }

      ADD_FAILURE() << ErrorMessage;
    }
  }

  return Success;
}

bool checkDiagnostics(llvm::StringRef SourceCode) {
  // Run in C++17 and C++20 mode to cover differences in the AST between modes
  // (e.g. C++20 can contain `CXXRewrittenBinaryOperator`).
  for (TestLanguage Lang : {TestLanguage::Lang_CXX17, TestLanguage::Lang_CXX20})
    if (!checkDiagnostics(SourceCode, Lang)) return false;
  return true;
}

}  // namespace clang::tidy::nullability
