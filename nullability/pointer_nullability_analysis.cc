// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/pointer_nullability_analysis.h"

#include <cassert>
#include <functional>
#include <optional>
#include <utility>

#include "absl/base/nullability.h"
#include "nullability/pointer_nullability.h"
#include "nullability/pointer_nullability_lattice.h"
#include "nullability/pragma.h"
#include "nullability/type_nullability.h"
#include "nullability/type_transferer.h"
#include "nullability/value_transferer.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/DeclTemplate.h"
#include "clang/AST/Expr.h"
#include "clang/AST/ExprCXX.h"
#include "clang/AST/NestedNameSpecifierBase.h"
#include "clang/AST/Stmt.h"
#include "clang/AST/TemplateBase.h"
#include "clang/AST/TypeBase.h"
#include "clang/Analysis/CFG.h"
#include "clang/Analysis/FlowSensitive/Arena.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysis.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysisContext.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/DataflowLattice.h"
#include "clang/Analysis/FlowSensitive/Formula.h"
#include "clang/Analysis/FlowSensitive/MatchSwitch.h"
#include "clang/Analysis/FlowSensitive/StorageLocation.h"
#include "clang/Analysis/FlowSensitive/Value.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/Specifiers.h"
#include "llvm/ADT/StringMap.h"

