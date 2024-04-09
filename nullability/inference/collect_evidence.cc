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

#include "absl/base/nullability.h"
#include "absl/container/flat_hash_map.h"
#include "absl/log/check.h"
#include "nullability/ast_helpers.h"
#include "nullability/inference/inferable.h"
#include "nullability/inference/inference.proto.h"
#include "nullability/inference/replace_macros.h"
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
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/ASTMatchers/ASTMatchers.h"
#include "clang/Analysis/CFG.h"
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
using ::clang::ast_matchers::callee;
using ::clang::ast_matchers::callExpr;
using ::clang::ast_matchers::forEachDescendant;
using ::clang::ast_matchers::functionDecl;
using ::clang::ast_matchers::qualType;
using ::clang::ast_matchers::returns;
using ::clang::dataflow::DataflowAnalysisContext;
using ::clang::dataflow::Environment;
using ::clang::dataflow::Formula;
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

// If Stmt is a dereference, returns its target and location.
static std::pair<Expr *, SourceLocation> describeDereference(const Stmt &Stmt) {
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
  return {nullptr, SourceLocation()};
}

// Inferable slots are nullability slots not explicitly annotated in source
// code that we are currently capable of handling. This returns a boolean
// constraint representing these slots having a) the nullability inferred from
// the previous round for this slot or b) Unknown nullability if no inference
// was made in the previous round or there was no previous round.
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

namespace {
class ImplementationEvidenceCollector {
 public:
  // Instantiate the class only in this static function, to restrict the
  // lifetime of the object, which holds reference parameters.
  static void collect(std::vector<InferableSlot> &InferableSlots,
                      const Formula &InferableSlotsConstraint,
                      llvm::function_ref<EvidenceEmitter> Emit,
                      const Stmt &Stmt,
                      const PointerNullabilityLattice &Lattice,
                      const Environment &Env) {
    ImplementationEvidenceCollector Collector(
        InferableSlots, InferableSlotsConstraint, Emit, Stmt, Lattice, Env);
    Collector.fromDereference();
    Collector.fromCallExpr();
    Collector.fromConstructExpr();
    Collector.fromReturn();
    Collector.fromAssignment();
  }

 private:
  ImplementationEvidenceCollector(std::vector<InferableSlot> &InferableSlots,
                                  const Formula &InferableSlotsConstraint,
                                  llvm::function_ref<EvidenceEmitter> Emit,
                                  const Stmt &Stmt,
                                  const PointerNullabilityLattice &Lattice,
                                  const Environment &Env)
      : InferableSlots(InferableSlots),
        InferableSlotsConstraint(InferableSlotsConstraint),
        Emit(Emit),
        Stmt(Stmt),
        Lattice(Lattice),
        Env(Env) {}

  // Records evidence derived from the necessity that `Value` is nonnull.
  // It may be dereferenced, passed as a nonnull param, etc, per `EvidenceKind`.
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

  // Records evidence for Nonnull-ness derived from the necessity that
  // `MustBeTrue` must be true.
  //
  // Does not consider the possibility that the formula can only be proven true
  // by marking a slot Nullable, as this is is not a pattern we have yet seen in
  // practice. This function could easily be extended to do so, though.
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

  void fromDereference() {
    auto [Target, Loc] = describeDereference(Stmt);
    if (!Target || !isSupportedPointerType(Target->getType())) return;

    // It is a dereference of a pointer. Now gather evidence from it.
    dataflow::PointerValue *DereferencedValue = getRawPointerValue(Target, Env);
    if (!DereferencedValue) return;
    mustBeNonnull(*DereferencedValue, Loc, Evidence::UNCHECKED_DEREFERENCE);
  }

  // Collect evidence for each of `InferableSlots` if that slot being marked
  // Nullable would imply `Value`'s FromNullable property.
  //
  // This function is called when we have reason to believe that `Value` must be
  // Nullable. As we can't directly retrieve the combination of Decl and Slot
  // that corresponds to `Value`'s nullability, we consider each inferable slot
  // and emit evidence for all inferable slots that, if marked Nullable, cause
  // `Value` to be considered explicitly Nullable.
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

