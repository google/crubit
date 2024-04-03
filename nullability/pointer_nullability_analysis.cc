// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/pointer_nullability_analysis.h"

#include <cassert>
#include <deque>
#include <functional>
#include <optional>
#include <vector>

#include "absl/base/nullability.h"
#include "absl/log/check.h"
#include "nullability/ast_helpers.h"
#include "nullability/pointer_nullability.h"
#include "nullability/pointer_nullability_lattice.h"
#include "nullability/pointer_nullability_matchers.h"
#include "nullability/pragma.h"
#include "nullability/type_nullability.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclTemplate.h"
#include "clang/AST/Expr.h"
#include "clang/AST/ExprCXX.h"
#include "clang/AST/OperationKinds.h"
#include "clang/AST/Stmt.h"
#include "clang/AST/Type.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/ASTMatchers/ASTMatchers.h"
#include "clang/Analysis/CFG.h"
#include "clang/Analysis/FlowSensitive/Arena.h"
#include "clang/Analysis/FlowSensitive/CFGMatchSwitch.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysis.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysisContext.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/Formula.h"
#include "clang/Analysis/FlowSensitive/StorageLocation.h"
#include "clang/Analysis/FlowSensitive/Value.h"
#include "clang/Basic/IdentifierTable.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/OperatorKinds.h"
#include "clang/Basic/Specifiers.h"
#include "llvm/ADT/StringMap.h"
#include "llvm/Support/Casting.h"
#include "llvm/Support/Debug.h"
#include "llvm/Support/raw_ostream.h"

namespace clang::tidy::nullability {

using ast_matchers::anyOf;
using ast_matchers::MatchFinder;
using dataflow::Arena;
using dataflow::BoolValue;
using dataflow::CFGMatchSwitchBuilder;
using dataflow::ComparisonResult;
using dataflow::DataflowAnalysisContext;
using dataflow::Environment;
using dataflow::Formula;
using dataflow::PointerValue;
using dataflow::RecordStorageLocation;
using dataflow::StorageLocation;
using dataflow::TransferState;
using dataflow::Value;

#define DEBUG_TYPE "pointer_nullability_analysis.cc"

namespace {

TypeNullability prepend(NullabilityKind Head, const TypeNullability &Tail) {
  TypeNullability Result = {Head};
  Result.insert(Result.end(), Tail.begin(), Tail.end());
  return Result;
}

void computeNullability(absl::Nonnull<const Expr *> E,
                        TransferState<PointerNullabilityLattice> &State,
                        std::function<TypeNullability()> Compute) {
  (void)State.Lattice.insertExprNullabilityIfAbsent(E, [&] {
    auto Nullability = Compute();
    if (unsigned ExpectedSize = countPointersInType(E);
        ExpectedSize != Nullability.size()) {
      // A nullability vector must have one entry per pointer in the type.
      // If this is violated, we probably failed to handle some AST node.
      LLVM_DEBUG({
        llvm::dbgs()
            << "=== Nullability vector has wrong number of entries: ===\n";
        llvm::dbgs() << "Expression: \n";
        dump(E, llvm::dbgs());
        llvm::dbgs() << "\nNullability (" << Nullability.size()
                     << " pointers): " << nullabilityToString(Nullability)
                     << "\n";
        llvm::dbgs() << "\nType (" << ExpectedSize << " pointers): \n";
        dump(exprType(E), llvm::dbgs());
        llvm::dbgs() << "=================================\n";
      });

      // We can't meaningfully interpret the vector, so discard it.
      // TODO: fix all broken cases and upgrade to CHECK or DCHECK or so.
      Nullability.assign(ExpectedSize, NullabilityKind::Unspecified);
    }
    return Nullability;
  });
}

// Returns the computed nullability for a subexpr of the current expression.
// This is always available as we compute bottom-up.
const TypeNullability &getNullabilityForChild(
    absl::Nonnull<const Expr *> E,
    TransferState<PointerNullabilityLattice> &State) {
  return State.Lattice.insertExprNullabilityIfAbsent(E, [&] {
    // Since we process child nodes before parents, we should already have
    // computed the child nullability. However, this is not true in all test
    // cases. So, we return unspecified nullability annotations.
    // TODO: fix this issue, and CHECK() instead.
    LLVM_DEBUG({
      llvm::dbgs() << "=== Missing child nullability: ===\n";
      dump(E, llvm::dbgs());
      llvm::dbgs() << "==================================\n";
    });

    return unspecifiedNullability(E);
  });
}

// The Resugarer describes the nullability of template arguments within types we
// query using getTypeNullability().
//
// When the template arguments are bound within the queried type, e.g.
//   getTypeNullability( vector<Nonnull<int*>>::value_type )
// then getTypeNullability() will record the sugar and resolve the
// SubstTemplateTypeParmType within `value_type` itself.
//
// However when the template arguments are bound elsewhere in the code, e.g.
//   vector<Nonnull<int*>> a;
//   getTypeNullability( a.front() )
// then we must provide the nullability vector, via the callback passed
// to getTypeNullability().
//
// This class implements that callback interface, based on the common patterns
// where template arguments can be determined from surrounding code.
struct Resugarer {
  using SubstTy = SubstTemplateTypeParmType;
  const TypeNullabilityDefaults &Defaults;

  // The entity referenced is nested within a class template, e.g. `a.front()`
  // where a is a vector<Nonnull<int*>>.
  // We have a nullability vector [Nonnull] for the specialization vector<int*>.
  struct FromEnclosingClassNullability {
    ClassTemplateSpecializationDecl *Specialization;
    const ArrayRef<PointerTypeNullability> SpecializationNullability;

    std::optional<TypeNullability> operator()(const SubstTy *ST) const {
      if (Specialization != ST->getAssociatedDecl()) return std::nullopt;
      // TODO: The code below does not deal correctly with partial
      // specializations. We should eventually handle these, but for now, just
      // bail out.
      if (isa<ClassTemplatePartialSpecializationDecl>(
              ST->getReplacedParameter()->getDeclContext()))
        return std::nullopt;

      unsigned ArgIndex = ST->getIndex();
      auto TemplateArgs = Specialization->getTemplateArgs().asArray();

      // TODO: If the type was substituted from a pack template argument,
      // we must find the slice that pertains to this particular type.
      // For now, just give up on resugaring this type.
      if (ST->getPackIndex().has_value()) return std::nullopt;

      unsigned PointerCount =
          countPointersInType(Specialization->getDeclContext());
      for (auto TA : TemplateArgs.take_front(ArgIndex)) {
        PointerCount += countPointersInType(TA);
      }
      unsigned SliceSize = countPointersInType(TemplateArgs[ArgIndex]);
      return SpecializationNullability.slice(PointerCount, SliceSize).vec();
    }
  };
  llvm::SmallVector<FromEnclosingClassNullability> Enclosing;

  // The entity is referenced using template arguments, e.g.
  // `make_unique<Nonnull<int*>>`. We have the template arguments.
  struct FromTemplateArgs {
    TemplateDecl *Template;
    ArrayRef<TemplateArgumentLoc> Args;

    std::optional<TypeNullability> operator()(
        const SubstTy *ST, const TypeNullabilityDefaults &Defaults) const {
      if (Template != ST->getAssociatedDecl()) return std::nullopt;
      // Some or all of the template arguments may be deduced, and we won't
      // see those on the `DeclRefExpr`. If the template argument was deduced,
      // we don't have any sugar for it.
      // TODO(b/268348533): Can we somehow obtain it from e.g. the function
      // param it was deduced from?
      // TODO(b/268345783): This check, as well as the index into
      // `template_arguments` below, may be incorrect in the presence of
      // parameters packs.  In function templates, parameter packs may appear
      // anywhere in the parameter list. The index may therefore refer to one
      // of the pack arguments, but we might incorrectly interpret it as
      // referring to an argument that follows the pack.
      if (ST->getIndex() >= Args.size()) return std::nullopt;

      TypeSourceInfo *TSI = Args[ST->getIndex()].getTypeSourceInfo();
      if (TSI == nullptr) return std::nullopt;
      return getTypeNullability(TSI->getTypeLoc(), Defaults);
    }
  };
  llvm::SmallVector<FromTemplateArgs> Template;

  // Add a FromTemplateArgs context reflecting that the specialization
  // `ResolvedTo` was chosen using the provided template arguments.
  void addTemplateArgs(const ValueDecl *ResolvedTo,
                       ArrayRef<TemplateArgumentLoc> UsingArgs) {
    if (const auto *VD = llvm::dyn_cast<VarDecl>(ResolvedTo)) {
      Template.push_back(
          {VD->getTemplateInstantiationPattern()->getDescribedVarTemplate(),
           UsingArgs});
    } else if (auto *FD = llvm::dyn_cast<FunctionDecl>(ResolvedTo)) {
      Template.push_back(
          {FD->getTemplateSpecializationInfo()->getTemplate(), UsingArgs});
    }
  }

