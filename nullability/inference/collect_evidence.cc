// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/collect_evidence.h"

#include <algorithm>
#include <cassert>
#include <memory>
#include <optional>
#include <string>
#include <string_view>
#include <utility>
#include <vector>

#include "absl/base/nullability.h"
#include "absl/container/flat_hash_map.h"
#include "absl/log/check.h"
#include "nullability/ast_helpers.h"
#include "nullability/inference/inferable.h"
#include "nullability/inference/inference.proto.h"
#include "nullability/inference/slot_fingerprint.h"
#include "nullability/macro_arg_capture.h"
#include "nullability/pointer_nullability.h"
#include "nullability/pointer_nullability_analysis.h"
#include "nullability/pointer_nullability_lattice.h"
#include "nullability/type_nullability.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/DeclGroup.h"
#include "clang/AST/Expr.h"
#include "clang/AST/ExprCXX.h"
#include "clang/AST/OperationKinds.h"
#include "clang/AST/RecursiveASTVisitor.h"
#include "clang/AST/Stmt.h"
#include "clang/AST/Type.h"
#include "clang/Analysis/CFG.h"
#include "clang/Analysis/FlowSensitive/ASTOps.h"
#include "clang/Analysis/FlowSensitive/AdornedCFG.h"
#include "clang/Analysis/FlowSensitive/Arena.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysis.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysisContext.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/Formula.h"
#include "clang/Analysis/FlowSensitive/Value.h"
#include "clang/Analysis/FlowSensitive/WatchedLiteralsSolver.h"
#include "clang/Basic/Builtins.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/OperatorKinds.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Basic/Specifiers.h"
#include "clang/Index/USRGeneration.h"
#include "llvm/ADT/DenseSet.h"
#include "llvm/ADT/FunctionExtras.h"
#include "llvm/ADT/STLFunctionalExtras.h"
#include "llvm/Support/Errc.h"
#include "llvm/Support/Error.h"
#include "llvm/Support/raw_ostream.h"

namespace clang::tidy::nullability {
using ::clang::dataflow::DataflowAnalysisContext;
using ::clang::dataflow::Environment;
using ::clang::dataflow::Formula;
using ::clang::dataflow::RecordInitListHelper;
using ::clang::dataflow::WatchedLiteralsSolver;

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
          << "Evidence emitted for a Target which is not an inference target: "
          << (dyn_cast<NamedDecl>(&Target)
                  ? dyn_cast<NamedDecl>(&Target)->getQualifiedNameAsString()
                  : "not a named decl");

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
class InferableSlot {
 public:
  InferableSlot(PointerTypeNullability Nullability, Slot Slot, const Decl &Decl)
      : SymbolicNullability(Nullability),
        TargetSlot(Slot),
        InferenceTarget(Decl) {}

  const PointerTypeNullability &getSymbolicNullability() const {
    return SymbolicNullability;
  }
  Slot getTargetSlot() const { return TargetSlot; }
  const Decl &getInferenceTarget() const { return InferenceTarget; }

 private:
  const PointerTypeNullability SymbolicNullability;
  const Slot TargetSlot;
  const Decl &InferenceTarget;
};
}  // namespace

/// If Stmt is a dereference, returns its target and location.
static std::pair<const Expr *, SourceLocation> describeDereference(
    const Stmt &Stmt) {
  if (auto *Op = dyn_cast<UnaryOperator>(&Stmt);
      Op && Op->getOpcode() == UO_Deref) {
    return {Op->getSubExpr(), Op->getOperatorLoc()};
  }
  if (auto *ME = dyn_cast<MemberExpr>(&Stmt); ME && ME->isArrow()) {
    return {ME->getBase(), ME->getOperatorLoc()};
  }
  // pointers to members; at the time of writing, they aren't a supported
  // pointer type, so this is a no-op.
  if (const auto *BO = dyn_cast<BinaryOperator>(&Stmt);
      BO && (BO->getOpcode() == clang::BinaryOperatorKind::BO_PtrMemD ||
             BO->getOpcode() == clang::BinaryOperatorKind::BO_PtrMemI)) {
    return {BO->getRHS(), BO->getOperatorLoc()};
  }
  if (const auto *OCE = dyn_cast<CXXOperatorCallExpr>(&Stmt);
      OCE && OCE->getOperator() == clang::OO_Star &&
      isSupportedSmartPointerType(OCE->getArg(0)->getType())) {
    return {OCE->getArg(0), OCE->getOperatorLoc()};
  }
  return {nullptr, SourceLocation()};
}

