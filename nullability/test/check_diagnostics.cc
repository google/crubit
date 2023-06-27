// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/test/check_diagnostics.h"

#include "nullability/pointer_nullability_analysis.h"
#include "nullability/pointer_nullability_diagnosis.h"
#include "clang/Analysis/CFG.h"
#include "third_party/llvm/llvm-project/clang/unittests/Analysis/FlowSensitive/TestingSupport.h"
#include "llvm/Testing/Support/Error.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang {
namespace tidy {
namespace nullability {

constexpr char kPreamble[] = R"cc(
  enum NullabilityKind {
    NK_nonnull,
    NK_nullable,
    NK_unspecified,
  };

  template <NullabilityKind... NK, typename T>
  void __assert_nullability(const T &);

  template <typename T>
  T value();
)cc";

constexpr char kNewHeader[] = R"cc(
  namespace std {
  struct nothrow_t {
    explicit nothrow_t() = default;
  };
  extern const nothrow_t nothrow;
  using size_t = decltype(sizeof(int));
  }  // namespace std
  void *operator new(std::size_t size, const std::nothrow_t &) noexcept;
)cc";

bool checkDiagnostics(llvm::StringRef SourceCode) {
  std::vector<CFGElement> Diagnostics;
  PointerNullabilityDiagnoser Diagnoser;
  bool Failed = false;
  EXPECT_THAT_ERROR(
      dataflow::test::checkDataflow<PointerNullabilityAnalysis>(
          dataflow::test::AnalysisInputs<PointerNullabilityAnalysis>(
              SourceCode, ast_matchers::hasName("target"),
              [](ASTContext &ASTCtx, dataflow::Environment &) {
                return PointerNullabilityAnalysis(ASTCtx);
              })
              .withPostVisitCFG([&Diagnostics, &Diagnoser](
                                    ASTContext &Ctx, const CFGElement &Elt,
                                    const dataflow::TransferStateForDiagnostics<
                                        PointerNullabilityLattice> &State) {
                auto EltDiagnostics = Diagnoser.diagnose(&Elt, Ctx, State);
                if (EltDiagnostics.has_value()) {
                  Diagnostics.push_back(EltDiagnostics.value());
                }
              })
              .withASTBuildVirtualMappedFiles(
                  {{"preamble.h", kPreamble}, {"new", kNewHeader}})
              .withASTBuildArgs({"-fsyntax-only", "-std=c++17",
                                 "-Wno-unused-value", "-Wno-nonnull",
                                 "-include", "preamble.h", "-I."}),
          [&Diagnostics, &Failed](
              const llvm::DenseMap<unsigned, std::string> &Annotations,
              const dataflow::test::AnalysisOutputs &AnalysisData) {
            // Note: use sorted sets for expected and actual lines to improve
            // readability of the error output in case the test fails.
            std::set<unsigned> ExpectedLines, ActualLines;
            for (const auto &[Line, _] : Annotations) {
              ExpectedLines.insert(Line);
            }
            auto &SrcMgr = AnalysisData.ASTCtx.getSourceManager();
            for (auto Element : Diagnostics) {
              if (std::optional<CFGStmt> stmt = Element.getAs<CFGStmt>()) {
                ActualLines.insert(SrcMgr.getPresumedLineNumber(
                    stmt->getStmt()->getBeginLoc()));
              } else if (std::optional<CFGInitializer> init =
                             Element.getAs<CFGInitializer>()) {
                ActualLines.insert(SrcMgr.getPresumedLineNumber(
                    init->getInitializer()->getSourceLocation()));
              } else {
                ADD_FAILURE() << "this code should not be reached";
              }
            }
            EXPECT_THAT(ActualLines, testing::ContainerEq(ExpectedLines));
            if (ActualLines != ExpectedLines) {
              Failed = true;
            }
          }),
      llvm::Succeeded());
  return !Failed;
}

}  // namespace nullability
}  // namespace tidy
}  // namespace clang
