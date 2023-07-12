// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/collect_evidence.h"

#include <optional>
#include <vector>

#include "absl/log/check.h"
#include "nullability/inference/inference.proto.h"
#include "nullability/pointer_nullability_analysis.h"
#include "nullability/pointer_nullability_lattice.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/Stmt.h"
#include "clang/AST/Type.h"
#include "clang/Analysis/CFG.h"
#include "clang/Analysis/FlowSensitive/ControlFlowContext.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysis.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysisContext.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/WatchedLiteralsSolver.h"
#include "clang/Basic/LLVM.h"
#include "clang/Index/USRGeneration.h"
#include "llvm/Support/Errc.h"
#include "llvm/Support/Error.h"
#include "llvm/Support/raw_ostream.h"

namespace clang::tidy::nullability {
using ::clang::dataflow::DataflowAnalysisContext;
using ::clang::dataflow::Environment;

llvm::Expected<std::vector<Evidence>> collectEvidence(
    const clang::FunctionDecl &Func, clang::ASTContext &Context) {
  // We want to make sure we use the declaration that the body comes from,
  // otherwise we will see references to `ParmVarDecl`s from a different
  // declaration.
  const clang::FunctionDecl *DeclWithBody = nullptr;
  if (!Func.getBody(DeclWithBody)) {
    return llvm::make_error<llvm::StringError>(llvm::errc::invalid_argument,
                                               "Function has no body.");
  }
  CHECK(DeclWithBody);

  llvm::Expected<clang::dataflow::ControlFlowContext> ControlFlowContext =
      clang::dataflow::ControlFlowContext::build(*DeclWithBody);
  if (!ControlFlowContext) return ControlFlowContext.takeError();

  DataflowAnalysisContext AnalysisContext(
      std::make_unique<clang::dataflow::WatchedLiteralsSolver>());
  Environment Environment(AnalysisContext, *DeclWithBody);
  PointerNullabilityAnalysis Analysis(Context);

  std::vector<Evidence> AllEvidence;
  llvm::Expected<std::vector<std::optional<
      clang::dataflow::DataflowAnalysisState<PointerNullabilityLattice>>>>
      BlockToOutputStateOrError = clang::dataflow::runDataflowAnalysis(
          *ControlFlowContext, Analysis, Environment,
          [](const CFGElement &Element,
             const dataflow::DataflowAnalysisState<PointerNullabilityLattice>
                 &State) {
            // TODO: collect Evidence values from element
          });

  return AllEvidence;
}
}  // namespace clang::tidy::nullability
