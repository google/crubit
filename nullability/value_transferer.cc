// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/value_transferer.h"

#include <cassert>
#include <functional>
#include <optional>

#include "absl/base/nullability.h"
#include "absl/log/check.h"
#include "nullability/ast_helpers.h"
#include "nullability/macro_arg_capture.h"
#include "nullability/pointer_nullability.h"
#include "nullability/pointer_nullability_lattice.h"
#include "nullability/pointer_nullability_matchers.h"
#include "nullability/type_nullability.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/DeclTemplate.h"
#include "clang/AST/Expr.h"
#include "clang/AST/ExprCXX.h"
#include "clang/AST/OperationKinds.h"
#include "clang/AST/TypeBase.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/ASTMatchers/ASTMatchers.h"
#include "clang/Analysis/CFG.h"
#include "clang/Analysis/FlowSensitive/Arena.h"
#include "clang/Analysis/FlowSensitive/CFGMatchSwitch.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysisContext.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/Formula.h"
#include "clang/Analysis/FlowSensitive/MatchSwitch.h"
#include "clang/Analysis/FlowSensitive/RecordOps.h"
#include "clang/Analysis/FlowSensitive/SmartPointerAccessorCaching.h"
#include "clang/Analysis/FlowSensitive/StorageLocation.h"
#include "clang/Analysis/FlowSensitive/Value.h"
#include "clang/Basic/IdentifierTable.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/OperatorKinds.h"
#include "clang/Basic/Specifiers.h"
#include "llvm/Support/raw_ostream.h"

