// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/pointer_nullability_analysis.h"

#include <cassert>
#include <deque>
#include <functional>
#include <optional>
#include <utility>
#include <vector>

#include "absl/base/nullability.h"
#include "absl/log/check.h"
#include "nullability/ast_helpers.h"
#include "nullability/macro_arg_capture.h"
#include "nullability/pointer_nullability.h"
#include "nullability/pointer_nullability_lattice.h"
#include "nullability/pointer_nullability_matchers.h"
#include "nullability/pragma.h"
#include "nullability/type_nullability.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclCXX.h"
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
#include "clang/Analysis/FlowSensitive/DataflowLattice.h"
#include "clang/Analysis/FlowSensitive/Formula.h"
#include "clang/Analysis/FlowSensitive/MatchSwitch.h"
#include "clang/Analysis/FlowSensitive/RecordOps.h"
#include "clang/Analysis/FlowSensitive/SmartPointerAccessorCaching.h"
#include "clang/Analysis/FlowSensitive/StorageLocation.h"
#include "clang/Analysis/FlowSensitive/Value.h"
#include "clang/Basic/Builtins.h"
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
using dataflow::LatticeEffect;
using dataflow::PointerValue;
using dataflow::RecordStorageLocation;
using dataflow::StorageLocation;
using dataflow::TransferState;
using dataflow::Value;
using dataflow::WidenResult;

#define DEBUG_TYPE "pointer_nullability_analysis.cc"

