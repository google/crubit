// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/collect_evidence.h"

#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "absl/log/check.h"
#include "nullability/inference/inference.proto.h"
#include "nullability/inference/inferrable.h"
#include "nullability/pointer_nullability.h"
#include "nullability/pointer_nullability_analysis.h"
#include "nullability/pointer_nullability_lattice.h"
#include "nullability/type_nullability.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/Expr.h"
#include "clang/AST/ExprCXX.h"
#include "clang/AST/OperationKinds.h"
#include "clang/AST/RecursiveASTVisitor.h"
#include "clang/AST/Stmt.h"
#include "clang/AST/Type.h"
#include "clang/Analysis/CFG.h"
#include "clang/Analysis/FlowSensitive/Arena.h"
#include "clang/Analysis/FlowSensitive/ControlFlowContext.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysis.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysisContext.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/Formula.h"
#include "clang/Analysis/FlowSensitive/Value.h"
#include "clang/Analysis/FlowSensitive/WatchedLiteralsSolver.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/Specifiers.h"
#include "clang/Index/USRGeneration.h"
#include "llvm/ADT/DenseMap.h"
#include "llvm/ADT/FunctionExtras.h"
#include "llvm/ADT/STLFunctionalExtras.h"
#include "llvm/Support/Error.h"
#include "llvm/Support/raw_ostream.h"