/// Inferable slots are nullability slots not explicitly annotated in source
/// code that we are currently capable of handling. This returns a boolean
/// constraint representing these slots having a) the nullability inferred from
/// the previous round for this slot or b) Unknown nullability if no inference
/// was made in the previous round or there was no previous round.
static const Formula &getInferableSlotsAsInferredOrUnknownConstraint(
    const std::vector<InferableSlot> &InferableSlots, USRCache &USRCache,
    const PreviousInferences &PreviousInferences, dataflow::Arena &A) {
  const Formula *Constraint = &A.makeLiteral(true);
  for (auto &IS : InferableSlots) {
    std::string_view USR = getOrGenerateUSR(USRCache, IS.getInferenceTarget());
    SlotFingerprint Fingerprint = fingerprint(USR, IS.getTargetSlot());
    auto Nullability = IS.getSymbolicNullability();
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

static auto getNullabilityAnnotationsFromTypeAndOverrides(
    QualType Type, absl::Nonnull<const Decl *> D,
    const PointerNullabilityLattice &Lattice) {
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

static Evidence::Kind getArgEvidenceKindFromNullability(
    NullabilityKind Nullability) {
  switch (Nullability) {
    case NullabilityKind::Nullable:
      return Evidence::NULLABLE_ARGUMENT;
    case NullabilityKind::NonNull:
      return Evidence::NONNULL_ARGUMENT;
    default:
      return Evidence::UNKNOWN_ARGUMENT;
  }
}

static std::optional<Evidence::Kind> evidenceKindFromDeclaredType(QualType T) {
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

namespace {
class DefinitionEvidenceCollector {
 public:
  // Instantiate the class only in this static function, to restrict the
  // lifetime of the object, which holds reference parameters.
  static void collect(std::vector<InferableSlot> &InferableSlots,
                      const Formula &InferableSlotsConstraint,
                      llvm::function_ref<EvidenceEmitter> Emit,
                      const CFGElement &CFGElem,
                      const PointerNullabilityLattice &Lattice,
                      const Environment &Env) {
    DefinitionEvidenceCollector Collector(
        InferableSlots, InferableSlotsConstraint, Emit, Lattice, Env);
    if (auto CFGStmt = CFGElem.getAs<clang::CFGStmt>()) {
      const Stmt *S = CFGStmt->getStmt();
      if (!S) return;
      Collector.fromDereference(*S);
      Collector.fromCallExpr(*S);
      Collector.fromConstructExpr(*S);
      Collector.fromReturn(*S);
      Collector.fromAssignment(*S);
      Collector.fromArithmetic(*S);
      Collector.fromAggregateInitialization(*S);
    } else if (auto CFGInit = CFGElem.getAs<clang::CFGInitializer>()) {
      Collector.fromCFGInitializer(*CFGInit);
    }
  }

 private:
  DefinitionEvidenceCollector(std::vector<InferableSlot> &InferableSlots,
                              const Formula &InferableSlotsConstraint,
                              llvm::function_ref<EvidenceEmitter> Emit,
                              const PointerNullabilityLattice &Lattice,
                              const Environment &Env)
      : InferableSlots(InferableSlots),
        InferableSlotsConstraint(InferableSlotsConstraint),
        Emit(Emit),
        Lattice(Lattice),
        Env(Env) {}

  /// Records evidence derived from the necessity that `Value` is nonnull.
  /// It may be dereferenced, passed as a nonnull param, etc, per
  /// `EvidenceKind`.
  void mustBeNonnull(const dataflow::PointerValue &Value, SourceLocation Loc,
                     Evidence::Kind EvidenceKind) {
    CHECK(hasPointerNullState(Value))
        << "Value should be the value of an expression. Cannot collect "
           "evidence for nonnull-ness if there is no null state.";
    auto *IsNull = getPointerNullState(Value).IsNull;
    // If `IsNull` is top, we can't infer anything about it.
    if (IsNull == nullptr) return;
    auto &A = Env.arena();
    mustBeTrue(A.makeNot(*IsNull), Loc, EvidenceKind);
  }

  /// Records evidence for Nonnull-ness derived from the necessity that
  /// `MustBeTrue` must be true.
  ///
  /// Does not consider the possibility that the formula can only be proven true
  /// by marking a slot Nullable, as this is is not a pattern we have yet seen
  /// in practice. This function could easily be extended to do so, though.
  void mustBeTrue(const Formula &MustBeTrue, SourceLocation Loc,
                  Evidence::Kind EvidenceKind) {
    auto &A = Env.arena();
    // If `Value` is already proven true or false (or both, which indicates
    // unsatisfiable flow conditions), collect no evidence.
    if (Env.proves(MustBeTrue) || Env.proves(A.makeNot(MustBeTrue))) return;

    for (auto &IS : InferableSlots) {
      auto &SlotNonnull = IS.getSymbolicNullability().isNonnull(A);
      auto &SlotNonnullImpliesFormulaTrue =
          A.makeImplies(SlotNonnull, MustBeTrue);
      // Don't collect evidence if the implication is true by virtue of
      // `SlotNonnull` being false.
      //
      // In practice, `SlotNonnull` can be made false by a flow condition, and
      // marking the slot Nonnull would make that conditioned block dead code.
      // Technically, this does make a dereference, etc. "safe", but we'd prefer
      // to mark a different slot Nonnull that has a more direct relationship
      // with `MustBeTrue`.
      //
      // e.g. We'd prefer to mark `q` Nonnull rather than `p` in the following:
      // ```
      // void target(int* p, int* q) {
      //   if (!p) {
      //     *q;
      //   }
      // }
      // ```
      if (Env.allows(SlotNonnull) &&
          Env.proves(SlotNonnullImpliesFormulaTrue)) {
        Emit(IS.getInferenceTarget(), IS.getTargetSlot(), EvidenceKind, Loc);
        return;
      }
    }
  }

  void fromDereference(const Stmt &S) {
    auto [Target, Loc] = describeDereference(S);
    if (!Target || !isSupportedPointerType(Target->getType())) return;

    // It is a dereference of a pointer. Now gather evidence from it.
    dataflow::PointerValue *DereferencedValue = getPointerValue(Target, Env);
    if (!DereferencedValue) return;
    mustBeNonnull(*DereferencedValue, Loc, Evidence::UNCHECKED_DEREFERENCE);
  }

  /// Collect evidence for each of `InferableSlots` if that slot being marked
  /// Nullable would imply `Value`'s FromNullable property.
  ///
  /// This function is called when we have reason to believe that `Value` must
  /// be Nullable. As we can't directly retrieve the combination of Decl and
  /// Slot that corresponds to `Value`'s nullability, we consider each inferable
  /// slot and emit evidence for all inferable slots that, if marked Nullable,
  /// cause `Value` to be considered explicitly Nullable.
  void mustBeMarkedNullable(const dataflow::PointerValue &Value,
                            SourceLocation Loc, Evidence::Kind EvidenceKind) {
    CHECK(hasPointerNullState(Value))
        << "Value should be the value of an expression. Cannot collect "
           "evidence for nonnull-ness if there is no null state.";
    auto *FromNullable = getPointerNullState(Value).FromNullable;
    // If `FromNullable` is top, we can't infer anything about it.
    if (FromNullable == nullptr) return;
    // If the flow conditions already imply that `Value` is from a Nullable,
    // then we don't have any new evidence of a necessary annotation.
    if (Env.proves(*FromNullable)) return;

    auto &A = Env.arena();
    // Otherwise, if an inferable slot being annotated Nullable would imply that
    // `Value` is from a Nullable, then we have evidence suggesting that slot
    // should be annotated. We collect this evidence for every slot that
    // connects in this way to `Value`.
    //
    // e.g. We should mark both `p` and `q` Nullable in the following:
    // ```
    // void target(int* p, int* q, bool b) {
    //   Nullable<int*>& x = b ? p : q;
    //   ...
    // }
    // ```
    // because at runtime, either `p` or `q` could be taken as a mutable
    // reference and later set to nullptr.
    for (auto &IS : InferableSlots) {
      auto &SlotNullableImpliesValueFromNullable = A.makeImplies(
          IS.getSymbolicNullability().isNullable(A), *FromNullable);
      if (Env.proves(SlotNullableImpliesValueFromNullable))
        Emit(IS.getInferenceTarget(), IS.getTargetSlot(), EvidenceKind, Loc);
    }
  }

  void fromAssignmentToType(QualType Type, TypeNullability &TypeNullability,
                            const dataflow::PointerValue &PointerValue,
                            SourceLocation ValueLoc) {
    //  TODO: Account for variance and each layer of nullability when we handle
    //  more than top-level pointers.
    if (TypeNullability.empty()) return;
    PointerTypeNullability &TopLevel = TypeNullability[0];
    dataflow::Arena &A = Env.arena();
    if (TopLevel.concrete() == NullabilityKind::NonNull ||
        (TopLevel.isSymbolic() &&
         Env.proves(
             A.makeImplies(InferableSlotsConstraint, TopLevel.isNonnull(A))))) {
      mustBeNonnull(PointerValue, ValueLoc, Evidence::BOUND_TO_NONNULL);
    } else if (!Type.isConstQualified() && Type->isReferenceType() &&
               (TopLevel.concrete() == NullabilityKind::Nullable ||
                (TopLevel.isSymbolic() &&
                 Env.proves(A.makeImplies(InferableSlotsConstraint,
                                          TopLevel.isNullable(A)))))) {
      mustBeMarkedNullable(PointerValue, ValueLoc,
                           Evidence::BOUND_TO_MUTABLE_NULLABLE);
    }
  }

  template <typename CallOrConstructExpr>
  void fromArgsAndParams(const FunctionDecl &CalleeDecl,
                         const CallOrConstructExpr &Expr) {
    bool CollectEvidenceForCallee = isInferenceTarget(CalleeDecl);
    bool CollectEvidenceForCaller = !InferableSlots.empty();

    for (ParamAndArgIterator<CallOrConstructExpr> Iter(CalleeDecl, Expr); Iter;
         ++Iter) {
      const auto ParamType = Iter.param().getType().getNonReferenceType();
      if (!isSupportedPointerType(ParamType)) continue;
      if (!isSupportedPointerType(Iter.arg().getType())) {
        // These builtins are declared with pointer type parameters even when
        // given a valid argument of type uintptr_t. In this case, there's
        // nothing to infer, but also nothing unexpected to crash over.
        auto BuiltinID = CalleeDecl.getBuiltinID();
        if (BuiltinID == Builtin::BI__builtin_is_aligned ||
            BuiltinID == Builtin::BI__builtin_align_up ||
            BuiltinID == Builtin::BI__builtin_align_down) {
          continue;
        }
      }
      // the corresponding argument should also be a pointer.
      CHECK(isSupportedPointerType(Iter.arg().getType()))
          << "Unsupported argument " << Iter.argIdx()
          << " type: " << Iter.arg().getType().getAsString();

      if (isa<clang::CXXDefaultArgExpr>(Iter.arg())) {
        // Evidence collection for the callee from default argument values is
        // handled when collecting from declarations, and there's no useful
        // evidence available to collect for the caller.
        return;
      }

      dataflow::PointerValue *PV = getPointerValue(&Iter.arg(), Env);
      if (!PV) continue;

      SourceLocation ArgLoc = Iter.arg().getExprLoc();

      if (CollectEvidenceForCaller) {
        auto ParamNullability = getNullabilityAnnotationsFromTypeAndOverrides(
            ParamType, &Iter.param(), Lattice);

        // Collect evidence from constraints that the parameter's nullability
        // places on the argument's nullability.
        fromAssignmentToType(Iter.param().getType(), ParamNullability, *PV,
                             ArgLoc);
      }

      if (CollectEvidenceForCallee) {
        // Emit evidence of the parameter's nullability. First, calculate that
        // nullability based on InferableSlots for the caller being assigned to
        // Unknown or their previously-inferred value, to reflect the current
        // annotations and not all possible annotations for them.
        NullabilityKind ArgNullability =
            getNullability(*PV, Env, &InferableSlotsConstraint);
        Emit(CalleeDecl, paramSlot(Iter.paramIdx()),
             getArgEvidenceKindFromNullability(ArgNullability), ArgLoc);
      }
    }
  }

  /// Collects evidence from the assignment of function arguments to the types
  /// of the corresponding parameter, used when we have a FunctionProtoType but
  /// no FunctionDecl.
  /// TODO: When we collect evidence for more complex slots than just top-level
  /// pointers, emit evidence of the function parameter's nullability as a slot
  /// in the appropriate declaration.
  void fromFunctionProtoTypeCall(const FunctionProtoType &CalleeType,
                                 const CallExpr &Expr) {
    // For each pointer parameter of the function, ...
    for (unsigned I = 0; I < CalleeType.getNumParams(); ++I) {
      const auto ParamType = CalleeType.getParamType(I);
      if (!isSupportedPointerType(ParamType.getNonReferenceType())) continue;
      // the corresponding argument should also be a pointer.
      CHECK(isSupportedPointerType(Expr.getArg(I)->getType()))
          << "Unsupported argument " << I
          << " type: " << Expr.getArg(I)->getType().getAsString();

      dataflow::PointerValue *PV = getPointerValue(Expr.getArg(I), Env);
      if (!PV) continue;

      // TODO: when we infer function pointer/reference parameters'
      // nullabilities, check for overrides from previous inference iterations.
      auto ParamNullability = getNullabilityAnnotationsFromType(ParamType);

      // Collect evidence from constraints that the parameter's nullability
      // places on the argument's nullability.
      fromAssignmentToType(ParamType, ParamNullability, *PV,
                           Expr.getArg(I)->getExprLoc());
    }
  }

  /// Collect evidence that the function pointer was dereferenced and from the
  /// matching up of parameter/argument nullabilities.
  void fromFunctionPointerCallExpr(const Type &CalleeFunctionType,
                                   const CallExpr &Expr) {
    if (InferableSlots.empty()) return;
    if (const auto *Callee = Expr.getCallee()) {
      // Function pointers are only ever raw pointers.
      if (const auto *PV = getRawPointerValue(Callee, Env)) {
        mustBeNonnull(*PV, Expr.getExprLoc(), Evidence::UNCHECKED_DEREFERENCE);
      }
    }

    auto *CalleeFunctionProtoType =
        CalleeFunctionType.getAs<FunctionProtoType>();
    CHECK(CalleeFunctionProtoType);
    fromFunctionProtoTypeCall(*CalleeFunctionProtoType, Expr);
  }

  /// Handles the case of a call to a function without a FunctionDecl, e.g. that
  /// is provided as a parameter or another decl, e.g. a field or local
  /// variable.
  ///
  /// Example: We can collect evidence for the nullability of `p` and (when we
  /// handle more than top-level pointer slots) `j` in the following, based on
  /// the call to `callee`:
  /// ```
  ///  void target(int* p, void (*callee)(Nonnull<int*> i, int* j)) {
  ///    callee(p, nullptr);
  ///  }
  /// ```
  ///
  /// With `CalleeDecl` in this case not being a FunctionDecl as in most
  /// CallExpr cases, distinct handling is needed.
  void fromCallExprWithoutFunctionCalleeDecl(const Decl &CalleeDecl,
                                             const CallExpr &Expr) {
    if (CalleeDecl.isFunctionPointerType()) {
      fromFunctionPointerCallExpr(*CalleeDecl.getFunctionType(), Expr);
      return;
    }

    // Ignore calls of pointers to members. The dereferencing of the pointer is
    // handled as a dereference at the BinaryOperator node, which additionally
    // captures pointers to fields.
    // TODO(b/309625642) Consider collecting evidence for the arguments being
    // passed as parameters to the pointed-to member.
    if (const auto *BinaryOpCallee = dyn_cast_or_null<BinaryOperator>(
            Expr.getCallee()->IgnoreParenImpCasts());
        BinaryOpCallee &&
        (BinaryOpCallee->getOpcode() == clang::BinaryOperatorKind::BO_PtrMemD ||
         BinaryOpCallee->getOpcode() ==
             clang::BinaryOperatorKind::BO_PtrMemI)) {
      return;
    }

    // Function references are a rare case, but similar to function pointers, we
    // can collect evidence from arguments bound to parameter types.
    if (auto *FuncType = CalleeDecl.getFunctionType()) {
      if (auto *FuncProtoType = FuncType->getAs<FunctionProtoType>()) {
        fromFunctionProtoTypeCall(*FuncProtoType, Expr);
        return;
      }
    }

    // A reference to a function pointer is another rare case, but we can
    // collect the same evidence we would for a function pointer.
    if (const auto *CalleeAsValueDecl =
            dyn_cast<clang::ValueDecl>(&CalleeDecl)) {
      if (QualType CalleeType = CalleeAsValueDecl->getType();
          CalleeType.getNonReferenceType()->isFunctionPointerType()) {
        fromFunctionPointerCallExpr(
            *(CalleeType.getNonReferenceType()->getPointeeType()), Expr);
        return;
      }
    }

    // If we run into other cases meeting this criterion, skip them, but log
    // first so we can potentially add support later.
    llvm::errs() << "Unsupported case of a CallExpr without a FunctionDecl. "
                    "Not collecting any evidence from this CallExpr:\n";
    Expr.getBeginLoc().dump(CalleeDecl.getASTContext().getSourceManager());
    Expr.dump();
    llvm::errs() << "Which is a call to:\n";
    CalleeDecl.dump();
  }

  /// Given a `CallExpr` for a call to our special macro single-argument capture
  /// function, collect evidence for a slot that can prevent the abort condition
  /// from being true if it is annotated Nonnull.
  ///
  /// e.g. From `CHECK(x)`, we collect evidence for a slot that can cause `x` to
  /// not be null.
  void fromAbortIfFalseMacroCall(const CallExpr &CallExpr) {
    CHECK_EQ(CallExpr.getNumArgs(), 1);
    const Expr *Arg = CallExpr.getArg(0);
    if (!Arg) return;
    QualType ArgType = Arg->getType();
    if (isSupportedPointerType(ArgType)) {
      const dataflow::PointerValue *PV = getPointerValue(Arg, Env);
      if (!PV) return;
      mustBeNonnull(*PV, Arg->getExprLoc(), Evidence::ABORT_IF_NULL);
    } else if (ArgType->isBooleanType()) {
      const dataflow::BoolValue *BV = Env.get<dataflow::BoolValue>(*Arg);
      if (!BV || BV->getKind() == dataflow::BoolValue::Kind::TopBool) return;
      mustBeTrue(BV->formula(), Arg->getExprLoc(), Evidence::ABORT_IF_NULL);
    }
  }

  /// Given a `CallExpr` for a call to our special macro two-argument capture
  /// function for not-equal checks, if one of the arguments is a nullptr
  /// constant or provably null, collect evidence for a slot that can prevent
  /// the other argument from being null.
  ///
  /// e.g. From `CHECK_NE(x, nullptr)`, we collect evidence for a slot that can
  /// cause `x` to not be null.
  void fromAbortIfEqualMacroCall(const CallExpr &CallExpr) {
    CHECK_EQ(CallExpr.getNumArgs(), 2);
    const Expr *First = CallExpr.getArg(0);
    const Expr *Second = CallExpr.getArg(1);
    bool FirstSupported = isSupportedPointerType(First->getType());
    bool SecondSupported = isSupportedPointerType(Second->getType());
    if (!FirstSupported && !SecondSupported) return;

    ASTContext &Context = CallExpr.getCalleeDecl()->getASTContext();
    const dataflow::PointerValue *ValueComparedToNull = nullptr;
    SourceLocation EvidenceLoc;
    if (First->isNullPointerConstant(Context,
                                     Expr::NPC_ValueDependentIsNotNull)) {
      if (!isSupportedPointerType(Second->getType())) return;
      ValueComparedToNull = getPointerValue(Second, Env);
      if (!ValueComparedToNull) return;
      EvidenceLoc = Second->getExprLoc();
    } else if (Second->isNullPointerConstant(
                   Context, Expr::NPC_ValueDependentIsNotNull)) {
      if (!isSupportedPointerType(First->getType())) return;
      ValueComparedToNull = getPointerValue(First, Env);
      if (!ValueComparedToNull) return;
      EvidenceLoc = First->getExprLoc();
    } else {
      if (!FirstSupported || !SecondSupported) {
        // If this happens outside of the nullptr literal case, we'd like to
        // know about it.
        llvm::errs() << "Value of a supported pointer type compared to a value "
                        "of a type that is not a supported pointer type.: \n";
        CallExpr.dump();
        CallExpr.getExprLoc().dump(
            CallExpr.getCalleeDecl()->getASTContext().getSourceManager());
        return;
      }

      const dataflow::PointerValue *FirstPV = getPointerValue(First, Env);
      if (!FirstPV) return;
      const dataflow::PointerValue *SecondPV = getPointerValue(Second, Env);
      if (!SecondPV) return;
      PointerNullState FirstNullState = getPointerNullState(*FirstPV);
      if (!FirstNullState.IsNull) return;
      PointerNullState SecondNullState = getPointerNullState(*SecondPV);
      if (!SecondNullState.IsNull) return;

      if (Env.proves(*FirstNullState.IsNull)) {
        ValueComparedToNull = SecondPV;
        EvidenceLoc = Second->getExprLoc();
      } else if (Env.proves(*SecondNullState.IsNull)) {
        ValueComparedToNull = FirstPV;
        EvidenceLoc = First->getExprLoc();
      } else {
        return;
      }
    }

    mustBeNonnull(*ValueComparedToNull, EvidenceLoc, Evidence::ABORT_IF_NULL);
  }

  void fromCallExpr(const Stmt &S) {
    auto *CallExpr = dyn_cast<clang::CallExpr>(&S);
    if (!CallExpr) return;
    auto *CalleeDecl = CallExpr->getCalleeDecl();
    if (!CalleeDecl) return;
    if (auto *CalleeFunctionDecl = dyn_cast<clang::FunctionDecl>(CalleeDecl)) {
      if (CalleeFunctionDecl->getDeclName().isIdentifier()) {
        llvm::StringRef Name = CalleeFunctionDecl->getName();
        if (Name == ArgCaptureAbortIfFalse) {
          fromAbortIfFalseMacroCall(*CallExpr);
          return;
        }
        if (Name == ArgCaptureAbortIfEqual) {
          fromAbortIfEqualMacroCall(*CallExpr);
          return;
        }
      }
      fromArgsAndParams(*CalleeFunctionDecl, *CallExpr);
    } else {
      fromCallExprWithoutFunctionCalleeDecl(*CalleeDecl, *CallExpr);
    }
  }

  void fromConstructExpr(const Stmt &S) {
    auto *ConstructExpr = dyn_cast<clang::CXXConstructExpr>(&S);
    if (!ConstructExpr) return;
    auto *ConstructorDecl = dyn_cast_or_null<clang::CXXConstructorDecl>(
        ConstructExpr->getConstructor());
    if (!ConstructorDecl) return;

    fromArgsAndParams(*ConstructorDecl, *ConstructExpr);
  }

  void fromReturn(const Stmt &S) {
    // Is this CFGElement a return statement?
    auto *ReturnStmt = dyn_cast<clang::ReturnStmt>(&S);
    if (!ReturnStmt) return;
    auto *ReturnExpr = ReturnStmt->getRetValue();
    if (!ReturnExpr || !isSupportedPointerType(ReturnExpr->getType())) return;
    const FunctionDecl *CurrentFunc = Env.getCurrentFunc();
    CHECK(CurrentFunc) << "A return statement outside of a function?";

    // Skip gathering evidence about the current function's return type if
    // the current function is not an inference target or the return type
    // already includes an annotation.
    if (isInferenceTarget(*CurrentFunc) &&
        !evidenceKindFromDeclaredType(CurrentFunc->getReturnType())) {
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

    const dataflow::PointerValue *PV = getPointerValue(ReturnExpr, Env);
    if (!PV) return;
    TypeNullability ReturnTypeNullability =
        getNullabilityAnnotationsFromTypeAndOverrides(
            CurrentFunc->getReturnType(),
            // The FunctionDecl is the key used for overrides for the return
            // type. To look up overrides for parameters, we would pass a
            // ParmVarDecl.
            CurrentFunc, Lattice);
    fromAssignmentToType(Env.getCurrentFunc()->getReturnType(),
                         ReturnTypeNullability, *PV, ReturnExpr->getExprLoc());
  }

  /// Checks whether PointerValue is null or nullable and if so, collects
  /// evidence for a slot that would, if marked Nullable, cause
  /// TypeNullability's first-layer nullability to be Nullable.
  ///
  /// e.g. This is used for example to collect from the following:
  /// ```
  /// void target(int* p, int* q, NullabilityUnknown<int*> r) {
  ///   p = nullptr;
  ///   if (!r) {
  ///     q = r;
  ///   }
  /// }
  /// ```
  /// evidence for each of the assignments of `p` and `q` that they were
  /// ASSIGNED_FROM_NULLABLE.
  void fromAssignmentFromNullable(TypeNullability &TypeNullability,
                                  const dataflow::PointerValue &PointerValue,
                                  SourceLocation ValueLoc,
                                  Evidence::Kind EvidenceKind) {
    if (TypeNullability.empty() || !hasPointerNullState(PointerValue)) return;
    dataflow::Arena &A = Env.arena();
    if (getNullability(PointerValue, Env, &InferableSlotsConstraint) ==
        NullabilityKind::Nullable) {
      const Formula &TypeIsNullable = TypeNullability[0].isNullable(A);
      if (!Env.allows(TypeIsNullable)) return;

      for (auto &IS : InferableSlots) {
        auto &Implication = A.makeImplies(
            IS.getSymbolicNullability().isNullable(A), TypeIsNullable);
        // It's not expected that a slot's isNullable formula could be proven
        // false by the environment alone (without the
        // InferableSlotsConstraint), but SAT calls are relatively expensive, so
        // only DCHECK.
        DCHECK(Env.allows(IS.getSymbolicNullability().isNullable(A)));
        if (Env.proves(Implication)) {
          Emit(IS.getInferenceTarget(), IS.getTargetSlot(), EvidenceKind,
               ValueLoc);
          return;
        }
      }
    }
  }

  /// Collects evidence based on an assignment of Expr to LHSDecl, through a
  /// direct assignment statement, aggregate initialization, etc.
  void fromAssignmentLike(const ValueDecl &LHSDecl, const Expr &Expr,
                          SourceLocation Loc,
                          Evidence::Kind EvidenceKindForAssignmentFromNullable =
                              Evidence::ASSIGNED_FROM_NULLABLE) {
    const dataflow::PointerValue *PV = getPointerValue(&Expr, Env);
    if (!PV) return;
    TypeNullability TypeNullability =
        getNullabilityAnnotationsFromTypeAndOverrides(LHSDecl.getType(),
                                                      &LHSDecl, Lattice);
    fromAssignmentToType(LHSDecl.getType(), TypeNullability, *PV, Loc);
    fromAssignmentFromNullable(TypeNullability, *PV, Loc,
                               EvidenceKindForAssignmentFromNullable);
  }

  /// Collects evidence from direct assignment statements, e.g. `p = nullptr`,
  /// whether initializing a new declaration or re-assigning to an existing
  /// declaration.
  void fromAssignment(const Stmt &S) {
    if (InferableSlots.empty()) return;

    // Initialization of new decl.
    if (auto *DeclStmt = dyn_cast<clang::DeclStmt>(&S)) {
      for (auto *Decl : DeclStmt->decls()) {
        if (auto *VarDecl = dyn_cast<clang::VarDecl>(Decl);
            VarDecl && VarDecl->hasInit()) {
          bool DeclTypeSupported =
              isSupportedPointerType(VarDecl->getType().getNonReferenceType());
          bool InitTypeSupported = isSupportedPointerType(
              VarDecl->getInit()->getType().getNonReferenceType());
          if (!DeclTypeSupported) return;
          if (!InitTypeSupported) {
            llvm::errs() << "Unsupported init type for pointer decl: "
                         << VarDecl->getInit()->getType() << "\n";
            return;
          }
          fromAssignmentLike(*VarDecl, *VarDecl->getInit(),
                             VarDecl->getInit()->getExprLoc());
        }
      }
    }

    // Assignment to existing decl.
    if (auto *BinaryOperator = dyn_cast<clang::BinaryOperator>(&S);
        BinaryOperator &&
        BinaryOperator->getOpcode() == clang::BinaryOperatorKind::BO_Assign) {
      const Expr *LHS = BinaryOperator->getLHS();
      const QualType LHSType = LHS->getType();
      if (!isSupportedRawPointerType(LHSType)) return;

      const ValueDecl *LHSDecl = nullptr;
      if (auto *DeclRefExpr = dyn_cast_or_null<clang::DeclRefExpr>(LHS)) {
        LHSDecl = DeclRefExpr->getDecl();
      } else if (auto *MemberExpr = dyn_cast_or_null<clang::MemberExpr>(LHS)) {
        LHSDecl = MemberExpr->getMemberDecl();
      } else if (auto *MemberCallExpr =
                     dyn_cast_or_null<clang::CXXMemberCallExpr>(LHS)) {
        LHSDecl = MemberCallExpr->getMethodDecl();
      } else if (auto *UnaryOp = dyn_cast_or_null<clang::UnaryOperator>(LHS)) {
        switch (UnaryOp->getOpcode()) {
          case UO_Deref:
            // An assignment like `*pp = nullptr`, where `pp` is a `T**`. We
            // don't do anything with these yet.
            // TODO(b/323509132) Handle these assignments.
            return;
          case UO_PreInc:
          case UO_PreDec:
            // An assignment like `++p = nullptr`, where `p` is a `T*`. This is
            // very rare, if used at all, but is for our purposes here the same
            // as `p = nullptr`.
            // TODO(b/309625642) Handle these assignments, possibly by
            // extracting this if/else block and recursing into the operand of
            // the arithmetic.
            return;
          default:
            // We don't expect to see any other unary operators here, but not to
            // a production-crash-worthy level, so assert instead of CHECK.
            llvm::errs() << "Unsupported LHS unary operator in assignment to "
                            "existing decl:\n";
            LHS->dump();
            assert(false);
            return;
        }
      } else if (auto *ArraySubscript =
                     dyn_cast_or_null<clang::ArraySubscriptExpr>(LHS)) {
        // An assignment like `a[0] = nullptr`, where `a` is an array of
        // pointers. We don't do anything with these yet.
        // TODO(b/323509132) Handle these assignments.
        return;
      } else {
        llvm::errs() << "Unsupported LHS expression type in assignment to "
                        "existing decl:\n";
        LHS->dump();
        return;
      }

      // Use the LHS as the Expr being assigned to LHSDecl because transfer
      // functions have already run, setting the Environment's value for the LHS
      // Expr to the result of the assignment. In cases where the RHS has been
      // modified, such as in `a = std::move(b)`, the Environment's value for
      // the RHS will no longer be an accurate representation of what was
      // assigned.
      fromAssignmentLike(*LHSDecl, *LHS, BinaryOperator->getOperatorLoc());
    }
  }

  void fromArithmeticArg(const Expr *Arg, SourceLocation Loc) {
    // No support needed for smart pointers, which do not support arithmetic
    // operations.
    if (!Arg || !isSupportedRawPointerType(Arg->getType())) return;
    if (auto *PV = getPointerValue(Arg, Env))
      mustBeNonnull(*PV, Loc, Evidence::ARITHMETIC);
  }

  void fromArithmetic(const Stmt &S) {
    // A nullptr can be added to 0 and nullptr can be subtracted from nullptr
    // without hitting UB. But for now, we skip handling these special cases and
    // assume all pointers involved in these operations must be nonnull.
    switch (S.getStmtClass()) {
      default:
        return;
      case Stmt::CompoundAssignOperatorClass: {
        auto *Op = cast<clang::CompoundAssignOperator>(&S);
        switch (Op->getOpcode()) {
          default:
            return;
          case BO_AddAssign:
          case BO_SubAssign:
            fromArithmeticArg(Op->getLHS(), Op->getExprLoc());
        }
        break;
      }
      case Stmt::BinaryOperatorClass: {
        auto *Op = cast<clang::BinaryOperator>(&S);
        switch (Op->getOpcode()) {
          default:
            return;
          case BO_Add:
          case BO_Sub:
            fromArithmeticArg(Op->getLHS(), Op->getExprLoc());
            fromArithmeticArg(Op->getRHS(), Op->getExprLoc());
        }
        break;
      }
      case Stmt::UnaryOperatorClass: {
        auto *Op = cast<clang::UnaryOperator>(&S);
        switch (Op->getOpcode()) {
          default:
            return;
          case UO_PostInc:
          case UO_PreInc:
          case UO_PostDec:
          case UO_PreDec:
            fromArithmeticArg(Op->getSubExpr(), Op->getExprLoc());
        }
        break;
      }
    }
  }

  void fromCFGInitializer(const CFGInitializer &CFGInit) {
    const CXXCtorInitializer *Initializer = CFGInit.getInitializer();
    if (!Initializer) {
      // We expect this not to be the case, but not to a production-crash-worthy
      // level, so assert instead of CHECK.
      llvm::errs() << "CFGInitializer with null CXXCtorInitializer.\n";
      CFGInit.dump();
      assert(Initializer);
    }

    // Base and delegating initializers are collected from when we see the
    // underlying CXXConstructExpr, so we don't need to handle those, only the
    // member initializers.
    const FieldDecl *Field = Initializer->getAnyMember();
    if (Field == nullptr) return;
    if (InferableSlots.empty()) return;
    bool IsDefaultInitializer = Initializer->isInClassMemberInitializer();
    const Expr *InitExpr = IsDefaultInitializer ? Field->getInClassInitializer()
                                                : Initializer->getInit();

    if (!isSupportedPointerType(Field->getType())) return;

    if (isSupportedSmartPointerType(Field->getType()) &&
        !IsDefaultInitializer && !Initializer->isWritten()) {
      // We skip unwritten non-default member initializers for smart pointer
      // fields because we check the end block of the constructor for the
      // fields' nullability later. This allows us to avoid inferring Nullable
      // for smart pointers that are always assigned to a Nonnull value in
      // constructor bodies.
      return;
    }

    fromAssignmentLike(*Field, *InitExpr, InitExpr->getExprLoc(),
                       IsDefaultInitializer
                           ? Evidence::NULLABLE_DEFAULT_MEMBER_INITIALIZER
                           : Evidence::ASSIGNED_FROM_NULLABLE);
  }

  void fromFieldInits(const RecordInitListHelper &Helper) {
    // Any initialization of base classes/fields will be collected from the
    // InitListExpr for the base initialization, so we only need to collect here
    // from the field inits.
    for (auto [Field, InitExpr] : Helper.field_inits()) {
      if (!isSupportedPointerType(Field->getType())) return;

      fromAssignmentLike(*Field, *InitExpr, InitExpr->getExprLoc());
    }
  }

  void fromAggregateInitialization(const Stmt &S) {
    if (auto *InitList = dyn_cast<clang::InitListExpr>(&S);
        InitList && InitList->getType()->isRecordType()) {
      fromFieldInits(RecordInitListHelper(InitList));
      return;
    }
    if (auto *ParenListInit = dyn_cast<clang::CXXParenListInitExpr>(&S);
        ParenListInit && ParenListInit->getType()->isRecordType())
      fromFieldInits(RecordInitListHelper(ParenListInit));
  }

  const std::vector<InferableSlot> &InferableSlots;
  const Formula &InferableSlotsConstraint;
  llvm::function_ref<EvidenceEmitter> Emit;
  const PointerNullabilityLattice &Lattice;
  const Environment &Env;
};
}  // namespace

/// Returns a function that the analysis can use to override Decl nullability
/// values from the source code being analyzed with previously inferred
/// nullabilities.
///
/// In practice, this should only override the default nullability for Decls
/// that do not spell out a nullability in source code, because we only pass in
/// inferences from the previous round which are non-trivial and annotations
/// "inferred" by reading an annotation from source code in the previous round
/// were marked trivial.
static auto getConcreteNullabilityOverrideFromPreviousInferences(
    ConcreteNullabilityCache &Cache, USRCache &USRCache,
    const PreviousInferences &PreviousInferences) {
  return [&](const Decl &D) -> std::optional<const PointerTypeNullability *> {
    auto [It, Inserted] = Cache.try_emplace(&D);
    if (Inserted) {
      std::optional<const Decl *> FingerprintedDecl;
      Slot Slot;
      if (auto *FD = dyn_cast<FunctionDecl>(&D)) {
        FingerprintedDecl = FD;
        Slot = SLOT_RETURN_TYPE;
      } else if (auto *PD = dyn_cast<ParmVarDecl>(&D)) {
        if (auto *Parent = dyn_cast_or_null<FunctionDecl>(
                PD->getParentFunctionOrMethod())) {
          FingerprintedDecl = Parent;
          Slot = paramSlot(PD->getFunctionScopeIndex());
        }
      }
      if (!FingerprintedDecl) return std::nullopt;
      auto Fingerprint =
          fingerprint(getOrGenerateUSR(USRCache, **FingerprintedDecl), Slot);
      if (PreviousInferences.Nullable.contains(Fingerprint)) {
        It->second.emplace(NullabilityKind::Nullable);
      } else if (PreviousInferences.Nonnull.contains(Fingerprint)) {
        It->second.emplace(NullabilityKind::NonNull);
      } else {
        It->second = std::nullopt;
      }
    }
    if (!It->second) return std::nullopt;
    return &*It->second;
  };
}

template <typename ContainerT>
static bool hasAnyInferenceTargets(const ContainerT &Decls) {
  return std::any_of(Decls.begin(), Decls.end(),
                     [](const Decl *D) { return D && isInferenceTarget(*D); });
}

static bool hasAnyInferenceTargets(dataflow::ReferencedDecls &RD) {
  return hasAnyInferenceTargets(RD.Fields) ||
         hasAnyInferenceTargets(RD.Globals) ||
         hasAnyInferenceTargets(RD.Functions);
}

llvm::Error collectEvidenceFromDefinition(
    const Decl &Definition, llvm::function_ref<EvidenceEmitter> Emit,
    USRCache &USRCache, const PreviousInferences PreviousInferences,
    unsigned MaxSATIterations) {
  ASTContext &Ctx = Definition.getASTContext();
  dataflow::ReferencedDecls ReferencedDecls;
  Stmt *TargetStmt = nullptr;
  std::optional<DeclStmt> DeclStmtForVarDecl;
  const auto *TargetAsFunc = dyn_cast<FunctionDecl>(&Definition);
  if (TargetAsFunc != nullptr) {
    if (!TargetAsFunc->doesThisDeclarationHaveABody()) {
      return llvm::createStringError(llvm::inconvertibleErrorCode(),
                                     "Function definitions must have a body.");
    }
    TargetStmt = TargetAsFunc->getBody();
    ReferencedDecls = dataflow::getReferencedDecls(*TargetAsFunc);
  } else if (auto *Var = dyn_cast<VarDecl>(&Definition)) {
    if (!Var->hasInit()) {
      return llvm::createStringError(
          llvm::inconvertibleErrorCode(),
          "Variable definitions must have an initializer.");
    }
    // Synthesize a temporary DeclStmt for the assignment of the variable to
    // its initializing expression. This is an unusual pattern that does not
    // perfectly reflect the CFG or AST for declaration or assignment of a
    // global variable, and it is possible that this may cause unexpected
    // behavior in clang tools/utilities.
    TargetStmt =
        &DeclStmtForVarDecl.emplace(DeclGroupRef(const_cast<VarDecl *>(Var)),
                                    Var->getBeginLoc(), Var->getEndLoc());
    ReferencedDecls = dataflow::getReferencedDecls(*TargetStmt);
    if (!isInferenceTarget(*Var) && !hasAnyInferenceTargets(ReferencedDecls)) {
      // If this variable is not an inference target and the initializer does
      // not reference any inference targets, we won't be able to collect any
      // useful evidence from the initializer.
      return llvm::Error::success();
    }
  } else {
    std::string Msg =
        "Unable to find a valid target definition from Definition:\n";
    llvm::raw_string_ostream Stream(Msg);
    Definition.dump(Stream);
    return llvm::createStringError(llvm::inconvertibleErrorCode(), Msg);
  }

  CHECK(TargetStmt) << "TargetStmt should have been assigned a non-null value.";

  llvm::Expected<dataflow::AdornedCFG> ACFG =
      dataflow::AdornedCFG::build(Definition, *TargetStmt, Ctx);
  if (!ACFG) return ACFG.takeError();

  auto OwnedSolver = std::make_unique<WatchedLiteralsSolver>(MaxSATIterations);
  const WatchedLiteralsSolver *Solver = OwnedSolver.get();
  DataflowAnalysisContext AnalysisContext(std::move(OwnedSolver));
  Environment Env = TargetAsFunc ? Environment(AnalysisContext, *TargetAsFunc)
                                 : Environment(AnalysisContext, *TargetStmt);
  PointerNullabilityAnalysis Analysis(Ctx, Env);

  std::vector<InferableSlot> InferableSlots;
  if (TargetAsFunc && isInferenceTarget(*TargetAsFunc)) {
    auto Parameters = TargetAsFunc->parameters();
    for (auto I = 0; I < Parameters.size(); ++I) {
      auto T = Parameters[I]->getType().getNonReferenceType();
      if (hasInferable(T) && !evidenceKindFromDeclaredType(T)) {
        InferableSlots.emplace_back(Analysis.assignNullabilityVariable(
                                        Parameters[I], AnalysisContext.arena()),
                                    paramSlot(I), *TargetAsFunc);
      }
    }
  }

  for (const FieldDecl *Field : ReferencedDecls.Fields) {
    if (isInferenceTarget(*Field) &&
        !evidenceKindFromDeclaredType(Field->getType())) {
      InferableSlots.emplace_back(
          Analysis.assignNullabilityVariable(Field, AnalysisContext.arena()),
          Slot(0), *Field);
    }
  }
  for (const VarDecl *Global : ReferencedDecls.Globals) {
    if (isInferenceTarget(*Global) &&
        !evidenceKindFromDeclaredType(Global->getType())) {
      InferableSlots.emplace_back(
          Analysis.assignNullabilityVariable(Global, AnalysisContext.arena()),
          Slot(0), *Global);
    }
  }
  for (const FunctionDecl *Function : ReferencedDecls.Functions) {
    if (isInferenceTarget(*Function) &&
        hasInferable(Function->getReturnType()) &&
        !evidenceKindFromDeclaredType(Function->getReturnType())) {
      InferableSlots.emplace_back(
          Analysis.assignNullabilityVariable(Function, AnalysisContext.arena()),
          SLOT_RETURN_TYPE, *Function);
    }
  }

  const auto &InferableSlotsConstraint =
      getInferableSlotsAsInferredOrUnknownConstraint(InferableSlots, USRCache,
                                                     PreviousInferences,
                                                     AnalysisContext.arena());

  ConcreteNullabilityCache ConcreteNullabilityCache;
  Analysis.assignNullabilityOverride(
      getConcreteNullabilityOverrideFromPreviousInferences(
          ConcreteNullabilityCache, USRCache, PreviousInferences));

  llvm::Error Error =
      dataflow::runDataflowAnalysis(
          *ACFG, Analysis, Env,
          [&](const CFGElement &Element,
              const dataflow::DataflowAnalysisState<PointerNullabilityLattice>
                  &State) {
            DefinitionEvidenceCollector::collect(
                InferableSlots, InferableSlotsConstraint, Emit, Element,
                State.Lattice, State.Env);
          })
          .takeError();

  // TODO(b/330702908) For constructor declarations, collect
  // ASSIGNED_FROM_NULLABLE evidence for smart pointer fields left uninitialized
  // in the final block of the constructor body.

  if (!Error && Solver->reachedLimit()) {
    return llvm::createStringError(llvm::errc::interrupted,
                                   "SAT solver reached iteration limit");
  }
  return Error;
}

static bool isOrIsConstructedFromNullPointerConstant(
    absl::Nonnull<const Expr *> E, ASTContext &Ctx) {
  if (E->isNullPointerConstant(Ctx, Expr::NPC_ValueDependentIsNotNull) !=
      Expr::NPCK_NotNull) {
    return true;
  }
  const Expr *SubExpr = &dataflow::ignoreCFGOmittedNodes(*E);
  if (auto *MaterializeTempExpr = dyn_cast<MaterializeTemporaryExpr>(SubExpr)) {
    SubExpr = MaterializeTempExpr->getSubExpr();
  }
  auto *BindTemp = dyn_cast<CXXBindTemporaryExpr>(SubExpr);
  if (BindTemp == nullptr) return false;
  auto *CE =
      dyn_cast<CXXConstructExpr>(BindTemp->getSubExpr()->IgnoreImpCasts());
  return CE != nullptr && CE->getNumArgs() == 1 &&
         CE->getArg(0)->isNullPointerConstant(
             Ctx, Expr::NPC_ValueDependentIsNotNull) != Expr::NPCK_NotNull;
}

static void collectEvidenceFromDefaultArgument(
    const clang::FunctionDecl &Fn, const clang::ParmVarDecl &ParamDecl,
    Slot ParamSlot, llvm::function_ref<EvidenceEmitter> Emit) {
  // We don't handle all cases of default arguments, because the expressions
  // used for the argument are not available in any CFG, because the AST nodes
  // are once-per-decl children of the ParmVarDecl, not once-per-call children
  // of the CallExpr. Including them in the callsite CFG would be a
  // significant undertaking, so for now, only handle nullptr literals (and 0)
  // and expressions whose types already include an annotation, which we can
  // handle just from declarations instead of call sites and should handle the
  // majority of cases.
  if (!isSupportedPointerType(ParamDecl.getType().getNonReferenceType()))
    return;
  if (!ParamDecl.hasDefaultArg()) return;
  if (ParamDecl.hasUnparsedDefaultArg() ||
      ParamDecl.hasUninstantiatedDefaultArg()) {
    Emit(Fn, ParamSlot, Evidence::UNKNOWN_ARGUMENT, ParamDecl.getEndLoc());
    return;
  }
  const Expr *DefaultArg = ParamDecl.getDefaultArg();
  CHECK(DefaultArg);

  if (isOrIsConstructedFromNullPointerConstant(DefaultArg,
                                               Fn.getASTContext())) {
    Emit(Fn, ParamSlot, Evidence::NULLABLE_ARGUMENT, DefaultArg->getExprLoc());
  } else {
    auto Nullability = getNullabilityAnnotationsFromType(DefaultArg->getType());
    if (auto K =
            getArgEvidenceKindFromNullability(Nullability.front().concrete())) {
      Emit(Fn, ParamSlot, K, DefaultArg->getExprLoc());
    } else {
      Emit(Fn, ParamSlot, Evidence::UNKNOWN_ARGUMENT, DefaultArg->getExprLoc());
    }
  }
}

void collectEvidenceFromTargetDeclaration(
    const clang::Decl &D, llvm::function_ref<EvidenceEmitter> Emit) {
  if (const auto *Fn = dyn_cast<clang::FunctionDecl>(&D)) {
    if (auto K = evidenceKindFromDeclaredType(Fn->getReturnType()))
      Emit(*Fn, SLOT_RETURN_TYPE, *K,
           Fn->getReturnTypeSourceRange().getBegin());
    for (unsigned I = 0; I < Fn->param_size(); ++I) {
      auto *ParamDecl = Fn->getParamDecl(I);
      if (auto K = evidenceKindFromDeclaredType(ParamDecl->getType())) {
        Emit(*Fn, paramSlot(I), *K, ParamDecl->getTypeSpecStartLoc());
      }

      collectEvidenceFromDefaultArgument(*Fn, *ParamDecl, paramSlot(I), Emit);
    }
  } else if (const auto *Field = dyn_cast<clang::FieldDecl>(&D)) {
    if (auto K = evidenceKindFromDeclaredType(Field->getType())) {
      Emit(*Field, Slot(0), *K, Field->getTypeSpecStartLoc());
    }
  } else if (const auto *Var = dyn_cast<clang::VarDecl>(&D)) {
    if (auto K = evidenceKindFromDeclaredType(Var->getType())) {
      Emit(*Var, Slot(0), *K, Var->getTypeSpecStartLoc());
    }
  }
}

EvidenceSites EvidenceSites::discover(ASTContext &Ctx) {
  struct Walker : public RecursiveASTVisitor<Walker> {
    EvidenceSites Out;

    // We do want to see concrete code, including function instantiations.
    bool shouldVisitTemplateInstantiations() const { return true; }

    // In order to collect from more default member initializers, we do want to
    // see defaulted default constructors, which are implicitly-defined
    // functions whether the declaration is implicit or explicit. We also want
    // to see lambda bodies in the form of operator() definitions that are not
    // themselves implicit but show up in an implicit context.
    bool shouldVisitImplicitCode() const { return true; }

    bool VisitFunctionDecl(absl::Nonnull<const FunctionDecl *> FD) {
      if (isInferenceTarget(*FD)) Out.Declarations.push_back(FD);

      // Visiting template instantiations is fine, these are valid functions!
      // But we'll be limited in what we can infer.
      bool IsUsefulDefinition =
          FD->doesThisDeclarationHaveABody() &&
          // We will not get anywhere with dependent code.
          !FD->isDependentContext() &&
          // Defaulted (aka implicitly-defined) default constructors give us a
          // chance to analyze default member initializers more thoroughly, but
          // otherwise implicit functions are not generally useful.
          (!FD->isImplicit() ||
           (isa<CXXConstructorDecl>(FD) &&
            cast<CXXConstructorDecl>(FD)->isDefaultConstructor()));
      if (IsUsefulDefinition) Out.Definitions.push_back(FD);

      return true;
    }

    bool VisitFieldDecl(absl::Nonnull<const FieldDecl *> FD) {
      if (isInferenceTarget(*FD)) Out.Declarations.push_back(FD);
      return true;
    }

    bool VisitVarDecl(absl::Nonnull<const VarDecl *> VD) {
      if (isInferenceTarget(*VD)) {
        Out.Declarations.push_back(VD);
      }
      // Variable initializers outside of function bodies may contain evidence
      // we won't otherwise see, even if the variable is not an inference
      // target.
      if (VD->hasInit() && !VD->getDeclContext()->isFunctionOrMethod())
        Out.Definitions.push_back(VD);
      return true;
    }
  };

  Walker W;
  W.TraverseAST(Ctx);
  return std::move(W.Out);
}

}  // namespace clang::tidy::nullability