namespace clang::tidy::nullability {
using ast_matchers::MatchFinder;
using dataflow::Arena;
using dataflow::BoolValue;
using dataflow::DataflowAnalysisContext;
using dataflow::Environment;
using dataflow::Formula;
using dataflow::PointerValue;
using dataflow::RecordStorageLocation;
using dataflow::StorageLocation;
using dataflow::TransferState;
using dataflow::Value;

PointerValue* absl_nullable ensureRawPointerHasValue(const Expr* absl_nonnull E,
                                                     Environment& Env) {
  if (!isSupportedRawPointerType(E->getType())) return nullptr;

  if (E->isPRValue()) {
    if (auto* Val = Env.get<PointerValue>(*E)) return Val;
    auto* Val = cast<PointerValue>(Env.createValue(E->getType()));
    Env.setValue(*E, *Val);
    return Val;
  }

  StorageLocation* Loc = Env.getStorageLocation(*E);
  if (Loc == nullptr) {
    Loc = &Env.createStorageLocation(*E);
    Env.setStorageLocation(*E, *Loc);
  }
  if (auto* Val = Env.get<PointerValue>(*Loc)) return Val;
  auto* Val = cast<PointerValue>(Env.createValue(E->getType()));
  Env.setValue(*Loc, *Val);
  return Val;
}

static PointerTypeNullability getPointerTypeNullability(
    const Expr* absl_nonnull E, PointerNullabilityLattice& L) {
  // TODO: handle this in non-flow-sensitive transfer instead
  if (auto FromClang = E->getType()->getNullability();
      FromClang && *FromClang != NullabilityKind::Unspecified)
    return *FromClang;

  if (const auto* TyNullability = L.getTypeNullability(E)) {
    if (!TyNullability->empty())
      // Return the nullability of the topmost pointer in the type.
      return TyNullability->front();
  }

  return NullabilityKind::Unspecified;
}

void initPointerFromTypeNullability(
    PointerValue& PointerVal, const Expr* absl_nonnull E,
    TransferState<PointerNullabilityLattice>& State) {
  initPointerNullState(PointerVal, State.Env.getDataflowAnalysisContext(),
                       getPointerTypeNullability(E, State.Lattice));
}

/// If the pointer value stored at `PointerLoc` has any "top" nullability
/// properties, creates a new pointer value referencing the same location with
/// the "top" properties unpacked into fresh atoms. Returns:
/// -  The unpacked pointer value if unpacking took place.
/// -  The original pointer value if no unpacking took place.
/// -  Null if `PointerLoc` is not associated with a value.
/// This is analogous to the unpacking done on `TopBoolValue`s in the framework.
static PointerValue* absl_nullable unpackPointerValue(
    StorageLocation& PointerLoc, Environment& Env) {
  auto* PointerVal = Env.get<PointerValue>(PointerLoc);
  if (!PointerVal) return nullptr;

  PointerNullState NullState = getPointerNullState(*PointerVal);
  if (NullState.FromNullable && NullState.IsNull) return PointerVal;

  auto& A = Env.getDataflowAnalysisContext().arena();

  if (NullState.FromNullable == nullptr)
    NullState.FromNullable = &A.makeAtomRef(A.makeAtom());
  if (NullState.IsNull == nullptr)
    NullState.IsNull = &A.makeAtomRef(A.makeAtom());

  auto& NewPointerVal = Env.create<PointerValue>(PointerVal->getPointeeLoc());
  initPointerNullState(NewPointerVal, Env.getDataflowAnalysisContext(),
                       NullState);
  Env.setValue(PointerLoc, NewPointerVal);

  return &NewPointerVal;
}

static void setToPointerWithNullability(StorageLocation& PtrLoc,
                                        NullabilityKind NK, Environment& Env) {
  auto& Val = *cast<PointerValue>(Env.createValue(PtrLoc.getType()));
  initPointerNullState(Val, Env.getDataflowAnalysisContext(), NK);
  Env.setValue(PtrLoc, Val);
}

static void initSmartPointerForExpr(
    const Expr* E, TransferState<PointerNullabilityLattice>& State) {
  RecordStorageLocation* Loc = nullptr;
  if (E->isPRValue()) {
    Loc = &State.Env.getResultObjectLocation(*E);
  } else {
    Loc = State.Env.get<RecordStorageLocation>(*E);
    if (Loc == nullptr) {
      Loc = &cast<RecordStorageLocation>(State.Env.createStorageLocation(*E));
      State.Env.setStorageLocation(*E, *Loc);
    }
  }

  StorageLocation& PtrLoc = Loc->getSyntheticField(PtrField);
  auto* Val = State.Env.get<PointerValue>(PtrLoc);
  if (Val == nullptr) {
    Val = cast<PointerValue>(State.Env.createValue(PtrLoc.getType()));
    State.Env.setValue(PtrLoc, *Val);
  }

  initPointerFromTypeNullability(*Val, E, State);
}

static void transferNullPointer(
    const Expr* absl_nonnull NullPointer, const MatchFinder::MatchResult&,
    TransferState<PointerNullabilityLattice>& State) {
  if (auto* PointerVal = ensureRawPointerHasValue(NullPointer, State.Env)) {
    initNullPointer(*PointerVal, State.Env.getDataflowAnalysisContext());
  }
}

static void transferPointerIncOrDec(
    const UnaryOperator* absl_nonnull UnaryOp, const MatchFinder::MatchResult&,
    TransferState<PointerNullabilityLattice>& State) {
  // The framework propagates the subexpression's value (in the case of post-
  // increment) or storage location (in the case of pre-increment). We just
  // need to create a new nonnull value.
  if (StorageLocation* Loc =
          State.Env.getStorageLocation(*UnaryOp->getSubExpr())) {
    auto* Val = cast<PointerValue>(State.Env.createValue(Loc->getType()));
    initPointerNullState(*Val, State.Env.getDataflowAnalysisContext(),
                         NullabilityKind::NonNull);
    State.Env.setValue(*Loc, *Val);
  }
}

static void transferPointerAddOrSubAssign(
    const BinaryOperator* absl_nonnull BinaryOp,
    const MatchFinder::MatchResult&,
    TransferState<PointerNullabilityLattice>& State) {
  // The framework propagates the storage location of the LHS, so we just need
  // to create a new nonnull value.
  if (StorageLocation* Loc =
          State.Env.getStorageLocation(*BinaryOp->getLHS())) {
    auto* Val = cast<PointerValue>(State.Env.createValue(Loc->getType()));
    initPointerNullState(*Val, State.Env.getDataflowAnalysisContext(),
                         NullabilityKind::NonNull);
    State.Env.setValue(*Loc, *Val);
  }
}

static void transferNotNullPointer(
    const Expr* absl_nonnull NotNullPointer, const MatchFinder::MatchResult&,
    TransferState<PointerNullabilityLattice>& State) {
  if (auto* PointerVal = ensureRawPointerHasValue(NotNullPointer, State.Env)) {
    initPointerNullState(*PointerVal, State.Env.getDataflowAnalysisContext(),
                         NullabilityKind::NonNull);
  }
}

static bool isStdWeakPtrType(QualType Ty) {
  const CXXRecordDecl* RD = Ty.getCanonicalType()->getAsCXXRecordDecl();
  if (RD == nullptr) return false;

  if (!RD->getDeclContext()->isStdNamespace()) return false;

  const IdentifierInfo* ID = RD->getIdentifier();
  if (ID == nullptr) return false;

  return ID->getName() == "weak_ptr";
}

static QualType underlyingRawPointerTypeFromSmartPointer(
    RecordStorageLocation& Loc) {
  return Loc.getSyntheticField(PtrField).getType();
}

static bool isPointerTypeConvertible(QualType From, QualType To) {
  assert(isSupportedRawPointerType(From));
  assert(isSupportedRawPointerType(To));

  if (From->getCanonicalTypeUnqualified() == To->getCanonicalTypeUnqualified())
    return true;

  auto* FromDecl = From->getPointeeType()->getAsCXXRecordDecl();
  auto* ToDecl = To->getPointeeType()->getAsCXXRecordDecl();

  // If these aren't pointers to records, don't consider them convertible.
  // Otherwise there could be strange type errors.
  // We assume array decay to pointers should already be covered.
  // - for example, C++ doesn't let you assign a `float*` to a `char*`, or
  //   assign an `Enum1*` to an `Enum2*`.
  // - if we have a scalar type on one side and a record type on the other,
  //   it could e.g., lead to looking up record member vars on an `int*`.
  if (FromDecl == nullptr || ToDecl == nullptr) return false;

  if (FromDecl == ToDecl) return true;
  // If we don't have the complete definition, we can't check isDerivedFrom.
  if (!FromDecl->isCompleteDefinition()) return false;
  return FromDecl->isDerivedFrom(ToDecl);
}

static void transferSmartPointerConstructor(
    const CXXConstructExpr* Ctor, const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  RecordStorageLocation& Loc = State.Env.getResultObjectLocation(*Ctor);

  // Default and `nullptr_t` constructor.
  if (Ctor->getConstructor()->isDefaultConstructor() ||
      (Ctor->getNumArgs() >= 1 &&
       Ctor->getArg(0)->getType()->isNullPtrType())) {
    setSmartPointerToNull(Loc, State.Env);
    return;
  }

  // Construct from raw pointer, but make sure the pointer types are
  // convertible.
  if (Ctor->getNumArgs() >= 1 &&
      isSupportedRawPointerType(Ctor->getArg(0)->getType()) &&
      isPointerTypeConvertible(Ctor->getArg(0)->getType(),
                               underlyingRawPointerTypeFromSmartPointer(Loc))) {
    setSmartPointerValue(Loc, getRawPointerValue(Ctor->getArg(0), State.Env),
                         State.Env);
    return;
  }

  // Copy or move from an existing smart pointer.
  if (Ctor->getNumArgs() >= 1 &&
      isSupportedSmartPointerType(Ctor->getArg(0)->getType())) {
    auto* SrcLoc = Ctor->getArg(0)->isGLValue()
                       ? State.Env.get<RecordStorageLocation>(*Ctor->getArg(0))
                       : &State.Env.getResultObjectLocation(*Ctor->getArg(0));
    if (Ctor->getNumArgs() == 2 &&
        isSupportedRawPointerType(Ctor->getArg(1)->getType())) {
      // `shared_ptr` aliasing constructor.
      if (isPointerTypeConvertible(
              Ctor->getArg(1)->getType(),
              underlyingRawPointerTypeFromSmartPointer(Loc))) {
        setSmartPointerValue(
            Loc, getRawPointerValue(Ctor->getArg(1), State.Env), State.Env);
      }
    } else {
      if (SrcLoc != nullptr &&
          isPointerTypeConvertible(
              underlyingRawPointerTypeFromSmartPointer(*SrcLoc),
              underlyingRawPointerTypeFromSmartPointer(Loc))) {
        setSmartPointerValue(
            Loc, getPointerValueFromSmartPointer(SrcLoc, State.Env), State.Env);
      }
    }

    // If this is the move constructor, set the source to null.
    if (Ctor->getConstructor()
            ->getParamDecl(0)
            ->getType()
            ->isRValueReferenceType() &&
        SrcLoc != nullptr) {
      setSmartPointerToNull(*SrcLoc, State.Env);
    }
    return;
  }

  // Construct from `weak_ptr`. This throws if the `weak_ptr` is empty, so we
  // can assume the `shared_ptr` is non-null if the constructor returns.
  if (Ctor->getNumArgs() == 1 && isStdWeakPtrType(Ctor->getArg(0)->getType()))
    setToPointerWithNullability(Loc.getSyntheticField(PtrField),
                                NullabilityKind::NonNull, State.Env);
}

static void transferSmartPointerAssignment(
    const CXXOperatorCallExpr* OpCall, const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  auto* Loc = State.Env.get<RecordStorageLocation>(*OpCall->getArg(0));
  if (Loc == nullptr) return;

  if (OpCall->getArg(1)->getType()->isNullPtrType()) {
    setSmartPointerToNull(*Loc, State.Env);
    return;
  }

  if (!isSupportedSmartPointerType(OpCall->getArg(1)->getType())) {
    // We don't know anything about the RHS, so set the LHS to an unspecified
    // nullability state.
    // TODO(b/376231871): We could handle more RHS cases, for example if RHS
    // is a raw pointer. We could potentially assume that, if RHS is anything
    // other than a raw pointer, smart pointer, or nullptr_t, then it's
    // nonnull (instead of unspecified).
    // If we add more cases, expand `diagnoseSmartPointerAssignment` as well.
    StorageLocation& PtrLoc = Loc->getSyntheticField(PtrField);
    setToPointerWithNullability(PtrLoc, NullabilityKind::Unspecified,
                                State.Env);
    return;
  }

  auto* SrcLoc = OpCall->getArg(1)->isGLValue()
                     ? State.Env.get<RecordStorageLocation>(*OpCall->getArg(1))
                     : &State.Env.getResultObjectLocation(*OpCall->getArg(1));
  if (SrcLoc != nullptr &&
      isPointerTypeConvertible(
          underlyingRawPointerTypeFromSmartPointer(*SrcLoc),
          underlyingRawPointerTypeFromSmartPointer(*Loc))) {
    setSmartPointerValue(
        *Loc, getPointerValueFromSmartPointer(SrcLoc, State.Env), State.Env);
  }

  // If this is the move assignment operator, set the source to null.
  auto* Method = dyn_cast_or_null<CXXMethodDecl>(OpCall->getCalleeDecl());
  if (SrcLoc != nullptr && Method != nullptr &&
      Method->getParamDecl(0)->getType()->isRValueReferenceType()) {
    setSmartPointerToNull(*SrcLoc, State.Env);
  }
}

static void transferSmartPointerReleaseCall(
    const CXXMemberCallExpr* MCE, const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  // If the return type isn't what we expect, bail out.
  // This can happen if the smart pointer doesn't declare a `pointer` or
  // `element_type` type alias and our fallback logic of using `T*` as the
  // underlying pointer type (where `T` is the first template argument) is
  // incorrect.
  if (MCE->getType()->getCanonicalTypeUnqualified() !=
      underlyingRawPointerType(MCE->getObjectType())
          ->getCanonicalTypeUnqualified()) {
    return;
  }

  RecordStorageLocation* Loc = getImplicitObjectLocation(*MCE, State.Env);
  if (Loc == nullptr) return;
  StorageLocation& PtrLoc = Loc->getSyntheticField(PtrField);

  if (auto* Val = State.Env.get<PointerValue>(PtrLoc))
    State.Env.setValue(*MCE, *Val);
  State.Env.setValue(
      PtrLoc, createNullPointer(PtrLoc.getType()->getPointeeType(), State.Env));
}

static void transferSmartPointerResetCall(
    const CXXMemberCallExpr* MCE, const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  RecordStorageLocation* Loc = getImplicitObjectLocation(*MCE, State.Env);
  if (Loc == nullptr) return;

  // Zero-arg and `nullptr_t` overloads, as well as single-argument constructor
  // with default argument.
  if (MCE->getNumArgs() == 0 ||
      (MCE->getNumArgs() == 1 && MCE->getArg(0)->getType()->isNullPtrType()) ||
      (MCE->getNumArgs() == 1 && MCE->getArg(0)->isDefaultArgument())) {
    setSmartPointerToNull(*Loc, State.Env);
    return;
  }

  // std::shared_ptr::reset can take >1 argument, so we don't restrict to just
  // getNumArgs() == 1.
  if (MCE->getNumArgs() >= 1 &&
      isSupportedRawPointerType(MCE->getArg(0)->getType()) &&
      isPointerTypeConvertible(
          MCE->getArg(0)->getType(),
          underlyingRawPointerTypeFromSmartPointer(*Loc))) {
    setSmartPointerValue(*Loc, getRawPointerValue(MCE->getArg(0), State.Env),
                         State.Env);
  }
}

static void swapSmartPointers(RecordStorageLocation* Loc1,
                              RecordStorageLocation* Loc2, Environment& Env) {
  PointerValue* Val1 = getPointerValueFromSmartPointer(Loc1, Env);
  PointerValue* Val2 = getPointerValueFromSmartPointer(Loc2, Env);

  if (Loc1) {
    if (Loc2 == nullptr ||
        isPointerTypeConvertible(
            underlyingRawPointerTypeFromSmartPointer(*Loc2),
            underlyingRawPointerTypeFromSmartPointer(*Loc1))) {
      setSmartPointerValue(*Loc1, Val2, Env);
    }
  }
  if (Loc2) {
    if (Loc1 == nullptr || isPointerTypeConvertible(
                               underlyingRawPointerTypeFromSmartPointer(*Loc1),
                               underlyingRawPointerTypeFromSmartPointer(*Loc2)))
      setSmartPointerValue(*Loc2, Val1, Env);
  }
}

static void transferSmartPointerMemberSwapCall(
    const CXXMemberCallExpr* MCE, const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  swapSmartPointers(getImplicitObjectLocation(*MCE, State.Env),
                    State.Env.get<RecordStorageLocation>(*MCE->getArg(0)),
                    State.Env);
}

static void transferSmartPointerFreeSwapCall(
    const CallExpr* CE, const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  swapSmartPointers(State.Env.get<RecordStorageLocation>(*CE->getArg(0)),
                    State.Env.get<RecordStorageLocation>(*CE->getArg(1)),
                    State.Env);
}

static void transferSmartPointerGetCall(
    const CXXMemberCallExpr* MCE, const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  // If the return type isn't what we expect, bail out.
  // See `transferSmartPointerReleaseCall()` for more details.
  if (MCE->getType()->getCanonicalTypeUnqualified() !=
      underlyingRawPointerType(MCE->getObjectType())
          ->getCanonicalTypeUnqualified()) {
    return;
  }
  if (Value* Val = getPointerValueFromSmartPointer(
          getImplicitObjectLocation(*MCE, State.Env), State.Env))
    State.Env.setValue(*MCE, *Val);
}

static void transferSmartPointerBoolConversionCall(
    const CXXMemberCallExpr* MCE, const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  if (PointerValue* Val = getPointerValueFromSmartPointer(
          getImplicitObjectLocation(*MCE, State.Env), State.Env)) {
    if (const Formula* IsNull = getPointerNullState(*Val).IsNull)
      State.Env.setValue(
          *MCE, State.Env.makeNot(State.Env.arena().makeBoolValue(*IsNull)));
  }
}

static QualType getReceiverIgnoringImpCastsType(
    const CXXOperatorCallExpr* OpCall) {
  // Matchers hasArgument() appears to ignore implicit casts, so we ignore them
  // here as well to get the same behavior:
  // https://github.com/llvm/llvm-project/blob/a58c3d3ac7c6b2fd9710ab2189d7971ef37e714f/clang/include/clang/ASTMatchers/ASTMatchers.h#L4563
  const Expr* Receiver = OpCall->getArg(0)->IgnoreImpCasts();
  if (Receiver->isPRValue() && Receiver->getType()->isPointerType())
    return Receiver->getType()->getPointeeType();
  return Receiver->getType();
}

static void transferSmartPointerOperatorStar(
    const CXXOperatorCallExpr* OpCall, const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  // If the return type isn't what we expect, bail out.
  // See `transferSmartPointerReleaseCall()` for more details.
  // Besides incorrectly guessing the underlyingRawPointerType, we could also
  // encounter this if the return type is not the pointee type, or reference
  // to the pointee type (e.g., if it is instead the pointer type).
  QualType ReturnType = OpCall->getType();
  if (ReturnType->isReferenceType()) ReturnType = ReturnType->getPointeeType();
  if (ReturnType->getCanonicalTypeUnqualified() !=
      underlyingRawPointerType(getReceiverIgnoringImpCastsType(OpCall))
          ->getPointeeType()
          ->getCanonicalTypeUnqualified()) {
    return;
  }
  if (PointerValue* Val = getSmartPointerValue(OpCall->getArg(0), State.Env)) {
    State.Env.setStorageLocation(*OpCall, Val->getPointeeLoc());
  }
}

static void transferSmartPointerOperatorArrow(
    const CXXOperatorCallExpr* OpCall, const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  // If the return type isn't what we expect, bail out.
  // See `transferSmartPointerReleaseCall()` for more details.
  if (OpCall->getType()->getCanonicalTypeUnqualified() !=
      underlyingRawPointerType(getReceiverIgnoringImpCastsType(OpCall))
          ->getCanonicalTypeUnqualified()) {
    return;
  }
  if (PointerValue* Val = getSmartPointerValue(OpCall->getArg(0), State.Env)) {
    State.Env.setValue(*OpCall, *Val);
  }
}

static void transferSmartPointerFactoryCall(
    const CallExpr* CE, const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  RecordStorageLocation& Loc = State.Env.getResultObjectLocation(*CE);
  StorageLocation& PtrLoc = Loc.getSyntheticField(PtrField);

  setToPointerWithNullability(PtrLoc, NullabilityKind::NonNull, State.Env);

  // If the smart pointer is a pointer to a raw pointer and is constructed from
  // one raw pointer or nullptr_t, initialize the null state of the pointee raw
  // pointer from the argument passed in.
  if (isSupportedRawPointerType(PtrLoc.getType()->getPointeeType()) &&
      CE->getNumArgs() == 1 &&
      (CE->getArg(0)->getType()->isPointerType() ||
       CE->getArg(0)->getType()->isNullPtrType())) {
    auto* SmartPV = State.Env.get<PointerValue>(PtrLoc);
    if (!SmartPV) return;
    auto* RawPV = State.Env.get<PointerValue>(SmartPV->getPointeeLoc());
    if (!RawPV) return;
    if (CE->getArg(0)->getType()->isNullPtrType()) {
      initNullPointer(*RawPV, State.Env.getDataflowAnalysisContext());
      return;
    }
    auto* ArgPV = getRawPointerValue(CE->getArg(0), State.Env);
    if (!ArgPV || !hasPointerNullState(*ArgPV)) return;
    initPointerNullState(*RawPV, State.Env.getDataflowAnalysisContext(),
                         getPointerNullState(*ArgPV));
  }
}

static void transferWrapUniqueCall(
    const CallExpr* CE, const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  if (CE->getNumArgs() != 1) {
    return;
  }
  const Expr* Arg = CE->getArg(0);
  RecordStorageLocation& Loc = State.Env.getResultObjectLocation(*CE);
  if (isSupportedRawPointerType(Arg->getType()) &&
      isPointerTypeConvertible(Arg->getType(),
                               underlyingRawPointerTypeFromSmartPointer(Loc))) {
    setSmartPointerValue(Loc, getRawPointerValue(Arg, State.Env), State.Env);
  }
}

static void transferSmartPointerComparisonOpCall(
    const CXXOperatorCallExpr* OpCall, const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  // Formula representing an equality (`==`) comparison of the two operands.
  // If the operator is `!=`, this will need to be negated below.
  const Formula* EqualityFormula = nullptr;

  bool NullPtr1 =
      OpCall->getArg(0)->IgnoreImpCasts()->getType()->isNullPtrType();
  bool NullPtr2 =
      OpCall->getArg(1)->IgnoreImpCasts()->getType()->isNullPtrType();
  assert(!NullPtr1 || !NullPtr2);

  PointerValue* Val1 = nullptr;
  if (!NullPtr1) Val1 = getSmartPointerValue(OpCall->getArg(0), State.Env);

  PointerValue* Val2 = nullptr;
  if (!NullPtr2) Val2 = getSmartPointerValue(OpCall->getArg(1), State.Env);

  if (NullPtr1) {
    if (Val2 == nullptr) return;
    EqualityFormula = getPointerNullState(*Val2).IsNull;
  } else if (NullPtr2) {
    if (Val1 == nullptr) return;
    EqualityFormula = getPointerNullState(*Val1).IsNull;
  } else {
    if (Val1 == nullptr || Val2 == nullptr) return;
    EqualityFormula = &State.Env.arena().makeLiteral(&Val1->getPointeeLoc() ==
                                                     &Val2->getPointeeLoc());
  }

  if (EqualityFormula == nullptr) return;

  BoolValue& EqualityValue = State.Env.arena().makeBoolValue(*EqualityFormula);

  if (OpCall->getOperator() == OO_EqualEqual)
    State.Env.setValue(*OpCall, EqualityValue);
  else
    State.Env.setValue(*OpCall, State.Env.makeNot(EqualityValue));
}

static void transferSharedPtrCastCall(
    const CallExpr* CE, const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  Environment& Env = State.Env;
  DataflowAnalysisContext& Ctx = Env.getDataflowAnalysisContext();
  Arena& A = Env.arena();

  auto* Callee = dyn_cast_or_null<FunctionDecl>(CE->getCalleeDecl());
  if (Callee == nullptr) return;

  auto* SrcLoc = Env.get<RecordStorageLocation>(*CE->getArg(0));
  if (SrcLoc == nullptr) return;
  StorageLocation& SrcPtrLoc = SrcLoc->getSyntheticField(PtrField);
  auto* SrcPtrVal = Env.get<PointerValue>(SrcPtrLoc);
  if (SrcPtrVal == nullptr) return;

  RecordStorageLocation& DestLoc = Env.getResultObjectLocation(*CE);
  StorageLocation& DestPtrLoc = DestLoc.getSyntheticField(PtrField);

  if (Callee->getName() == "const_pointer_cast") {
    // A `const_pointer_cast` will definitely produce a pointer with the same
    // storage location as the source, so we can simply copy the underlying
    // pointer value.
    Env.setValue(DestPtrLoc, *SrcPtrVal);
  } else {
    auto& DestPtrVal =
        *cast<PointerValue>(Env.createValue(DestPtrLoc.getType()));
    initPointerNullState(DestPtrVal, Ctx);
    State.Env.setValue(DestPtrLoc, DestPtrVal);

    PointerNullState SrcNullability = getPointerNullState(*SrcPtrVal);
    PointerNullState DestNullability = getPointerNullState(DestPtrVal);
    assert(DestNullability.IsNull != nullptr);
    assert(DestNullability.FromNullable != nullptr);

    if (Callee->getName() == "dynamic_pointer_cast") {
      // A `dynamic_pointer_cast` may fail. So source `IsNull` implies
      // destination `IsNull` (but not the other way around), and the result is
      // always nullable.
      if (SrcNullability.IsNull != nullptr)
        Env.assume(
            A.makeImplies(*SrcNullability.IsNull, *DestNullability.IsNull));
      Env.assume(*DestNullability.FromNullable);
    } else {
      if (SrcNullability.IsNull != nullptr)
        Env.assume(
            A.makeEquals(*SrcNullability.IsNull, *DestNullability.IsNull));
      if (SrcNullability.FromNullable != nullptr)
        Env.assume(A.makeEquals(*SrcNullability.FromNullable,
                                *DestNullability.FromNullable));
    }
  }

  // Is this an overload taking an rvalue reference?
  if (Callee->getParamDecl(0)->getType()->isRValueReferenceType()) {
    if (Callee->getName() == "dynamic_pointer_cast") {
      // `dynamic_pointer_cast` sets its argument to null only if the cast
      // succeeded. So, replace the argument with a new Nullable (but not
      // definitely Null) pointer.
      setToPointerWithNullability(SrcPtrLoc, NullabilityKind::Nullable,
                                  State.Env);
    } else {
      setSmartPointerToNull(*SrcLoc, State.Env);
    }
  }
}

static void transferWeakPtrLockCall(
    const CXXMemberCallExpr* MCE, const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  RecordStorageLocation& Loc = State.Env.getResultObjectLocation(*MCE);
  StorageLocation& PtrLoc = Loc.getSyntheticField(PtrField);

  setToPointerWithNullability(PtrLoc, NullabilityKind::Nullable, State.Env);
}

static void transferSmartPointerArrowMemberExpr(
    const MemberExpr* ME, const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  // Most accesses of a smart pointer involve a glvalue of smart pointer type,
  // and `transferSmartPointer` will ensure in this case that the
  // nullability properties of the underlying raw pointer are initialized.
  // An exception to this is if we access members of a smart pointer using
  // arrow syntax; in this case, there is no glvalue of smart pointer type,
  // and this function handles initialization of the underlying raw pointer
  // in this case.

  const Expr& Base = *ME->getBase();
  auto* BasePtrVal = State.Env.get<PointerValue>(Base);
  if (BasePtrVal == nullptr) {
    BasePtrVal = cast<PointerValue>(State.Env.createValue(Base.getType()));
    State.Env.setValue(Base, *BasePtrVal);
  }

  auto& SmartPtrLoc = cast<RecordStorageLocation>(BasePtrVal->getPointeeLoc());
  StorageLocation& PtrLoc = SmartPtrLoc.getSyntheticField(PtrField);
  auto* PtrVal = State.Env.get<PointerValue>(PtrLoc);
  if (PtrVal == nullptr) {
    PtrVal = cast<PointerValue>(State.Env.createValue(PtrLoc.getType()));
    State.Env.setValue(PtrLoc, *PtrVal);
  }

  PointerTypeNullability Nullability = NullabilityKind::Unspecified;
  if (const auto* TyNullability =
          State.Lattice.getTypeNullability(ME->getBase())) {
    if (TyNullability->size() >= 2) Nullability = (*TyNullability)[1];
  }

  initPointerNullState(*PtrVal, State.Env.getDataflowAnalysisContext(),
                       Nullability);
}

static void transferPointer(const Expr* absl_nonnull PointerExpr,
                            const MatchFinder::MatchResult& Result,
                            TransferState<PointerNullabilityLattice>& State) {
  auto* PointerVal = ensureRawPointerHasValue(PointerExpr, State.Env);
  if (!PointerVal) return;

  initPointerFromTypeNullability(*PointerVal, PointerExpr, State);

  if (const auto* Cast = dyn_cast<CastExpr>(PointerExpr);
      Cast && Cast->getCastKind() == CK_LValueToRValue) {
    if (StorageLocation* Loc =
            State.Env.getStorageLocation(*Cast->getSubExpr())) {
      if (PointerValue* Val = unpackPointerValue(*Loc, State.Env)) {
        State.Env.setValue(*PointerExpr, *Val);
      }
    }
  }
}

// `ComparisonFormula` represents the comparison between the two pointer values.
//
// `LHSNull` and `RHSNull` represent the nullability of the left- and right-hand
// expressions, respectively. A nullptr value is interpreted as Top.
static BoolValue* absl_nullable processPointerComparison(
    const Formula& ComparisonFormula, const Formula* absl_nullable LHSNull,
    const Formula* absl_nullable RHSNull, BinaryOperatorKind Opcode,
    Environment& Env) {
  auto& A = Env.arena();

  // If the null state of either pointer is "top", the result of the comparison
  // is a top bool, and we don't have any knowledge we can add to the flow
  // condition.
  if (LHSNull == nullptr || RHSNull == nullptr) {
    return &A.makeTopValue();
  }

  // Special case: Are we comparing against `nullptr`?
  // We can avoid modifying the flow condition in this case and simply propagate
  // the nullability of the other operand (potentially with a negation).
  if (LHSNull->isLiteral(true))
    return &A.makeBoolValue(Opcode == BO_EQ ? *RHSNull : A.makeNot(*RHSNull));

  if (RHSNull->isLiteral(true))
    return &A.makeBoolValue(Opcode == BO_EQ ? *LHSNull : A.makeNot(*LHSNull));

  CHECK(Opcode == BO_EQ || Opcode == BO_NE);
  auto& PointerEQ =
      Opcode == BO_EQ ? ComparisonFormula : A.makeNot(ComparisonFormula);
  auto& PointerNE =
      Opcode == BO_EQ ? A.makeNot(ComparisonFormula) : ComparisonFormula;

  // nullptr == nullptr
  Env.assume(A.makeImplies(A.makeAnd(*LHSNull, *RHSNull), PointerEQ));
  // nullptr != notnull
  Env.assume(
      A.makeImplies(A.makeAnd(*LHSNull, A.makeNot(*RHSNull)), PointerNE));
  // notnull != nullptr
  Env.assume(
      A.makeImplies(A.makeAnd(A.makeNot(*LHSNull), *RHSNull), PointerNE));

  // We used the pre-existing formula, so nothing to return.
  return nullptr;
}

// TODO(b/233582219): Implement promotion of nullability for initially
// unknown pointers when there is evidence that it is nullable, for example
// when the pointer is compared to nullptr, or cast to boolean.
static void transferNullCheckComparison(
    const BinaryOperator* absl_nonnull BinaryOp,
    const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  auto* LHS = BinaryOp->getLHS();
  auto* RHS = BinaryOp->getRHS();
  assert(LHS != nullptr && RHS != nullptr);

  // Boolean representing the comparison between the two pointer values.
  // We can rely on the dataflow framework to have produced a value for this.
  auto* ComparisonVal = State.Env.get<BoolValue>(*BinaryOp);
  assert(ComparisonVal != nullptr);
  auto& ComparisonFormula = ComparisonVal->formula();

  auto* LHSVal = getRawPointerValue(LHS, State.Env);
  if (!LHSVal || !hasPointerNullState(*LHSVal)) return;
  auto* RHSVal = getRawPointerValue(RHS, State.Env);
  if (!RHSVal || !hasPointerNullState(*RHSVal)) return;

  if (auto* Val = processPointerComparison(ComparisonFormula,
                                           getPointerNullState(*LHSVal).IsNull,
                                           getPointerNullState(*RHSVal).IsNull,
                                           BinaryOp->getOpcode(), State.Env))
    State.Env.setValue(*BinaryOp, *Val);
}

static void transferNullCheckImplicitCastPtrToBool(
    const Expr* absl_nonnull CastExpr, const MatchFinder::MatchResult&,
    TransferState<PointerNullabilityLattice>& State) {
  auto& A = State.Env.arena();
  if (auto* CE = dyn_cast<clang::CastExpr>(CastExpr);
      CE && CE->getSubExpr()->getType()->isNullPtrType()) {
    // nullptr_t values might have PointerValues, but never have modeled null
    // state. Skip over casts of them to bool.
    // We could explicitly set the boolean value for the cast to false, but the
    // compiler already detects the branch as unreachable, so we don't traverse
    // CFG elements that would use this boolean.
    return;
  }
  auto* PointerVal = getRawPointerValue(CastExpr->IgnoreImplicit(), State.Env);
  if (!PointerVal) return;

  auto Nullability = getPointerNullState(*PointerVal);
  if (Nullability.IsNull != nullptr)
    State.Env.setValue(*CastExpr,
                       A.makeBoolValue(A.makeNot(*Nullability.IsNull)));
  else
    State.Env.setValue(*CastExpr, A.makeTopValue());
}

static void initializeOutputParameter(
    const Expr* absl_nonnull Arg,
    TransferState<PointerNullabilityLattice>& State, const VarDecl& Param) {
  // When a function has an "output parameter" - a non-const pointer or
  // reference to a pointer of unknown nullability - assume that the function
  // may set the pointer to non-null.
  //
  // For example, in the following code sequence we assume that the function may
  // modify the pointer in a way that makes a subsequent dereference safe:
  //
  //   void maybeModify(int ** _Nonnull);
  //
  //   int *p = nullptr;
  //   initializePointer(&p);
  //   *p; // safe

  QualType ParamTy = Param.getType();
  if (ParamTy.isNull()) return;
  if (!ParamTy->isPointerType() && !ParamTy->isReferenceType()) return;
  if (!isSupportedPointerType(ParamTy->getPointeeType())) return;
  if (ParamTy->getPointeeType().isConstQualified()) return;

  // TODO: if the called function was instantiated from a template, examining
  // the instantiated param decl may miss nullability from template params.
  // TODO(b/298200521): This should extend support to annotations that suggest
  // different in/out state
  TypeNullability OuterNullability =
      getTypeNullability(Param, State.Lattice.defaults());
  auto InnerNullability = ParamTy->getAs<ReferenceType>()
                              ? ArrayRef(OuterNullability)
                              : ArrayRef(OuterNullability).drop_front();
  if (InnerNullability.front().concrete() != NullabilityKind::Unspecified)
    return;

  StorageLocation* Loc = nullptr;
  if (ParamTy->isPointerType()) {
    if (PointerValue* OuterPointer = getRawPointerValue(Arg, State.Env))
      Loc = &OuterPointer->getPointeeLoc();
  } else if (ParamTy->isReferenceType()) {
    Loc = State.Env.getStorageLocation(*Arg);
  }
  if (Loc == nullptr) return;

  if (isSupportedRawPointerType(ParamTy->getPointeeType())) {
    auto* InnerPointer =
        cast<PointerValue>(State.Env.createValue(ParamTy->getPointeeType()));
    initPointerNullState(*InnerPointer, State.Env.getDataflowAnalysisContext(),
                         NullabilityKind::Unspecified);

    State.Env.setValue(*Loc, *InnerPointer);
  } else {
    auto& SmartPointerLoc = *cast<RecordStorageLocation>(Loc);
    setToPointerWithNullability(SmartPointerLoc.getSyntheticField(PtrField),
                                NullabilityKind::Unspecified, State.Env);
  }
}

// `D` is declared somewhere in `absl` or `util`, either directly or nested.
static bool isDeclaredInAbseilOrUtil(const Decl& D) {
  const auto* DC = D.getDeclContext();
  if (DC == nullptr || DC->isTranslationUnit()) return false;

  // Find the topmost, non-TU DeclContext.
  const DeclContext* Parent = DC->getParent();
  while (Parent != nullptr && !Parent->isTranslationUnit()) {
    DC = Parent;
    Parent = DC->getParent();
  }

  // Check if it is the `absl` namespace.
  const auto* NS = dyn_cast_or_null<NamespaceDecl>(DC);
  return NS != nullptr && NS->getDeclName().isIdentifier() &&
         (NS->getName() == "absl" || NS->getName() == "util");
}

// Models the `GetReferenceableValue` functions used in Abseil logging and
// elsewhere.
static void modelGetReferenceableValue(const CallExpr& CE, Environment& Env) {
  // We only model the `GetReferenceableValue` overload that takes and returns a
  // reference.
  if (!CE.isGLValue()) return;
  assert(CE.getNumArgs() == 1);
  assert(CE.getArg(0) != nullptr);
  if (StorageLocation* Loc = Env.getStorageLocation(*CE.getArg(0)))
    Env.setStorageLocation(CE, *Loc);
}

// Models the Abseil-logging `CheckNE_Impl` function. Essentially, associates
// the `IsNull` of the call result with the comparison `arg0 != arg1`.
static void modelCheckNE(const CallExpr& CE, Environment& Env) {
  assert(isSupportedRawPointerType(CE.getType()));
  auto* PointerVal = getRawPointerValue(&CE, Env);
  if (!PointerVal)
    PointerVal = cast<PointerValue>(Env.createValue(CE.getType()));
  // Force the pointer state to `Nullable`, which we will then potentially
  // refine below.
  // TODO Add the annotation in the logging library so that we don't have
  // to hard-code this here.
  initPointerNullState(*PointerVal, Env.getDataflowAnalysisContext(),
                       NullabilityKind::Nullable);
  Env.setValue(CE, *PointerVal);
  const Formula* IsNull = getPointerNullState(*PointerVal).IsNull;
  assert(IsNull != nullptr && "`IsNull` can never be 'Top' here");

  auto* LHS = CE.getArg(0);
  auto* RHS = CE.getArg(1);
  assert(LHS != nullptr && RHS != nullptr);
  auto LTy = LHS->getType();
  auto RTy = RHS->getType();

  if (!isSupportedPointerType(LTy) && !LTy->isNullPtrType()) return;
  if (!isSupportedPointerType(RTy) && !RTy->isNullPtrType()) return;

  const Formula* LHSNull = nullptr;
  if (LTy->isNullPtrType()) {
    // Values of nullptr type are not themselves pointers and so not
    // modeled directly. They are only modeled if and when they are cast
    // to pointers. So, we need to supply a formula directly.
    LHSNull = &Env.arena().makeLiteral(true);
  } else {
    auto* V = getPointerValue(LHS, Env);
    if (!V) return;
    assert(hasPointerNullState(*V));
    LHSNull = getPointerNullState(*V).IsNull;
  }

  const Formula* RHSNull = nullptr;
  if (RTy->isNullPtrType()) {
    RHSNull = &Env.arena().makeLiteral(true);
  } else {
    auto* V = getPointerValue(RHS, Env);
    if (!V) return;
    assert(hasPointerNullState(*V));
    RHSNull = getPointerNullState(*V).IsNull;
  }

  if (auto* Val =
          processPointerComparison(*IsNull, LHSNull, RHSNull, BO_NE, Env))
    Env.assume(Env.arena().makeEquals(Val->formula(), *IsNull));
}

static bool isMethodOfAbslStatusOr(const FunctionDecl* F) {
  const auto* Method = dyn_cast<CXXMethodDecl>(F);
  if (!Method) return false;
  const CXXRecordDecl* Parent = Method->getParent();
  if (!Parent) return false;
  const CXXRecordDecl* CanonicalParent = Parent->getCanonicalDecl();
  if (!CanonicalParent) return false;
  return CanonicalParent->getQualifiedNameAsString() == "absl::StatusOr";
}

static void transferCallExpr(const CallExpr* absl_nonnull CE,
                             const MatchFinder::MatchResult& Result,
                             TransferState<PointerNullabilityLattice>& State) {
  // The dataflow framework itself generally does not model `CallExpr`s
  // (including creating values for the results). We model some specific
  // function calls and handle value creation for certain types.

  const auto* FuncDecl = CE->getDirectCallee();
  const IdentifierInfo* FunII = nullptr;
  if (FuncDecl != nullptr) {
    if ((FunII = FuncDecl->getDeclName().getAsIdentifierInfo())) {
      if (FunII->isStr("__assert_nullability")) return;

      // This is part of the implementation of `CHECK_NE`.
      if (FunII->isStr("GetReferenceableValue") &&
          isDeclaredInAbseilOrUtil(*FuncDecl)) {
        modelGetReferenceableValue(*CE, State.Env);
        return;
      }
      if (FunII->isStr("Check_NEImpl") && isDeclaredInAbseilOrUtil(*FuncDecl)) {
        modelCheckNE(*CE, State.Env);
        return;
      }
    }
  }

  StorageLocation* Loc = nullptr;
  if (CE->isGLValue()) {
    // The function returned a reference. Create a storage location for the
    // expression so that if code creates a pointer from the reference, we will
    // produce a `PointerValue`.
    Loc = State.Env.getStorageLocation(*CE);
    if (!Loc) {
      // This is subtle: We call `createStorageLocation(QualType)`, not
      // `createStorageLocation(const Expr &)`, so that we create a new
      // storage location every time.
      Loc = &State.Env.createStorageLocation(CE->getType());
      State.Env.setStorageLocation(*CE, *Loc);
    }
  }

  // Create a pointer value for any supported pointer type so that we can attach
  // nullability to it and have the nullability propagate with the pointer.
  if (isSupportedRawPointerType(CE->getType())) {
    auto* PointerVal = getRawPointerValue(CE, State.Env);
    if (!PointerVal) {
      PointerVal = cast<PointerValue>(State.Env.createValue(CE->getType()));
    }

    initPointerFromTypeNullability(*PointerVal, CE, State);

    if (Loc != nullptr)
      State.Env.setValue(*Loc, *PointerVal);
    else
      // `Loc` is set iff `CE` is a glvalue, so we know here that it must
      // be a prvalue.
      State.Env.setValue(*CE, *PointerVal);
  } else if (isSupportedSmartPointerType(CE->getType())) {
    if (Loc == nullptr) {
      // `CE` must be a prvalue; see above.
      Loc = &State.Env.getResultObjectLocation(*CE);
    }
    StorageLocation& PtrLoc =
        cast<RecordStorageLocation>(Loc)->getSyntheticField(PtrField);
    auto* Val = State.Env.get<PointerValue>(PtrLoc);
    if (Val == nullptr) {
      Val = cast<PointerValue>(State.Env.createValue(PtrLoc.getType()));
      State.Env.setValue(PtrLoc, *Val);
    }

    initPointerFromTypeNullability(*Val, CE, State);
  }

  if (CE->isCallToStdMove() || FuncDecl == nullptr) return;

  // Don't treat parameters of our macro replacement argument-capture functions
  // or of absl::StatusOr::value_or as output parameters.
  if (FunII && (FunII->isStr(ArgCaptureAbortIfFalse) ||
                FunII->isStr(ArgCaptureAbortIfEqual) ||
                (FunII->isStr("value_or") && isMethodOfAbslStatusOr(FuncDecl))))
    return;
  // Make output parameters (with unknown nullability) initialized to unknown.
  for (ParamAndArgIterator<CallExpr> Iter(*FuncDecl, *CE); Iter; ++Iter)
    initializeOutputParameter(&Iter.arg(), State, Iter.param());
}

static void transferAccessorCall(
    const CXXMemberCallExpr* absl_nonnull MCE,
    const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  auto* Member = Result.Nodes.getNodeAs<clang::ValueDecl>("member-decl");
  PointerValue* PointerVal = nullptr;
  StorageLocation* FieldLoc = nullptr;
  if (dataflow::RecordStorageLocation* RecordLoc =
          dataflow::getImplicitObjectLocation(*MCE, State.Env)) {
    FieldLoc = RecordLoc->getChild(*Member);
    PointerVal = dyn_cast_or_null<PointerValue>(State.Env.getValue(*FieldLoc));
  }
  if (!PointerVal) {
    PointerVal = ensureRawPointerHasValue(MCE, State.Env);
  }
  if (PointerVal) {
    State.Env.setValue(*MCE, *PointerVal);
    if (FieldLoc != nullptr) {
      State.Env.setValue(*FieldLoc, *PointerVal);
    }
    initPointerFromTypeNullability(*PointerVal, MCE, State);
  }
}

static std::function<void(StorageLocation&)>
initCallbackForStorageLocationIfSmartPointer(const CallExpr* absl_nonnull CE,
                                             dataflow::Environment& Env) {
  if (!isSupportedSmartPointerType(CE->getType()))
    return [](StorageLocation& Loc) {};
  return [CE, &Env](StorageLocation& Loc) {
    setSmartPointerValue(cast<RecordStorageLocation>(Loc),
                         cast<PointerValue>(Env.createValue(
                             underlyingRawPointerType(CE->getType()))),
                         Env);
  };
}

static void handleConstMemberCall(
    const CallExpr* absl_nonnull CE,
    dataflow::RecordStorageLocation* absl_nullable RecordLoc,
    const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  if (RecordLoc == nullptr) {
    // Perform default handling
    transferCallExpr(CE, Result, State);
    return;
  }

  // If the const method returns a smart pointer, handle it separately.
  // Smart pointers are represented as RecordStorangeLocations, so their
  // treatment is different from booleans or raw pointers, which are
  // represented as Values.
  if (isSupportedSmartPointerType(CE->getType())) {
    const FunctionDecl* DirectCallee = CE->getDirectCallee();
    if (DirectCallee == nullptr) {
      // Perform default handling
      transferCallExpr(CE, Result, State);
      return;
    }

    StorageLocation& Loc =
        State.Lattice.getOrCreateConstMethodReturnStorageLocation(
            *RecordLoc, DirectCallee, State.Env,
            initCallbackForStorageLocationIfSmartPointer(CE, State.Env));
    if (CE->isGLValue()) {
      // If the call to the const method returns a reference to a smart pointer,
      // we can use link the call expression to the smart pointer via
      // setStorageLocation.
      State.Env.setStorageLocation(*CE, Loc);
    } else {
      // If the call to the const method returns a smart pointer by value, we
      // need to use CopyRecord to link the smart pointer to the result object
      // of the call expression.
      copyRecord(cast<RecordStorageLocation>(Loc),
                 State.Env.getResultObjectLocation(*CE), State.Env);
    }
    return;
  }

  // If the const method returns a raw pointer or boolean (represented as
  // Values) handle them appropriately.
  if (CE->isPRValue() && (isSupportedRawPointerType(CE->getType()) ||
                          CE->getType()->isBooleanType())) {
    Value* Val = State.Lattice.getOrCreateConstMethodReturnValue(*RecordLoc, CE,
                                                                 State.Env);
    if (Val == nullptr) {
      // Perform default handling
      transferCallExpr(CE, Result, State);
      return;
    }

    State.Env.setValue(*CE, *Val);
    if (auto* PointerVal = dyn_cast<PointerValue>(Val))
      initPointerFromTypeNullability(*PointerVal, CE, State);
    return;
  }

  // If the const method returns a reference, handle it separately.
  const FunctionDecl* DirectCallee = CE->getDirectCallee();
  if (DirectCallee != nullptr &&
      DirectCallee->getReturnType()->isReferenceType()) {
    StorageLocation& Loc =
        State.Lattice.getOrCreateConstMethodReturnStorageLocation(
            *RecordLoc, DirectCallee, State.Env, [](StorageLocation& Loc) {});
    State.Env.setStorageLocation(*CE, Loc);
    return;
  }

  // Perform default handling for remaining return types
  transferCallExpr(CE, Result, State);
}

static void transferConstMemberCall(
    const CXXMemberCallExpr* absl_nonnull MCE,
    const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  handleConstMemberCall(
      MCE, dataflow::getImplicitObjectLocation(*MCE, State.Env), Result, State);
}

static void transferConstMemberOperatorCall(
    const CXXOperatorCallExpr* OCE, const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  auto* RecordLoc = cast_or_null<dataflow::RecordStorageLocation>(
      State.Env.getStorageLocation(*OCE->getArg(0)));
  handleConstMemberCall(OCE, RecordLoc, Result, State);
}

static void handleNonConstMemberCall(
    const CallExpr* absl_nonnull CE, dataflow::RecordStorageLocation* RecordLoc,
    const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  // When a non-const member function is called, clear all (non-const)
  // pointer-type fields of the receiver. Const-qualified fields can't be
  // changed (at least, not without UB).
  if (RecordLoc != nullptr) {
    for (const auto [Field, FieldLoc] : RecordLoc->children()) {
      QualType FieldType = Field->getType();
      if (FieldType.isConstQualified() || !isSupportedRawPointerType(FieldType))
        continue;

      const auto* FieldStronglyTyped = dyn_cast<FieldDecl>(Field);
      if (FieldStronglyTyped == nullptr) continue;
      if (isa<ClassTemplateSpecializationDecl>(
              FieldStronglyTyped->getParent())) {
        // We can't produce a new `PointerValue` here because we don't
        // know what to initialize its nullability properties with: A
        // `ClassTemplateSpecializationDecl` uses canonical types for
        // its type arguments (there is only one specialization for the same
        // canonical type arguments), so the `FieldDecl` doesn't contain
        // nullability annotations. The best thing we can do, therefore, is to
        // clear the value.
        // TODO{mboehme): We should resugar the type of the field, similar to
        // the way this is done in `transferType_DeclRefExpr()`.
        State.Env.clearValue(*FieldLoc);
      } else {
        auto* Val = cast<PointerValue>(
            State.Env.createValue(FieldStronglyTyped->getType()));
        State.Env.setValue(*FieldLoc, *Val);
        TypeNullability N =
            State.Lattice.getTypeNullabilityWithOverrides(*FieldStronglyTyped);
        if (N.empty()) {
          // The field has pointer type, so it should have nullability.
          // In a release build, just ignore and move on.
          assert(false);
          continue;
        }
        initPointerNullState(*Val, State.Env.getDataflowAnalysisContext(),
                             N.front());
      }
    }
    State.Lattice.clearConstMethodReturnValues(*RecordLoc);
    State.Lattice.clearConstMethodReturnStorageLocations(*RecordLoc);
  }

  // Perform default handling.
  transferCallExpr(CE, Result, State);
}

static void transferNonConstMemberCall(
    const CXXMemberCallExpr* absl_nonnull MCE,
    const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  handleNonConstMemberCall(
      MCE, dataflow::getImplicitObjectLocation(*MCE, State.Env), Result, State);
}

static void transferNonConstMemberOperatorCall(
    const CXXOperatorCallExpr* absl_nonnull OCE,
    const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  auto* RecordLoc = cast_or_null<dataflow::RecordStorageLocation>(
      State.Env.getStorageLocation(*OCE->getArg(0)));
  handleNonConstMemberCall(OCE, RecordLoc, Result, State);
}

static void transferStatusOrValueOrCall(
    const CXXMemberCallExpr* absl_nonnull MCE,
    const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  // Some overloads are const, some are not. Start with default handling as
  // appropriate for constness. Do this first so that the result value will be
  // initialized for us; we don't mind that the const method cache is cleared
  // before we proceed if this is a non-const call.
  if (auto* MethodDecl = MCE->getMethodDecl();
      MethodDecl && !MethodDecl->isConst()) {
    transferNonConstMemberCall(MCE, Result, State);
  } else {
    transferCallExpr(MCE, Result, State);
  }

  // absl::StatusOr::value_or can take any argument convertible to the template
  // argument type, and claims to return the template argument type. But it
  // considers a nullable value or type to be convertible to a nonnull type.
  // Rather than warn on an actually non-convertible argument to value_or, we
  // model the return value as having the null state of potentially either of
  // the argument or the StatusOr's contained pointer, depending on the
  // untracked internal state of the StatusOr, and warn on any incompatible
  // usage later.
  if (!isSupportedPointerType(MCE->getType()) ||
      // getNumArgs does not count the implicit *this argument.
      MCE->getNumArgs() != 1)
    return;
  Arena& A = State.Env.arena();

  PointerValue* ResultPV = getPointerValue(MCE, State.Env);
  if (!ResultPV) return;
  PointerNullState ResultState = getPointerNullState(*ResultPV);

  // Get the null state of the value_or argument.
  const Expr* ArgExpr = MCE->getArg(0);
  if (!ArgExpr) return;
  PointerValue* Arg = getPointerValue(ArgExpr, State.Env);
  std::optional<PointerNullState> ArgState;
  if (Arg) {
    ArgState = getPointerNullState(*Arg);
  } else if (ArgExpr->getType()->isNullPtrType() &&
             isReachableNullptrLiteral(State.Env)) {
    ArgState = PointerNullState{.FromNullable = &A.makeLiteral(true),
                                .IsNull = &A.makeLiteral(true)};
  } else {
    // This is never expected to happen, so always log, and assert-fail when
    // enabled.
    llvm::errs() << "Unable to determine PointerNullState for an argument to "
                    "absl::StatusOr<SupportedPointerType>::value_or. Please "
                    "file a bug at <internal link> if you see this.\n";
    assert(false);
    return;
  }

  // The null state corresponding to the StatusOr's template argument type is
  // captured as the current null state of the call's result. Re-assign the null
  // state properties of the call's result to be fresh atoms implied by the
  // untracked state of the StatusOr, also modeled as a fresh atom, to be equal
  // to the null state properties drawn from either the template argument type
  // or the value_or argument.
  //
  // value_or creates copies of the contained pointer or the argument, so only
  // the current null states are relevant; we don't need to account for later
  // modification of e.g. a referenced decl.
  const Formula& StatusOrIsOk = A.makeAtomRef(A.makeAtom());
  DataflowAnalysisContext& DACtx = State.Env.getDataflowAnalysisContext();
  if (ResultState.FromNullable != nullptr) {
    if (ArgState->FromNullable == nullptr) {
      ResultState.FromNullable = nullptr;
    } else {
      const Formula& OldResultFromNullable = *ResultState.FromNullable;
      ResultState.FromNullable = &A.makeAtomRef(A.makeAtom());
      DACtx.addInvariant(A.makeImplies(
          StatusOrIsOk,
          A.makeEquals(*ResultState.FromNullable, OldResultFromNullable)));
      DACtx.addInvariant(A.makeImplies(
          A.makeNot(StatusOrIsOk),
          A.makeEquals(*ResultState.FromNullable, *ArgState->FromNullable)));
    }
  }
  if (ResultState.IsNull != nullptr) {
    if (ArgState->IsNull == nullptr) {
      ResultState.IsNull = nullptr;
    } else {
      const Formula& OldResultIsNull = *ResultState.IsNull;
      ResultState.IsNull = &A.makeAtomRef(A.makeAtom());
      DACtx.addInvariant(A.makeImplies(
          StatusOrIsOk, A.makeEquals(*ResultState.IsNull, OldResultIsNull)));
      DACtx.addInvariant(
          A.makeImplies(A.makeNot(StatusOrIsOk),
                        A.makeEquals(*ResultState.IsNull, *ArgState->IsNull)));
    }
  }

  auto& NewPointerVal =
      State.Env.create<PointerValue>(ResultPV->getPointeeLoc());
  initPointerNullState(NewPointerVal, DACtx, ResultState);
  if (isSupportedRawPointerType(MCE->getType())) {
    State.Env.setValue(*MCE, NewPointerVal);
  } else if (isSupportedSmartPointerType(MCE->getType())) {
    setSmartPointerValue(State.Env.getResultObjectLocation(*MCE),
                         &NewPointerVal, State.Env);
  }
}

dataflow::CFGMatchSwitch<dataflow::TransferState<PointerNullabilityLattice>>
buildValueTransferer() {
  // The value transfer functions must establish:
  // - if we're transferring over an Expr
  // - and the Expr has a supported pointer type
  // - and the Expr's value is modeled by the framework (or this analysis)
  // - then the PointerValue has nullability properties (is_null/from_nullable)
  return dataflow::CFGMatchSwitchBuilder<
             TransferState<PointerNullabilityLattice>>()
      // Handles initialization of the null states of pointers.
      .CaseOfCFGStmt<Expr>(isAddrOf(), transferNotNullPointer)
      // TODO(mboehme): I believe we should be able to move handling of null
      // pointers to the non-flow-sensitive part of the analysis.
      .CaseOfCFGStmt<Expr>(isNullPointerLiteral(), transferNullPointer)
      .CaseOfCFGStmt<CXXScalarValueInitExpr>(isRawPointerValueInit(),
                                             transferNullPointer)
      .CaseOfCFGStmt<ImplicitValueInitExpr>(isRawPointerImplicitValueInit(),
                                            transferNullPointer)
      .CaseOfCFGStmt<CXXDefaultInitExpr>(isNullPointerDefaultInit(),
                                         transferNullPointer)
      .CaseOfCFGStmt<UnaryOperator>(isPointerIncOrDec(),
                                    transferPointerIncOrDec)
      .CaseOfCFGStmt<BinaryOperator>(isPointerAddOrSubAssign(),
                                     transferPointerAddOrSubAssign)
      .CaseOfCFGStmt<CXXConstructExpr>(isSmartPointerConstructor(),
                                       transferSmartPointerConstructor)
      .CaseOfCFGStmt<CXXOperatorCallExpr>(isSmartPointerOperatorCall("=", 2),
                                          transferSmartPointerAssignment)
      .CaseOfCFGStmt<CXXMemberCallExpr>(
          isSmartPointerMethodCall("release", "Release"),
          transferSmartPointerReleaseCall)
      .CaseOfCFGStmt<CXXMemberCallExpr>(
          isSmartPointerMethodCall("reset", "Reset"),
          transferSmartPointerResetCall)
      .CaseOfCFGStmt<CXXMemberCallExpr>(
          isSmartPointerMethodCall("swap", "Swap"),
          transferSmartPointerMemberSwapCall)
      .CaseOfCFGStmt<CallExpr>(isSmartPointerFreeSwapCall(),
                               transferSmartPointerFreeSwapCall)
      .CaseOfCFGStmt<CXXMemberCallExpr>(isSmartPointerMethodCall("get", "Get"),
                                        transferSmartPointerGetCall)
      .CaseOfCFGStmt<CXXMemberCallExpr>(isSmartPointerBoolConversionCall(),
                                        transferSmartPointerBoolConversionCall)
      .CaseOfCFGStmt<CXXOperatorCallExpr>(isSmartPointerOperatorCall("*", 1),
                                          transferSmartPointerOperatorStar)
      .CaseOfCFGStmt<CXXOperatorCallExpr>(isSmartPointerOperatorCall("->", 1),
                                          transferSmartPointerOperatorArrow)
      .CaseOfCFGStmt<CallExpr>(isSmartPointerFactoryCall(),
                               transferSmartPointerFactoryCall)
      .CaseOfCFGStmt<CallExpr>(isWrapUniqueCall(), transferWrapUniqueCall)
      .CaseOfCFGStmt<CXXOperatorCallExpr>(isSmartPointerComparisonOpCall(),
                                          transferSmartPointerComparisonOpCall)
      .CaseOfCFGStmt<CallExpr>(isSharedPtrCastCall(), transferSharedPtrCastCall)
      .CaseOfCFGStmt<CXXMemberCallExpr>(isWeakPtrLockCall(),
                                        transferWeakPtrLockCall)
      .CaseOfCFGStmt<CXXMemberCallExpr>(isSupportedPointerAccessorCall(),
                                        transferAccessorCall)
      .CaseOfCFGStmt<CXXOperatorCallExpr>(
          dataflow::isSmartPointerLikeOperatorStar(),
          [](const CXXOperatorCallExpr* CE,
             const MatchFinder::MatchResult& Result,
             TransferState<PointerNullabilityLattice>& State) {
            auto* RecordLoc = cast_or_null<dataflow::RecordStorageLocation>(
                State.Env.getStorageLocation(*CE->getArg(0)));
            dataflow::transferSmartPointerLikeCachedDeref(
                CE, RecordLoc, State,
                initCallbackForStorageLocationIfSmartPointer(CE, State.Env));
          })
      .CaseOfCFGStmt<CXXOperatorCallExpr>(
          dataflow::isSmartPointerLikeOperatorArrow(),
          [](const CXXOperatorCallExpr* CE,
             const MatchFinder::MatchResult& Result,
             TransferState<PointerNullabilityLattice>& State) {
            auto* RecordLoc = cast_or_null<dataflow::RecordStorageLocation>(
                State.Env.getStorageLocation(*CE->getArg(0)));
            dataflow::transferSmartPointerLikeCachedGet(
                CE, RecordLoc, State,
                initCallbackForStorageLocationIfSmartPointer(CE, State.Env));
          })
      .CaseOfCFGStmt<CXXMemberCallExpr>(
          dataflow::isSmartPointerLikeValueMethodCall(),
          [](const CXXMemberCallExpr* CE,
             const MatchFinder::MatchResult& Result,
             TransferState<PointerNullabilityLattice>& State) {
            dataflow::transferSmartPointerLikeCachedDeref(
                CE, getImplicitObjectLocation(*CE, State.Env), State,
                initCallbackForStorageLocationIfSmartPointer(CE, State.Env));
          })
      .CaseOfCFGStmt<CXXMemberCallExpr>(
          dataflow::isSmartPointerLikeGetMethodCall(),
          [](const CXXMemberCallExpr* CE,
             const MatchFinder::MatchResult& Result,
             TransferState<PointerNullabilityLattice>& State) {
            dataflow::transferSmartPointerLikeCachedGet(
                CE, getImplicitObjectLocation(*CE, State.Env), State,
                initCallbackForStorageLocationIfSmartPointer(CE, State.Env));
          })
      .CaseOfCFGStmt<CXXMemberCallExpr>(isStatusOrValueOrCall(),
                                        transferStatusOrValueOrCall)
      .CaseOfCFGStmt<CXXMemberCallExpr>(isZeroParamConstMemberCall(),
                                        transferConstMemberCall)
      .CaseOfCFGStmt<CXXOperatorCallExpr>(isZeroParamConstMemberOperatorCall(),
                                          transferConstMemberOperatorCall)
      .CaseOfCFGStmt<CXXMemberCallExpr>(isNonConstMemberCall(),
                                        transferNonConstMemberCall)
      .CaseOfCFGStmt<CXXOperatorCallExpr>(isNonConstMemberOperatorCall(),
                                          transferNonConstMemberOperatorCall)
      .CaseOfCFGStmt<CallExpr>(ast_matchers::callExpr(), transferCallExpr)
      .CaseOfCFGStmt<MemberExpr>(isSmartPointerArrowMemberExpr(),
                                 transferSmartPointerArrowMemberExpr)
      .CaseOfCFGStmt<Expr>(isPointerExpr(), transferPointer)
      // Handles comparison between 2 pointers.
      .CaseOfCFGStmt<BinaryOperator>(isPointerCheckBinOp(),
                                     transferNullCheckComparison)
      // Handles checking of pointer as boolean.
      .CaseOfCFGStmt<Expr>(isImplicitCastPointerToBool(),
                           transferNullCheckImplicitCastPtrToBool)
      .Build();
}

void ensureSmartPointerInitialized(
    const CFGElement& Elt, TransferState<PointerNullabilityLattice>& State) {
  auto S = Elt.getAs<CFGStmt>();
  if (!S) return;

  auto* E = dyn_cast<Expr>(S->getStmt());
  if (E == nullptr || !isSupportedSmartPointerType(E->getType())) return;

  initSmartPointerForExpr(E, State);

  auto* SmartPtrLoc = E->isGLValue() ? State.Env.get<RecordStorageLocation>(*E)
                                     : &State.Env.getResultObjectLocation(*E);
  if (SmartPtrLoc == nullptr) return;
  StorageLocation& PtrLoc = SmartPtrLoc->getSyntheticField(PtrField);
  unpackPointerValue(PtrLoc, State.Env);
}
}  // namespace clang::tidy::nullability