  // Implement the getTypeNullability() callback interface by searching
  // all our contexts for a match.
  std::optional<TypeNullability> operator()(const SubstTy *ST) const {
    for (const auto &R : Enclosing)
      if (auto Ret = R(ST)) return Ret;
    for (const auto &R : Template)
      if (auto Ret = R(ST, Defaults)) return Ret;
    return std::nullopt;
  }
};

PointerTypeNullability getPointerTypeNullability(
    absl::Nonnull<const Expr *> E, PointerNullabilityAnalysis::Lattice &L) {
  // TODO: handle this in non-flow-sensitive transfer instead
  if (auto FromClang = E->getType()->getNullability();
      FromClang && *FromClang != NullabilityKind::Unspecified)
    return *FromClang;

  if (const auto *NonFlowSensitive = L.getExprNullability(E)) {
    if (!NonFlowSensitive->empty())
      // Return the nullability of the topmost pointer in the type.
      return NonFlowSensitive->front();
  }

  return NullabilityKind::Unspecified;
}

void initPointerFromTypeNullability(
    PointerValue &PointerVal, absl::Nonnull<const Expr *> E,
    TransferState<PointerNullabilityLattice> &State) {
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
absl::Nullable<PointerValue *> unpackPointerValue(StorageLocation &PointerLoc,
                                                  Environment &Env) {
  auto *PointerVal = Env.get<PointerValue>(PointerLoc);
  if (!PointerVal) return nullptr;

  PointerNullState NullState = getPointerNullState(*PointerVal);
  if (NullState.FromNullable && NullState.IsNull) return PointerVal;

  auto &A = Env.getDataflowAnalysisContext().arena();

  if (NullState.FromNullable == nullptr)
    NullState.FromNullable = &A.makeAtomRef(A.makeAtom());
  if (NullState.IsNull == nullptr)
    NullState.IsNull = &A.makeAtomRef(A.makeAtom());

  auto &NewPointerVal = Env.create<PointerValue>(PointerVal->getPointeeLoc());
  initPointerNullState(NewPointerVal, Env.getDataflowAnalysisContext(),
                       NullState);
  Env.setValue(PointerLoc, NewPointerVal);

  return &NewPointerVal;
}

void setToPointerWithNullability(StorageLocation &PtrLoc, NullabilityKind NK,
                                 Environment &Env) {
  auto &Val = *cast<PointerValue>(Env.createValue(PtrLoc.getType()));
  initPointerNullState(Val, Env.getDataflowAnalysisContext(), NK);
  Env.setValue(PtrLoc, Val);
}

void initSmartPointerForExpr(const Expr *E,
                             TransferState<PointerNullabilityLattice> &State) {
  RecordStorageLocation *Loc = nullptr;
  if (E->isPRValue()) {
    Loc = &State.Env.getResultObjectLocation(*E);
  } else {
    Loc = State.Env.get<RecordStorageLocation>(*E);
    if (Loc == nullptr) {
      Loc = &cast<RecordStorageLocation>(State.Env.createStorageLocation(*E));
      State.Env.setStorageLocation(*E, *Loc);
    }
  }

  StorageLocation &PtrLoc = Loc->getSyntheticField(PtrField);
  auto *Val = State.Env.get<PointerValue>(PtrLoc);
  if (Val == nullptr) {
    Val = cast<PointerValue>(State.Env.createValue(PtrLoc.getType()));
    State.Env.setValue(PtrLoc, *Val);
  }

  initPointerFromTypeNullability(*Val, E, State);
}

void transferValue_NullPointer(
    absl::Nonnull<const Expr *> NullPointer, const MatchFinder::MatchResult &,
    TransferState<PointerNullabilityLattice> &State) {
  if (auto *PointerVal = getRawPointerValue(NullPointer, State.Env)) {
    initNullPointer(*PointerVal, State.Env.getDataflowAnalysisContext());
  }
}

void transferValue_NotNullPointer(
    absl::Nonnull<const Expr *> NotNullPointer,
    const MatchFinder::MatchResult &,
    TransferState<PointerNullabilityLattice> &State) {
  if (auto *PointerVal = getRawPointerValue(NotNullPointer, State.Env)) {
    initPointerNullState(*PointerVal, State.Env.getDataflowAnalysisContext(),
                         NullabilityKind::NonNull);
  }
}

bool isStdWeakPtrType(QualType Ty) {
  const CXXRecordDecl *RD = Ty.getCanonicalType()->getAsCXXRecordDecl();
  if (RD == nullptr) return false;

  if (!RD->getDeclContext()->isStdNamespace()) return false;

  const IdentifierInfo *ID = RD->getIdentifier();
  if (ID == nullptr) return false;

  return ID->getName() == "weak_ptr";
}

bool isPointerTypeConvertible(QualType From, QualType To) {
  assert(isSupportedRawPointerType(From));
  assert(isSupportedRawPointerType(To));

  if (From == To) return true;

  auto *FromDecl = From->getPointeeType()->getAsCXXRecordDecl();
  auto *ToDecl = To->getPointeeType()->getAsCXXRecordDecl();

  // If these aren't pointers to records, then just assume they're convertible.
  if (FromDecl == nullptr || ToDecl == nullptr) return true;

  return FromDecl == ToDecl || FromDecl->isDerivedFrom(ToDecl);
}

void transferValue_SmartPointerConstructor(
    const CXXConstructExpr *Ctor, const MatchFinder::MatchResult &Result,
    TransferState<PointerNullabilityLattice> &State) {
  RecordStorageLocation &Loc = State.Env.getResultObjectLocation(*Ctor);
  // Create a `RecordValue`, associate it with the `Loc` and the expression.
  State.Env.setValue(*Ctor, refreshRecordValue(Loc, State.Env));

  // Default and `nullptr_t` constructor.
  if (Ctor->getConstructor()->isDefaultConstructor() ||
      (Ctor->getNumArgs() >= 1 &&
       Ctor->getArg(0)->getType()->isNullPtrType())) {
    setSmartPointerToNull(Loc, State.Env);
    return;
  }

  // Construct from raw pointer.
  if (Ctor->getNumArgs() >= 1 &&
      isSupportedRawPointerType(Ctor->getArg(0)->getType()) &&
      isPointerTypeConvertible(Ctor->getArg(0)->getType(),
                               Loc.getSyntheticField(PtrField).getType())) {
    setSmartPointerValue(Loc, getRawPointerValue(Ctor->getArg(0), State.Env),
                         State.Env);
    return;
  }

  // Copy or move from an existing smart pointer.
  if (Ctor->getNumArgs() >= 1 &&
      isSupportedSmartPointerType(Ctor->getArg(0)->getType())) {
    auto *SrcLoc = State.Env.get<RecordStorageLocation>(*Ctor->getArg(0));
    if (Ctor->getNumArgs() == 2 &&
        isSupportedRawPointerType(Ctor->getArg(1)->getType())) {
      // `shared_ptr` aliasing constructor.
      setSmartPointerValue(Loc, getRawPointerValue(Ctor->getArg(1), State.Env),
                           State.Env);
    } else {
      setSmartPointerValue(
          Loc, getPointerValueFromSmartPointer(SrcLoc, State.Env), State.Env);
    }

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

void transferValue_SmartPointerAssignment(
    const CXXOperatorCallExpr *OpCall, const MatchFinder::MatchResult &Result,
    TransferState<PointerNullabilityLattice> &State) {
  auto *Loc = State.Env.get<RecordStorageLocation>(*OpCall->getArg(0));
  if (Loc == nullptr) return;

  if (OpCall->getArg(1)->getType()->isNullPtrType()) {
    setSmartPointerToNull(*Loc, State.Env);
    return;
  }

  auto *SrcLoc = State.Env.get<RecordStorageLocation>(*OpCall->getArg(1));
  setSmartPointerValue(*Loc, getPointerValueFromSmartPointer(SrcLoc, State.Env),
                       State.Env);

  // If this is the move assignment operator, set the source to null.
  auto *Method = dyn_cast_or_null<CXXMethodDecl>(OpCall->getCalleeDecl());
  if (Method != nullptr &&
      Method->getParamDecl(0)->getType()->isRValueReferenceType()) {
    setSmartPointerToNull(*SrcLoc, State.Env);
  }
}

void transferValue_SmartPointerReleaseCall(
    const CXXMemberCallExpr *MCE, const MatchFinder::MatchResult &Result,
    TransferState<PointerNullabilityLattice> &State) {
  RecordStorageLocation *Loc = getImplicitObjectLocation(*MCE, State.Env);
  if (Loc == nullptr) return;
  StorageLocation &PtrLoc = Loc->getSyntheticField(PtrField);

  if (auto *Val = State.Env.get<PointerValue>(PtrLoc))
    State.Env.setValue(*MCE, *Val);
  State.Env.setValue(
      PtrLoc, createNullPointer(PtrLoc.getType()->getPointeeType(), State.Env));
}

void transferValue_SmartPointerResetCall(
    const CXXMemberCallExpr *MCE, const MatchFinder::MatchResult &Result,
    TransferState<PointerNullabilityLattice> &State) {
  RecordStorageLocation *Loc = getImplicitObjectLocation(*MCE, State.Env);
  if (Loc == nullptr) return;

  // Zero-arg and `nullptr_t` overloads, as well as single-argument constructor
  // with default argument.
  if (MCE->getNumArgs() == 0 ||
      (MCE->getNumArgs() == 1 && MCE->getArg(0)->getType()->isNullPtrType()) ||
      (MCE->getNumArgs() == 1 && MCE->getArg(0)->isDefaultArgument())) {
    setSmartPointerToNull(*Loc, State.Env);
    return;
  }

  setSmartPointerValue(*Loc, getRawPointerValue(MCE->getArg(0), State.Env),
                       State.Env);
}

void swapSmartPointers(RecordStorageLocation *Loc1, RecordStorageLocation *Loc2,
                       Environment &Env) {
  PointerValue *Val1 = getPointerValueFromSmartPointer(Loc1, Env);
  PointerValue *Val2 = getPointerValueFromSmartPointer(Loc2, Env);

  if (Loc1) setSmartPointerValue(*Loc1, Val2, Env);
  if (Loc2) setSmartPointerValue(*Loc2, Val1, Env);
}

void transferValue_SmartPointerMemberSwapCall(
    const CXXMemberCallExpr *MCE, const MatchFinder::MatchResult &Result,
    TransferState<PointerNullabilityLattice> &State) {
  swapSmartPointers(getImplicitObjectLocation(*MCE, State.Env),
                    State.Env.get<RecordStorageLocation>(*MCE->getArg(0)),
                    State.Env);
}

void transferValue_SmartPointerFreeSwapCall(
    const CallExpr *CE, const MatchFinder::MatchResult &Result,
    TransferState<PointerNullabilityLattice> &State) {
  swapSmartPointers(State.Env.get<RecordStorageLocation>(*CE->getArg(0)),
                    State.Env.get<RecordStorageLocation>(*CE->getArg(1)),
                    State.Env);
}

void transferValue_SmartPointerGetCall(
    const CXXMemberCallExpr *MCE, const MatchFinder::MatchResult &Result,
    TransferState<PointerNullabilityLattice> &State) {
  if (Value *Val = getPointerValueFromSmartPointer(
          getImplicitObjectLocation(*MCE, State.Env), State.Env))
    State.Env.setValue(*MCE, *Val);
}

void transferValue_SmartPointerBoolConversionCall(
    const CXXMemberCallExpr *MCE, const MatchFinder::MatchResult &Result,
    TransferState<PointerNullabilityLattice> &State) {
  if (PointerValue *Val = getPointerValueFromSmartPointer(
          getImplicitObjectLocation(*MCE, State.Env), State.Env)) {
    if (const Formula *IsNull = getPointerNullState(*Val).IsNull)
      State.Env.setValue(
          *MCE, State.Env.makeNot(State.Env.arena().makeBoolValue(*IsNull)));
  }
}

void transferValue_SmartPointerOperatorStar(
    const CXXOperatorCallExpr *OpCall, const MatchFinder::MatchResult &Result,
    TransferState<PointerNullabilityLattice> &State) {
  if (PointerValue *Val = getSmartPointerValue(OpCall->getArg(0), State.Env)) {
    State.Env.setStorageLocation(*OpCall, Val->getPointeeLoc());
  }
}

void transferValue_SmartPointerOperatorArrow(
    const CXXOperatorCallExpr *OpCall, const MatchFinder::MatchResult &Result,
    TransferState<PointerNullabilityLattice> &State) {
  if (PointerValue *Val = getSmartPointerValue(OpCall->getArg(0), State.Env)) {
    State.Env.setValue(*OpCall, *Val);
  }
}

void transferValue_SmartPointerFactoryCall(
    const CallExpr *CE, const MatchFinder::MatchResult &Result,
    TransferState<PointerNullabilityLattice> &State) {
  RecordStorageLocation &Loc = State.Env.getResultObjectLocation(*CE);
  // Create a `RecordValue`, associate it with the `Loc` and the expression.
  State.Env.setValue(*CE, refreshRecordValue(Loc, State.Env));
  StorageLocation &PtrLoc = Loc.getSyntheticField(PtrField);

  setToPointerWithNullability(PtrLoc, NullabilityKind::NonNull, State.Env);
}

void transferValue_SmartPointerComparisonOpCall(
    const CXXOperatorCallExpr *OpCall, const MatchFinder::MatchResult &Result,
    TransferState<PointerNullabilityLattice> &State) {
  // Formula representing an equality (`==`) comparison of the two operands.
  // If the operator is `!=`, this will need to be negated below.
  const Formula *EqualityFormula = nullptr;

  bool NullPtr1 = OpCall->getArg(0)->getType()->isNullPtrType();
  bool NullPtr2 = OpCall->getArg(1)->getType()->isNullPtrType();
  assert(!NullPtr1 || !NullPtr2);

  PointerValue *Val1 = nullptr;
  if (!NullPtr1) Val1 = getSmartPointerValue(OpCall->getArg(0), State.Env);

  PointerValue *Val2 = nullptr;
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

  BoolValue &EqualityValue = State.Env.arena().makeBoolValue(*EqualityFormula);

  if (OpCall->getOperator() == OO_EqualEqual)
    State.Env.setValue(*OpCall, EqualityValue);
  else
    State.Env.setValue(*OpCall, State.Env.makeNot(EqualityValue));
}

void transferValue_SharedPtrCastCall(
    const CallExpr *CE, const MatchFinder::MatchResult &Result,
    TransferState<PointerNullabilityLattice> &State) {
  if (!smartPointersEnabled()) return;

  Environment &Env = State.Env;
  DataflowAnalysisContext &Ctx = Env.getDataflowAnalysisContext();
  Arena &A = Env.arena();

  auto *Callee = dyn_cast_or_null<FunctionDecl>(CE->getCalleeDecl());
  if (Callee == nullptr) return;

  auto *SrcLoc = Env.get<RecordStorageLocation>(*CE->getArg(0));
  if (SrcLoc == nullptr) return;
  StorageLocation &SrcPtrLoc = SrcLoc->getSyntheticField(PtrField);
  auto *SrcPtrVal = Env.get<PointerValue>(SrcPtrLoc);
  if (SrcPtrVal == nullptr) return;

  RecordStorageLocation &DestLoc = Env.getResultObjectLocation(*CE);
  // Create a `RecordValue`, associate it with the `DestLoc` and the expression.
  Env.setValue(*CE, refreshRecordValue(DestLoc, Env));
  StorageLocation &DestPtrLoc = DestLoc.getSyntheticField(PtrField);

  if (Callee->getName() == "const_pointer_cast") {
    // A `const_pointer_cast` will definitely produce a pointer with the same
    // storage location as the source, so we can simply copy the underlying
    // pointer value.
    Env.setValue(DestPtrLoc, *SrcPtrVal);
  } else {
    auto &DestPtrVal =
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
      // succeeded. So if the argument wasn't yet nullable, replace it with a
      // new nullable pointer.
      PointerNullState SrcNullability = getPointerNullState(*SrcPtrVal);
      if (SrcNullability.FromNullable == nullptr ||
          !Env.proves(*SrcNullability.FromNullable))
        setToPointerWithNullability(SrcPtrLoc, NullabilityKind::Nullable,
                                    State.Env);
    } else {
      setSmartPointerToNull(*SrcLoc, State.Env);
    }
  }
}

void transferValue_WeakPtrLockCall(
    const CXXMemberCallExpr *MCE, const MatchFinder::MatchResult &Result,
    TransferState<PointerNullabilityLattice> &State) {
  if (!smartPointersEnabled()) return;

  RecordStorageLocation &Loc = State.Env.getResultObjectLocation(*MCE);
  // Create a `RecordValue`, associate it with the `Loc` and the expression.
  State.Env.setValue(*MCE, refreshRecordValue(Loc, State.Env));
  StorageLocation &PtrLoc = Loc.getSyntheticField(PtrField);

  setToPointerWithNullability(PtrLoc, NullabilityKind::Nullable, State.Env);
}

void transferValue_SmartPointer(
    const Expr *PointerExpr, const MatchFinder::MatchResult &Result,
    TransferState<PointerNullabilityLattice> &State) {
  initSmartPointerForExpr(PointerExpr, State);

  auto *SmartPtrLoc = State.Env.get<RecordStorageLocation>(*PointerExpr);
  if (SmartPtrLoc == nullptr) return;
  StorageLocation &PtrLoc = SmartPtrLoc->getSyntheticField(PtrField);
  unpackPointerValue(PtrLoc, State.Env);
}

void transferValue_SmartPointerArrowMemberExpr(
    const MemberExpr *ME, const MatchFinder::MatchResult &Result,
    TransferState<PointerNullabilityLattice> &State) {
  // Most accesses of a smart pointer involve a glvalue of smart pointer type,
  // and `transferValue_SmartPointer` will ensure in this case that the
  // nullability properties of the underlying raw pointer are initialized.
  // An exception to this is if we access members of a smart pointer using
  // arrow syntax; in this case, there is no glvalue of smart pointer type,
  // and this function handles initialization of the underlying raw pointer
  // in this case.

  const Expr &Base = *ME->getBase();
  auto *BasePtrVal = State.Env.get<PointerValue>(Base);
  if (BasePtrVal == nullptr) {
    BasePtrVal = cast<PointerValue>(State.Env.createValue(Base.getType()));
    State.Env.setValue(Base, *BasePtrVal);
  }

  auto &SmartPtrLoc = cast<RecordStorageLocation>(BasePtrVal->getPointeeLoc());
  StorageLocation &PtrLoc = SmartPtrLoc.getSyntheticField(PtrField);
  auto *PtrVal = State.Env.get<PointerValue>(PtrLoc);
  if (PtrVal == nullptr) {
    PtrVal = cast<PointerValue>(State.Env.createValue(PtrLoc.getType()));
    State.Env.setValue(PtrLoc, *PtrVal);
  }

  PointerTypeNullability Nullability = NullabilityKind::Unspecified;
  if (const auto *ExprNullability =
          State.Lattice.getExprNullability(ME->getBase())) {
    if (ExprNullability->size() >= 2) Nullability = (*ExprNullability)[1];
  }

  initPointerNullState(*PtrVal, State.Env.getDataflowAnalysisContext(),
                       Nullability);
}

void transferValue_Pointer(absl::Nonnull<const Expr *> PointerExpr,
                           const MatchFinder::MatchResult &Result,
                           TransferState<PointerNullabilityLattice> &State) {
  auto *PointerVal = getRawPointerValue(PointerExpr, State.Env);
  if (!PointerVal) return;

  initPointerFromTypeNullability(*PointerVal, PointerExpr, State);

  if (const auto *Cast = dyn_cast<CastExpr>(PointerExpr);
      Cast && Cast->getCastKind() == CK_LValueToRValue) {
    if (StorageLocation *Loc =
            State.Env.getStorageLocation(*Cast->getSubExpr())) {
      if (PointerValue *Val = unpackPointerValue(*Loc, State.Env)) {
        State.Env.setValue(*PointerExpr, *Val);
      }
    }
  }
}

// `ComparisonFormula` represents the comparison between the two pointer values.
//
// `LHSNull` and `RHSNull` represent the nullability of the left- and right-hand
// expresssions, respectively. A nullptr value is interpreted as Top.
absl::Nullable<BoolValue *> processPointerComparison(
    const Formula &ComparisonFormula, absl::Nullable<const Formula *> LHSNull,
    absl::Nullable<const Formula *> RHSNull, BinaryOperatorKind Opcode,
    Environment &Env) {
  auto &A = Env.arena();

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
  auto &PointerEQ =
      Opcode == BO_EQ ? ComparisonFormula : A.makeNot(ComparisonFormula);
  auto &PointerNE =
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
void transferValue_NullCheckComparison(
    absl::Nonnull<const BinaryOperator *> BinaryOp,
    const MatchFinder::MatchResult &Result,
    TransferState<PointerNullabilityLattice> &State) {
  auto *LHS = BinaryOp->getLHS();
  auto *RHS = BinaryOp->getRHS();
  assert(LHS != nullptr && RHS != nullptr);

  // Boolean representing the comparison between the two pointer values.
  // We can rely on the dataflow framework to have produced a value for this.
  auto *ComparisonVal = State.Env.get<BoolValue>(*BinaryOp);
  assert(ComparisonVal != nullptr);
  auto &ComparisonFormula = ComparisonVal->formula();

  auto *LHSVal = getRawPointerValue(LHS, State.Env);
  if (!LHSVal || !hasPointerNullState(*LHSVal)) return;
  auto *RHSVal = getRawPointerValue(RHS, State.Env);
  if (!RHSVal || !hasPointerNullState(*RHSVal)) return;

  if (auto *Val = processPointerComparison(ComparisonFormula,
                                           getPointerNullState(*LHSVal).IsNull,
                                           getPointerNullState(*RHSVal).IsNull,
                                           BinaryOp->getOpcode(), State.Env))
    State.Env.setValue(*BinaryOp, *Val);
}

void transferValue_NullCheckImplicitCastPtrToBool(
    absl::Nonnull<const Expr *> CastExpr, const MatchFinder::MatchResult &,
    TransferState<PointerNullabilityLattice> &State) {
  auto &A = State.Env.arena();
  auto *PointerVal = getRawPointerValue(CastExpr->IgnoreImplicit(), State.Env);
  if (!PointerVal) return;

  auto Nullability = getPointerNullState(*PointerVal);
  if (Nullability.IsNull != nullptr)
    State.Env.setValue(*CastExpr,
                       A.makeBoolValue(A.makeNot(*Nullability.IsNull)));
  else
    State.Env.setValue(*CastExpr, A.makeTopValue());
}

void initializeOutputParameter(absl::Nonnull<const Expr *> Arg,
                               TransferState<PointerNullabilityLattice> &State,
                               const VarDecl &Param) {
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
  if (ParamTy->getPointeeType().isNull()) return;
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

  StorageLocation *Loc = nullptr;
  if (ParamTy->isPointerType()) {
    if (PointerValue *OuterPointer = getRawPointerValue(Arg, State.Env))
      Loc = &OuterPointer->getPointeeLoc();
  } else if (ParamTy->isReferenceType()) {
    Loc = State.Env.getStorageLocation(*Arg);
  }
  if (Loc == nullptr) return;

  if (isSupportedRawPointerType(ParamTy->getPointeeType())) {
    auto *InnerPointer =
        cast<PointerValue>(State.Env.createValue(ParamTy->getPointeeType()));
    initPointerNullState(*InnerPointer, State.Env.getDataflowAnalysisContext(),
                         NullabilityKind::Unspecified);

    State.Env.setValue(*Loc, *InnerPointer);
  } else {
    auto &SmartPointerLoc = *cast<RecordStorageLocation>(Loc);
    setToPointerWithNullability(SmartPointerLoc.getSyntheticField(PtrField),
                                NullabilityKind::Unspecified, State.Env);
  }
}

// `D` is declared somewhere in `absl`, either directly or nested.
bool isDeclaredInAbseil(const Decl &D) {
  const auto *DC = D.getDeclContext();
  if (DC == nullptr || DC->isTranslationUnit()) return false;

  // Find the topmost, non-TU DeclContext.
  const DeclContext *Parent = DC->getParent();
  while (Parent != nullptr && !Parent->isTranslationUnit()) {
    DC = Parent;
    Parent = DC->getParent();
  }

  // Check if it is the `absl` namespace.
  const auto *NS = dyn_cast_or_null<NamespaceDecl>(DC);
  return NS != nullptr && NS->getDeclName().isIdentifier() &&
         NS->getName() == "absl";
}

// Models the Abseil logging `GetReferenceableValue` function.
void modelAbseilGetReferenceableValue(const CallExpr &CE, Environment &Env) {
  // We only model the `GetReferenceableValue` overload that takes and returns a
  // reference.
  if (!CE.isGLValue()) return;
  assert(CE.getNumArgs() == 1);
  assert(CE.getArg(0) != nullptr);
  if (StorageLocation *Loc = Env.getStorageLocation(*CE.getArg(0)))
    Env.setStorageLocation(CE, *Loc);
}

// Models the Abseil logging `CheckNE_Impl` function. Essentially, associates
// the `IsNull` of the call result with the comparison `arg0 != arg1`.
void modelAbseilCheckNE(const CallExpr &CE, Environment &Env) {
  assert(isSupportedRawPointerType(CE.getType()));
  auto *PointerVal = getRawPointerValue(&CE, Env);
  if (!PointerVal)
    PointerVal = cast<PointerValue>(Env.createValue(CE.getType()));
  // Force the pointer state to `Nullable`, which we will then potentially
  // refine below.
  // TODO Add the annotation in the logging library so that we don't have
  // to hard-code this here.
  initPointerNullState(*PointerVal, Env.getDataflowAnalysisContext(),
                       NullabilityKind::Nullable);
  Env.setValue(CE, *PointerVal);
  const Formula *IsNull = getPointerNullState(*PointerVal).IsNull;
  assert(IsNull != nullptr && "`IsNull` can never be 'Top' here");

  auto *LHS = CE.getArg(0);
  auto *RHS = CE.getArg(1);
  assert(LHS != nullptr && RHS != nullptr);
  auto LTy = LHS->getType();
  auto RTy = RHS->getType();

  if (!isSupportedPointerType(LTy) && !LTy->isNullPtrType()) return;
  if (!isSupportedPointerType(RTy) && !RTy->isNullPtrType()) return;

  const Formula *LHSNull = nullptr;
  if (LTy->isNullPtrType()) {
    // Values of nullptr type are not themselves pointers and so not
    // modeled directly. They are only modeled if and when they are cast
    // to pointers. So, we need to supply a formula directly.
    LHSNull = &Env.arena().makeLiteral(true);
  } else {
    auto *V = getPointerValue(LHS, Env);
    if (!V) return;
    assert(hasPointerNullState(*V));
    LHSNull = getPointerNullState(*V).IsNull;
  }

  const Formula *RHSNull = nullptr;
  if (RTy->isNullPtrType()) {
    RHSNull = &Env.arena().makeLiteral(true);
  } else {
    auto *V = getPointerValue(RHS, Env);
    if (!V) return;
    assert(hasPointerNullState(*V));
    RHSNull = getPointerNullState(*V).IsNull;
  }

  if (auto *Val =
          processPointerComparison(*IsNull, LHSNull, RHSNull, BO_NE, Env))
    Env.assume(Env.arena().makeEquals(Val->formula(), *IsNull));
}

void transferValue_CallExpr(absl::Nonnull<const CallExpr *> CE,
                            const MatchFinder::MatchResult &Result,
                            TransferState<PointerNullabilityLattice> &State) {
  // The dataflow framework itself generally does not model `CallExpr`s
  // (including creating values for the results). We model some specific
  // function calls and handle value creation for certain types.

  const auto *FuncDecl = CE->getDirectCallee();
  if (FuncDecl != nullptr) {
    if (const IdentifierInfo *FunII =
            FuncDecl->getDeclName().getAsIdentifierInfo()) {
      if (FunII->isStr("__assert_nullability")) return;

      // This is part of the implementation of `CHECK_NE`.
      if (FunII->isStr("GetReferenceableValue") &&
          isDeclaredInAbseil(*FuncDecl)) {
        modelAbseilGetReferenceableValue(*CE, State.Env);
        return;
      }
      if (FunII->isStr("Check_NEImpl") && isDeclaredInAbseil(*FuncDecl)) {
        modelAbseilCheckNE(*CE, State.Env);
        return;
      }
    }
  }

  StorageLocation *Loc = nullptr;
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

  if (isSupportedRawPointerType(CE->getType())) {
    // Create a pointer so that we can attach nullability to it and have the
    // nullability propagate with the pointer.
    auto *PointerVal = getRawPointerValue(CE, State.Env);
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
    initSmartPointerForExpr(CE, State);
  }

  if (CE->isCallToStdMove() || FuncDecl == nullptr) return;

  // Don't treat parameters of our macro replacement argument-capture functions
  // as output parameters.
  if (const IdentifierInfo *FunII =
          FuncDecl->getDeclName().getAsIdentifierInfo();
      FunII && (FunII->isStr("clang_tidy_nullability_internal_abortIfFalse") ||
                FunII->isStr("clang_tidy_nullability_internal_abortIfEqual")))
    return;
  // Make output parameters (with unknown nullability) initialized to unknown.
  for (ParamAndArgIterator<CallExpr> Iter(*FuncDecl, *CE); Iter; ++Iter)
    initializeOutputParameter(&Iter.arg(), State, Iter.param());
}

void transferValue_AccessorCall(
    absl::Nonnull<const CXXMemberCallExpr *> MCE,
    const MatchFinder::MatchResult &Result,
    TransferState<PointerNullabilityLattice> &State) {
  auto *member = Result.Nodes.getNodeAs<clang::ValueDecl>("member-decl");
  PointerValue *PointerVal = nullptr;
  if (dataflow::RecordStorageLocation *RecordLoc =
          dataflow::getImplicitObjectLocation(*MCE, State.Env)) {
    StorageLocation *Loc = RecordLoc->getChild(*member);
    PointerVal = dyn_cast_or_null<PointerValue>(State.Env.getValue(*Loc));
  }
  if (!PointerVal) {
    // Use value that may have been set by the builtin transfer function or by
    // `ensurePointerHasValue()`.
    PointerVal = getRawPointerValue(MCE, State.Env);
  }
  if (PointerVal) {
    State.Env.setValue(*MCE, *PointerVal);
    initPointerFromTypeNullability(*PointerVal, MCE, State);
  }
}

void handleConstMemberCall(absl::Nonnull<const CallExpr *> CE,
                           dataflow::RecordStorageLocation *RecordLoc,
                           const MatchFinder::MatchResult &Result,
                           TransferState<PointerNullabilityLattice> &State) {
  if ((!isSupportedRawPointerType(CE->getType()) &&
       !CE->getType()->isBooleanType()) ||
      !CE->isPRValue() || RecordLoc == nullptr) {
    // Perform default handling.
    transferValue_CallExpr(CE, Result, State);
    return;
  }
  Value *Val =
      State.Lattice.getConstMethodReturnValue(*RecordLoc, CE, State.Env);
  if (Val == nullptr) return;

  State.Env.setValue(*CE, *Val);
  if (auto *PointerVal = dyn_cast<PointerValue>(Val))
    initPointerFromTypeNullability(*PointerVal, CE, State);
}

void transferValue_ConstMemberCall(
    absl::Nonnull<const CXXMemberCallExpr *> MCE,
    const MatchFinder::MatchResult &Result,
    TransferState<PointerNullabilityLattice> &State) {
  handleConstMemberCall(
      MCE, dataflow::getImplicitObjectLocation(*MCE, State.Env), Result, State);
}

void transferValue_OptionalOperatorArrowCall(
    absl::Nonnull<const CXXOperatorCallExpr *> OCE,
    const MatchFinder::MatchResult &Result,
    TransferState<PointerNullabilityLattice> &State) {
  auto *RecordLoc = cast_or_null<dataflow::RecordStorageLocation>(
      State.Env.getStorageLocation(*OCE->getArg(0)));
  // `optional::operator->` isn't necessarily const, but it behaves the way we
  // model "const member calls": It always returns the same pointer if the
  // optional wasn't mutated in the meantime.
  handleConstMemberCall(OCE, RecordLoc, Result, State);
}

void handleNonConstMemberCall(absl::Nonnull<const CallExpr *> CE,
                              dataflow::RecordStorageLocation *RecordLoc,
                              const MatchFinder::MatchResult &Result,
                              TransferState<PointerNullabilityLattice> &State) {
  // When a non-const member function is called, clear all pointer-type fields
  // of the receiver.
  if (RecordLoc != nullptr) {
    for (const auto [Field, FieldLoc] : RecordLoc->children()) {
      // We can't produce a new `PointerValue` here because we don't necessarily
      // know what to initialize its nullability properties with. The record may
      // be a `ClassTemplateSpecializationDecl`, which uses canonical types for
      // its type arguments (there only be one specialization for the same
      // canonical type arguments), so the `FieldDecl` doesn't contain
      // nullability annotations. The best thing we can do, therefore, is to
      // clear the value.
      if (isSupportedRawPointerType(Field->getType()))
        State.Env.clearValue(*FieldLoc);
    }
    State.Lattice.clearConstMethodReturnValues(*RecordLoc);
  }

  // Perform default handling.
  transferValue_CallExpr(CE, Result, State);
}

void transferValue_NonConstMemberCall(
    absl::Nonnull<const CXXMemberCallExpr *> MCE,
    const MatchFinder::MatchResult &Result,
    TransferState<PointerNullabilityLattice> &State) {
  handleNonConstMemberCall(
      MCE, dataflow::getImplicitObjectLocation(*MCE, State.Env), Result, State);
}

void transferValue_NonConstMemberOperatorCall(
    absl::Nonnull<const CXXOperatorCallExpr *> OCE,
    const MatchFinder::MatchResult &Result,
    TransferState<PointerNullabilityLattice> &State) {
  auto *RecordLoc = cast_or_null<dataflow::RecordStorageLocation>(
      State.Env.getStorageLocation(*OCE->getArg(0)));
  handleNonConstMemberCall(OCE, RecordLoc, Result, State);
}

void transferType_DeclRefExpr(absl::Nonnull<const DeclRefExpr *> DRE,
                              const MatchFinder::MatchResult &MR,
                              TransferState<PointerNullabilityLattice> &State) {
  computeNullability(DRE, State, [&] {
    Resugarer Resugar(State.Lattice.defaults());

    if (DRE->hasExplicitTemplateArgs())
      Resugar.addTemplateArgs(DRE->getDecl(), DRE->template_arguments());
    std::deque<TypeNullability> ScopeNullabilityStorage;
    for (auto NNS = DRE->getQualifierLoc(); NNS; NNS = NNS.getPrefix()) {
      if (auto *CTSD = llvm::dyn_cast_or_null<ClassTemplateSpecializationDecl>(
              NNS.getNestedNameSpecifier()->getAsRecordDecl())) {
        ScopeNullabilityStorage.push_back(
            getTypeNullability(NNS.getTypeLoc(), State.Lattice.defaults()));
        Resugar.Enclosing.push_back({CTSD, ScopeNullabilityStorage.back()});
      }
    }

    auto Nullability =
        getTypeNullability(*DRE->getDecl(), State.Lattice.defaults(), Resugar);
    State.Lattice.overrideNullabilityFromDecl(DRE->getDecl(), Nullability);
    return Nullability;
  });
}

void transferType_MemberExpr(absl::Nonnull<const MemberExpr *> ME,
                             const MatchFinder::MatchResult &MR,
                             TransferState<PointerNullabilityLattice> &State) {
  computeNullability(ME, State, [&]() {
    auto *Member = ME->getMemberDecl();
    auto BaseType = ME->getBase()->getType();
    auto BaseNullability =
        ArrayRef(getNullabilityForChild(ME->getBase(), State));
    if (ME->isArrow() && BaseType->isPointerType()) {
      BaseType = BaseType->getPointeeType();
      BaseNullability = BaseNullability.drop_front();
    }

    Resugarer Resugar(State.Lattice.defaults());
    if (const auto *RT = BaseType->getAs<RecordType>()) {
      if (auto *CTSpec =
              dyn_cast<ClassTemplateSpecializationDecl>(RT->getDecl())) {
        Resugar.Enclosing.push_back({CTSpec, BaseNullability});
      }
    }
    if (ME->hasExplicitTemplateArgs())
      Resugar.addTemplateArgs(ME->getMemberDecl(), ME->template_arguments());

    auto Nullability =
        getTypeNullability(*Member, State.Lattice.defaults(), Resugar);
    State.Lattice.overrideNullabilityFromDecl(ME->getMemberDecl(), Nullability);
    return Nullability;
  });
}

void transferType_CastExpr(absl::Nonnull<const CastExpr *> CE,
                           const MatchFinder::MatchResult &MR,
                           TransferState<PointerNullabilityLattice> &State) {
  computeNullability(CE, State, [&]() -> TypeNullability {
    // Most casts that can convert ~unrelated types drop nullability in general.
    // As a special case, preserve nullability of outer pointer types.
    // For example, int* p; (void*)p; is a BitCast, but preserves nullability.
    auto PreserveTopLevelPointers = [&](TypeNullability V) {
      auto ArgNullability = getNullabilityForChild(CE->getSubExpr(), State);
      const PointerType *ArgType = dyn_cast<PointerType>(
          CE->getSubExpr()->getType().getCanonicalType().getTypePtr());
      const PointerType *CastType =
          dyn_cast<PointerType>(CE->getType().getCanonicalType().getTypePtr());
      for (int I = 0; ArgType && CastType; ++I) {
        V[I] = ArgNullability[I];
        ArgType = dyn_cast<PointerType>(ArgType->getPointeeType().getTypePtr());
        CastType =
            dyn_cast<PointerType>(CastType->getPointeeType().getTypePtr());
      }
      return V;
    };

    switch (CE->getCastKind()) {
      // Casts between unrelated types: we can't say anything about nullability.
      case CK_LValueBitCast:
      case CK_BitCast:
      case CK_LValueToRValueBitCast:
        return PreserveTopLevelPointers(unspecifiedNullability(CE));

      // Casts between equivalent types.
      case CK_LValueToRValue:
      case CK_NoOp:
      case CK_AtomicToNonAtomic:
      case CK_NonAtomicToAtomic:
      case CK_AddressSpaceConversion:
        return getNullabilityForChild(CE->getSubExpr(), State);

      // Controlled conversions between types
      // TODO: these should be doable somehow
      case CK_BaseToDerived:
      case CK_DerivedToBase:
      case CK_UncheckedDerivedToBase:
        return PreserveTopLevelPointers(unspecifiedNullability(CE));
      case CK_UserDefinedConversion:
        return unspecifiedNullability(CE);
      case CK_ConstructorConversion:
        if (auto *CCE = llvm::dyn_cast<CXXConstructExpr>(CE->getSubExpr())) {
          // This node is syntactic only.
          return getNullabilityForChild(CE->getSubExpr(), State);
        }
        return unspecifiedNullability(CE);

      case CK_Dynamic: {
        auto Result = unspecifiedNullability(CE);
        // A dynamic_cast to pointer is null if the runtime check fails.
        if (isa<PointerType>(CE->getType().getCanonicalType()))
          Result.front() = NullabilityKind::Nullable;
        return Result;
      }

      // Primitive values have no nullability.
      case CK_ToVoid:
      case CK_MemberPointerToBoolean:
      case CK_PointerToBoolean:
      case CK_PointerToIntegral:
      case CK_IntegralCast:
      case CK_IntegralToBoolean:
      case CK_IntegralToFloating:
      case CK_FloatingToFixedPoint:
      case CK_FixedPointToFloating:
      case CK_FixedPointCast:
      case CK_FixedPointToIntegral:
      case CK_IntegralToFixedPoint:
      case CK_FixedPointToBoolean:
      case CK_FloatingToIntegral:
      case CK_FloatingToBoolean:
      case CK_BooleanToSignedIntegral:
      case CK_FloatingCast:
      case CK_FloatingRealToComplex:
      case CK_FloatingComplexToReal:
      case CK_FloatingComplexToBoolean:
      case CK_FloatingComplexCast:
      case CK_FloatingComplexToIntegralComplex:
      case CK_IntegralRealToComplex:
      case CK_IntegralComplexToReal:
      case CK_IntegralComplexToBoolean:
      case CK_IntegralComplexCast:
      case CK_IntegralComplexToFloatingComplex:
        return {};

      // This can definitely be null!
      case CK_NullToPointer: {
        TypeNullability Nullability;
        // Explicit casts get the inner of the written type.
        if (const auto *ECE = dyn_cast<ExplicitCastExpr>(CE))
          Nullability =
              getTypeNullability(ECE->getTypeInfoAsWritten()->getTypeLoc(),
                                 State.Lattice.defaults());
        else
          Nullability = unspecifiedNullability(CE);
        // Despite the name `NullToPointer`, the destination type of the cast
        // may be `nullptr_t` (which is, itself, not a pointer type).
        if (!CE->getType()->isNullPtrType())
          Nullability.front() = NullabilityKind::Nullable;
        return Nullability;
      }

      // Pointers out of thin air, who knows?
      case CK_IntegralToPointer:
        return unspecifiedNullability(CE);

      // Decayed objects are never null.
      case CK_ArrayToPointerDecay:
      case CK_FunctionToPointerDecay:
        return prepend(NullabilityKind::NonNull,
                       getNullabilityForChild(CE->getSubExpr(), State));

      // Despite its name, the result type of `BuiltinFnToFnPtr` is a function,
      // not a function pointer, so nullability doesn't change.
      case CK_BuiltinFnToFnPtr:
        return getNullabilityForChild(CE->getSubExpr(), State);

      // TODO: what is our model of member pointers?
      case CK_BaseToDerivedMemberPointer:
      case CK_DerivedToBaseMemberPointer:
      case CK_NullToMemberPointer:
      case CK_ReinterpretMemberPointer:
      case CK_ToUnion:  // and unions?
        return unspecifiedNullability(CE);

      // TODO: Non-C/C++ constructs, do we care about these?
      case CK_CPointerToObjCPointerCast:
      case CK_ObjCObjectLValueCast:
      case CK_MatrixCast:
      case CK_VectorSplat:
      case CK_BlockPointerToObjCPointerCast:
      case CK_AnyPointerToBlockPointerCast:
      case CK_ARCProduceObject:
      case CK_ARCConsumeObject:
      case CK_ARCReclaimReturnedObject:
      case CK_ARCExtendBlockObject:
      case CK_CopyAndAutoreleaseBlockObject:
      case CK_ZeroToOCLOpaqueType:
      case CK_IntToOCLSampler:
      case CK_HLSLVectorTruncation:
        return unspecifiedNullability(CE);

      case CK_Dependent:
        CHECK(false) << "Shouldn't see dependent casts here?";
    }
  });
}

void transferType_MaterializeTemporaryExpr(
    absl::Nonnull<const MaterializeTemporaryExpr *> MTE,
    const MatchFinder::MatchResult &MR,
    TransferState<PointerNullabilityLattice> &State) {
  computeNullability(MTE, State, [&]() {
    return getNullabilityForChild(MTE->getSubExpr(), State);
  });
}

void transferType_CXXBindTemporaryExpr(
    const CXXBindTemporaryExpr *BTE, const MatchFinder::MatchResult &MR,
    TransferState<PointerNullabilityLattice> &State) {
  computeNullability(BTE, State, [&]() {
    return getNullabilityForChild(BTE->getSubExpr(), State);
  });
}

void transferType_CopyOrMoveConstruct(
    const CXXConstructExpr *CCE, const MatchFinder::MatchResult &MR,
    TransferState<PointerNullabilityLattice> &State) {
  computeNullability(CCE, State, [&]() {
    return getNullabilityForChild(CCE->getArg(0), State);
  });
}

void transferType_CallExpr(absl::Nonnull<const CallExpr *> CE,
                           const MatchFinder::MatchResult &MR,
                           TransferState<PointerNullabilityLattice> &State) {
  computeNullability(CE, State, [&]() {
    TypeNullability CalleeNullability =
        getNullabilityForChild(CE->getCallee(), State);
    ArrayRef ResultNullability = CalleeNullability;
    if (CE->getCallee()->getType()->isPointerType())  // Callee is usually fptr.
      ResultNullability = ResultNullability.drop_front();
    // Return value nullability is at the front of the function type.
    ResultNullability =
        ResultNullability.take_front(countPointersInType(CE->getType()));
    return ResultNullability.vec();
  });
}

void transferType_UnaryOperator(
    absl::Nonnull<const UnaryOperator *> UO, const MatchFinder::MatchResult &MR,
    TransferState<PointerNullabilityLattice> &State) {
  computeNullability(UO, State, [&]() -> TypeNullability {
    switch (UO->getOpcode()) {
      case UO_AddrOf:
        return prepend(NullabilityKind::NonNull,
                       getNullabilityForChild(UO->getSubExpr(), State));
      case UO_Deref:
        return ArrayRef(getNullabilityForChild(UO->getSubExpr(), State))
            .drop_front()
            .vec();

      case UO_PostInc:
      case UO_PostDec:
      case UO_PreInc:
      case UO_PreDec:
      case UO_Plus:
      case UO_Minus:
      case UO_Not:
      case UO_LNot:
      case UO_Real:
      case UO_Imag:
      case UO_Extension:
        return getNullabilityForChild(UO->getSubExpr(), State);

      case UO_Coawait:
        // TODO: work out what to do here!
        return unspecifiedNullability(UO);
    }
  });
}

void transferType_BinaryOperator(
    absl::Nonnull<const BinaryOperator *> BO,
    const MatchFinder::MatchResult &MR,
    TransferState<PointerNullabilityLattice> &State) {
  computeNullability(BO, State, [&]() -> TypeNullability {
    switch (BO->getOpcode()) {
      case BO_PtrMemD:
      case BO_PtrMemI:
        // TODO: pointers-to-member should really have nullability vectors
        return unspecifiedNullability(BO);
      case BO_Assign:
      case BO_Comma:
        return getNullabilityForChild(BO->getRHS(), State);
      default:
        // No other built-in binary operators can be pointer-valued
        return unspecifiedNullability(BO);
    }
  });
}

void transferType_NewExpr(absl::Nonnull<const CXXNewExpr *> NE,
                          const MatchFinder::MatchResult &MR,
                          TransferState<PointerNullabilityLattice> &State) {
  computeNullability(NE, State, [&]() {
    TypeNullability ObjectNullability =
        getTypeNullability(NE->getAllocatedTypeSourceInfo()->getTypeLoc(),
                           State.Lattice.defaults());
    return prepend(NE->shouldNullCheckAllocation() ? NullabilityKind::Nullable
                                                   : NullabilityKind::NonNull,
                   ObjectNullability);
  });
}

void transferType_ArraySubscriptExpr(
    absl::Nonnull<const ArraySubscriptExpr *> ASE,
    const MatchFinder::MatchResult &MR,
    TransferState<PointerNullabilityLattice> &State) {
  computeNullability(ASE, State, [&]() {
    auto &BaseNullability = getNullabilityForChild(ASE->getBase(), State);
    QualType BaseType = ASE->getBase()->getType();
    CHECK(isSupportedRawPointerType(BaseType) || BaseType->isVectorType());
    return isSupportedRawPointerType(BaseType)
               ? ArrayRef(BaseNullability).slice(1).vec()
               : BaseNullability;
  });
}

void transferType_ThisExpr(absl::Nonnull<const CXXThisExpr *> TE,
                           const MatchFinder::MatchResult &MR,
                           TransferState<PointerNullabilityLattice> &State) {
  computeNullability(TE, State, [&]() {
    // If the current class is an instantiation, we can't assume any particular
    // nullability of its arguments.
    TypeNullability Result = unspecifiedNullability(TE);
    Result.front() = NullabilityKind::NonNull;
    return Result;
  });
}

auto buildTypeTransferer() {
  return CFGMatchSwitchBuilder<TransferState<PointerNullabilityLattice>>()
      .CaseOfCFGStmt<DeclRefExpr>(ast_matchers::declRefExpr(),
                                  transferType_DeclRefExpr)
      .CaseOfCFGStmt<MemberExpr>(ast_matchers::memberExpr(),
                                 transferType_MemberExpr)
      .CaseOfCFGStmt<CastExpr>(ast_matchers::castExpr(), transferType_CastExpr)
      .CaseOfCFGStmt<MaterializeTemporaryExpr>(
          ast_matchers::materializeTemporaryExpr(),
          transferType_MaterializeTemporaryExpr)
      .CaseOfCFGStmt<CXXBindTemporaryExpr>(ast_matchers::cxxBindTemporaryExpr(),
                                           transferType_CXXBindTemporaryExpr)
      .CaseOfCFGStmt<CallExpr>(ast_matchers::callExpr(), transferType_CallExpr)
      .CaseOfCFGStmt<UnaryOperator>(ast_matchers::unaryOperator(),
                                    transferType_UnaryOperator)
      .CaseOfCFGStmt<BinaryOperator>(ast_matchers::binaryOperator(),
                                     transferType_BinaryOperator)
      .CaseOfCFGStmt<CXXNewExpr>(ast_matchers::cxxNewExpr(),
                                 transferType_NewExpr)
      .CaseOfCFGStmt<ArraySubscriptExpr>(ast_matchers::arraySubscriptExpr(),
                                         transferType_ArraySubscriptExpr)
      .CaseOfCFGStmt<CXXThisExpr>(ast_matchers::cxxThisExpr(),
                                  transferType_ThisExpr)
      .CaseOfCFGStmt<CXXConstructExpr>(
          ast_matchers::cxxConstructExpr(
              ast_matchers::argumentCountIs(1),
              ast_matchers::hasDeclaration(ast_matchers::cxxConstructorDecl(
                  anyOf(ast_matchers::isCopyConstructor(),
                        ast_matchers::isMoveConstructor())))),
          transferType_CopyOrMoveConstruct)
      .Build();
}

auto buildValueTransferer() {
  // The value transfer functions must establish:
  // - if we're transferring over an Expr
  // - and the Expr has a supported pointer type
  // - and the Expr's value is modeled by the framework (or this analysis)
  // - then the PointerValue has nullability properties (is_null/from_nullable)
  return CFGMatchSwitchBuilder<TransferState<PointerNullabilityLattice>>()
      // Handles initialization of the null states of pointers.
      .CaseOfCFGStmt<Expr>(isAddrOf(), transferValue_NotNullPointer)
      // TODO(mboehme): I believe we should be able to move handling of null
      // pointers to the non-flow-sensitive part of the analysis.
      .CaseOfCFGStmt<Expr>(isNullPointerLiteral(), transferValue_NullPointer)
      .CaseOfCFGStmt<CXXScalarValueInitExpr>(isRawPointerValueInit(),
                                             transferValue_NullPointer)
      .CaseOfCFGStmt<CXXConstructExpr>(isSmartPointerConstructor(),
                                       transferValue_SmartPointerConstructor)
      .CaseOfCFGStmt<CXXOperatorCallExpr>(isSmartPointerOperatorCall("="),
                                          transferValue_SmartPointerAssignment)
      .CaseOfCFGStmt<CXXMemberCallExpr>(isSmartPointerMethodCall("release"),
                                        transferValue_SmartPointerReleaseCall)
      .CaseOfCFGStmt<CXXMemberCallExpr>(isSmartPointerMethodCall("reset"),
                                        transferValue_SmartPointerResetCall)
      .CaseOfCFGStmt<CXXMemberCallExpr>(
          isSmartPointerMethodCall("swap"),
          transferValue_SmartPointerMemberSwapCall)
      .CaseOfCFGStmt<CallExpr>(isSmartPointerFreeSwapCall(),
                               transferValue_SmartPointerFreeSwapCall)
      .CaseOfCFGStmt<CXXMemberCallExpr>(isSmartPointerMethodCall("get"),
                                        transferValue_SmartPointerGetCall)
      .CaseOfCFGStmt<CXXMemberCallExpr>(
          isSmartPointerBoolConversionCall(),
          transferValue_SmartPointerBoolConversionCall)
      .CaseOfCFGStmt<CXXOperatorCallExpr>(
          isSmartPointerOperatorCall("*"),
          transferValue_SmartPointerOperatorStar)
      .CaseOfCFGStmt<CXXOperatorCallExpr>(
          isSmartPointerOperatorCall("->"),
          transferValue_SmartPointerOperatorArrow)
      .CaseOfCFGStmt<CallExpr>(isSmartPointerFactoryCall(),
                               transferValue_SmartPointerFactoryCall)
      .CaseOfCFGStmt<CXXOperatorCallExpr>(
          isSmartPointerComparisonOpCall(),
          transferValue_SmartPointerComparisonOpCall)
      .CaseOfCFGStmt<CallExpr>(isSharedPtrCastCall(),
                               transferValue_SharedPtrCastCall)
      .CaseOfCFGStmt<CXXMemberCallExpr>(isWeakPtrLockCall(),
                                        transferValue_WeakPtrLockCall)
      .CaseOfCFGStmt<CXXMemberCallExpr>(isSupportedPointerAccessorCall(),
                                        transferValue_AccessorCall)
      .CaseOfCFGStmt<CXXMemberCallExpr>(isZeroParamConstMemberCall(),
                                        transferValue_ConstMemberCall)
      .CaseOfCFGStmt<CXXOperatorCallExpr>(
          isOptionalOperatorArrowCall(),
          transferValue_OptionalOperatorArrowCall)
      .CaseOfCFGStmt<CXXMemberCallExpr>(isNonConstMemberCall(),
                                        transferValue_NonConstMemberCall)
      .CaseOfCFGStmt<CXXOperatorCallExpr>(
          isNonConstMemberOperatorCall(),
          transferValue_NonConstMemberOperatorCall)
      .CaseOfCFGStmt<CallExpr>(ast_matchers::callExpr(), transferValue_CallExpr)
      .CaseOfCFGStmt<Expr>(isSmartPointerGlValue(), transferValue_SmartPointer)
      .CaseOfCFGStmt<MemberExpr>(isSmartPointerArrowMemberExpr(),
                                 transferValue_SmartPointerArrowMemberExpr)
      .CaseOfCFGStmt<Expr>(isPointerExpr(), transferValue_Pointer)
      // Handles comparison between 2 pointers.
      .CaseOfCFGStmt<BinaryOperator>(isPointerCheckBinOp(),
                                     transferValue_NullCheckComparison)
      // Handles checking of pointer as boolean.
      .CaseOfCFGStmt<Expr>(isImplicitCastPointerToBool(),
                           transferValue_NullCheckImplicitCastPtrToBool)
      .Build();
}

// Ensure all prvalue expressions of pointer type have a `PointerValue`
// associated with them so we can track nullability through them.
void ensurePointerHasValue(const CFGElement &Elt, Environment &Env) {
  auto S = Elt.getAs<CFGStmt>();
  if (!S) return;

  auto *E = dyn_cast<Expr>(S->getStmt());
  if (E == nullptr || !E->isPRValue() ||
      !isSupportedRawPointerType(E->getType()))
    return;

  if (Env.getValue(*E) == nullptr)
    // `createValue()` always produces a value for pointer types.
    Env.setValue(*E, *Env.createValue(E->getType()));
}

}  // namespace

PointerNullabilityAnalysis::PointerNullabilityAnalysis(ASTContext &Context,
                                                       Environment &Env)
    : DataflowAnalysis<PointerNullabilityAnalysis, PointerNullabilityLattice>(
          Context),
      TypeTransferer(buildTypeTransferer()),
      ValueTransferer(buildValueTransferer()) {
  Env.getDataflowAnalysisContext().setSyntheticFieldCallback(
      [](QualType Ty) -> llvm::StringMap<QualType> {
        QualType RawPointerTy = underlyingRawPointerType(Ty);
        if (RawPointerTy.isNull()) return {};
        return {{PtrField, RawPointerTy}};
      });
}

PointerNullabilityAnalysis::PointerNullabilityAnalysis(
    ASTContext &Context, Environment &Env, const NullabilityPragmas &Pragmas)
    : PointerNullabilityAnalysis(Context, Env) {
  NFS.Defaults = TypeNullabilityDefaults(Context, Pragmas);
}

PointerTypeNullability PointerNullabilityAnalysis::assignNullabilityVariable(
    absl::Nonnull<const ValueDecl *> D, dataflow::Arena &A) {
  auto [It, Inserted] = NFS.DeclTopLevelNullability.try_emplace(D);
  if (Inserted) It->second = PointerTypeNullability::createSymbolic(A);
  return It->second;
}

void PointerNullabilityAnalysis::transfer(const CFGElement &Elt,
                                          PointerNullabilityLattice &Lattice,
                                          Environment &Env) {
  TransferState<PointerNullabilityLattice> State(Lattice, Env);

  ensurePointerHasValue(Elt, Env);
  TypeTransferer(Elt, getASTContext(), State);
  ValueTransferer(Elt, getASTContext(), State);
}

static absl::Nullable<const Formula *> mergeFormulas(
    absl::Nullable<const Formula *> Bool1, const Environment &Env1,
    absl::Nullable<const Formula *> Bool2, const Environment &Env2,
    Environment &MergedEnv) {
  if (Bool1 == Bool2) {
    return Bool1;
  }

  if (Bool1 == nullptr || Bool2 == nullptr) return nullptr;

  auto &A = MergedEnv.arena();

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

  auto &MergedBool = A.makeAtomRef(A.makeAtom());
  // TODO(b/233582219): Flow conditions are not necessarily mutually
  // exclusive, a fix is in order: https://reviews.llvm.org/D130270. Update
  // this section when the patch is commited.
  auto FC1 = Env1.getFlowConditionToken();
  auto FC2 = Env2.getFlowConditionToken();
  MergedEnv.assume(A.makeOr(
      A.makeAnd(A.makeAtomRef(FC1), A.makeEquals(MergedBool, *Bool1)),
      A.makeAnd(A.makeAtomRef(FC2), A.makeEquals(MergedBool, *Bool2))));
  return &MergedBool;
}

void PointerNullabilityAnalysis::join(QualType Type, const Value &Val1,
                                      const Environment &Env1,
                                      const Value &Val2,
                                      const Environment &Env2, Value &MergedVal,
                                      Environment &MergedEnv) {
  if (!isSupportedRawPointerType(Type)) return;

  if (!hasPointerNullState(cast<PointerValue>(Val1)) ||
      !hasPointerNullState(cast<PointerValue>(Val2))) {
    // It can happen that we merge pointers without null state, if either or
    // both of the pointers has not appeared in an expression (and has not
    // otherwise been initialized with nullability properties) before the merge.
    // We return true to keep the `MergedVal` produced by the framework. When
    // the merged value appears in an expression, `tranferValue_Pointer` will
    // take care of initializing it with nullability properties.
    return;
  }

  auto Nullability1 = getPointerNullState(cast<PointerValue>(Val1));
  auto Nullability2 = getPointerNullState(cast<PointerValue>(Val2));

  auto *FromNullable =
      mergeFormulas(Nullability1.FromNullable, Env1, Nullability2.FromNullable,
                    Env2, MergedEnv);
  auto *Null = mergeFormulas(Nullability1.IsNull, Env1, Nullability2.IsNull,
                             Env2, MergedEnv);

  initPointerNullState(cast<PointerValue>(MergedVal),
                       MergedEnv.getDataflowAnalysisContext(),
                       {FromNullable, Null});
}

ComparisonResult PointerNullabilityAnalysis::compare(QualType Type,
                                                     const Value &Val1,
                                                     const Environment &Env1,
                                                     const Value &Val2,
                                                     const Environment &Env2) {
  if (const auto *PointerVal1 = dyn_cast<PointerValue>(&Val1)) {
    const auto &PointerVal2 = cast<PointerValue>(Val2);

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

namespace {
enum class WidenedProperty {
  Identical,
  False,
  True,
  Top,
};
}  // namespace

// Returns the result of widening a nullability property.
// `Prev` is the formula in the previous iteration, `Cur` is the formula in the
// current iteration.
// Returns `Identical`, if `Prev == Cur`, Otherwise, if they are
// (only) equivalent, returns `True` or `False`, depending on the formulas'
// (common) truth value. Otherwise, returns `Top`, indicating the lack of common
// truth value.
static WidenedProperty widenNullabilityProperty(
    absl::Nullable<const Formula *> Prev, const Environment &PrevEnv,
    absl::Nullable<const Formula *> Cur, Environment &CurEnv) {
  if (Prev == Cur) return WidenedProperty::Identical;
  if (Prev == nullptr || Cur == nullptr) return WidenedProperty::Top;

  Arena &A = CurEnv.arena();

  if (PrevEnv.proves(*Prev)) {
    // Check for a dead-code environment, which would allow `Prev`, no matter
    // its value. As an optimization, we skip the check when `Prev` is the true
    // literal, because, in that case, the environment is irrelevant.
    //
    // We only need to consider `PrevEnv`, because it is queried
    // first. If `PrevEnv` is not dead and `CurEnv` is dead, we'll implicitly
    // use the state in `PrevEnv`, which is the desired outcome. Note: we do not
    // know of a scenario in which this can occur, but the logic holds
    // regardless.
    auto &True = A.makeLiteral(true);
    if (Prev != &True && !PrevEnv.allows(True)) {
      // TODO: Ideally, we'd just preserve `Cur`, rather than trying to
      // determine its truth value. There's no reason for further processing
      // except to meet the constraints of the API.
      if (CurEnv.proves(A.makeNot(*Cur))) return WidenedProperty::False;
    }
    if (CurEnv.proves(*Cur)) return WidenedProperty::True;
  } else if (PrevEnv.proves(A.makeNot(*Prev)) &&
             CurEnv.proves(A.makeNot(*Cur))) {
    return WidenedProperty::False;
  }

  return WidenedProperty::Top;
}

// Assumes `Prev` or its negation in `Env`, based on `W`. `W` may not be `Top`.
static void maybeAssumeNullabilityProperty(WidenedProperty W,
                                           const Formula &Prev,
                                           Environment &Env) {
  switch (W) {
    case WidenedProperty::Identical:
      // No action needs to be taken because `Prev` is identical to the current
      // property (and therefore sufficiently valid in `Env` already).
      break;
    case WidenedProperty::False:
      Env.assume(Env.arena().makeNot(Prev));
      break;
    case WidenedProperty::True:
      Env.assume(Prev);
      break;
    case WidenedProperty::Top:
      assert(false);
      break;
  }
}

absl::Nullable<Value *> PointerNullabilityAnalysis::widen(
    QualType Type, Value &Prev, const Environment &PrevEnv, Value &Current,
    Environment &CurrentEnv) {
  // Widen pointers to a pointer with a "top" storage location.
  if (auto *PrevPtr = dyn_cast<PointerValue>(&Prev)) {
    auto &CurPtr = cast<PointerValue>(Current);

    DataflowAnalysisContext &DACtx = CurrentEnv.getDataflowAnalysisContext();
    assert(&PrevEnv.getDataflowAnalysisContext() == &DACtx);

    if (!hasPointerNullState(*PrevPtr) || !hasPointerNullState(CurPtr))
      return nullptr;

    auto [FromNullablePrev, NullPrev] = getPointerNullState(*PrevPtr);
    auto [FromNullableCur, NullCur] = getPointerNullState(CurPtr);

    WidenedProperty FromNullableWidened = widenNullabilityProperty(
        FromNullablePrev, PrevEnv, FromNullableCur, CurrentEnv);
    WidenedProperty NullWidened =
        widenNullabilityProperty(NullPrev, PrevEnv, NullCur, CurrentEnv);

    // Is `PrevPtr` already equivalent to either of the current pointer or the
    // widened pointer we are about to produce? If so, return `PrevPtr` to
    // signal this.
    if ((&PrevPtr->getPointeeLoc() == &CurPtr.getPointeeLoc() ||
         &PrevPtr->getPointeeLoc() ==
             &getTopStorageLocation(DACtx,
                                    PrevPtr->getPointeeLoc().getType())) &&
        // Check whether
        // - the previous nullability property is equivalent to the current
        //   property (in which case the widened property is non-Top), or
        // - the previous nullability property is already "top" (i.e. null)
        (FromNullableWidened != WidenedProperty::Top ||
         FromNullablePrev == nullptr) &&
        (NullWidened != WidenedProperty::Top || NullPrev == nullptr)) {
      // The formulas of the nullability properties in `Prev` may only be valid
      // in `PrevEnv`. So, we need to re-assert them in the current environment
      // to keep `PrevPtr` valid.
      if (FromNullablePrev != nullptr)
        maybeAssumeNullabilityProperty(FromNullableWidened, *FromNullablePrev,
                                       CurrentEnv);
      if (NullPrev != nullptr)
        maybeAssumeNullabilityProperty(NullWidened, *NullPrev, CurrentEnv);

      return PrevPtr;
    }

    // Widen the nullability properties.
    auto &WidenedPtr = CurrentEnv.create<PointerValue>(
        getTopStorageLocation(DACtx, CurPtr.getPointeeLoc().getType()));
    initPointerNullState(
        WidenedPtr, CurrentEnv.getDataflowAnalysisContext(),
        {FromNullableWidened == WidenedProperty::Top ? nullptr
                                                     : FromNullableCur,
         NullWidened == WidenedProperty::Top ? nullptr : NullCur});

    return &WidenedPtr;
  }

  return nullptr;
}

StorageLocation &PointerNullabilityAnalysis::getTopStorageLocation(
    DataflowAnalysisContext &DACtx, QualType Ty) {
  auto [It, Inserted] = TopStorageLocations.try_emplace(Ty, nullptr);
  if (Inserted) It->second = &DACtx.createStorageLocation(Ty);
  return *It->second;
}

}  // namespace clang::tidy::nullability
