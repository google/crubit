// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/collect_evidence.h"

#include <memory>
#include <optional>
#include <string>
#include <string_view>
#include <utility>
#include <vector>

#include "absl/log/check.h"
#include "nullability/inference/inferable.h"
#include "nullability/inference/inference.proto.h"
#include "nullability/inference/slot_fingerprint.h"
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
#include "clang/Basic/SourceLocation.h"
#include "clang/Basic/Specifiers.h"
#include "clang/Index/USRGeneration.h"
#include "llvm/ADT/DenseSet.h"
#include "llvm/ADT/FunctionExtras.h"
#include "llvm/ADT/STLFunctionalExtras.h"
#include "llvm/Support/Error.h"
#include "llvm/Support/raw_ostream.h"

namespace clang::tidy::nullability {
using ::clang::dataflow::DataflowAnalysisContext;
using ::clang::dataflow::Environment;
using ::clang::dataflow::Formula;

std::string_view getOrGenerateUSR(USRCache &Cache, const Decl &Decl) {
  auto [It, Inserted] = Cache.try_emplace(&Decl);
  if (Inserted) {
    llvm::SmallString<128> USR;
    if (!index::generateUSRForDecl(&Decl, USR)) It->second = USR.str();
  }
  return It->second;
}

llvm::unique_function<EvidenceEmitter> evidenceEmitter(
    llvm::unique_function<void(const Evidence &) const> Emit,
    nullability::USRCache &USRCache) {
  class EvidenceEmitterImpl {
   public:
    EvidenceEmitterImpl(
        llvm::unique_function<void(const Evidence &) const> Emit,
        nullability::USRCache &USRCache)
        : Emit(std::move(Emit)), USRCache(USRCache) {}

    void operator()(const Decl &Target, Slot S, Evidence::Kind Kind,
                    SourceLocation Loc) const {
      CHECK(isInferenceTarget(Target))
          << "Evidence emitted for a Target which is not an inference target.";

      Evidence E;
      E.set_slot(S);
      E.set_kind(Kind);

      std::string_view USR = getOrGenerateUSR(USRCache, Target);
      if (USR.empty()) return;  // Can't emit without a USR
      E.mutable_symbol()->set_usr(USR);

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
    llvm::unique_function<void(const Evidence &) const> Emit;
    nullability::USRCache &USRCache;
  };
  return EvidenceEmitterImpl(std::move(Emit), USRCache);
}

namespace {

// If Element is a dereference, returns its target and location.
std::pair<Expr *, SourceLocation> describeDereference(
    const CFGElement &Element) {
  if (auto CFGStmt = Element.getAs<clang::CFGStmt>()) {
    if (auto *Op = dyn_cast<UnaryOperator>(CFGStmt->getStmt());
        Op && Op->getOpcode() == UO_Deref) {
      return {Op->getSubExpr(), Op->getOperatorLoc()};
    }
    if (auto *ME = dyn_cast<MemberExpr>(CFGStmt->getStmt());
        ME && ME->isArrow()) {
      return {ME->getBase(), ME->getOperatorLoc()};
    }
  }
  return {nullptr, SourceLocation()};
}

// Records evidence derived from the assumption that Value is nonnull.
// It may be dereferenced, passed as a nonnull param, etc, per EvidenceKind.
void collectMustBeNonnullEvidence(
    const dataflow::PointerValue &Value, const dataflow::Environment &Env,
    SourceLocation Loc,
    std::vector<std::pair<PointerTypeNullability, Slot>> &InferableSlots,
    Evidence::Kind EvidenceKind, llvm::function_ref<EvidenceEmitter> Emit) {
  auto &A = Env.getDataflowAnalysisContext().arena();
  auto &NotIsNull = A.makeNot(getPointerNullState(Value).IsNull);

  // If the flow conditions already imply that Value is not null, then we don't
  // have any new evidence of a necessary annotation.
  if (Env.flowConditionImplies(NotIsNull)) return;

  // Otherwise, if an inferable slot being annotated Nonnull would imply that
  // Value is not null, then we have evidence suggesting that slot should be
  // annotated. For now, we simply choose the first such slot, sidestepping
  // complexities around the possibility of multiple such slots, any one of
  // which would be sufficient if annotated Nonnull.
  for (auto &[Nullability, Slot] : InferableSlots) {
    auto &SlotNonnullImpliesValueNonnull =
        A.makeImplies(Nullability.isNonnull(A), NotIsNull);
    if (Env.flowConditionImplies(SlotNonnullImpliesValueNonnull))
      Emit(*Env.getCurrentFunc(), Slot, EvidenceKind, Loc);
  }
}

void collectEvidenceFromDereference(
    std::vector<std::pair<PointerTypeNullability, Slot>> &InferableSlots,
    const CFGElement &Element, const dataflow::Environment &Env,
    llvm::function_ref<EvidenceEmitter> Emit) {
  auto [Target, Loc] = describeDereference(Element);
  if (!Target || !Target->getType()->isPointerType()) return;

  // It is a dereference of a pointer. Now gather evidence from it.

  // Skip gathering evidence about the current function if the current
  // function is not an inference target.
  if (!isInferenceTarget(*Env.getCurrentFunc())) return;

  dataflow::PointerValue *DereferencedValue =
      getPointerValueFromExpr(Target, Env);
  if (!DereferencedValue) return;
  collectMustBeNonnullEvidence(*DereferencedValue, Env, Loc, InferableSlots,
                               Evidence::UNCHECKED_DEREFERENCE, Emit);
}

// Inferable slots are nullability slots not explicitly annotated in source
// code that we are currently capable of handling. This returns a boolean
// constraint representing these slots having a) the nullability inferred from
// the previous round for this slot or b) Unknown nullability if no inference
// was made in the previous round or there was no previous round.
const Formula *getInferableSlotsAsInferredOrUnknownConstraint(
    std::vector<std::pair<PointerTypeNullability, Slot>> &InferableSlots,
    const PreviousInferences &PreviousInferences, USRCache &USRCache,
    dataflow::Arena &A, const Decl &CurrentFunc) {
  const Formula *Constraint = &A.makeLiteral(true);
  std::string_view USR = getOrGenerateUSR(USRCache, CurrentFunc);
  for (auto &[Nullability, Slot] : InferableSlots) {
    SlotFingerprint Fingerprint = fingerprint(USR, Slot);
    const Formula &Nullable = PreviousInferences.Nullable.contains(Fingerprint)
                                  ? Nullability.isNullable(A)
                                  : A.makeNot(Nullability.isNullable(A));
    const Formula &Nonnull = PreviousInferences.Nonnull.contains(Fingerprint)
                                 ? Nullability.isNonnull(A)
                                 : A.makeNot(Nullability.isNonnull(A));
    Constraint = &A.makeAnd(*Constraint, A.makeAnd(Nullable, Nonnull));
  }
  return Constraint;
}

void collectEvidenceFromParamAnnotation(
    TypeNullability &ParamNullability, const dataflow::PointerValue &ArgPV,
    std::vector<std::pair<PointerTypeNullability, Slot>> &InferableCallerSlots,
    const dataflow::Environment &Env, SourceLocation ArgLoc,
    llvm::function_ref<EvidenceEmitter> Emit) {
  //  TODO: Account for variance and each layer of nullability when we handle
  //  more than top-level pointers.
  if (ParamNullability.empty()) return;
  if (ParamNullability[0].concrete() == NullabilityKind::NonNull) {
    collectMustBeNonnullEvidence(ArgPV, Env, ArgLoc, InferableCallerSlots,
                                 Evidence::PASSED_TO_NONNULL, Emit);
  }
}

void collectEvidenceFromCallExpr(
    std::vector<std::pair<PointerTypeNullability, Slot>> &InferableCallerSlots,
    const Formula *InferableSlotsConstraint, const CFGElement &Element,
    const dataflow::Environment &Env,
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
    auto ParamType =
        CalleeDecl->getParamDecl(ParamI)->getType().getNonReferenceType();
    if (!isSupportedPointerType(ParamType)) continue;
    // the corresponding argument should also be a pointer.
    CHECK(isSupportedPointerType(CallExpr->getArg(ArgI)->getType()));

    dataflow::PointerValue *PV =
        getPointerValueFromExpr(CallExpr->getArg(ArgI), Env);
    if (!PV) continue;

    SourceLocation ArgLoc = CallExpr->getArg(ArgI)->getExprLoc();

    // TODO: Include inferred annotations from previous rounds when propagating.
    auto ParamNullability = getNullabilityAnnotationsFromType(ParamType);

    // Collect evidence from the binding of the argument to the parameter's
    // nullability, if known.
    collectEvidenceFromParamAnnotation(ParamNullability, *PV,
                                       InferableCallerSlots, Env, ArgLoc, Emit);

    // Emit evidence of the parameter's nullability. First, calculate that
    // nullability based on InferableSlots for the caller being assigned to
    // Unknown, to reflect the current annotations and not all possible
    // annotations for them.
    NullabilityKind ArgNullability =
        getNullability(*PV, Env, InferableSlotsConstraint);
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
    Emit(*CalleeDecl, paramSlot(ParamI), ArgEvidenceKind, ArgLoc);
  }
}

void collectEvidenceFromReturn(
    std::vector<std::pair<PointerTypeNullability, Slot>> &InferableSlots,
    const Formula *InferableSlotsConstraint, const CFGElement &Element,
    const dataflow::Environment &Env,
    llvm::function_ref<EvidenceEmitter> Emit) {
  // Is this CFGElement a return statement?
  auto CFGStmt = Element.getAs<clang::CFGStmt>();
  if (!CFGStmt) return;
  auto *ReturnStmt = dyn_cast_or_null<clang::ReturnStmt>(CFGStmt->getStmt());
  if (!ReturnStmt) return;
  auto *ReturnExpr = ReturnStmt->getRetValue();
  if (!ReturnExpr || !isSupportedPointerType(ReturnExpr->getType())) return;

  // Skip gathering evidence about the current function if the current function
  // is not an inference target.
  if (!isInferenceTarget(*Env.getCurrentFunc())) return;

  NullabilityKind ReturnNullability =
      getNullability(ReturnExpr, Env, InferableSlotsConstraint);
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
    std::vector<std::pair<PointerTypeNullability, Slot>> InferableSlots,
    const Formula *InferableSlotsConstraint, const CFGElement &Element,
    const Environment &Env, llvm::function_ref<EvidenceEmitter> Emit) {
  collectEvidenceFromDereference(InferableSlots, Element, Env, Emit);
  collectEvidenceFromCallExpr(InferableSlots, InferableSlotsConstraint, Element,
                              Env, Emit);
  collectEvidenceFromReturn(InferableSlots, InferableSlotsConstraint, Element,
                            Env, Emit);
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
    const Decl &Decl, llvm::function_ref<EvidenceEmitter> Emit,
    USRCache &USRCache, const PreviousInferences PreviousInferences) {
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
      std::make_unique<dataflow::WatchedLiteralsSolver>(100000));
  Environment Environment(AnalysisContext, *Func);
  PointerNullabilityAnalysis Analysis(
      Decl.getDeclContext()->getParentASTContext());
  std::vector<std::pair<PointerTypeNullability, Slot>> InferableSlots;
  auto Parameters = Func->parameters();
  for (auto I = 0; I < Parameters.size(); ++I) {
    auto T = Parameters[I]->getType().getNonReferenceType();
    if (isSupportedPointerType(T) && !evidenceKindFromDeclaredType(T)) {
      InferableSlots.push_back(
          std::make_pair(Analysis.assignNullabilityVariable(
                             Parameters[I], AnalysisContext.arena()),
                         paramSlot(I)));
    }
  }
  const auto *InferableSlotsConstraint =
      getInferableSlotsAsInferredOrUnknownConstraint(
          InferableSlots, PreviousInferences, USRCache, AnalysisContext.arena(),
          Decl);

  return dataflow::runDataflowAnalysis(
             *ControlFlowContext, Analysis, Environment,
             [&](const CFGElement &Element,
                 const dataflow::DataflowAnalysisState<
                     PointerNullabilityLattice> &State) {
               collectEvidenceFromElement(InferableSlots,
                                          InferableSlotsConstraint, Element,
                                          State.Env, Emit);
             })
      .takeError();
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
