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

#include "absl/container/flat_hash_map.h"
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
#include "clang/AST/DeclCXX.h"
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

using ConcreteNullabilityCache =
    absl::flat_hash_map<const Decl *,
                        std::optional<const PointerTypeNullability>>;

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
  auto *IsNull = getPointerNullState(Value).IsNull;
  // If `IsNull` is top, we can't infer anything about it.
  if (IsNull == nullptr) return;
  auto &NotIsNull = A.makeNot(*IsNull);

  // If the flow conditions already imply that Value is not null, then we don't
  // have any new evidence of a necessary annotation.
  if (Env.proves(NotIsNull)) return;

  // Otherwise, if an inferable slot being annotated Nonnull would imply that
  // Value is not null, then we have evidence suggesting that slot should be
  // annotated. For now, we simply choose the first such slot, sidestepping
  // complexities around the possibility of multiple such slots, any one of
  // which would be sufficient if annotated Nonnull.
  for (auto &[Nullability, Slot] : InferableSlots) {
    auto &SlotNonnullImpliesValueNonnull =
        A.makeImplies(Nullability.isNonnull(A), NotIsNull);
    if (Env.proves(SlotNonnullImpliesValueNonnull))
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
const Formula &getInferableSlotsAsInferredOrUnknownConstraint(
    std::vector<std::pair<PointerTypeNullability, Slot>> &InferableSlots,
    USRCache &USRCache, const PreviousInferences &PreviousInferences,
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
  return *Constraint;
}

auto getNullabilityAnnotationsFromTypeAndOverrides(
    QualType Type, const Decl *D, const PointerNullabilityLattice &Lattice) {
  auto N = getNullabilityAnnotationsFromType(Type);
  if (N.empty()) {
    // We expect this not to be the case, but not to a crash-worthy level, so
    // just log if it is.
    llvm::errs() << "Nullability for type " << Type.getAsString();
    if (auto *ND = dyn_cast_or_null<clang::NamedDecl>(D)) {
      llvm::errs() << "for Decl named " << ND->getName();
    }
    llvm::errs() << " requested with overrides, but is an empty vector.\n";
  } else {
    Lattice.overrideNullabilityFromDecl(D, N);
  }
  return N;
}

void collectEvidenceFromBindingToType(
    TypeNullability &TypeNullability,
    const dataflow::PointerValue &PointerValue,
    std::vector<std::pair<PointerTypeNullability, Slot>>
        &InferableSlotsFromValueContext,
    const Formula &InferableSlotsConstraint, const dataflow::Environment &Env,
    SourceLocation ValueLoc, llvm::function_ref<EvidenceEmitter> Emit) {
  //  TODO: Account for variance and each layer of nullability when we handle
  //  more than top-level pointers.
  if (TypeNullability.empty()) return;
  PointerTypeNullability &TopLevel = TypeNullability[0];
  dataflow::Arena &A = Env.arena();
  if (TopLevel.concrete() == NullabilityKind::NonNull ||
      (TopLevel.isSymbolic() &&
       Env.proves(
           A.makeImplies(InferableSlotsConstraint, TopLevel.isNonnull(A))))) {
    collectMustBeNonnullEvidence(PointerValue, Env, ValueLoc,
                                 InferableSlotsFromValueContext,
                                 Evidence::BOUND_TO_NONNULL, Emit);
  }
}

template <typename CallOrConstructExpr>
void collectEvidenceFromArgsAndParams(
    const FunctionDecl &CalleeDecl, const CallOrConstructExpr &Expr,
    std::vector<std::pair<PointerTypeNullability, Slot>> &InferableCallerSlots,
    const Formula &InferableSlotsConstraint,
    const PointerNullabilityLattice &Lattice, const dataflow::Environment &Env,
    llvm::function_ref<EvidenceEmitter> Emit) {
  unsigned ParamI = 0;
  unsigned ArgI = 0;
  // Member operator calls hold the function object as the first argument,
  // offsetting the indices of parameters and corresponding arguments by 1.
  // For example: Given struct S { bool operator+(int*); }
  // The CXXMethodDecl has one parameter, but a call S{}+p is a
  // CXXOperatorCallExpr with two arguments: an S and an int*.
  if (isa<clang::CXXOperatorCallExpr>(Expr) &&
      isa<clang::CXXMethodDecl>(CalleeDecl))
    ++ArgI;

  // For each pointer parameter of the callee, ...
  for (; ParamI < CalleeDecl.param_size(); ++ParamI, ++ArgI) {
    const auto *ParamDecl = CalleeDecl.getParamDecl(ParamI);
    const auto ParamType = ParamDecl->getType().getNonReferenceType();
    if (!isSupportedPointerType(ParamType)) continue;
    // the corresponding argument should also be a pointer.
    CHECK(isSupportedPointerType(Expr.getArg(ArgI)->getType()));

    dataflow::PointerValue *PV =
        getPointerValueFromExpr(Expr.getArg(ArgI), Env);
    if (!PV) continue;

    SourceLocation ArgLoc = Expr.getArg(ArgI)->getExprLoc();

    auto ParamNullability = getNullabilityAnnotationsFromTypeAndOverrides(
        ParamType, ParamDecl, Lattice);

    // Collect evidence from the binding of the argument to the parameter's
    // nullability, if known.
    collectEvidenceFromBindingToType(
        ParamNullability, *PV, InferableCallerSlots, InferableSlotsConstraint,
        Env, ArgLoc, Emit);

    // Emit evidence of the parameter's nullability. First, calculate that
    // nullability based on InferableSlots for the caller being assigned to
    // Unknown or their previously-inferred value, to reflect the current
    // annotations and not all possible annotations for them.
    NullabilityKind ArgNullability =
        getNullability(*PV, Env, &InferableSlotsConstraint);
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
    Emit(CalleeDecl, paramSlot(ParamI), ArgEvidenceKind, ArgLoc);
  }
}

void collectEvidenceFromCallExpr(
    std::vector<std::pair<PointerTypeNullability, Slot>> &InferableCallerSlots,
    const Formula &InferableSlotsConstraint, const CFGElement &Element,
    const PointerNullabilityLattice &Lattice, const dataflow::Environment &Env,
    llvm::function_ref<EvidenceEmitter> Emit) {
  // Is this CFGElement a call to a function?
  auto CFGStmt = Element.getAs<clang::CFGStmt>();
  if (!CFGStmt) return;
  auto *CallExpr = dyn_cast_or_null<clang::CallExpr>(CFGStmt->getStmt());
  if (!CallExpr || !CallExpr->getCalleeDecl()) return;
  auto *CalleeDecl =
      dyn_cast_or_null<clang::FunctionDecl>(CallExpr->getCalleeDecl());
  if (!CalleeDecl || !isInferenceTarget(*CalleeDecl)) return;

  collectEvidenceFromArgsAndParams(*CalleeDecl, *CallExpr, InferableCallerSlots,
                                   InferableSlotsConstraint, Lattice, Env,
                                   Emit);
}

void collectEvidenceFromConstructExpr(
    std::vector<std::pair<PointerTypeNullability, Slot>> &InferableSlots,
    const Formula &InferableSlotsConstraint, const CFGElement &Element,
    const PointerNullabilityLattice &Lattice, const dataflow::Environment &Env,
    llvm::function_ref<EvidenceEmitter> Emit) {
  auto CFGStmt = Element.getAs<clang::CFGStmt>();
  if (!CFGStmt) return;
  auto *ConstructExpr =
      dyn_cast_or_null<clang::CXXConstructExpr>(CFGStmt->getStmt());
  if (!ConstructExpr || !ConstructExpr->getConstructor()) return;
  auto *ConstructorDecl = dyn_cast_or_null<clang::CXXConstructorDecl>(
      ConstructExpr->getConstructor());
  if (!ConstructorDecl || !isInferenceTarget(*ConstructorDecl)) return;

  collectEvidenceFromArgsAndParams(*ConstructorDecl, *ConstructExpr,
                                   InferableSlots, InferableSlotsConstraint,
                                   Lattice, Env, Emit);
}

void collectEvidenceFromReturn(
    std::vector<std::pair<PointerTypeNullability, Slot>> &InferableSlots,
    const Formula &InferableSlotsConstraint, const CFGElement &Element,
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
      getNullability(ReturnExpr, Env, &InferableSlotsConstraint);
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

void collectEvidenceFromAssignment(
    std::vector<std::pair<PointerTypeNullability, Slot>> &InferableSlots,
    const Formula &InferableSlotsConstraint, const CFGElement &Element,
    const PointerNullabilityLattice &Lattice, const dataflow::Environment &Env,
    llvm::function_ref<EvidenceEmitter> Emit) {
  auto CFGStmt = Element.getAs<clang::CFGStmt>();
  if (!CFGStmt) return;

  // Initialization of new decl.
  if (auto *DeclStmt = dyn_cast_or_null<clang::DeclStmt>(CFGStmt->getStmt())) {
    for (auto *Decl : DeclStmt->decls()) {
      if (auto *VarDecl = dyn_cast_or_null<clang::VarDecl>(Decl);
          VarDecl && isSupportedPointerType(VarDecl->getType()) &&
          VarDecl->hasInit()) {
        auto *PV = getPointerValueFromExpr(VarDecl->getInit(), Env);
        if (!PV) return;
        TypeNullability TypeNullability =
            getNullabilityAnnotationsFromTypeAndOverrides(VarDecl->getType(),
                                                          VarDecl, Lattice);
        collectEvidenceFromBindingToType(
            TypeNullability, *PV, InferableSlots, InferableSlotsConstraint, Env,
            VarDecl->getInit()->getExprLoc(), Emit);
      }
    }
  }

  // Assignment to existing decl.
  if (auto *BinaryOperator =
          dyn_cast_or_null<clang::BinaryOperator>(CFGStmt->getStmt());
      BinaryOperator && BinaryOperator->isAssignmentOp() &&
      isSupportedPointerType(BinaryOperator->getLHS()->getType())) {
    auto *PV = getPointerValueFromExpr(BinaryOperator->getRHS(), Env);
    if (!PV) return;
    TypeNullability TypeNullability;
    if (auto *DeclRefExpr =
            dyn_cast_or_null<clang::DeclRefExpr>(BinaryOperator->getLHS())) {
      TypeNullability = getNullabilityAnnotationsFromTypeAndOverrides(
          BinaryOperator->getLHS()->getType(), DeclRefExpr->getDecl(), Lattice);
    } else {
      TypeNullability = getNullabilityAnnotationsFromType(
          BinaryOperator->getLHS()->getType());
    }
    collectEvidenceFromBindingToType(
        TypeNullability, *PV, InferableSlots, InferableSlotsConstraint, Env,
        BinaryOperator->getRHS()->getExprLoc(), Emit);
  }
}

void collectEvidenceFromElement(
    std::vector<std::pair<PointerTypeNullability, Slot>> InferableSlots,
    const Formula &InferableSlotsConstraint, const CFGElement &Element,
    const PointerNullabilityLattice &Lattice, const Environment &Env,
    llvm::function_ref<EvidenceEmitter> Emit) {
  collectEvidenceFromDereference(InferableSlots, Element, Env, Emit);
  collectEvidenceFromCallExpr(InferableSlots, InferableSlotsConstraint, Element,
                              Lattice, Env, Emit);
  collectEvidenceFromConstructExpr(InferableSlots, InferableSlotsConstraint,
                                   Element, Lattice, Env, Emit);
  collectEvidenceFromReturn(InferableSlots, InferableSlotsConstraint, Element,
                            Env, Emit);
  collectEvidenceFromAssignment(InferableSlots, InferableSlotsConstraint,
                                Element, Lattice, Env, Emit);
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

// Returns a function that the analysis can use to override Decl nullability
// values from the source code being analyzed with previously inferred
// nullabilities.
//
// In practice, this should only override the default nullability for Decls that
// do not spell out a nullability in source code, because we only pass in
// inferences from the previous round which are non-trivial and annotations
// "inferred" by reading an annotation from source code in the previous round
// were marked trivial.
auto getConcreteNullabilityOverrideFromPreviousInferences(
    ConcreteNullabilityCache &Cache, USRCache &USRCache,
    const PreviousInferences &PreviousInferences) {
  return [&](const Decl &D) -> std::optional<const PointerTypeNullability *> {
    auto [It, Inserted] = Cache.try_emplace(&D);
    if (Inserted) {
      std::optional<const Decl *> fingerprintedDecl;
      Slot Slot;
      if (auto *FD = clang::dyn_cast_or_null<FunctionDecl>(&D)) {
        fingerprintedDecl = FD;
        Slot = SLOT_RETURN_TYPE;
      } else if (auto *PD = clang::dyn_cast_or_null<ParmVarDecl>(&D)) {
        if (auto *Parent = clang::dyn_cast_or_null<FunctionDecl>(
                PD->getParentFunctionOrMethod())) {
          fingerprintedDecl = Parent;
          Slot = paramSlot(PD->getFunctionScopeIndex());
        }
      }
      if (!fingerprintedDecl) return std::nullopt;
      auto fp =
          fingerprint(getOrGenerateUSR(USRCache, **fingerprintedDecl), Slot);
      if (PreviousInferences.Nullable.contains(fp)) {
        It->second.emplace(NullabilityKind::Nullable);
      } else if (PreviousInferences.Nonnull.contains(fp)) {
        It->second.emplace(NullabilityKind::NonNull);
      } else {
        It->second = std::nullopt;
      }
    }
    if (!It->second) return std::nullopt;
    return &*It->second;
  };
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
  const auto &InferableSlotsConstraint =
      getInferableSlotsAsInferredOrUnknownConstraint(
          InferableSlots, USRCache, PreviousInferences, AnalysisContext.arena(),
          Decl);

  ConcreteNullabilityCache ConcreteNullabilityCache;
  Analysis.assignNullabilityOverride(
      getConcreteNullabilityOverrideFromPreviousInferences(
          ConcreteNullabilityCache, USRCache, PreviousInferences));

  return dataflow::runDataflowAnalysis(
             *ControlFlowContext, Analysis, Environment,
             [&](const CFGElement &Element,
                 const dataflow::DataflowAnalysisState<
                     PointerNullabilityLattice> &State) {
               collectEvidenceFromElement(InferableSlots,
                                          InferableSlotsConstraint, Element,
                                          State.Lattice, State.Env, Emit);
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