namespace clang::tidy::nullability {
using ::clang::dataflow::DataflowAnalysisContext;
using ::clang::dataflow::Environment;

llvm::unique_function<EvidenceEmitter> evidenceEmitter(
    llvm::unique_function<void(const Evidence &) const> Emit) {
  class EvidenceEmitterImpl {
   public:
    EvidenceEmitterImpl(
        llvm::unique_function<void(const Evidence &) const> Emit)
        : Emit(std::move(Emit)) {}

    void operator()(const Decl &Target, Slot S, Evidence::Kind Kind,
                    SourceLocation Loc) const {
      Evidence E;
      E.set_slot(S);
      E.set_kind(Kind);

      auto [It, Inserted] = USRCache.try_emplace(&Target);
      if (Inserted) {
        llvm::SmallString<128> USR;
        if (!index::generateUSRForDecl(&Target, USR)) It->second = USR.str();
      }
      if (It->second.empty()) return;  // Can't emit without a USR
      E.mutable_symbol()->set_usr(It->second);

      // TODO: make collecting and propagating location information optional?
      auto &SM =
          Target.getDeclContext()->getParentASTContext().getSourceManager();
      // TODO: are macro locations actually useful enough for debugging?
      //       we could leave them out, and make room for non-macro samples.
      if (Loc = SM.getFileLoc(Loc); Loc.isValid())
        E.set_location(Loc.printToString(SM));

      Emit(E);
    }

   private:
    mutable llvm::DenseMap<const Decl *, std::string> USRCache;
    llvm::unique_function<void(const Evidence &) const> Emit;
  };
  return EvidenceEmitterImpl(std::move(Emit));
}

namespace {

void collectEvidenceFromDereference(
    std::vector<std::pair<PointerTypeNullability, Slot>> &InferrableSlots,
    const CFGElement &Element, const dataflow::Environment &Env,
    llvm::function_ref<EvidenceEmitter> Emit) {
  // Is this CFGElement a dereference of a pointer?
  auto CFGStmt = Element.getAs<clang::CFGStmt>();
  if (!CFGStmt) return;
  auto *Op = dyn_cast_or_null<UnaryOperator>(CFGStmt->getStmt());
  if (!Op || Op->getOpcode() != UO_Deref) return;
  auto *DereferencedExpr = Op->getSubExpr();
  if (!DereferencedExpr || !DereferencedExpr->getType()->isPointerType())
    return;

  // It is a dereference of a pointer. Now gather evidence from it.
  dataflow::PointerValue *DereferencedValue =
      getPointerValueFromExpr(DereferencedExpr, Env);
  if (!DereferencedValue) return;
  auto &A = Env.getDataflowAnalysisContext().arena();
  auto &NotIsNull =
      A.makeNot(getPointerNullState(*DereferencedValue).second.formula());

  // If the flow conditions already imply the dereferenced value is not null,
  // then we don't have any new evidence of a necessary annotation.
  if (Env.flowConditionImplies(NotIsNull)) return;

  // Otherwise, if an inferrable slot being annotated Nonnull would imply that
  // the dereferenced value is not null, then we have evidence suggesting that
  // slot should be annotated. For now, we simply choose the first such slot,
  // sidestepping complexities around the possibility of multiple such slots,
  // any one of which would be sufficient if annotated Nonnull.
  for (auto &[Nullability, Slot] : InferrableSlots) {
    auto &SlotNonnullImpliesDerefValueNonnull =
        A.makeImplies(Nullability.isNonnull(A), NotIsNull);
    if (Env.flowConditionImplies(SlotNonnullImpliesDerefValueNonnull))
      Emit(*Env.getCurrentFunc(), Slot, Evidence::UNCHECKED_DEREFERENCE,
           Op->getOperatorLoc());
  }
}

const dataflow::Formula *getInferrableSlotsUnknownConstraint(
    std::vector<std::pair<PointerTypeNullability, Slot>> &InferrableSlots,
    const dataflow::Environment &Env) {
  dataflow::Arena &A = Env.getDataflowAnalysisContext().arena();
  const dataflow::Formula *CallerSlotsUnknown = &A.makeLiteral(true);
  for (auto &[Nullability, Slot] : InferrableSlots) {
    CallerSlotsUnknown = &A.makeAnd(
        *CallerSlotsUnknown, A.makeAnd(A.makeNot(Nullability.isNullable(A)),
                                       A.makeNot(Nullability.isNonnull(A))));
  }
  return CallerSlotsUnknown;
}

void collectEvidenceFromCallExpr(
    std::vector<std::pair<PointerTypeNullability, Slot>> &InferrableCallerSlots,
    const CFGElement &Element, const dataflow::Environment &Env,
    llvm::function_ref<EvidenceEmitter> Emit) {
  // Is this CFGElement a call to a function?
  auto CFGStmt = Element.getAs<clang::CFGStmt>();
  if (!CFGStmt) return;
  auto *CallExpr = dyn_cast_or_null<clang::CallExpr>(CFGStmt->getStmt());
  if (!CallExpr || !CallExpr->getCalleeDecl()) return;
  auto *CalleeDecl =
      dyn_cast_or_null<clang::FunctionDecl>(CallExpr->getCalleeDecl());
  if (!CalleeDecl || !isInferenceTarget(*CalleeDecl)) return;

  unsigned ParamI = 0;
  unsigned ArgI = 0;
  // Member operator calls hold the function object as the first argument,
  // offsetting the indices of parameters and corresponding arguments by 1.
  // For example: Given struct S { bool operator+(int*); }
  // The CXXMethodDecl has one parameter, but a call S{}+p is a
  // CXXOperatorCallExpr with two arguments: an S and an int*.
  if (isa<clang::CXXOperatorCallExpr>(CallExpr) &&
      isa<clang::CXXMethodDecl>(CalleeDecl))
    ++ArgI;

  // For each pointer parameter of the callee, ...
  for (; ParamI < CalleeDecl->param_size(); ++ParamI, ++ArgI) {
    if (!isSupportedPointerType(
            CalleeDecl->getParamDecl(ParamI)->getType().getNonReferenceType()))
      continue;
    // the corresponding argument should also be a pointer.
    CHECK(isSupportedPointerType(CallExpr->getArg(ArgI)->getType()));

    dataflow::PointerValue *PV =
        getPointerValueFromExpr(CallExpr->getArg(ArgI), Env);
    if (!PV) continue;

    // TODO: Check if the parameter is annotated. If annotated Nonnull, (instead
    // of collecting evidence for it?) collect evidence similar to a
    // dereference, i.e. if the argument is not already proven Nonnull, collect
    // evidence for a parameter that could be annotated Nonnull as a way to
    // force the argument to be Nonnull.

    // Emit evidence of the parameter's nullability. First, calculate that
    // nullability based on InferrableSlots for the caller being assigned to
    // Unknown, to reflect the current annotations and not all possible
    // annotations for them.
    NullabilityKind ArgNullability = getNullability(
        *PV, Env,
        getInferrableSlotsUnknownConstraint(InferrableCallerSlots, Env));
    Evidence::Kind ArgEvidenceKind;
    switch (ArgNullability) {
      case NullabilityKind::Nullable:
        ArgEvidenceKind = Evidence::NULLABLE_ARGUMENT;
        break;
      case NullabilityKind::NonNull:
        ArgEvidenceKind = Evidence::NONNULL_ARGUMENT;
        break;
      default:
        ArgEvidenceKind = Evidence::UNKNOWN_ARGUMENT;
    }
    Emit(*CalleeDecl, paramSlot(ParamI), ArgEvidenceKind,
         CallExpr->getArg(ArgI)->getExprLoc());
  }
}

void collectEvidenceFromReturn(
    std::vector<std::pair<PointerTypeNullability, Slot>> &InferrableSlots,
    const CFGElement &Element, const dataflow::Environment &Env,
    llvm::function_ref<EvidenceEmitter> Emit) {
  // Is this CFGElement a return statement?
  auto CFGStmt = Element.getAs<clang::CFGStmt>();
  if (!CFGStmt) return;
  auto *ReturnStmt = dyn_cast_or_null<clang::ReturnStmt>(CFGStmt->getStmt());
  if (!ReturnStmt) return;
  auto *ReturnExpr = ReturnStmt->getRetValue();
  if (!ReturnExpr || !isSupportedPointerType(ReturnExpr->getType())) return;

  NullabilityKind ReturnNullability =
      getNullability(ReturnExpr, Env,
                     getInferrableSlotsUnknownConstraint(InferrableSlots, Env));
  Evidence::Kind ReturnEvidenceKind;
  switch (ReturnNullability) {
    case NullabilityKind::Nullable:
      ReturnEvidenceKind = Evidence::NULLABLE_RETURN;
      break;
    case NullabilityKind::NonNull:
      ReturnEvidenceKind = Evidence::NONNULL_RETURN;
      break;
    default:
      ReturnEvidenceKind = Evidence::UNKNOWN_RETURN;
  }
  Emit(*Env.getCurrentFunc(), SLOT_RETURN_TYPE, ReturnEvidenceKind,
       ReturnExpr->getExprLoc());
}

void collectEvidenceFromElement(
    std::vector<std::pair<PointerTypeNullability, Slot>> InferrableSlots,
    const CFGElement &Element, const Environment &Env,
    llvm::function_ref<EvidenceEmitter> Emit) {
  collectEvidenceFromDereference(InferrableSlots, Element, Env, Emit);
  collectEvidenceFromCallExpr(InferrableSlots, Element, Env, Emit);
  collectEvidenceFromReturn(InferrableSlots, Element, Env, Emit);
  // TODO: add location information.
  // TODO: add more heuristic collections here
}

std::optional<Evidence::Kind> evidenceKindFromDeclaredType(QualType T) {
  if (!isSupportedPointerType(T.getNonReferenceType())) return std::nullopt;
  auto Nullability = getNullabilityAnnotationsFromType(T);
  switch (Nullability.front().concrete()) {
    default:
      return std::nullopt;
    case NullabilityKind::NonNull:
      return Evidence::ANNOTATED_NONNULL;
    case NullabilityKind::Nullable:
      return Evidence::ANNOTATED_NULLABLE;
  }
}
}  // namespace

llvm::Error collectEvidenceFromImplementation(
    const Decl &Decl, llvm::function_ref<EvidenceEmitter> Emit) {
  const FunctionDecl *Func = dyn_cast<FunctionDecl>(&Decl);
  if (!Func || !Func->doesThisDeclarationHaveABody()) {
    return llvm::createStringError(
        llvm::inconvertibleErrorCode(),
        "Implementation must be a function with a body.");
  }

  llvm::Expected<dataflow::ControlFlowContext> ControlFlowContext =
      dataflow::ControlFlowContext::build(*Func);
  if (!ControlFlowContext) return ControlFlowContext.takeError();

  DataflowAnalysisContext AnalysisContext(
      std::make_unique<dataflow::WatchedLiteralsSolver>());
  Environment Environment(AnalysisContext, *Func);
  PointerNullabilityAnalysis Analysis(
      Decl.getDeclContext()->getParentASTContext());
  std::vector<std::pair<PointerTypeNullability, Slot>> InferrableSlots;
  auto Parameters = Func->parameters();
  for (auto I = 0; I < Parameters.size(); ++I) {
    auto T = Parameters[I]->getType().getNonReferenceType();
    if (isSupportedPointerType(T) && !evidenceKindFromDeclaredType(T)) {
      InferrableSlots.push_back(
          std::make_pair(Analysis.assignNullabilityVariable(
                             Parameters[I], AnalysisContext.arena()),
                         paramSlot(I)));
    }
  }

  std::vector<Evidence> AllEvidence;
  llvm::Expected<std::vector<std::optional<
      dataflow::DataflowAnalysisState<PointerNullabilityLattice>>>>
      BlockToOutputStateOrError = dataflow::runDataflowAnalysis(
          *ControlFlowContext, Analysis, Environment,
          [&](const CFGElement &Element,
              const dataflow::DataflowAnalysisState<PointerNullabilityLattice>
                  &State) {
            collectEvidenceFromElement(InferrableSlots, Element, State.Env,
                                       Emit);
          });

  return llvm::Error::success();
}

void collectEvidenceFromTargetDeclaration(
    const clang::Decl &D, llvm::function_ref<EvidenceEmitter> Emit) {
  // For now, we can only describe the nullability of functions.
  const auto *Fn = dyn_cast<clang::FunctionDecl>(&D);
  if (!Fn) return;

  if (auto K = evidenceKindFromDeclaredType(Fn->getReturnType()))
    Emit(*Fn, SLOT_RETURN_TYPE, *K, Fn->getReturnTypeSourceRange().getBegin());
  for (unsigned I = 0; I < Fn->param_size(); ++I) {
    if (auto K = evidenceKindFromDeclaredType(Fn->getParamDecl(I)->getType()))
      Emit(*Fn, paramSlot(I), *K, Fn->getParamDecl(I)->getTypeSpecStartLoc());
  }
}

EvidenceSites EvidenceSites::discover(ASTContext &Ctx) {
  struct Walker : public RecursiveASTVisitor<Walker> {
    EvidenceSites Out;

    // We do want to see concrete code, including function instantiations.
    bool shouldVisitTemplateInstantiations() const { return true; }

    bool VisitFunctionDecl(const FunctionDecl *FD) {
      if (isInferenceTarget(*FD)) Out.Declarations.push_back(FD);

      // Visiting template instantiations is fine, these are valid functions!
      // But we'll be limited in what we can infer.
      bool IsUsefulImplementation =
          FD->doesThisDeclarationHaveABody() &&
          // We will not get anywhere with dependent code.
          !FD->isDependentContext();
      if (IsUsefulImplementation) Out.Implementations.push_back(FD);

      return true;
    }
  };

  Walker W;
  W.TraverseAST(Ctx);
  return std::move(W.Out);
}

}  // namespace clang::tidy::nullability
