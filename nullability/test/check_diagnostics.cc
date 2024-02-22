// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/test/check_diagnostics.h"

#include <algorithm>
#include <iterator>
#include <vector>

#include "nullability/pointer_nullability_diagnosis.h"
#include "nullability/test/test_headers.h"
#include "clang/Analysis/CFG.h"
#include "clang/Tooling/Tooling.h"
#include "third_party/llvm/llvm-project/clang/unittests/Analysis/FlowSensitive/TestingSupport.h"
#include "llvm/ADT/ArrayRef.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/raw_ostream.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {

static bool checkDiagnostics(llvm::StringRef SourceCode, const char *CxxMode) {
  using ast_matchers::BoundNodes;
  using ast_matchers::functionDecl;
  using ast_matchers::hasName;
  using ast_matchers::match;
  using ast_matchers::stmt;

  llvm::Annotations AnnotatedCode(SourceCode);
  std::vector<std::string> ASTBuildArgs = {"-fsyntax-only",
                                           CxxMode,
                                           "-Wno-unused-value",
                                           "-Wno-nonnull",
                                           "-include",
                                           "check_diagnostics_preamble.h",
                                           "-I."};

  tooling::FileContentMappings TestHeaders;
  for (const auto &Entry :
       llvm::ArrayRef(test_headers_create(), test_headers_size()))
    TestHeaders.emplace_back(Entry.name, Entry.data);

  auto Unit = tooling::buildASTFromCodeWithArgs(
      AnnotatedCode.code(), ASTBuildArgs, "input.cc", "nullability-test",
      std::make_shared<PCHContainerOperations>(),
      tooling::getClangStripDependencyFileAdjuster(), TestHeaders);
  auto &Context = Unit->getASTContext();

  if (Context.getDiagnostics().getClient()->getNumErrors() != 0) {
    ADD_FAILURE() << "encountered compile errors (printed to the test log)";
    return false;
  }

  SmallVector<BoundNodes, 1> MatchResult =
      match(functionDecl(hasName("target")).bind("target"), Context);
  if (MatchResult.empty()) {
    ADD_FAILURE() << "didn't find target function";
    return false;
  }

  bool Success = true;
  for (const ast_matchers::BoundNodes &BN : MatchResult) {
    const FunctionDecl *Target = BN.getNodeAs<FunctionDecl>("target");

    llvm::DenseMap<unsigned, std::string> Annotations =
        dataflow::test::buildLineToAnnotationMapping(
            Context.getSourceManager(), Context.getLangOpts(),
            Target->getSourceRange(), AnnotatedCode);

    llvm::SmallVector<PointerNullabilityDiagnostic> Diagnostics;
    if (llvm::Error Err =
            diagnosePointerNullability(Target).moveInto(Diagnostics)) {
      ADD_FAILURE() << Err;
      return false;
    }

    // Note: use sorted sets for expected and actual lines to improve
    // readability of the error output in case the test fails.
    std::set<unsigned> ExpectedLines, ActualLines;
    for (const auto &[Line, _] : Annotations) {
      ExpectedLines.insert(Line);
    }
    auto &SrcMgr = Context.getSourceManager();
    for (const auto &Diag : Diagnostics)
      ActualLines.insert(SrcMgr.getPresumedLineNumber(Diag.Range.getBegin()));
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
  for (const char *CxxMode : {"-std=c++17", "-std=c++20"})
    if (!checkDiagnostics(SourceCode, CxxMode)) return false;
  return true;
}

}  // namespace clang::tidy::nullability
