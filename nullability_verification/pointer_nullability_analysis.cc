// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability_verification/pointer_nullability_analysis.h"

#include <string>

#include "absl/log/check.h"
#include "nullability_verification/pointer_nullability.h"
#include "nullability_verification/pointer_nullability_matchers.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Expr.h"
#include "clang/AST/OperationKinds.h"
#include "clang/AST/Stmt.h"
#include "clang/AST/Type.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/Analysis/FlowSensitive/CFGMatchSwitch.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/NoopLattice.h"
#include "clang/Analysis/FlowSensitive/Value.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/Specifiers.h"

namespace clang {
namespace tidy {
namespace nullability {

using ast_matchers::MatchFinder;
using dataflow::BoolValue;
using dataflow::CFGMatchSwitchBuilder;
using dataflow::Environment;
using dataflow::NoopLattice;
using dataflow::PointerValue;
using dataflow::SkipPast;
using dataflow::TransferState;
using dataflow::Value;

namespace {

void getNullabilityAnnotationsFromTypeImpl(
    QualType T, std::vector<NullabilityKind>& Result) {
  if (auto ET = T->getAs<ElaboratedType>()) {
    getNullabilityAnnotationsFromTypeImpl(ET->getNamedType(), Result);
  } else if (auto TST = T->getAs<TemplateSpecializationType>()) {
    for (auto TA : TST->template_arguments()) {
      if (TA.getKind() == TemplateArgument::Type) {
        getNullabilityAnnotationsFromTypeImpl(TA.getAsType(), Result);
      }
    }
  } else if (auto AT = T->getAs<AttributedType>()) {
    Optional<NullabilityKind> NK = AT->getImmediateNullability();
    if (NK.has_value()) {
      Result.push_back(AT->getImmediateNullability().value());
      QualType MT = AT->getModifiedType();
      if (auto PT = MT->getAs<PointerType>()) {
        getNullabilityAnnotationsFromTypeImpl(PT->getPointeeType(), Result);
      } else {
        // TODO: Handle this unusual yet possible (e.g. through typedefs)
        // case.
        llvm::dbgs() << "\nThe type " << T
                     << "contains a nullability annotation that is not "
                     << "succeeded by a pointer type. "
                     << "This occurence is not currently handled.\n";
      }
    } else {
      getNullabilityAnnotationsFromTypeImpl(AT->getModifiedType(), Result);
    }
  } else if (auto PtrT = T->getAs<PointerType>()) {
    Result.push_back(NullabilityKind::Unspecified);
    getNullabilityAnnotationsFromTypeImpl(PtrT->getPointeeType(), Result);
  }
}

/// Traverse over a type to get its nullability. For example, if T is the type
/// Struct3Arg<int * _Nonnull, int, pair<int * _Nullable, int *>> * _Nonnull,
/// the resulting nullability annotations will be {_Nonnull, _Nonnull,
/// _Nullable, _Unknown}. Note that non-pointer elements (e.g., the second
/// argument of Struct3Arg) do not get a nullability annotation.
std::vector<NullabilityKind> getNullabilityAnnotationsFromType(QualType T) {
  std::vector<NullabilityKind> Result;
  getNullabilityAnnotationsFromTypeImpl(T, Result);
  return Result;
}

unsigned countPointersInType(QualType T) {
  if (auto ET = T->getAs<ElaboratedType>()) {
    return countPointersInType(ET->getNamedType());
  } else if (auto AT = T->getAs<AttributedType>()) {
    return countPointersInType(AT->getModifiedType());
  } else if (auto PtrT = T->getAs<PointerType>()) {
    return 1 + countPointersInType(PtrT->getPointeeType());
  }
  return 0;
}

unsigned countPointersInType(TemplateArgument TA) {
  if (TA.getKind() == TemplateArgument::Type) {
    return countPointersInType(TA.getAsType());
  }
  return 0;
}

/// Use the nullability annotations of the base type to compute the nullability
/// of a type that was originally written as a template type parameter.
/// For example, consider the following code:
///
/// template <typename T0, typename T1>
/// struct S {
///   T0 arg0;
///   T1 arg1;
/// };
/// void target(S<pair<int * _Nullable, int *> * _Nonnull, int * _Nullable> p) {
///   p.arg0; // (*)
/// }
///
/// Suppose we wish to find the nullability annotations of arg0. The nullability
/// annotation list of Struct2Arg is {_Nonnull, _Nullable, _Unknown, _Nullable}.
/// We use this list and information about S to infer that the
/// nullability annotation list of arg0 is {_Nonnull, _Nullable, _Unknown}.
ArrayRef<NullabilityKind> getNullabilityForTemplateParameter(
    const SubstTemplateTypeParmType* STTPT,
    ArrayRef<NullabilityKind> BaseNullabilityAnnotations, QualType BaseType) {
  unsigned PointerCount = 0;
  unsigned ArgIndex = STTPT->getIndex();
  if (auto TST = BaseType->getAs<TemplateSpecializationType>()) {
    for (auto TA : TST->template_arguments().take_front(ArgIndex)) {
      PointerCount += countPointersInType(TA);
    }
    unsigned SliceSize =
        countPointersInType(TST->template_arguments()[ArgIndex]);
    return BaseNullabilityAnnotations.slice(PointerCount, SliceSize);
  }
  return ArrayRef<NullabilityKind>();
}

void substituteNullabilityAnnotationsInTemplateImpl(
    std::vector<NullabilityKind>& Result, QualType T,
    ArrayRef<NullabilityKind> BaseNullabilityAnnotations, QualType BaseType) {
  if (auto ST = T->getAs<SubstTemplateTypeParmType>()) {
    for (auto NK : getNullabilityForTemplateParameter(
             ST, BaseNullabilityAnnotations, BaseType)) {
      Result.push_back(NK);
    }
  } else if (auto PtrT = T->getAs<PointerType>()) {
    Result.push_back(NullabilityKind::Unspecified);
    substituteNullabilityAnnotationsInTemplateImpl(
        Result, PtrT->getPointeeType(), BaseNullabilityAnnotations, BaseType);
  } else if (auto ET = T->getAs<ElaboratedType>()) {
    substituteNullabilityAnnotationsInTemplateImpl(
        Result, ET->getNamedType(), BaseNullabilityAnnotations, BaseType);
  } else if (auto TST = T->getAs<TemplateSpecializationType>()) {
    for (auto TA : TST->template_arguments()) {
      if (TA.getKind() == TemplateArgument::Type) {
        substituteNullabilityAnnotationsInTemplateImpl(
            Result, TA.getAsType(), BaseNullabilityAnnotations, BaseType);
      }
    }
  }
}

/// Similar to getNullabilityForTemplateParameter, but here we get the
/// nullability annotation for a type that *contains* another type that was
/// originally written as a template type parameter. For example, consider the
/// following code:
///
/// template <typename T0, typename T1>
/// struct Struct2Arg {
///   T1 *_Nullable getNullableT1Ptr();
/// };
/// void target(Struct2Arg<int *, int *_Nonnull> &x) {
///   x.getNullableT1Ptr();
/// }
///
/// Suppose we wish to find the nullability annotations of x.getNullableT1Ptr().
/// The return type of this method call is T1 * _Nullable, so its outer
/// nullability is "_Nullable". Then, we continue recursing over this type to
/// find the rest of the nullability annotation. We call
/// getNullabilityFromTemplateParameter to find that T1 has nullability
/// annotation {_Nonnull}. Thus, our complete nullability annotation for this
/// member call is {_Nullable, _Nonnull}.
std::vector<NullabilityKind> substituteNullabilityAnnotationsInTemplate(
    QualType T, ArrayRef<NullabilityKind> BaseNullabilityAnnotations,
    QualType BaseType) {
  std::vector<NullabilityKind> Result;
  substituteNullabilityAnnotationsInTemplateImpl(
      Result, T, BaseNullabilityAnnotations, BaseType);
  return Result;
}

/// Get nullability annotations of the base type. For example, in the member
/// expression x.f or the member call x.getF(), x is the base object and its
/// type is the base type.
std::vector<NullabilityKind> getBaseNullabilityAnnotations(const Expr* E) {
  if (auto ME = dyn_cast<MemberExpr>(E)) {
    return getBaseNullabilityAnnotations(ME->getBase());
  } else if (auto MC = dyn_cast<CXXMemberCallExpr>(E)) {
    return getBaseNullabilityAnnotations(MC->getImplicitObjectArgument());
  } else if (auto DRE = dyn_cast<DeclRefExpr>(E)) {
    return getNullabilityAnnotationsFromType(DRE->getType());
  }
  // TODO: Handle other expression shapes.
  return std::vector<NullabilityKind>();
}

QualType getBaseType(const Expr* E) {
  if (auto ME = dyn_cast<MemberExpr>(E)) {
    return getBaseType(ME->getBase());
  } else if (auto MC = dyn_cast<CXXMemberCallExpr>(E)) {
    return getBaseType(MC->getImplicitObjectArgument());
  } else if (auto DRE = dyn_cast<DeclRefExpr>(E)) {
    return DRE->getType();
  }
  // TODO: Handle other expression shapes and base types.
  else {
    llvm::dbgs() << "\nWe cannot get this base type yet...\n";
  }
  return QualType();
}

/// Given an expression E that refers to a member variable or a member function
/// of a template specialization, construct the nullability vector
/// of its base type and use it to compute the nullability of E. E's nullability
/// will itself be a vector; this is to account for cases in which E is
/// composed of more than one pointer. We return the first element of E's
/// nullability vector (i.e., E's "outer" nullability).
NullabilityKind getNullabilityFromTemplatedExpression(const Expr* E) {
  std::vector<NullabilityKind> BaseNullabilityAnnotations =
      getBaseNullabilityAnnotations(E);
  QualType BaseType = getBaseType(E);
  std::vector<NullabilityKind> NullabilityAnnotations =
      substituteNullabilityAnnotationsInTemplate(
          E->getType(), BaseNullabilityAnnotations, BaseType);
  if (NullabilityAnnotations.empty()) {
    return NullabilityKind::Unspecified;
  }
  return NullabilityAnnotations[0];
}

NullabilityKind getPointerNullability(const Expr* E, ASTContext& Ctx) {
  QualType ExprType = E->getType();
  NullabilityKind Nullability =
      ExprType->getNullability(Ctx).value_or(NullabilityKind::Unspecified);
  if (Nullability == NullabilityKind::Unspecified) {
    // If the type does not contain nullability information, try to gather it
    // from the expression itself.
    Nullability = getNullabilityFromTemplatedExpression(E);
  }
  return Nullability;
}

void initPointerFromAnnotations(PointerValue& PointerVal, const Expr* E,
                                Environment& Env, ASTContext& Ctx) {
  NullabilityKind Nullability = getPointerNullability(E, Ctx);
  switch (Nullability) {
    case NullabilityKind::NonNull:
      initNotNullPointer(PointerVal, Env);
      break;
    case NullabilityKind::Nullable:
      initNullablePointer(PointerVal, Env);
      break;
    default:
      initUnknownPointer(PointerVal, Env);
  }
}

void transferNullPointer(const Expr* NullPointer,
                         const MatchFinder::MatchResult&,
                         TransferState<NoopLattice>& State) {
  if (auto* PointerVal = getPointerValueFromExpr(NullPointer, State.Env)) {
    initNullPointer(*PointerVal, State.Env);
  }
}

void transferNotNullPointer(const Expr* NotNullPointer,
                            const MatchFinder::MatchResult&,
                            TransferState<NoopLattice>& State) {
  if (auto* PointerVal = getPointerValueFromExpr(NotNullPointer, State.Env)) {
    initNotNullPointer(*PointerVal, State.Env);
  }
}

void transferPointer(const Expr* PointerExpr,
                     const MatchFinder::MatchResult& Result,
                     TransferState<NoopLattice>& State) {
  if (auto* PointerVal = getPointerValueFromExpr(PointerExpr, State.Env)) {
    initPointerFromAnnotations(*PointerVal, PointerExpr, State.Env,
                               *Result.Context);
  }
}

// TODO(b/233582219): Implement promotion of nullability knownness for initially
// unknown pointers when there is evidence that it is nullable, for example
// when the pointer is compared to nullptr, or casted to boolean.
void transferNullCheckComparison(const BinaryOperator* BinaryOp,
                                 const MatchFinder::MatchResult& result,
                                 TransferState<NoopLattice>& State) {
  // Boolean representing the comparison between the two pointer values,
  // automatically created by the dataflow framework.
  auto& PointerComparison =
      *cast<BoolValue>(State.Env.getValue(*BinaryOp, SkipPast::None));

  CHECK(BinaryOp->getOpcode() == BO_EQ || BinaryOp->getOpcode() == BO_NE);
  auto& PointerEQ = BinaryOp->getOpcode() == BO_EQ
                        ? PointerComparison
                        : State.Env.makeNot(PointerComparison);
  auto& PointerNE = BinaryOp->getOpcode() == BO_EQ
                        ? State.Env.makeNot(PointerComparison)
                        : PointerComparison;

  auto* LHS = getPointerValueFromExpr(BinaryOp->getLHS(), State.Env);
  auto* RHS = getPointerValueFromExpr(BinaryOp->getRHS(), State.Env);

  if (!LHS || !RHS) return;

  auto [LHSKnown, LHSNull] = getPointerNullState(*LHS, State.Env);
  auto [RHSKnown, RHSNull] = getPointerNullState(*RHS, State.Env);
  auto& LHSKnownNotNull =
      State.Env.makeAnd(LHSKnown, State.Env.makeNot(LHSNull));
  auto& RHSKnownNotNull =
      State.Env.makeAnd(RHSKnown, State.Env.makeNot(RHSNull));
  auto& LHSKnownNull = State.Env.makeAnd(LHSKnown, LHSNull);
  auto& RHSKnownNull = State.Env.makeAnd(RHSKnown, RHSNull);

  // nullptr == nullptr
  State.Env.addToFlowCondition(State.Env.makeImplication(
      State.Env.makeAnd(LHSKnownNull, RHSKnownNull), PointerEQ));
  // nullptr != notnull
  State.Env.addToFlowCondition(State.Env.makeImplication(
      State.Env.makeAnd(LHSKnownNull, RHSKnownNotNull), PointerNE));
  // notnull != nullptr
  State.Env.addToFlowCondition(State.Env.makeImplication(
      State.Env.makeAnd(LHSKnownNotNull, RHSKnownNull), PointerNE));
}

void transferNullCheckImplicitCastPtrToBool(const Expr* CastExpr,
                                            const MatchFinder::MatchResult&,
                                            TransferState<NoopLattice>& State) {
  auto* PointerVal =
      getPointerValueFromExpr(CastExpr->IgnoreImplicit(), State.Env);
  if (!PointerVal) return;

  auto [PointerKnown, PointerNull] =
      getPointerNullState(*PointerVal, State.Env);
  auto& CastExprLoc = State.Env.createStorageLocation(*CastExpr);
  State.Env.setValue(CastExprLoc, State.Env.makeNot(PointerNull));
  State.Env.setStorageLocation(*CastExpr, CastExprLoc);
}

void transferCallExpr(const CallExpr* CallExpr,
                      const MatchFinder::MatchResult& Result,
                      TransferState<NoopLattice>& State) {
  auto ReturnType = CallExpr->getType();
  if (!ReturnType->isAnyPointerType()) return;

  auto* PointerVal = getPointerValueFromExpr(CallExpr, State.Env);
  if (!PointerVal) {
    PointerVal = cast<PointerValue>(State.Env.createValue(ReturnType));
    auto& CallExprLoc = State.Env.createStorageLocation(*CallExpr);
    State.Env.setValue(CallExprLoc, *PointerVal);
    State.Env.setStorageLocation(*CallExpr, CallExprLoc);
  }
  initPointerFromAnnotations(*PointerVal, CallExpr, State.Env, *Result.Context);
}

auto buildTransferer() {
  return CFGMatchSwitchBuilder<TransferState<NoopLattice>>()
      // Handles initialization of the null states of pointers.
      .CaseOfCFGStmt<Expr>(isPointerVariableReference(), transferPointer)
      .CaseOfCFGStmt<Expr>(isCXXThisExpr(), transferNotNullPointer)
      .CaseOfCFGStmt<Expr>(isAddrOf(), transferNotNullPointer)
      .CaseOfCFGStmt<Expr>(isNullPointerLiteral(), transferNullPointer)
      .CaseOfCFGStmt<MemberExpr>(isMemberOfPointerType(), transferPointer)
      .CaseOfCFGStmt<CallExpr>(isCallExpr(), transferCallExpr)
      // Handles comparison between 2 pointers.
      .CaseOfCFGStmt<BinaryOperator>(isPointerCheckBinOp(),
                                     transferNullCheckComparison)
      // Handles checking of pointer as boolean.
      .CaseOfCFGStmt<Expr>(isImplicitCastPointerToBool(),
                           transferNullCheckImplicitCastPtrToBool)
      .Build();
}
}  // namespace

PointerNullabilityAnalysis::PointerNullabilityAnalysis(ASTContext& Context)
    : DataflowAnalysis<PointerNullabilityAnalysis, NoopLattice>(Context),
      Transferer(buildTransferer()) {}

void PointerNullabilityAnalysis::transfer(const CFGElement* Elt,
                                          NoopLattice& Lattice,
                                          Environment& Env) {
  TransferState<NoopLattice> State(Lattice, Env);
  Transferer(*Elt, getASTContext(), State);
}

BoolValue& mergeBoolValues(BoolValue& Bool1, const Environment& Env1,
                           BoolValue& Bool2, const Environment& Env2,
                           Environment& MergedEnv) {
  if (&Bool1 == &Bool2) {
    return Bool1;
  }

  auto& MergedBool = MergedEnv.makeAtomicBoolValue();

  // If `Bool1` and `Bool2` is constrained to the same true / false value,
  // `MergedBool` can be constrained similarly without needing to consider the
  // path taken - this simplifies the flow condition tracked in `MergedEnv`.
  // Otherwise, information about which path was taken is used to associate
  // `MergedBool` with `Bool1` and `Bool2`.
  if (Env1.flowConditionImplies(Bool1) && Env2.flowConditionImplies(Bool2)) {
    MergedEnv.addToFlowCondition(MergedBool);
  } else if (Env1.flowConditionImplies(Env1.makeNot(Bool1)) &&
             Env2.flowConditionImplies(Env2.makeNot(Bool2))) {
    MergedEnv.addToFlowCondition(MergedEnv.makeNot(MergedBool));
  } else {
    // TODO(b/233582219): Flow conditions are not necessarily mutually
    // exclusive, a fix is in order: https://reviews.llvm.org/D130270. Update
    // this section when the patch is commited.
    auto& FC1 = Env1.getFlowConditionToken();
    auto& FC2 = Env2.getFlowConditionToken();
    MergedEnv.addToFlowCondition(MergedEnv.makeOr(
        MergedEnv.makeAnd(FC1, MergedEnv.makeIff(MergedBool, Bool1)),
        MergedEnv.makeAnd(FC2, MergedEnv.makeIff(MergedBool, Bool2))));
  }
  return MergedBool;
}

bool PointerNullabilityAnalysis::merge(QualType Type, const Value& Val1,
                                       const Environment& Env1,
                                       const Value& Val2,
                                       const Environment& Env2,
                                       Value& MergedVal,
                                       Environment& MergedEnv) {
  if (!Type->isAnyPointerType()) {
    return false;
  }

  auto [Known1, Null1] = getPointerNullState(cast<PointerValue>(Val1), Env1);
  auto [Known2, Null2] = getPointerNullState(cast<PointerValue>(Val2), Env2);

  auto& Known = mergeBoolValues(Known1, Env1, Known2, Env2, MergedEnv);
  auto& Null = mergeBoolValues(Null1, Env1, Null2, Env2, MergedEnv);

  initPointerNullState(cast<PointerValue>(MergedVal), MergedEnv, &Known, &Null);

  return true;
}
}  // namespace nullability
}  // namespace tidy
}  // namespace clang