  void fromBindingToType(QualType Type, TypeNullability &TypeNullability,
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
      if (!isSupportedRawPointerType(ParamType)) continue;
      if (!isSupportedRawPointerType(Iter.arg().getType())) {
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
      CHECK(isSupportedRawPointerType(Iter.arg().getType()))
          << "Unsupported argument " << Iter.argIdx()
          << " type: " << Iter.arg().getType().getAsString();

      if (isa<clang::CXXDefaultArgExpr>(Iter.arg())) {
        // Evidence collection for the callee from default argument values is
        // handled when collecting from declarations, and there's no useful
        // evidence available to collect for the caller.
        return;
      }

      dataflow::PointerValue *PV = getRawPointerValue(&Iter.arg(), Env);
      if (!PV) continue;

      SourceLocation ArgLoc = Iter.arg().getExprLoc();

      if (CollectEvidenceForCaller) {
        auto ParamNullability = getNullabilityAnnotationsFromTypeAndOverrides(
            ParamType, &Iter.param(), Lattice);

        // Collect evidence from the binding of the argument to the parameter's
        // nullability, if known.
        fromBindingToType(Iter.param().getType(), ParamNullability, *PV,
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

  // Collects evidence from the binding of function arguments to the types of
  // the corresponding parameter, used when we have a FunctionProtoType but no
  // FunctionDecl.
  // TODO: When we collect evidence for more complex slots than just top-level
  // pointers, emit evidence of the function parameter's nullability as a slot
  // in the appropriate declaration.
  void fromFunctionProtoTypeCall(const FunctionProtoType &CalleeType,
                                 const CallExpr &Expr) {
    // For each pointer parameter of the function, ...
    for (unsigned I = 0; I < CalleeType.getNumParams(); ++I) {
      const auto ParamType = CalleeType.getParamType(I);
      if (!isSupportedRawPointerType(ParamType.getNonReferenceType())) continue;
      // the corresponding argument should also be a pointer.
      CHECK(isSupportedRawPointerType(Expr.getArg(I)->getType()))
          << "Unsupported argument " << I
          << " type: " << Expr.getArg(I)->getType().getAsString();

      dataflow::PointerValue *PV = getRawPointerValue(Expr.getArg(I), Env);
      if (!PV) continue;

      // TODO: when we infer function pointer/reference parameters'
      // nullabilities, check for overrides from previous inference iterations.
      auto ParamNullability = getNullabilityAnnotationsFromType(ParamType);

      // Collect evidence from the binding of the argument to the parameter's
      // nullability, if known.
      fromBindingToType(ParamType, ParamNullability, *PV,
                        Expr.getArg(I)->getExprLoc());
    }
  }

  // Similar to collectEvidenceFromArgsAndParams, but handles the case of a call
  // to a function pointer that is provided as a parameter or another decl, e.g.
  // a field or local variable.
  //
  // e.g. We can collect evidence for the nullability of `p` and (when we handle
  // more than top-level pointer slots) `j` in the following, based on the call
  // to `callee`:
  // ```
  //  void target(int* p, void (*callee)(Nonnull<int*> i, int* j)) {
  //    callee(p, nullptr);
  //  }
  // ```
  //
  // With `CalleeDecl` in this case not being a FunctionDecl as in most CallExpr
  // cases, distinct handling is needed.
  void fromFunctionPointerCallExpr(const Decl &CalleeDecl,
                                   const CallExpr &Expr) {
    if (InferableSlots.empty()) return;
    if (const auto *Callee = Expr.getCallee()) {
      if (const auto *PV = getRawPointerValue(Callee, Env)) {
        mustBeNonnull(*PV, Expr.getExprLoc(), Evidence::UNCHECKED_DEREFERENCE);
      }
    }

    auto *CalleeType = CalleeDecl.getFunctionType()->getAs<FunctionProtoType>();
    CHECK(CalleeType);
    fromFunctionProtoTypeCall(*CalleeType, Expr);
  }

  void fromCallExprWithoutFunctionCalleeDecl(const Decl &CalleeDecl,
                                             const CallExpr &Expr) {
    if (CalleeDecl.isFunctionPointerType()) {
      fromFunctionPointerCallExpr(CalleeDecl, Expr);
      return;
    }

    // Ignore calls of pointers to members. These are handled as dereferences at
    // the BinaryOperator node, which additionally captures pointers to fields.
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

    // If we run into other cases meeting this criterion, skip them, but log
    // first so we can potentially add support later.
    llvm::errs() << "Unsupported case of a CallExpr without a FunctionDecl. "
                    "Not collecting any evidence from this CallExpr:\n";
    Expr.getBeginLoc().dump(CalleeDecl.getASTContext().getSourceManager());
    Expr.dump();
    llvm::errs() << "Which is a call to:\n";
    CalleeDecl.dump();
  }

  // Given a `CallExpr` for a call to our special macro single-argument capture
  // function, collect evidence for a slot that can prevent the abort condition
  // from being true if it is annotated Nonnull.
  //
  // e.g. From `CHECK(x)`, we collect evidence for a slot that can cause `x` to
  // not be null.
  void fromAbortIfFalseMacroCall(const CallExpr &CallExpr) {
    CHECK_EQ(CallExpr.getNumArgs(), 1);
    const Expr *Arg = CallExpr.getArg(0);
    if (!Arg) return;
    QualType ArgType = Arg->getType();
    if (isSupportedRawPointerType(ArgType)) {
      const dataflow::PointerValue *PV = getRawPointerValue(Arg, Env);
      if (!PV) return;
      mustBeNonnull(*PV, Arg->getExprLoc(), Evidence::ABORT_IF_NULL);
    } else if (ArgType->isBooleanType()) {
      const dataflow::BoolValue *BV = Env.get<dataflow::BoolValue>(*Arg);
      if (!BV || BV->getKind() == dataflow::BoolValue::Kind::TopBool) return;
      mustBeTrue(BV->formula(), Arg->getExprLoc(), Evidence::ABORT_IF_NULL);
    }
  }

  // Given a `CallExpr` for a call to our special macro two-argument capture
  // function for not-equal checks, if one of the arguments is a nullptr
  // constant or provably null, collect evidence for a slot that can prevent the
  // other argument from being null.
  //
  // e.g. From `CHECK_NE(x, nullptr)`, we collect evidence for a slot that can
  // cause `x` to not be null.
  void fromAbortIfEqualMacroCall(const CallExpr &CallExpr) {
    CHECK_EQ(CallExpr.getNumArgs(), 2);
    const Expr *First = CallExpr.getArg(0);
    const Expr *Second = CallExpr.getArg(1);
    bool FirstSupported = isSupportedRawPointerType(First->getType());
    bool SecondSupported = isSupportedRawPointerType(Second->getType());
    if (!FirstSupported && !SecondSupported) return;

    ASTContext &Context = CallExpr.getCalleeDecl()->getASTContext();
    const dataflow::PointerValue *ValueComparedToNull = nullptr;
    SourceLocation EvidenceLoc;
    if (First->isNullPointerConstant(Context,
                                     Expr::NPC_ValueDependentIsNotNull)) {
      if (!isSupportedRawPointerType(Second->getType())) return;
      ValueComparedToNull = getRawPointerValue(Second, Env);
      if (!ValueComparedToNull) return;
      EvidenceLoc = Second->getExprLoc();
    } else if (Second->isNullPointerConstant(
                   Context, Expr::NPC_ValueDependentIsNotNull)) {
      if (!isSupportedRawPointerType(First->getType())) return;
      ValueComparedToNull = getRawPointerValue(First, Env);
      if (!ValueComparedToNull) return;
      EvidenceLoc = First->getExprLoc();
    } else {
      if (!FirstSupported || !SecondSupported) {
        // If this happens outside of the nullptr literal case, we'd like to
        // know about it.
        llvm::errs()
            << "Value of a supported raw pointer type compared to a value "
               "of a type that is not a supported raw pointer type.: \n";
        CallExpr.dump();
        CallExpr.getExprLoc().dump(
            CallExpr.getCalleeDecl()->getASTContext().getSourceManager());
        return;
      }

      const dataflow::PointerValue *FirstPV = getRawPointerValue(First, Env);
      if (!FirstPV) return;
      const dataflow::PointerValue *SecondPV = getRawPointerValue(Second, Env);
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

  void fromCallExpr() {
    auto *CallExpr = dyn_cast<clang::CallExpr>(&Stmt);
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

  void fromConstructExpr() {
    auto *ConstructExpr = dyn_cast<clang::CXXConstructExpr>(&Stmt);
    if (!ConstructExpr) return;
    auto *ConstructorDecl = dyn_cast_or_null<clang::CXXConstructorDecl>(
        ConstructExpr->getConstructor());
    if (!ConstructorDecl) return;

    fromArgsAndParams(*ConstructorDecl, *ConstructExpr);
  }

  void fromReturn() {
    // Is this CFGElement a return statement?
    auto *ReturnStmt = dyn_cast<clang::ReturnStmt>(&Stmt);
    if (!ReturnStmt) return;
    auto *ReturnExpr = ReturnStmt->getRetValue();
    if (!ReturnExpr || !isSupportedRawPointerType(ReturnExpr->getType()))
      return;

    // Skip gathering evidence about the current function if the current
    // function is not an inference target.
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

  // Checks whether PointerValue is null or nullable and if so, collects
  // evidence for a slot that would, if marked Nullable, cause TypeNullability's
  // first-layer nullability to be Nullable.
  //
  // e.g. This is used for example to collect from the following:
  // ```
  // void target(int* p, int* q, NullabilityUnknown<int*> r) {
  //   p = nullptr;
  //   if (!r) {
  //     q = r;
  //   }
  // }
  // ```
  // evidence for each of the assignments of `p` and `q` that they were
  // ASSIGNED_FROM_NULLABLE.
  void fromAssignmentFromNullable(TypeNullability &TypeNullability,
                                  dataflow::PointerValue &PointerValue,
                                  SourceLocation ValueLoc) {
    if (TypeNullability.empty() || !hasPointerNullState(PointerValue)) return;
    dataflow::Arena &A = Env.arena();
    if (getNullability(PointerValue, Env, &InferableSlotsConstraint) ==
        NullabilityKind::Nullable) {
      const Formula &TypeIsNullable = TypeNullability[0].isNullable(A);
      if (!Env.allows(TypeIsNullable)) return;

      for (auto &IS : InferableSlots) {
        auto &Implication = A.makeImplies(
            IS.getSymbolicNullability().isNullable(A), TypeIsNullable);
        // It's not expected that a slot's Nullable atom could be proven
        // false by the environment alone (without the
        // InferableSlotsConstraint), but SAT calls are relatively expensive, so
        // only DCHECK.
        DCHECK(Env.allows(IS.getSymbolicNullability().isNullable(A)));
        if (Env.proves(Implication)) {
          Emit(IS.getInferenceTarget(), IS.getTargetSlot(),
               Evidence::ASSIGNED_FROM_NULLABLE, ValueLoc);
          return;
        }
      }
    }
  }

  void fromAssignment() {
    if (InferableSlots.empty()) return;

    // Initialization of new decl.
    if (auto *DeclStmt = dyn_cast<clang::DeclStmt>(&Stmt)) {
      for (auto *Decl : DeclStmt->decls()) {
        if (auto *VarDecl = dyn_cast_or_null<clang::VarDecl>(Decl);
            VarDecl && VarDecl->hasInit()) {
          bool DeclTypeSupported = isSupportedRawPointerType(
              VarDecl->getType().getNonReferenceType());
          bool InitTypeSupported = isSupportedPointerType(
              VarDecl->getInit()->getType().getNonReferenceType());
          if (!DeclTypeSupported) return;
          if (!InitTypeSupported) {
            llvm::errs() << "Unsupported init type for pointer decl: "
                         << VarDecl->getInit()->getType() << "\n";
            return;
          }
          auto *PV = getRawPointerValue(VarDecl->getInit(), Env);
          if (!PV) return;
          TypeNullability TypeNullability =
              getNullabilityAnnotationsFromTypeAndOverrides(VarDecl->getType(),
                                                            VarDecl, Lattice);
          fromBindingToType(VarDecl->getType(), TypeNullability, *PV,
                            VarDecl->getInit()->getExprLoc());

          fromAssignmentFromNullable(TypeNullability, *PV,
                                     VarDecl->getInit()->getExprLoc());
        }
      }
    }

    // Assignment to existing decl.
    if (auto *BinaryOperator = dyn_cast<clang::BinaryOperator>(&Stmt);
        BinaryOperator &&
        BinaryOperator->getOpcode() == clang::BinaryOperatorKind::BO_Assign) {
      bool LhsSupported =
          isSupportedRawPointerType(BinaryOperator->getLHS()->getType());
      bool RhsSupported =
          isSupportedRawPointerType(BinaryOperator->getRHS()->getType());
      if (!LhsSupported) return;
      if (!RhsSupported) {
        llvm::errs() << "Unsupported RHS type in assignment to pointer decl: "
                     << BinaryOperator->getRHS()->getType() << "\n";
      }
      auto *PV = getRawPointerValue(BinaryOperator->getRHS(), Env);
      if (!PV) return;
      TypeNullability TypeNullability;
      if (auto *DeclRefExpr =
              dyn_cast_or_null<clang::DeclRefExpr>(BinaryOperator->getLHS())) {
        TypeNullability = getNullabilityAnnotationsFromTypeAndOverrides(
            BinaryOperator->getLHS()->getType(), DeclRefExpr->getDecl(),
            Lattice);
      } else {
        TypeNullability = getNullabilityAnnotationsFromType(
            BinaryOperator->getLHS()->getType());
      }
      fromBindingToType(BinaryOperator->getLHS()->getType(), TypeNullability,
                        *PV, BinaryOperator->getRHS()->getExprLoc());

      fromAssignmentFromNullable(TypeNullability, *PV,
                                 BinaryOperator->getRHS()->getExprLoc());
    }
  }

  const std::vector<InferableSlot> &InferableSlots;
  const Formula &InferableSlotsConstraint;
  llvm::function_ref<EvidenceEmitter> Emit;
  const Stmt &Stmt;
  const PointerNullabilityLattice &Lattice;
  const Environment &Env;
};
}  // namespace

static std::optional<Evidence::Kind> evidenceKindFromDeclaredType(QualType T) {
  if (!isSupportedRawPointerType(T.getNonReferenceType())) return std::nullopt;
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

// Adds InferableSlots for the return types of functions called by
// `CurrentFunction`. If a called function's return value is dereferenced,
// this enables us to collect evidence that the return type should be Nonnull.
static void addInferableSlotsForCalledFunctions(
    const FunctionDecl &CurrentFunction,
    std::vector<InferableSlot> &InferableSlots,
    PointerNullabilityAnalysis &Analysis, dataflow::Arena &Arena) {
  static constexpr std::string_view ReturnTypeNodeId = "ReturnType";
  static constexpr std::string_view FunctionDeclNodeId = "FunctionDecl";

  llvm::DenseSet<const FunctionDecl *> Functions;
  for (const auto &Match : clang::ast_matchers::match(
           functionDecl(forEachDescendant(callExpr(
               callee(functionDecl(returns(qualType().bind(ReturnTypeNodeId)))
                          .bind(FunctionDeclNodeId))))),
           CurrentFunction, CurrentFunction.getASTContext())) {
    auto *ReturnType = Match.getNodeAs<QualType>(ReturnTypeNodeId);
    if (!ReturnType || !hasInferable(*ReturnType) ||
        evidenceKindFromDeclaredType(*ReturnType))
      continue;
    auto *CalledFunction = Match.getNodeAs<FunctionDecl>(FunctionDeclNodeId);
    if (!CalledFunction || !isInferenceTarget(*CalledFunction)) continue;
    auto [it, inserted] = Functions.insert(CalledFunction);
    if (inserted) {
      InferableSlots.emplace_back(
          Analysis.assignNullabilityVariable(CalledFunction, Arena),
          SLOT_RETURN_TYPE, *CalledFunction);
    }
  }
}

llvm::Error collectEvidenceFromImplementation(
    const Decl &ImplementationDecl, llvm::function_ref<EvidenceEmitter> Emit,
    USRCache &USRCache, const PreviousInferences PreviousInferences,
    unsigned MaxSATIterations) {
  const auto *Func = dyn_cast<FunctionDecl>(&ImplementationDecl);
  if (!Func || !Func->doesThisDeclarationHaveABody()) {
    return llvm::createStringError(
        llvm::inconvertibleErrorCode(),
        "Implementation must be a function with a body.");
  }

  llvm::Expected<dataflow::AdornedCFG> ACFG =
      dataflow::AdornedCFG::build(*Func);
  if (!ACFG) return ACFG.takeError();

  auto OwnedSolver = std::make_unique<WatchedLiteralsSolver>(MaxSATIterations);
  const WatchedLiteralsSolver *Solver = OwnedSolver.get();
  DataflowAnalysisContext AnalysisContext(std::move(OwnedSolver));
  Environment Environment(AnalysisContext, *Func);
  PointerNullabilityAnalysis Analysis(
      Func->getDeclContext()->getParentASTContext(), Environment);
  std::vector<InferableSlot> InferableSlots;
  if (isInferenceTarget(*Func)) {
    auto Parameters = Func->parameters();
    for (auto I = 0; I < Parameters.size(); ++I) {
      auto T = Parameters[I]->getType().getNonReferenceType();
      if (isSupportedRawPointerType(T) && !evidenceKindFromDeclaredType(T)) {
        InferableSlots.emplace_back(Analysis.assignNullabilityVariable(
                                        Parameters[I], AnalysisContext.arena()),
                                    paramSlot(I), *Func);
      }
    }
  }
  addInferableSlotsForCalledFunctions(*Func, InferableSlots, Analysis,
                                      AnalysisContext.arena());
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
          *ACFG, Analysis, Environment,
          [&](const CFGElement &Element,
              const dataflow::DataflowAnalysisState<PointerNullabilityLattice>
                  &State) {
            auto CFGStmt = Element.getAs<clang::CFGStmt>();
            if (!CFGStmt) return;
            auto *Stmt = CFGStmt->getStmt();
            if (!Stmt) return;
            ImplementationEvidenceCollector::collect(
                InferableSlots, InferableSlotsConstraint, Emit, *Stmt,
                State.Lattice, State.Env);
          })
          .takeError();
  if (!Error && Solver->reachedLimit()) {
    return llvm::createStringError(llvm::errc::interrupted,
                                   "SAT solver reached iteration limit");
  }
  return Error;
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
  if (!isSupportedRawPointerType(ParamDecl.getType().getNonReferenceType()))
    return;
  if (!ParamDecl.hasDefaultArg()) return;
  if (ParamDecl.hasUnparsedDefaultArg() ||
      ParamDecl.hasUninstantiatedDefaultArg()) {
    Emit(Fn, ParamSlot, Evidence::UNKNOWN_ARGUMENT, ParamDecl.getEndLoc());
    return;
  }
  const Expr *DefaultArg = ParamDecl.getDefaultArg();
  CHECK(DefaultArg);

  if (DefaultArg->isNullPointerConstant(Fn.getASTContext(),
                                        Expr::NPC_ValueDependentIsNotNull)) {
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
  // For now, we can only describe the nullability of functions.
  const auto *Fn = dyn_cast<clang::FunctionDecl>(&D);
  if (!Fn) return;

  if (auto K = evidenceKindFromDeclaredType(Fn->getReturnType()))
    Emit(*Fn, SLOT_RETURN_TYPE, *K, Fn->getReturnTypeSourceRange().getBegin());
  for (unsigned I = 0; I < Fn->param_size(); ++I) {
    auto *ParamDecl = Fn->getParamDecl(I);
    if (auto K = evidenceKindFromDeclaredType(ParamDecl->getType())) {
      Emit(*Fn, paramSlot(I), *K, ParamDecl->getTypeSpecStartLoc());
    }

    collectEvidenceFromDefaultArgument(*Fn, *ParamDecl, paramSlot(I), Emit);
  }
}

EvidenceSites EvidenceSites::discover(ASTContext &Ctx) {
  struct Walker : public RecursiveASTVisitor<Walker> {
    EvidenceSites Out;

    // We do want to see concrete code, including function instantiations.
    bool shouldVisitTemplateInstantiations() const { return true; }

    bool VisitFunctionDecl(absl::Nonnull<const FunctionDecl *> FD) {
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
