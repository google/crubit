// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability_verification/pointer_nullability_analysis.h"

#include <string>

#include "absl/log/check.h"
#include "nullability_verification/pointer_nullability.h"
#include "nullability_verification/pointer_nullability_lattice.h"
#include "nullability_verification/pointer_nullability_matchers.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Expr.h"
#include "clang/AST/OperationKinds.h"
#include "clang/AST/Stmt.h"
#include "clang/AST/Type.h"
#include "clang/AST/TypeVisitor.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/Analysis/FlowSensitive/CFGMatchSwitch.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
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
using dataflow::PointerValue;
using dataflow::SkipPast;
using dataflow::TransferState;
using dataflow::Value;

namespace {

class GetNullabilityAnnotationsFromTypeVisitor
    : public TypeVisitor<GetNullabilityAnnotationsFromTypeVisitor> {
  std::vector<NullabilityKind> NullabilityAnnotations;

 public:
  std::vector<NullabilityKind> getNullabilityAnnotations() && {
    return std::move(NullabilityAnnotations);
  }

  void Visit(QualType T) { TypeVisitor::Visit(T.getTypePtr()); }

  void VisitElaboratedType(const ElaboratedType* ET) {
    Visit(ET->getNamedType());
  }

  void VisitTemplateSpecializationType(const TemplateSpecializationType* TST) {
    for (auto TA : TST->template_arguments()) {
      if (TA.getKind() == TemplateArgument::Type) {
        Visit(TA.getAsType());
      }
    }
  }

  void VisitAttributedType(const AttributedType* AT) {
    Optional<NullabilityKind> NK = AT->getImmediateNullability();
    if (NK.has_value()) {
      NullabilityAnnotations.push_back(AT->getImmediateNullability().value());
      QualType MT = AT->getModifiedType();
      if (auto PT = MT->getAs<PointerType>()) {
        Visit(PT->getPointeeType());
      } else {
        // TODO: Handle this unusual yet possible (e.g. through typedefs)
        // case.
        llvm::dbgs() << "\nThe type " << AT
                     << "contains a nullability annotation that is not "
                     << "succeeded by a pointer type. "
                     << "This occurence is not currently handled.\n";
      }
    } else {
      Visit(AT->getModifiedType());
    }
  }

  void VisitPointerType(const PointerType* PT) {
    NullabilityAnnotations.push_back(NullabilityKind::Unspecified);
    Visit(PT->getPointeeType());
  }
};

/// Traverse over a type to get its nullability. For example, if T is the type
/// Struct3Arg<int * _Nonnull, int, pair<int * _Nullable, int *>> * _Nonnull,
/// the resulting nullability annotations will be {_Nonnull, _Nonnull,
/// _Nullable, _Unknown}. Note that non-pointer elements (e.g., the second
/// argument of Struct3Arg) do not get a nullability annotation.
std::vector<NullabilityKind> getNullabilityAnnotationsFromType(QualType T) {
  GetNullabilityAnnotationsFromTypeVisitor AnnotationVisitor;
  AnnotationVisitor.Visit(T);
  return std::move(AnnotationVisitor).getNullabilityAnnotations();
}

class CountPointersInTypeVisitor
    : public TypeVisitor<CountPointersInTypeVisitor> {
  unsigned count = 0;

 public:
  CountPointersInTypeVisitor() {}

  unsigned getCount() { return count; }

  void Visit(QualType T) { TypeVisitor::Visit(T.getTypePtrOrNull()); }

  void VisitElaboratedType(const ElaboratedType* ET) {
    Visit(ET->getNamedType());
  }

  void VisitAttributedType(const AttributedType* AT) {
    Visit(AT->getModifiedType());
  }

  void VisitPointerType(const PointerType* PT) {
    count += 1;
    Visit(PT->getPointeeType());
  }

  void Visit(TemplateArgument TA) {
    if (TA.getKind() == TemplateArgument::Type) {
      Visit(TA.getAsType());
    }
  }

  void VisitRecordType(const RecordType* RT) {
    if (auto* CTSD = dyn_cast<ClassTemplateSpecializationDecl>(RT->getDecl())) {
      for (auto& TA : CTSD->getTemplateArgs().asArray()) {
        Visit(TA);
      }
    }
  }

  void VisitTemplateSpecializationType(const TemplateSpecializationType* TST) {
    Visit(TST->desugar());
  }
};

unsigned countPointersInType(QualType T) {
  CountPointersInTypeVisitor PointerCountVisitor;
  PointerCountVisitor.Visit(T);
  return PointerCountVisitor.getCount();
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
  if (auto RT = BaseType->getAs<RecordType>()) {
    if (auto CTSD = dyn_cast<ClassTemplateSpecializationDecl>(RT->getDecl())) {
      auto TemplateArgs = CTSD->getTemplateArgs().asArray();
      for (auto TA : TemplateArgs.take_front(ArgIndex)) {
        PointerCount += countPointersInType(TA);
      }
      // TODO: Correctly handle the indexing of nested templates (e.g.
      // PointerNullabilityTest.MemberFunctionTemplateOfTemplateStruct), then
      // remove this fallback.
      if (TemplateArgs.size() <= ArgIndex) {
        return {};
      }
      unsigned SliceSize = countPointersInType(TemplateArgs[ArgIndex]);
      if (BaseNullabilityAnnotations.size() < PointerCount + SliceSize) {
        // TODO: Currently, BaseNullabilityAnnotations can be erroneously empty
        // due to lack of expression coverage. Use the dataflow lattice to
        // retrieve correct base type annotations. Then, remove this fallback.
        return {};
      } else {
        return BaseNullabilityAnnotations.slice(PointerCount, SliceSize);
      }
    }
  }
  return ArrayRef<NullabilityKind>();
}

class SubstituteNullabilityAnnotationsInTemplateVisitor
    : public TypeVisitor<SubstituteNullabilityAnnotationsInTemplateVisitor> {
  QualType BaseType;
  ArrayRef<NullabilityKind> BaseNullabilityAnnotations;
  std::vector<NullabilityKind> NullabilityAnnotations;

 public:
  SubstituteNullabilityAnnotationsInTemplateVisitor(
      QualType BaseType, ArrayRef<NullabilityKind> BaseNullabilityAnnotations)
      : BaseType(BaseType),
        BaseNullabilityAnnotations(BaseNullabilityAnnotations) {}

  std::vector<NullabilityKind> getNullabilityAnnotations() && {
    return std::move(NullabilityAnnotations);
  }

  void Visit(QualType T) { TypeVisitor::Visit(T.getTypePtr()); }

  void VisitSubstTemplateTypeParmType(const SubstTemplateTypeParmType* ST) {
    for (auto NK : getNullabilityForTemplateParameter(
             ST, BaseNullabilityAnnotations, BaseType)) {
      NullabilityAnnotations.push_back(NK);
    }
  }

  void VisitPointerType(const PointerType* PT) {
    NullabilityAnnotations.push_back(NullabilityKind::Unspecified);
    Visit(PT->getPointeeType());
  }

  void VisitElaboratedType(const ElaboratedType* ET) {
    Visit(ET->getNamedType());
  }

  void VisitTemplateSpecializationType(const TemplateSpecializationType* TST) {
    for (auto TA : TST->template_arguments()) {
      if (TA.getKind() == TemplateArgument::Type) {
        Visit(TA.getAsType());
      }
    }
  }
};

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
  SubstituteNullabilityAnnotationsInTemplateVisitor AnnotationVisitor(
      BaseType, BaseNullabilityAnnotations);
  AnnotationVisitor.Visit(T);
  return std::move(AnnotationVisitor).getNullabilityAnnotations();
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
    return ME->getBase()->getType();
  } else if (auto MC = dyn_cast<CXXMemberCallExpr>(E)) {
    return MC->getImplicitObjectArgument()->getType();
  } else if (auto ICE = dyn_cast<ImplicitCastExpr>(E)) {
    return ICE->getSubExpr()->getType();
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
  if (BaseType.isNull()) {
    return NullabilityKind::Unspecified;
  }
  std::vector<NullabilityKind> NullabilityAnnotations =
      substituteNullabilityAnnotationsInTemplate(
          E->getType(), BaseNullabilityAnnotations, BaseType);
  if (NullabilityAnnotations.empty()) {
    return NullabilityKind::Unspecified;
  }
  return NullabilityAnnotations[0];
}

