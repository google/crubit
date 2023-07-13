// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/collect_evidence.h"

#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "absl/log/check.h"
#include "nullability/inference/inference.proto.h"
#include "nullability/pointer_nullability.h"
#include "nullability/pointer_nullability_analysis.h"
#include "nullability/pointer_nullability_lattice.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/Expr.h"
#include "clang/AST/OperationKinds.h"
#include "clang/AST/Stmt.h"
#include "clang/AST/Type.h"
#include "clang/Analysis/CFG.h"
#include "clang/Analysis/FlowSensitive/ControlFlowContext.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysis.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysisContext.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/Value.h"
#include "clang/Analysis/FlowSensitive/WatchedLiteralsSolver.h"
#include "clang/Basic/LLVM.h"
#include "clang/Index/USRGeneration.h"
#include "llvm/Support/Errc.h"
#include "llvm/Support/Error.h"
#include "llvm/Support/raw_ostream.h"

namespace clang::tidy::nullability {
using ::clang::dataflow::DataflowAnalysisContext;
using ::clang::dataflow::Environment;

namespace {
std::optional<Evidence> collectEvidenceFromDereference(
    const FunctionDecl &Func,
    std::vector<std::pair<PointerTypeNullability, Slot>> InferrableSlots,
    const std::string &USR, const CFGElement &Element,
    const PointerNullabilityLattice &Lattice,
    const dataflow::Environment &Env) {
  // Is this CFGElement a dereference of a pointer?
  auto CFGStmt = Element.getAs<clang::CFGStmt>();
  if (!CFGStmt) return std::nullopt;
  auto *Op = dyn_cast_or_null<UnaryOperator>(CFGStmt->getStmt());
  if (!Op || Op->getOpcode() != UO_Deref) return std::nullopt;
  auto *DereferencedExpr = Op->getSubExpr();
  if (!DereferencedExpr || !DereferencedExpr->getType()->isPointerType())
    return std::nullopt;

  // It is a dereference of a pointer. Now gather evidence from it.
  dataflow::PointerValue *DereferencedValue =
      getPointerValueFromExpr(DereferencedExpr, Env);
  if (!DereferencedValue) return std::nullopt;
  auto &A = Env.getDataflowAnalysisContext().arena();
  auto &NotIsNull =
      A.makeNot(getPointerNullState(*DereferencedValue).second.formula());

  // If the flow conditions already imply the dereferenced value is not null,
  // then we don't have any new evidence of a necessary annotation.
  if (Env.flowConditionImplies(NotIsNull)) return std::nullopt;

  // Otherwise, if an inferrable slot being annotated Nonnull would imply that
  // the dereferenced value is not null, then we have evidence suggesting that
  // slot should be annotated. For now, we simply choose the first such slot,
  // sidestepping complexities around the possibility of multiple such slots,
  // any one of which would be sufficient if annotated Nonnull.
  for (auto &[Nullability, Slot] : InferrableSlots) {
    auto &SlotNonnullImpliesDerefValueNonnull =
        A.makeImplies(Nullability.Nonnull->formula(), NotIsNull);
    if (Env.flowConditionImplies(SlotNonnullImpliesDerefValueNonnull)) {
      Evidence Evidence;
      Evidence.mutable_constraint()->set_must_be_nonnull(true);
      *Evidence.mutable_slot() = Slot;
      Evidence.mutable_symbol()->set_usr(USR);
      return Evidence;
    }
  }

  return std::nullopt;
}

void appendEvidence(
    std::vector<Evidence> &AllEvidence, const FunctionDecl &Func,
    std::vector<std::pair<PointerTypeNullability, Slot>> InferrableSlots,
    const std::string &USR, const CFGElement &Element,
    const PointerNullabilityLattice &Lattice, const Environment &Env) {
  if (std::optional<Evidence> NewEvidence = collectEvidenceFromDereference(
          Func, InferrableSlots, USR, Element, Lattice, Env)) {
    AllEvidence.push_back(std::move(*NewEvidence));
  }
  // TODO: add more heuristic collections here
}
}  // namespace

llvm::Expected<std::vector<Evidence>> collectEvidence(const FunctionDecl &Func,
                                                      ASTContext &Context) {
  // We want to make sure we use the declaration that the body comes from,
  // otherwise we will see references to `ParmVarDecl`s from a different
  // declaration.
  const FunctionDecl *DeclWithBody = nullptr;
  if (!Func.getBody(DeclWithBody)) {
    return llvm::make_error<llvm::StringError>(llvm::errc::invalid_argument,
                                               "Function has no body.");
  }
  CHECK(DeclWithBody);

  llvm::Expected<dataflow::ControlFlowContext> ControlFlowContext =
      dataflow::ControlFlowContext::build(*DeclWithBody);
  if (!ControlFlowContext) return ControlFlowContext.takeError();

  DataflowAnalysisContext AnalysisContext(
      std::make_unique<dataflow::WatchedLiteralsSolver>());
  Environment Environment(AnalysisContext, *DeclWithBody);
  PointerNullabilityAnalysis Analysis(Context);
  std::vector<std::pair<PointerTypeNullability, Slot>> InferrableSlots;
  auto Parameters = Func.parameters();
  for (auto i = 0; i < Parameters.size(); ++i) {
    if (Parameters[i]->getType().getNonReferenceType()->isPointerType()) {
      // TODO: Skip assigning variables for already-annotated parameters,
      // potentially configurably.
      Slot slot;
      slot.set_parameter(i);
      InferrableSlots.push_back(
          std::make_pair(Analysis.assignNullabilityVariable(
                             Parameters[i], AnalysisContext.arena()),
                         std::move(slot)));
    }
  }

  std::vector<Evidence> AllEvidence;
  llvm::SmallString<128> USR;
  index::generateUSRForDecl(&Func, USR);
  llvm::Expected<std::vector<std::optional<
      dataflow::DataflowAnalysisState<PointerNullabilityLattice>>>>
      BlockToOutputStateOrError = dataflow::runDataflowAnalysis(
          *ControlFlowContext, Analysis, Environment,
          [&](const CFGElement &Element,
              const dataflow::DataflowAnalysisState<PointerNullabilityLattice>
                  &State) {
            appendEvidence(AllEvidence, *DeclWithBody, InferrableSlots,
                           std::string(USR.data(), USR.size()), Element,
                           State.Lattice, State.Env);
          });

  return AllEvidence;
}
}  // namespace clang::tidy::nullability