namespace {

TypeNullability prepend(NullabilityKind Head, const TypeNullability& Tail) {
  TypeNullability Result = {Head};
  Result.insert(Result.end(), Tail.begin(), Tail.end());
  return Result;
}

// If `E` is already associated with a `PointerValue`, returns it.
// Otherwise, associates a newly created `PointerValue` with `E` and returns it.
// Returns null iff `E` is not a raw pointer expression.
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

void computeNullability(const Expr* absl_nonnull E,
                        TransferState<PointerNullabilityLattice>& State,
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
const TypeNullability& getNullabilityForChild(
    const Expr* absl_nonnull E,
    TransferState<PointerNullabilityLattice>& State) {
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

static const Decl* absl_nullable getAssociatedTemplateDecl(
    const SubstTemplateTypeParmType* ST) {
  const Decl* AssociatedDecl = ST->getAssociatedDecl();
  if (!AssociatedDecl) return nullptr;
  if (isa<RedeclarableTemplateDecl>(AssociatedDecl)) return AssociatedDecl;
  if (auto* VTSD = dyn_cast<VarTemplateSpecializationDecl>(AssociatedDecl))
    return VTSD->getSpecializedTemplate();
  if (auto* CTSD = dyn_cast<ClassTemplateSpecializationDecl>(AssociatedDecl))
    return CTSD->getSpecializedTemplate();
  if (auto* FD = dyn_cast<FunctionDecl>(AssociatedDecl);
      FD && FD->isTemplateInstantiation())
    return FD->getPrimaryTemplate();
  return nullptr;
}

// The Resugarer describes the nullability of template arguments within types we
// query using getTypeNullability().
//
// When the template arguments are bound within the queried type, e.g.
//   getTypeNullability( vector<int* _Nonnull>::value_type )
// then getTypeNullability() will record the sugar and resolve the
// SubstTemplateTypeParmType within `value_type` itself.
//
// However when the template arguments are bound elsewhere in the code, e.g.
//   vector<int* _Nonnull> a;
//   getTypeNullability( a.front() )
// then we must provide the nullability vector, via the callback passed
// to getTypeNullability().
//
// This class implements that callback interface, based on the common patterns
// where template arguments can be determined from surrounding code.
struct Resugarer {
  using SubstTy = SubstTemplateTypeParmType;
  const TypeNullabilityDefaults& Defaults;

  Resugarer(const TypeNullabilityDefaults& Defaults) : Defaults(Defaults) {}

  // The entity referenced is nested within a class template, e.g. `a.front()`
  // where a is a vector<int* _Nonnull>.
  // We have a nullability vector [Nonnull] for the specialization vector<int*>.
  struct FromEnclosingClassNullability {
    ClassTemplateSpecializationDecl* Specialization;
    const ArrayRef<PointerTypeNullability> SpecializationNullability;

    std::optional<TypeNullability> operator()(const SubstTy* ST) const {
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
      if (ST->getPackIndex()) return std::nullopt;

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
  // `make_unique<int* _Nonnull>`. We have the template arguments.
  struct FromTemplateArgs {
    TemplateDecl* Template;
    ArrayRef<TemplateArgumentLoc> Args;

    std::optional<TypeNullability> operator()(
        const SubstTy* ST, const TypeNullabilityDefaults& Defaults) const {
      if (Template != getAssociatedTemplateDecl(ST)) return std::nullopt;
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

      TypeSourceInfo* TSI = Args[ST->getIndex()].getTypeSourceInfo();
      if (TSI == nullptr) return std::nullopt;
      return getTypeNullability(TSI->getTypeLoc(), Defaults);
    }
  };
  llvm::SmallVector<FromTemplateArgs> Template;

  // Add a FromTemplateArgs context reflecting that the specialization
  // `ResolvedTo` was chosen using the provided template arguments.
  void addTemplateArgs(const ValueDecl* ResolvedTo,
                       ArrayRef<TemplateArgumentLoc> UsingArgs) {
    if (const auto* VD = llvm::dyn_cast<VarDecl>(ResolvedTo)) {
      Template.push_back(
          {cast<VarTemplateSpecializationDecl>(VD)->getSpecializedTemplate(),
           UsingArgs});
    } else if (auto* FD = llvm::dyn_cast<FunctionDecl>(ResolvedTo)) {
      // TODO(b/268345783): We don't currently handle template arguments for
      // function templates with template parameter packs correctly when looking
      // up arguments later. For function templates, other template parameters
      // can follow a template parameter pack and we may report incorrect
      // information for those, so we go out of our way here to avoid that and
      // skip resugaring any arguments for function template specializations
      // with template parameter packs followed by other template parameters.
      auto TemplateArgs = FD->getTemplateSpecializationArgs()->asArray();
      bool SeenPack = false;
      for (const auto& TA : TemplateArgs) {
        if (SeenPack) return;
        if (TA.getKind() == TemplateArgument::Pack) SeenPack = true;
      }
      Template.push_back(
          {FD->getTemplateSpecializationInfo()->getTemplate(), UsingArgs});
    }
  }

  // Implement the getTypeNullability() callback interface by searching
  // all our contexts for a match.
  std::optional<TypeNullability> operator()(const SubstTy* ST) const {
    for (const auto& R : Enclosing)
      if (auto Ret = R(ST)) return Ret;
    for (const auto& R : Template)
      if (auto Ret = R(ST, Defaults)) return Ret;
    return std::nullopt;
  }
};

PointerTypeNullability getPointerTypeNullability(
    const Expr* absl_nonnull E, PointerNullabilityAnalysis::Lattice& L) {
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

// If `Elt` is an expression of raw pointer type, ensures that it has a
// `PointerValue` associated with it. Also ensure that it has nullability
// state.
void ensureRawPointerHasValueAndNullability(
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

/// If the pointer value stored at `PointerLoc` has any "top" nullability
/// properties, creates a new pointer value referencing the same location with
/// the "top" properties unpacked into fresh atoms. Returns:
/// -  The unpacked pointer value if unpacking took place.
/// -  The original pointer value if no unpacking took place.
/// -  Null if `PointerLoc` is not associated with a value.
/// This is analogous to the unpacking done on `TopBoolValue`s in the framework.
PointerValue* absl_nullable unpackPointerValue(StorageLocation& PointerLoc,
                                               Environment& Env) {
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

void setToPointerWithNullability(StorageLocation& PtrLoc, NullabilityKind NK,
                                 Environment& Env) {
  auto& Val = *cast<PointerValue>(Env.createValue(PtrLoc.getType()));
  initPointerNullState(Val, Env.getDataflowAnalysisContext(), NK);
  Env.setValue(PtrLoc, Val);
}

void initSmartPointerForExpr(const Expr* E,
                             TransferState<PointerNullabilityLattice>& State) {
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

void transferValue_NullPointer(
    const Expr* absl_nonnull NullPointer, const MatchFinder::MatchResult&,
    TransferState<PointerNullabilityLattice>& State) {
  if (auto* PointerVal = ensureRawPointerHasValue(NullPointer, State.Env)) {
    initNullPointer(*PointerVal, State.Env.getDataflowAnalysisContext());
  }
}

void transferValue_PointerIncOrDec(
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

void transferValue_PointerAddOrSubAssign(
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

void transferValue_NotNullPointer(
    const Expr* absl_nonnull NotNullPointer, const MatchFinder::MatchResult&,
    TransferState<PointerNullabilityLattice>& State) {
  if (auto* PointerVal = ensureRawPointerHasValue(NotNullPointer, State.Env)) {
    initPointerNullState(*PointerVal, State.Env.getDataflowAnalysisContext(),
                         NullabilityKind::NonNull);
  }
}

bool isStdWeakPtrType(QualType Ty) {
  const CXXRecordDecl* RD = Ty.getCanonicalType()->getAsCXXRecordDecl();
  if (RD == nullptr) return false;

  if (!RD->getDeclContext()->isStdNamespace()) return false;

  const IdentifierInfo* ID = RD->getIdentifier();
  if (ID == nullptr) return false;

  return ID->getName() == "weak_ptr";
}

QualType underlyingRawPointerTypeFromSmartPointer(RecordStorageLocation& Loc) {
  return Loc.getSyntheticField(PtrField).getType();
}

bool isPointerTypeConvertible(QualType From, QualType To) {
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

void transferValue_SmartPointerConstructor(
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
      if (isPointerTypeConvertible(
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

void transferValue_SmartPointerAssignment(
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

void transferValue_SmartPointerReleaseCall(
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

void transferValue_SmartPointerResetCall(
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

void swapSmartPointers(RecordStorageLocation* Loc1, RecordStorageLocation* Loc2,
                       Environment& Env) {
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

void transferValue_SmartPointerMemberSwapCall(
    const CXXMemberCallExpr* MCE, const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  swapSmartPointers(getImplicitObjectLocation(*MCE, State.Env),
                    State.Env.get<RecordStorageLocation>(*MCE->getArg(0)),
                    State.Env);
}

void transferValue_SmartPointerFreeSwapCall(
    const CallExpr* CE, const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  swapSmartPointers(State.Env.get<RecordStorageLocation>(*CE->getArg(0)),
                    State.Env.get<RecordStorageLocation>(*CE->getArg(1)),
                    State.Env);
}

void transferValue_SmartPointerGetCall(
    const CXXMemberCallExpr* MCE, const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  // If the return type isn't what we expect, bail out.
  // See `transferValue_SmartPointerReleaseCall()` for more details.
  if (MCE->getType()->getCanonicalTypeUnqualified() !=
      underlyingRawPointerType(MCE->getObjectType())
          ->getCanonicalTypeUnqualified()) {
    return;
  }
  if (Value* Val = getPointerValueFromSmartPointer(
          getImplicitObjectLocation(*MCE, State.Env), State.Env))
    State.Env.setValue(*MCE, *Val);
}

void transferValue_SmartPointerBoolConversionCall(
    const CXXMemberCallExpr* MCE, const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  if (PointerValue* Val = getPointerValueFromSmartPointer(
          getImplicitObjectLocation(*MCE, State.Env), State.Env)) {
    if (const Formula* IsNull = getPointerNullState(*Val).IsNull)
      State.Env.setValue(
          *MCE, State.Env.makeNot(State.Env.arena().makeBoolValue(*IsNull)));
  }
}

QualType getReceiverIgnoringImpCastsType(const CXXOperatorCallExpr* OpCall) {
  // Matchers hasArgument() appears to ignore implicit casts, so we ignore them
  // here as well to get the same behavior:
  // https://github.com/llvm/llvm-project/blob/a58c3d3ac7c6b2fd9710ab2189d7971ef37e714f/clang/include/clang/ASTMatchers/ASTMatchers.h#L4563
  const Expr* Receiver = OpCall->getArg(0)->IgnoreImpCasts();
  if (Receiver->isPRValue() && Receiver->getType()->isPointerType())
    return Receiver->getType()->getPointeeType();
  return Receiver->getType();
}

void transferValue_SmartPointerOperatorStar(
    const CXXOperatorCallExpr* OpCall, const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  // If the return type isn't what we expect, bail out.
  // See `transferValue_SmartPointerReleaseCall()` for more details.
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

void transferValue_SmartPointerOperatorArrow(
    const CXXOperatorCallExpr* OpCall, const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  // If the return type isn't what we expect, bail out.
  // See `transferValue_SmartPointerReleaseCall()` for more details.
  if (OpCall->getType()->getCanonicalTypeUnqualified() !=
      underlyingRawPointerType(getReceiverIgnoringImpCastsType(OpCall))
          ->getCanonicalTypeUnqualified()) {
    return;
  }
  if (PointerValue* Val = getSmartPointerValue(OpCall->getArg(0), State.Env)) {
    State.Env.setValue(*OpCall, *Val);
  }
}

void transferValue_SmartPointerFactoryCall(
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

void transferValue_SmartPointerComparisonOpCall(
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

void transferValue_SharedPtrCastCall(
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
    const CXXMemberCallExpr* MCE, const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  RecordStorageLocation& Loc = State.Env.getResultObjectLocation(*MCE);
  StorageLocation& PtrLoc = Loc.getSyntheticField(PtrField);

  setToPointerWithNullability(PtrLoc, NullabilityKind::Nullable, State.Env);
}

void transferValue_SmartPointerArrowMemberExpr(
    const MemberExpr* ME, const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  // Most accesses of a smart pointer involve a glvalue of smart pointer type,
  // and `transferValue_SmartPointer` will ensure in this case that the
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

void transferValue_Pointer(const Expr* absl_nonnull PointerExpr,
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
// expresssions, respectively. A nullptr value is interpreted as Top.
BoolValue* absl_nullable processPointerComparison(
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
void transferValue_NullCheckComparison(
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

void transferValue_NullCheckImplicitCastPtrToBool(
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

void initializeOutputParameter(const Expr* absl_nonnull Arg,
                               TransferState<PointerNullabilityLattice>& State,
                               const VarDecl& Param) {
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
bool isDeclaredInAbseilOrUtil(const Decl& D) {
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
void modelGetReferenceableValue(const CallExpr& CE, Environment& Env) {
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
void modelCheckNE(const CallExpr& CE, Environment& Env) {
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

void transferValue_CallExpr(const CallExpr* absl_nonnull CE,
                            const MatchFinder::MatchResult& Result,
                            TransferState<PointerNullabilityLattice>& State) {
  // The dataflow framework itself generally does not model `CallExpr`s
  // (including creating values for the results). We model some specific
  // function calls and handle value creation for certain types.

  const auto* FuncDecl = CE->getDirectCallee();
  if (FuncDecl != nullptr) {
    if (const IdentifierInfo* FunII =
            FuncDecl->getDeclName().getAsIdentifierInfo()) {
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

  if (isSupportedRawPointerType(CE->getType())) {
    // Create a pointer so that we can attach nullability to it and have the
    // nullability propagate with the pointer.
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
  }

  if (CE->isCallToStdMove() || FuncDecl == nullptr) return;

  // Don't treat parameters of our macro replacement argument-capture functions
  // as output parameters.
  if (const IdentifierInfo* FunII =
          FuncDecl->getDeclName().getAsIdentifierInfo();
      FunII && (FunII->isStr(ArgCaptureAbortIfFalse) ||
                FunII->isStr(ArgCaptureAbortIfEqual)))
    return;
  // Make output parameters (with unknown nullability) initialized to unknown.
  for (ParamAndArgIterator<CallExpr> Iter(*FuncDecl, *CE); Iter; ++Iter)
    initializeOutputParameter(&Iter.arg(), State, Iter.param());
}

void transferValue_AccessorCall(
    const CXXMemberCallExpr* absl_nonnull MCE,
    const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  auto* member = Result.Nodes.getNodeAs<clang::ValueDecl>("member-decl");
  PointerValue* PointerVal = nullptr;
  StorageLocation* FieldLoc = nullptr;
  if (dataflow::RecordStorageLocation* RecordLoc =
          dataflow::getImplicitObjectLocation(*MCE, State.Env)) {
    FieldLoc = RecordLoc->getChild(*member);
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

std::function<void(StorageLocation&)>
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

void handleConstMemberCall(
    const CallExpr* absl_nonnull CE,
    dataflow::RecordStorageLocation* absl_nullable RecordLoc,
    const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  if (RecordLoc == nullptr) {
    // Perform default handling
    transferValue_CallExpr(CE, Result, State);
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
      transferValue_CallExpr(CE, Result, State);
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
      transferValue_CallExpr(CE, Result, State);
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
  transferValue_CallExpr(CE, Result, State);
}

void transferValue_ConstMemberCall(
    const CXXMemberCallExpr* absl_nonnull MCE,
    const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  handleConstMemberCall(
      MCE, dataflow::getImplicitObjectLocation(*MCE, State.Env), Result, State);
}

void transferValue_ConstMemberOperatorCall(
    const CXXOperatorCallExpr* OCE, const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  auto* RecordLoc = cast_or_null<dataflow::RecordStorageLocation>(
      State.Env.getStorageLocation(*OCE->getArg(0)));
  handleConstMemberCall(OCE, RecordLoc, Result, State);
}

void handleNonConstMemberCall(const CallExpr* absl_nonnull CE,
                              dataflow::RecordStorageLocation* RecordLoc,
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
  transferValue_CallExpr(CE, Result, State);
}

void transferValue_NonConstMemberCall(
    const CXXMemberCallExpr* absl_nonnull MCE,
    const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  handleNonConstMemberCall(
      MCE, dataflow::getImplicitObjectLocation(*MCE, State.Env), Result, State);
}

void transferValue_NonConstMemberOperatorCall(
    const CXXOperatorCallExpr* absl_nonnull OCE,
    const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  auto* RecordLoc = cast_or_null<dataflow::RecordStorageLocation>(
      State.Env.getStorageLocation(*OCE->getArg(0)));
  handleNonConstMemberCall(OCE, RecordLoc, Result, State);
}

void transferType_DeclRefExpr(const DeclRefExpr* absl_nonnull DRE,
                              const MatchFinder::MatchResult& MR,
                              TransferState<PointerNullabilityLattice>& State) {
  computeNullability(DRE, State, [&] {
    Resugarer Resugar(State.Lattice.defaults());

    if (DRE->hasExplicitTemplateArgs())
      Resugar.addTemplateArgs(DRE->getDecl(), DRE->template_arguments());
    std::deque<TypeNullability> ScopeNullabilityStorage;
    for (auto NNS = DRE->getQualifierLoc(); NNS;) {
      if (auto* CTSD = llvm::dyn_cast_or_null<ClassTemplateSpecializationDecl>(
              NNS.getNestedNameSpecifier().getAsRecordDecl())) {
        ScopeNullabilityStorage.push_back(
            getTypeNullability(NNS.getAsTypeLoc(), State.Lattice.defaults()));
        Resugar.Enclosing.push_back({CTSD, ScopeNullabilityStorage.back()});
      }
      if (NNS.getNestedNameSpecifier().getKind() ==
          clang::NestedNameSpecifier::Kind::Namespace)
        NNS = NNS.getAsNamespaceAndPrefix().Prefix;
      else if (NNS.getNestedNameSpecifier().getKind() ==
               clang::NestedNameSpecifier::Kind::Type)
        NNS = NNS.getAsTypeLoc().getPrefix();
      else
        NNS = clang::NestedNameSpecifierLoc();
    }

    return State.Lattice.getTypeNullabilityWithOverrides(*DRE->getDecl(),
                                                         Resugar);
  });
}

void transferType_MemberExpr(const MemberExpr* absl_nonnull ME,
                             const MatchFinder::MatchResult& MR,
                             TransferState<PointerNullabilityLattice>& State) {
  computeNullability(ME, State, [&]() {
    auto* Member = ME->getMemberDecl();
    auto BaseType = ME->getBase()->getType();
    auto BaseNullability =
        ArrayRef(getNullabilityForChild(ME->getBase(), State));
    if (ME->isArrow() && BaseType->isPointerType()) {
      BaseType = BaseType->getPointeeType();
      BaseNullability = BaseNullability.drop_front();
    }

    Resugarer Resugar(State.Lattice.defaults());
    if (const auto* RT = BaseType->getAs<RecordType>()) {
      if (auto* CTSpec = dyn_cast<ClassTemplateSpecializationDecl>(
              RT->getOriginalDecl())) {
        Resugar.Enclosing.push_back({CTSpec, BaseNullability});
      }
    }
    if (ME->hasExplicitTemplateArgs())
      Resugar.addTemplateArgs(ME->getMemberDecl(), ME->template_arguments());

    return State.Lattice.getTypeNullabilityWithOverrides(*Member, Resugar);
  });
}

void transferType_CastExpr(const CastExpr* absl_nonnull CE,
                           const MatchFinder::MatchResult& MR,
                           TransferState<PointerNullabilityLattice>& State) {
  computeNullability(CE, State, [&]() -> TypeNullability {
    // Most casts that can convert ~unrelated types drop nullability in general.
    // As a special case, preserve nullability of outer raw pointer types.
    // For example, int* p; (void*)p; is a BitCast, but preserves nullability.
    // TODO: b/396242014 - Consider applying the target type's nullability
    // annotations for explicit casts rather than preserving the argument's
    // nullability.
    auto PreserveOuterRawPointers = [&](TypeNullability V) {
      auto ArgNullability = getNullabilityForChild(CE->getSubExpr(), State);
      const PointerType* ArgType = dyn_cast<PointerType>(
          CE->getSubExpr()->getType().getCanonicalType().getTypePtr());
      const PointerType* CastType =
          dyn_cast<PointerType>(CE->getType().getCanonicalType().getTypePtr());
      for (int I = 0; ArgType && CastType; ++I) {
        V[I] = ArgNullability[I];
        ArgType = dyn_cast<PointerType>(ArgType->getPointeeType().getTypePtr());
        CastType =
            dyn_cast<PointerType>(CastType->getPointeeType().getTypePtr());
      }
      return V;
    };

    // We can't assume that all nullability information is identical between
    // base and derived types, such as when a derived type has fewer entries
    // because it always supplies the same template argument(s) to the base
    // type.
    //
    // We preserve the nullability of outer raw pointers, but for implicit
    // casts, for the record type with the inheritance relationship, we attempt
    // some resugaring of the result type nullability from the argument's
    // template arguments.
    auto PreserveAndResugarFromBaseOrDerived = [&](TypeNullability V) {
      V = PreserveOuterRawPointers(V);
      if (auto* ECE = dyn_cast<ExplicitCastExpr>(CE)) {
        // TODO: b/396242014 - Apply the nullability annotations from the target
        // type rather than preserving the argument's nullability. Do this both
        // for outer pointers and for template arguments.
        return V;
      }
      if (auto* ICE = dyn_cast<ImplicitCastExpr>(CE);
          ICE && ICE->isPartOfExplicitCast()) {
        // Let the nullability be picked up from the explicit cast; no need to
        // do work here.
        return V;
      }
      if (CE->path_empty()) {
        llvm::errs() << "Empty path for cast between base and derived types.\n";
        assert(false);
        return V;
      }

      int NumOuterRawPointers = 0;
      const Type* UnderPointers =
          CE->getSubExpr()->getType().getCanonicalType().getTypePtr();
      while (auto* PT = dyn_cast<PointerType>(UnderPointers)) {
        UnderPointers = PT->getPointeeType().getTypePtr();
        ++NumOuterRawPointers;
      }
      if (NumOuterRawPointers < V.size()) {
        // For the elements of V after the nullability for any outer raw
        // pointers, resugar the result type from the argument's template
        // arguments.
        const TypeNullability& ArgNullability =
            getNullabilityForChild(CE->getSubExpr(), State);
        TypeNullability UnderPointersNullability;
        for (int J = NumOuterRawPointers; J < ArgNullability.size(); ++J) {
          UnderPointersNullability.push_back(ArgNullability[J]);
        }
        Resugarer Resugar(State.Lattice.defaults());
        // Resugar from class template arguments, if any.
        if (const auto* RT = UnderPointers->getAs<RecordType>()) {
          if (auto* CTSpec = dyn_cast<ClassTemplateSpecializationDecl>(
                  RT->getOriginalDecl())) {
            Resugar.Enclosing.push_back({CTSpec, UnderPointersNullability});
          }
        }
        auto CastNullability = getTypeNullability(
            (*(CE->path_end() - 1))->getTypeSourceInfo()->getTypeLoc(),
            State.Lattice.defaults(), Resugar);
        if (CastNullability.size() + NumOuterRawPointers != V.size()) {
          llvm::errs()
              << "CastNullability.size() + NumOuterRawPointers != V.size(): "
              << (CastNullability.size() + NumOuterRawPointers) << " vs "
              << V.size() << "\n";
          CE->dump();
          assert(false);
        }
        for (int I = 0;
             I + NumOuterRawPointers < V.size() && I < CastNullability.size();
             ++I) {
          V[I + NumOuterRawPointers] = CastNullability[I];
        }
      }
      return V;
    };

    switch (CE->getCastKind()) {
      // Casts between unrelated types: we can't say anything about nullability.
      case CK_LValueBitCast:
      case CK_BitCast:
      case CK_LValueToRValueBitCast:
        return PreserveOuterRawPointers(unspecifiedNullability(CE));

      // Casts between equivalent types.
      case CK_LValueToRValue:
      case CK_NoOp:
      case CK_AtomicToNonAtomic:
      case CK_NonAtomicToAtomic:
      case CK_AddressSpaceConversion:
        return getNullabilityForChild(CE->getSubExpr(), State);

      // Controlled conversions between types
      case CK_BaseToDerived:
      case CK_DerivedToBase:
      case CK_UncheckedDerivedToBase:
        return PreserveAndResugarFromBaseOrDerived(unspecifiedNullability(CE));
      case CK_UserDefinedConversion:
        return unspecifiedNullability(CE);
      case CK_ConstructorConversion:
        if (auto* CCE = llvm::dyn_cast<CXXConstructExpr>(CE->getSubExpr())) {
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
        if (const auto* ECE = dyn_cast<ExplicitCastExpr>(CE))
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
      case CK_HLSLArrayRValue:
      case CK_HLSLAggregateSplatCast:
      case CK_HLSLElementwiseCast:
        return unspecifiedNullability(CE);

      case CK_Dependent:
        CHECK(false) << "Shouldn't see dependent casts here?";
    }
  });
}

void transferType_MaterializeTemporaryExpr(
    const MaterializeTemporaryExpr* absl_nonnull MTE,
    const MatchFinder::MatchResult& MR,
    TransferState<PointerNullabilityLattice>& State) {
  computeNullability(MTE, State, [&]() {
    return getNullabilityForChild(MTE->getSubExpr(), State);
  });
}

void transferType_CXXBindTemporaryExpr(
    const CXXBindTemporaryExpr* BTE, const MatchFinder::MatchResult& MR,
    TransferState<PointerNullabilityLattice>& State) {
  computeNullability(BTE, State, [&]() {
    return getNullabilityForChild(BTE->getSubExpr(), State);
  });
}

void transferType_CopyOrMoveConstruct(
    const CXXConstructExpr* CCE, const MatchFinder::MatchResult& MR,
    TransferState<PointerNullabilityLattice>& State) {
  computeNullability(CCE, State, [&]() {
    return getNullabilityForChild(CCE->getArg(0), State);
  });
}

TypeNullability computeTypeNullabilityForCallExpr(
    const CallExpr* absl_nonnull CE,
    TransferState<PointerNullabilityLattice>& State) {
  TypeNullability CalleeNullability =
      getNullabilityForChild(CE->getCallee(), State);
  ArrayRef ResultNullability = CalleeNullability;
  if (CE->getCallee()->getType()->isPointerType())  // Callee is usually fptr.
    ResultNullability = ResultNullability.drop_front();
  // Return value nullability is at the front of the function type.
  ResultNullability =
      ResultNullability.take_front(countPointersInType(CE->getType()));
  return ResultNullability.vec();
}

void transferType_CallExpr(const CallExpr* absl_nonnull CE,
                           const MatchFinder::MatchResult& MR,
                           TransferState<PointerNullabilityLattice>& State) {
  computeNullability(CE, State, [&]() {
    if (auto ID = CE->getBuiltinCallee();
        (ID == Builtin::BIforward || ID == Builtin::BImove) &&
        CE->getNumArgs() == 1) {
      return getNullabilityForChild(CE->getArg(0), State);
    }

    return computeTypeNullabilityForCallExpr(CE, State);
  });
}

void transferType_CXXOperatorCallExpr(
    const CXXOperatorCallExpr* absl_nonnull CE,
    const MatchFinder::MatchResult& MR,
    TransferState<PointerNullabilityLattice>& State) {
  computeNullability(CE, State, [&]() {
    // If this is a method call, see if it is a template specialization
    // and whether resugaring with the Base (arg 0)'s nullability helps
    // refine the return type nullability, similar to transferType_MemberExpr.
    // This only helps refine the return type nullability, not callee's
    // nullability including the params. TODO(b/405355053): see if we can refine
    // the params too.
    if (auto* Callee = dyn_cast<CXXMethodDecl>(CE->getCalleeDecl())) {
      const auto* Base = CE->getArg(0);
      TypeNullability BaseNullability = getNullabilityForChild(Base, State);
      Resugarer Resugar(State.Lattice.defaults());
      if (const auto* RT = Base->getType()->getAs<RecordType>()) {
        if (auto* CTSpec = dyn_cast<ClassTemplateSpecializationDecl>(
                RT->getOriginalDecl())) {
          Resugar.Enclosing.push_back({CTSpec, BaseNullability});
        }
      }

      TypeNullability Nullability =
          State.Lattice.getTypeNullabilityWithOverrides(*Callee, Resugar);
      ArrayRef ResultNullability = Nullability;
      // Return value nullability is at the front of the function type.
      ResultNullability =
          ResultNullability.take_front(countPointersInType(CE->getType()));
      return ResultNullability.vec();
    }

    // Not a member operator call.
    return computeTypeNullabilityForCallExpr(CE, State);
  });
}

void transferType_UnaryOperator(
    const UnaryOperator* absl_nonnull UO, const MatchFinder::MatchResult& MR,
    TransferState<PointerNullabilityLattice>& State) {
  computeNullability(UO, State, [&]() -> TypeNullability {
    switch (UO->getOpcode()) {
      case UO_AddrOf:
        return prepend(NullabilityKind::NonNull,
                       getNullabilityForChild(UO->getSubExpr(), State));
      case UO_Deref:
        return ArrayRef(getNullabilityForChild(UO->getSubExpr(), State))
            .drop_front()
            .vec();

      case UO_PreInc:
      case UO_PreDec: {
        TypeNullability SubNullability =
            getNullabilityForChild(UO->getSubExpr(), State);
        if (!isSupportedRawPointerType(UO->getSubExpr()->getType()))
          return SubNullability;
        assert(!SubNullability.empty());
        SubNullability[0] = NullabilityKind::NonNull;
        return SubNullability;
      }

      case UO_PostInc:
      case UO_PostDec:
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
    const BinaryOperator* absl_nonnull BO, const MatchFinder::MatchResult& MR,
    TransferState<PointerNullabilityLattice>& State) {
  computeNullability(BO, State, [&]() -> TypeNullability {
    switch (BO->getOpcode()) {
      case BO_PtrMemD:
      case BO_PtrMemI:
        // TODO: pointers-to-member should really have nullability vectors
        return unspecifiedNullability(BO);
      case BO_Assign:
      case BO_Comma:
        return getNullabilityForChild(BO->getRHS(), State);
      case BO_Add:
      case BO_Sub:
      // The `+=` and `-=` operators will always take the "LHS" branch below but
      // can otherwise be handled using the same code as `+` and `-`, so we do.
      case BO_AddAssign:
      case BO_SubAssign: {
        bool LhsIsPointer = isSupportedRawPointerType(BO->getLHS()->getType());
        bool RhsIsPointer = isSupportedRawPointerType(BO->getRHS()->getType());
        // Pointer difference.
        if (LhsIsPointer && RhsIsPointer) {
          assert(BO->getOpcode() == BO_Sub);
          assert(BO->getType()->isIntegerType());
          return {};
        }
        TypeNullability PtrNullability;
        if (LhsIsPointer)
          PtrNullability = getNullabilityForChild(BO->getLHS(), State);
        else if (RhsIsPointer)
          PtrNullability = getNullabilityForChild(BO->getRHS(), State);
        else
          return unspecifiedNullability(BO);
        assert(!PtrNullability.empty());
        PtrNullability[0] = NullabilityKind::NonNull;
        return PtrNullability;
      }
      default:
        // No other built-in binary operators can be pointer-valued
        return unspecifiedNullability(BO);
    }
  });
}

void transferType_NewExpr(const CXXNewExpr* absl_nonnull NE,
                          const MatchFinder::MatchResult& MR,
                          TransferState<PointerNullabilityLattice>& State) {
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
    const ArraySubscriptExpr* absl_nonnull ASE,
    const MatchFinder::MatchResult& MR,
    TransferState<PointerNullabilityLattice>& State) {
  computeNullability(ASE, State, [&]() {
    auto& BaseNullability = getNullabilityForChild(ASE->getBase(), State);
    QualType BaseType = ASE->getBase()->getType();
    CHECK(isSupportedRawPointerType(BaseType) || BaseType->isVectorType());
    return isSupportedRawPointerType(BaseType)
               ? ArrayRef(BaseNullability).slice(1).vec()
               : BaseNullability;
  });
}

void transferType_ThisExpr(const CXXThisExpr* absl_nonnull TE,
                           const MatchFinder::MatchResult& MR,
                           TransferState<PointerNullabilityLattice>& State) {
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
      .CaseOfCFGStmt<CXXOperatorCallExpr>(ast_matchers::cxxOperatorCallExpr(),
                                          transferType_CXXOperatorCallExpr)
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
      .CaseOfCFGStmt<ImplicitValueInitExpr>(isRawPointerImplicitValueInit(),
                                            transferValue_NullPointer)
      .CaseOfCFGStmt<CXXDefaultInitExpr>(isNullPointerDefaultInit(),
                                         transferValue_NullPointer)
      .CaseOfCFGStmt<UnaryOperator>(isPointerIncOrDec(),
                                    transferValue_PointerIncOrDec)
      .CaseOfCFGStmt<BinaryOperator>(isPointerAddOrSubAssign(),
                                     transferValue_PointerAddOrSubAssign)
      .CaseOfCFGStmt<CXXConstructExpr>(isSmartPointerConstructor(),
                                       transferValue_SmartPointerConstructor)
      .CaseOfCFGStmt<CXXOperatorCallExpr>(isSmartPointerOperatorCall("=", 2),
                                          transferValue_SmartPointerAssignment)
      .CaseOfCFGStmt<CXXMemberCallExpr>(
          isSmartPointerMethodCall("release", "Release"),
          transferValue_SmartPointerReleaseCall)
      .CaseOfCFGStmt<CXXMemberCallExpr>(
          isSmartPointerMethodCall("reset", "Reset"),
          transferValue_SmartPointerResetCall)
      .CaseOfCFGStmt<CXXMemberCallExpr>(
          isSmartPointerMethodCall("swap", "Swap"),
          transferValue_SmartPointerMemberSwapCall)
      .CaseOfCFGStmt<CallExpr>(isSmartPointerFreeSwapCall(),
                               transferValue_SmartPointerFreeSwapCall)
      .CaseOfCFGStmt<CXXMemberCallExpr>(isSmartPointerMethodCall("get"),
                                        transferValue_SmartPointerGetCall)
      .CaseOfCFGStmt<CXXMemberCallExpr>(
          isSmartPointerBoolConversionCall(),
          transferValue_SmartPointerBoolConversionCall)
      .CaseOfCFGStmt<CXXOperatorCallExpr>(
          isSmartPointerOperatorCall("*", 1),
          transferValue_SmartPointerOperatorStar)
      .CaseOfCFGStmt<CXXOperatorCallExpr>(
          isSmartPointerOperatorCall("->", 1),
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
      .CaseOfCFGStmt<CXXMemberCallExpr>(isZeroParamConstMemberCall(),
                                        transferValue_ConstMemberCall)
      .CaseOfCFGStmt<CXXOperatorCallExpr>(isZeroParamConstMemberOperatorCall(),
                                          transferValue_ConstMemberOperatorCall)
      .CaseOfCFGStmt<CXXMemberCallExpr>(isNonConstMemberCall(),
                                        transferValue_NonConstMemberCall)
      .CaseOfCFGStmt<CXXOperatorCallExpr>(
          isNonConstMemberOperatorCall(),
          transferValue_NonConstMemberOperatorCall)
      .CaseOfCFGStmt<CallExpr>(ast_matchers::callExpr(), transferValue_CallExpr)
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

// Ensure that all expressions of smart pointer type have an underlying
// raw pointer initialized from the type nullability.
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

}  // namespace

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
    // We return true to keep the `MergedVal` produced by the framework. When
    // the merged value appears in an expression, `tranferValue_Pointer` will
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