namespace clang::tidy::nullability {

using dataflow::Arena;
using dataflow::ComparisonResult;
using dataflow::DataflowAnalysisContext;
using dataflow::Environment;
using dataflow::Formula;
using dataflow::LatticeEffect;
using dataflow::PointerValue;
using dataflow::StorageLocation;
using dataflow::TransferState;
using dataflow::Value;
using dataflow::WidenResult;

// If `Elt` is an expression of raw pointer type, ensures that it has a
// `PointerValue` associated with it. Also ensure that it has nullability
// state.
static void ensureRawPointerHasValueAndNullability(
    const CFGElement& Elt, Environment& Env,
    TransferState<PointerNullabilityLattice>& State) {
  auto S = Elt.getAs<CFGStmt>();
  if (!S) return;

  const Expr* E = dyn_cast<Expr>(S->getStmt());
  if (!E) return;

  if (auto* PointerVal = ensureRawPointerHasValue(E, Env)) {
    if (!hasPointerNullState(*PointerVal)) {
      initPointerFromTypeNullability(*PointerVal, E, State);
    }
  }
}

PointerNullabilityAnalysis::PointerNullabilityAnalysis(
    ASTContext& Context, Environment& Env, const NullabilityPragmas& Pragmas)
    : DataflowAnalysis<PointerNullabilityAnalysis, PointerNullabilityLattice>(
          Context),
      TypeTransferer(buildTypeTransferer()),
      ValueTransferer(buildValueTransferer()) {
  Env.getDataflowAnalysisContext().setSyntheticFieldCallback(
      [](QualType Ty) -> llvm::StringMap<QualType> {
        QualType RawPointerTy = underlyingRawPointerType(Ty, AS_private);
        if (RawPointerTy.isNull()) return {};
        return {{PtrField, RawPointerTy}};
      });
  NFS.Defaults = TypeNullabilityDefaults(Context, Pragmas);
}

PointerTypeNullability PointerNullabilityAnalysis::assignNullabilityVariable(
    const ValueDecl* absl_nonnull D, dataflow::Arena& A) {
  auto [It, Inserted] = NFS.DeclTopLevelNullability.try_emplace(
      cast<ValueDecl>(D->getCanonicalDecl()));
  if (Inserted) It->second = PointerTypeNullability::createSymbolic(A);
  return It->second;
}

void PointerNullabilityAnalysis::transfer(const CFGElement& Elt,
                                          PointerNullabilityLattice& Lattice,
                                          Environment& Env) {
  TransferState<PointerNullabilityLattice> State(Lattice, Env);

  TypeTransferer(Elt, getASTContext(), State);
  ValueTransferer(Elt, getASTContext(), State);
  ensureRawPointerHasValueAndNullability(Elt, Env, State);
  ensureSmartPointerInitialized(Elt, State);
}

static const Formula* absl_nullable mergeFormulas(
    const Formula* absl_nullable Bool1, const Environment& Env1,
    const Formula* absl_nullable Bool2, const Environment& Env2,
    Environment& MergedEnv) {
  if (Bool1 == Bool2) {
    return Bool1;
  }

  if (Bool1 == nullptr || Bool2 == nullptr) return nullptr;

  auto& A = MergedEnv.arena();

  // If `Bool1` and `Bool2` is constrained to the same true / false value, that
  // can serve as the return value - this simplifies the flow condition tracked
  // in `MergedEnv`.  Otherwise, information about which path was taken is used
  // to associate the return value with `Bool1` and `Bool2`.
  if (Env1.proves(*Bool1)) {
    if (Env2.proves(*Bool2)) {
      return &A.makeLiteral(true);
    }
  } else if (Env1.proves(A.makeNot(*Bool1)) && Env2.proves(A.makeNot(*Bool2))) {
    return &A.makeLiteral(false);
  }

  auto& MergedBool = A.makeAtomRef(A.makeAtom());
  // TODO(b/233582219): Flow conditions are not necessarily mutually
  // exclusive, a fix is in order: https://reviews.llvm.org/D130270. Update
  // this section when the patch is committed.
  auto FC1 = Env1.getFlowConditionToken();
  auto FC2 = Env2.getFlowConditionToken();
  MergedEnv.assume(A.makeOr(
      A.makeAnd(A.makeAtomRef(FC1), A.makeEquals(MergedBool, *Bool1)),
      A.makeAnd(A.makeAtomRef(FC2), A.makeEquals(MergedBool, *Bool2))));
  return &MergedBool;
}

void PointerNullabilityAnalysis::join(QualType Type, const Value& Val1,
                                      const Environment& Env1,
                                      const Value& Val2,
                                      const Environment& Env2, Value& MergedVal,
                                      Environment& MergedEnv) {
  if (!isSupportedRawPointerType(Type)) return;

  if (!hasPointerNullState(cast<PointerValue>(Val1)) ||
      !hasPointerNullState(cast<PointerValue>(Val2))) {
    // It can happen that we merge pointers without null state, if either or
    // both of the pointers has not appeared in an expression (and has not
    // otherwise been initialized with nullability properties) before the merge.
    // When the merged value appears in an expression, `transferPointer` will
    // take care of initializing it with nullability properties.
    return;
  }

  auto Nullability1 = getPointerNullState(cast<PointerValue>(Val1));
  auto Nullability2 = getPointerNullState(cast<PointerValue>(Val2));

  auto* FromNullable =
      mergeFormulas(Nullability1.FromNullable, Env1, Nullability2.FromNullable,
                    Env2, MergedEnv);
  auto* Null = mergeFormulas(Nullability1.IsNull, Env1, Nullability2.IsNull,
                             Env2, MergedEnv);

  initPointerNullState(cast<PointerValue>(MergedVal),
                       MergedEnv.getDataflowAnalysisContext(),
                       {FromNullable, Null});
}

ComparisonResult PointerNullabilityAnalysis::compare(QualType Type,
                                                     const Value& Val1,
                                                     const Environment& Env1,
                                                     const Value& Val2,
                                                     const Environment& Env2) {
  if (const auto* PointerVal1 = dyn_cast<PointerValue>(&Val1)) {
    const auto& PointerVal2 = cast<PointerValue>(Val2);

    if (&PointerVal1->getPointeeLoc() != &PointerVal2.getPointeeLoc())
      return ComparisonResult::Different;

    if (hasPointerNullState(*PointerVal1) != hasPointerNullState(PointerVal2))
      return ComparisonResult::Different;

    if (!hasPointerNullState(*PointerVal1)) return ComparisonResult::Same;

    auto Nullability1 = getPointerNullState(*PointerVal1);
    auto Nullability2 = getPointerNullState(PointerVal2);

    // Ideally, we would be checking for equivalence of formulas, but that's
    // expensive, so we simply check for identity instead.
    return Nullability1.FromNullable == Nullability2.FromNullable &&
                   Nullability1.IsNull == Nullability2.IsNull
               ? ComparisonResult::Same
               : ComparisonResult::Different;
  }

  return ComparisonResult::Unknown;
}

// Returns the result of widening a nullability property.
// `Prev` is the formula in the previous iteration, `Cur` is the formula in the
// current iteration.
// Returns `nullptr` (Top), if `Prev` is already Top or `Prev` and `Cur` cannot
// be proven equivalent. Otherwise, (`Prev` and `Cur` are provably equivalent),
// returns `Cur`. Returns `Cur`, if `Prev` is equivalent to `Cur`. Otherwise,
// returns `Top`.
static std::pair<const Formula* absl_nullable, LatticeEffect>
widenNullabilityProperty(const Formula* absl_nullable Prev,
                         const Environment& PrevEnv,
                         const Formula* absl_nullable Cur,
                         Environment& CurEnv) {
  if (Prev == Cur) return {Cur, LatticeEffect::Unchanged};
  if (Prev == nullptr) return {nullptr, LatticeEffect::Unchanged};
  if (Cur == nullptr) return {nullptr, LatticeEffect::Changed};

  Arena& A = CurEnv.arena();

  // Note that either of `PrevEnv` or `CurEnv` may be self-contradictory
  // (unsatisfiable). So, we're careful to check only that both are consistent
  // in their conclusions. We do not draw conclusions from them independently.
  // For example, if PrevEnv => Prev`, we do *not* conclude that
  // `PrevEnv => !Prev` is false, and use that to optimize the branches below.
  if ((PrevEnv.proves(*Prev) && CurEnv.proves(*Cur)) ||
      (PrevEnv.proves(A.makeNot(*Prev)) && CurEnv.proves(A.makeNot(*Cur))))
    return {Cur, LatticeEffect::Unchanged};

  return {nullptr, LatticeEffect::Changed};
}

std::optional<WidenResult> PointerNullabilityAnalysis::widen(
    QualType Type, Value& Prev, const Environment& PrevEnv, Value& Current,
    Environment& CurrentEnv) {
  auto* PrevPtr = dyn_cast<PointerValue>(&Prev);
  if (PrevPtr == nullptr) return std::nullopt;

  // Widen pointers (when different) to a pointer with a "top" storage location.
  auto& CurPtr = cast<PointerValue>(Current);

  DataflowAnalysisContext& DACtx = CurrentEnv.getDataflowAnalysisContext();
  assert(&PrevEnv.getDataflowAnalysisContext() == &DACtx);

  bool LocUnchanged = &PrevPtr->getPointeeLoc() == &CurPtr.getPointeeLoc();

  // If either `PrevPtr` or `CurPtr` lack null state, we consider the modeled
  // value to be outside the scope. TODO: we should consider all pointers in
  // scope and handle this case accordingly. We will widen the pointer location,
  // but (always) return a pointer value with no null state.
  if (!hasPointerNullState(*PrevPtr) || !hasPointerNullState(CurPtr))
    return std::nullopt;

  auto [FromNullablePrev, NullPrev] = getPointerNullState(*PrevPtr);
  auto [FromNullableCur, NullCur] = getPointerNullState(CurPtr);

  auto [FromNullableWidened, FNWEffect] = widenNullabilityProperty(
      FromNullablePrev, PrevEnv, FromNullableCur, CurrentEnv);
  auto [NullWidened, NWEffect] =
      widenNullabilityProperty(NullPrev, PrevEnv, NullCur, CurrentEnv);

  if (LocUnchanged && FNWEffect == LatticeEffect::Unchanged &&
      NWEffect == LatticeEffect::Unchanged)
    return WidenResult{&CurPtr, LatticeEffect::Unchanged};

  // Widen the loc if needed.
  StorageLocation* WidenedLoc =
      LocUnchanged
          ? &CurPtr.getPointeeLoc()
          : &getTopStorageLocation(DACtx, CurPtr.getPointeeLoc().getType());

  // Construct the new, widened value.
  auto& WidenedPtr = CurrentEnv.create<PointerValue>(*WidenedLoc);
  initPointerNullState(WidenedPtr, CurrentEnv.getDataflowAnalysisContext(),
                       {FromNullableWidened, NullWidened});

  LatticeEffect Effect = (WidenedLoc == &PrevPtr->getPointeeLoc() &&
                          FNWEffect == LatticeEffect::Unchanged &&
                          NWEffect == LatticeEffect::Unchanged)
                             ? LatticeEffect::Unchanged
                             : LatticeEffect::Changed;
  return WidenResult{&WidenedPtr, Effect};
}

StorageLocation& PointerNullabilityAnalysis::getTopStorageLocation(
    DataflowAnalysisContext& DACtx, QualType Ty) {
  auto [It, Inserted] = TopStorageLocations.try_emplace(Ty, nullptr);
  if (Inserted) It->second = &DACtx.createStorageLocation(Ty);
  return *It->second;
}

}  // namespace clang::tidy::nullability
