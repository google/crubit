// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/test/check_diagnostics.h"

#include <algorithm>
#include <array>
#include <iterator>
#include <memory>
#include <vector>

#include "nullability/pointer_nullability_diagnosis.h"
#include "nullability/pragma.h"
#include "nullability/test/test_headers.h"
#include "clang/include/clang/AST/ASTConsumer.h"
#include "clang/include/clang/AST/Decl.h"
#include "clang/include/clang/ASTMatchers/ASTMatchers.h"
#include "clang/include/clang/Analysis/CFG.h"
#include "clang/include/clang/Frontend/FrontendActions.h"
#include "clang/include/clang/Testing/CommandLineArgs.h"
#include "clang/include/clang/Testing/TestAST.h"
#include "clang/include/clang/Tooling/Tooling.h"
#include "clang/unittests/Analysis/FlowSensitive/TestingSupport.h"
#include "llvm/include/llvm/ADT/ArrayRef.h"
#include "llvm/include/llvm/ADT/StringRef.h"
#include "llvm/include/llvm/Support/raw_ostream.h"
#include "external/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"
#include "external/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {

static bool checkDiagnostics(llvm::StringRef SourceCode, TestLanguage Lang,
                             bool AllowUntracked) {
  using ast_matchers::BoundNodes;
  using ast_matchers::hasName;
  using ast_matchers::match;
  using ast_matchers::stmt;
  using ast_matchers::valueDecl;

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
      match(valueDecl(hasName("target")).bind("target"), AST.context());
  if (MatchResult.empty()) {
    ADD_FAILURE() << "didn't find target declaration";
    return false;
  }

  bool Success = true;
  // We already checked MatchResult is not empty above, but we skip
  // isTemplated() functions. Make sure we didn't skip every MatchResult.
  bool FoundMatch = false;
  for (const ast_matchers::BoundNodes &BN : MatchResult) {
    const auto *Target = BN.getNodeAs<ValueDecl>("target");
    // Skip templates and only analyze instantiations
    // (where isTemplated is false)
    if (Target->isTemplated()) continue;
    FoundMatch = true;

    llvm::DenseMap<unsigned, std::string> Annotations =
        dataflow::test::buildLineToAnnotationMapping(
            AST.sourceManager(), AST.context().getLangOpts(),
            Target->getSourceRange(), AnnotatedCode);

    llvm::SmallVector<PointerNullabilityDiagnostic> Diagnostics;
    if (llvm::Error Err =
            diagnosePointerNullability(Target, Pragmas).moveInto(Diagnostics)) {
      ADD_FAILURE() << Err;
      return false;
    }

    // Note: use sorted sets for expected and actual lines to improve
    // readability of the error output in case the test fails.
    std::set<unsigned> ExpectedLines, ActualLines;
    for (const auto &[Line, _] : Annotations) {
      ExpectedLines.insert(Line);
    }
    for (const auto &Diag : Diagnostics) {
      // Untracked errors are not reported in production by default, so only
      // consider those if explicitly requested by AllowUntracked.
      if (AllowUntracked ||
          Diag.Code != PointerNullabilityDiagnostic::ErrorCode::Untracked) {
        ActualLines.insert(
            AST.sourceManager().getPresumedLineNumber(Diag.Range.getBegin()));
      }
    }
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
  if (!FoundMatch) {
    ADD_FAILURE() << "didn't find target declaration (after skipping templated "
                     "functions) -- add an instantiation?";
    return false;
  }

  return Success;
}

// Run in C++17 and C++20 mode to cover differences in the AST between modes
// (e.g. C++20 can contain `CXXRewrittenBinaryOperator`).
static constexpr std::array<TestLanguage, 2> CXXLanguagesToTest = {
    TestLanguage::Lang_CXX17, TestLanguage::Lang_CXX20};

bool checkDiagnostics(llvm::StringRef SourceCode) {
  for (TestLanguage Lang : CXXLanguagesToTest)
    if (!checkDiagnostics(SourceCode, Lang, /*AllowUntracked=*/false))
      return false;
  return true;
}

bool checkDiagnosticsHasUntracked(llvm::StringRef SourceCode) {
  for (TestLanguage Lang : CXXLanguagesToTest)
    if (!checkDiagnostics(SourceCode, Lang, /*AllowUntracked=*/true))
      return false;
  return true;
}

bool checkDiagnosticsWithMin(llvm::StringRef SourceCode, TestLanguage Min) {
  for (TestLanguage Lang : CXXLanguagesToTest) {
    if (Lang < Min) continue;
    if (!checkDiagnostics(SourceCode, Lang, /*AllowUntracked=*/false))
      return false;
  }
  return true;
}

}  // namespace clang::tidy::nullability