NullabilityKind getPointerNullability(const Expr* E,
                                      PointerNullabilityAnalysis::Lattice& L) {
  QualType ExprType = E->getType();
  Optional<NullabilityKind> Nullability = ExprType->getNullability();

  // If the expression's type does not contain nullability information, it may
  // be a template instantiation. Look up the nullability in the
  // `ExprToNullability` map.
  if (Nullability.value_or(NullabilityKind::Unspecified) ==
      NullabilityKind::Unspecified) {
    if (auto MaybeNullability = L.getExprNullability(E)) {
      if (!MaybeNullability->empty()) {
        // Return the nullability of the topmost pointer in the type.
        Nullability = (*MaybeNullability)[0];
      }
    }
  }
  // TODO: Expand the dataflow analysis algorithm to propagate the nullability
  // of more expression shapes (e.g., method calls), and then delete
  // getNullabilityFromTemplatedExpression.
  if (!Nullability.has_value()) {
    Nullability = getNullabilityFromTemplatedExpression(E);
  }
  return Nullability.value_or(NullabilityKind::Unspecified);
}

void initPointerFromAnnotations(
    PointerValue& PointerVal, const Expr* E,
    TransferState<PointerNullabilityLattice>& State) {
  NullabilityKind Nullability = getPointerNullability(E, State.Lattice);
  switch (Nullability) {
    case NullabilityKind::NonNull:
      initNotNullPointer(PointerVal, State.Env);
      break;
    case NullabilityKind::Nullable:
      initNullablePointer(PointerVal, State.Env);
      break;
    default:
      initUnknownPointer(PointerVal, State.Env);
  }
}

void transferFlowSensitiveNullPointer(
    const Expr* NullPointer, const MatchFinder::MatchResult&,
    TransferState<PointerNullabilityLattice>& State) {
  if (auto* PointerVal = getPointerValueFromExpr(NullPointer, State.Env)) {
    initNullPointer(*PointerVal, State.Env);
  }
}

void transferFlowSensitiveNotNullPointer(
    const Expr* NotNullPointer, const MatchFinder::MatchResult&,
    TransferState<PointerNullabilityLattice>& State) {
  if (auto* PointerVal = getPointerValueFromExpr(NotNullPointer, State.Env)) {
    initNotNullPointer(*PointerVal, State.Env);
  }
}

void transferFlowSensitivePointer(
    const Expr* PointerExpr, const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  if (auto* PointerVal = getPointerValueFromExpr(PointerExpr, State.Env)) {
    initPointerFromAnnotations(*PointerVal, PointerExpr, State);
  }
}

// TODO(b/233582219): Implement promotion of nullability knownness for initially
// unknown pointers when there is evidence that it is nullable, for example
// when the pointer is compared to nullptr, or casted to boolean.
void transferFlowSensitiveNullCheckComparison(
    const BinaryOperator* BinaryOp, const MatchFinder::MatchResult& result,
    TransferState<PointerNullabilityLattice>& State) {
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

void transferFlowSensitiveNullCheckImplicitCastPtrToBool(
    const Expr* CastExpr, const MatchFinder::MatchResult&,
    TransferState<PointerNullabilityLattice>& State) {
  auto* PointerVal =
      getPointerValueFromExpr(CastExpr->IgnoreImplicit(), State.Env);
  if (!PointerVal) return;

  auto [PointerKnown, PointerNull] =
      getPointerNullState(*PointerVal, State.Env);
  auto& CastExprLoc = State.Env.createStorageLocation(*CastExpr);
  State.Env.setValue(CastExprLoc, State.Env.makeNot(PointerNull));
  State.Env.setStorageLocation(*CastExpr, CastExprLoc);
}

void transferFlowSensitiveCallExpr(
    const CallExpr* CallExpr, const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  auto ReturnType = CallExpr->getType();
  if (!ReturnType->isAnyPointerType()) return;

  auto* PointerVal = getPointerValueFromExpr(CallExpr, State.Env);
  if (!PointerVal) {
    PointerVal = cast<PointerValue>(State.Env.createValue(ReturnType));
    auto& CallExprLoc = State.Env.createStorageLocation(*CallExpr);
    State.Env.setValue(CallExprLoc, *PointerVal);
    State.Env.setStorageLocation(*CallExpr, CallExprLoc);
  }
  initPointerFromAnnotations(*PointerVal, CallExpr, State);
}

void transferNonFlowSensitiveDeclRefExpr(
    const DeclRefExpr* DRE, const MatchFinder::MatchResult& MR,
    TransferState<PointerNullabilityLattice>& State) {
  State.Lattice.insertExprNullabilityIfAbsent(
      DRE, [&]() { return getNullabilityAnnotationsFromType(DRE->getType()); });
  if (DRE->getType()->isPointerType()) {
    transferFlowSensitivePointer(DRE, MR, State);
  }
}

void transferNonFlowSensitiveMemberExpr(
    const MemberExpr* ME, const MatchFinder::MatchResult& MR,
    TransferState<PointerNullabilityLattice>& State) {
  State.Lattice.insertExprNullabilityIfAbsent(ME, [&]() {
    auto BaseNullability = State.Lattice.getExprNullability(ME->getBase());
    if (BaseNullability.has_value()) {
      return substituteNullabilityAnnotationsInTemplate(
          ME->getType(), *BaseNullability, ME->getBase()->getType());
    } else {
      // Since we process child nodes before parents, we should already have
      // computed the base (child) nullability. However, this is not true in all
      // test cases. So, we return unspecified nullability annotations.
      // TODO: Fix this issue, add ._has_value() as a CHECK statement and remove
      // else branch.
      return std::vector<NullabilityKind>(countPointersInType(ME->getType()),
                                          NullabilityKind::Unspecified);
    }
  });
  if (ME->getType()->isPointerType()) {
    transferFlowSensitivePointer(ME, MR, State);
  }
}

auto buildNonFlowSensitiveTransferer() {
  return CFGMatchSwitchBuilder<TransferState<PointerNullabilityLattice>>()
      .CaseOfCFGStmt<DeclRefExpr>(ast_matchers::declRefExpr(),
                                  transferNonFlowSensitiveDeclRefExpr)
      .CaseOfCFGStmt<MemberExpr>(ast_matchers::memberExpr(),
                                 transferNonFlowSensitiveMemberExpr)
      .Build();
}

auto buildFlowSensitiveTransferer() {
  return CFGMatchSwitchBuilder<TransferState<PointerNullabilityLattice>>()
      // Handles initialization of the null states of pointers.
      .CaseOfCFGStmt<Expr>(isCXXThisExpr(), transferFlowSensitiveNotNullPointer)
      .CaseOfCFGStmt<Expr>(isAddrOf(), transferFlowSensitiveNotNullPointer)
      .CaseOfCFGStmt<Expr>(isNullPointerLiteral(),
                           transferFlowSensitiveNullPointer)
      .CaseOfCFGStmt<CallExpr>(isCallExpr(), transferFlowSensitiveCallExpr)
      .CaseOfCFGStmt<Expr>(isPointerExpr(), transferFlowSensitivePointer)
      // Handles comparison between 2 pointers.
      .CaseOfCFGStmt<BinaryOperator>(isPointerCheckBinOp(),
                                     transferFlowSensitiveNullCheckComparison)
      // Handles checking of pointer as boolean.
      .CaseOfCFGStmt<Expr>(isImplicitCastPointerToBool(),
                           transferFlowSensitiveNullCheckImplicitCastPtrToBool)
      .Build();
}
}  // namespace

PointerNullabilityAnalysis::PointerNullabilityAnalysis(ASTContext& Context)
    : DataflowAnalysis<PointerNullabilityAnalysis, PointerNullabilityLattice>(
          Context),
      NonFlowSensitiveTransferer(buildNonFlowSensitiveTransferer()),
      FlowSensitiveTransferer(buildFlowSensitiveTransferer()) {}

void PointerNullabilityAnalysis::transfer(const CFGElement* Elt,
                                          PointerNullabilityLattice& Lattice,
                                          Environment& Env) {
  TransferState<PointerNullabilityLattice> State(Lattice, Env);
  NonFlowSensitiveTransferer(*Elt, getASTContext(), State);
  FlowSensitiveTransferer(*Elt, getASTContext(), State);
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
