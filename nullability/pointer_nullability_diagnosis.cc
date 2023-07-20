// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/pointer_nullability_diagnosis.h"

#include "absl/log/check.h"
#include "nullability/pointer_nullability.h"
#include "nullability/pointer_nullability_matchers.h"
#include "nullability/type_nullability.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/Expr.h"
#include "clang/AST/ExprCXX.h"
#include "clang/AST/Stmt.h"
#include "clang/AST/Type.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/Analysis/CFG.h"
#include "clang/Analysis/FlowSensitive/CFGMatchSwitch.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Basic/Specifiers.h"
#include "llvm/ADT/SmallVector.h"
#include "llvm/Support/Debug.h"

#define DEBUG_TYPE "nullability-diagnostic"

namespace clang::tidy::nullability {

using ast_matchers::MatchFinder;
using dataflow::CFGMatchSwitchBuilder;
using dataflow::Environment;
using dataflow::TransferStateForDiagnostics;
using ::llvm::SmallVector;

namespace {

// Diagnoses whether `E` violates the expectation that it is nonnull.
SmallVector<PointerNullabilityDiagnostic> diagnoseNonnullExpected(
    const Expr *E, const Environment &Env) {
  if (auto *ActualVal = getPointerValueFromExpr(E, Env)) {
    if (isNullable(*ActualVal, Env))
      return {{PointerNullabilityDiagnostic::ErrorCode::ExpectedNonnull,
               CharSourceRange::getTokenRange(E->getSourceRange())}};
    return {};
  }

  LLVM_DEBUG({
    llvm::dbgs()
        << "The dataflow analysis framework does not model a PointerValue "
           "for "
           "the following Expr, and thus its dereference is marked as "
           "unsafe:\n";
    E->dump();
  });
  return {{PointerNullabilityDiagnostic::ErrorCode::Untracked,
           CharSourceRange::getTokenRange(E->getSourceRange())}};
}

// Diagnoses whether the nullability of `E` is incompatible with the expectation
// set by `DeclaredType`.
SmallVector<PointerNullabilityDiagnostic> diagnoseTypeExprCompatibility(
    QualType DeclaredType, const Expr *E, const Environment &Env,
    ASTContext &Ctx) {
  CHECK(DeclaredType->isAnyPointerType());
  return getNullabilityKind(DeclaredType, Ctx) == NullabilityKind::NonNull
             ? diagnoseNonnullExpected(E, Env)
             : SmallVector<PointerNullabilityDiagnostic>{};
}

SmallVector<PointerNullabilityDiagnostic> diagnoseDereference(
    const UnaryOperator *UnaryOp, const MatchFinder::MatchResult &,
    const TransferStateForDiagnostics<PointerNullabilityLattice> &State) {
  return diagnoseNonnullExpected(UnaryOp->getSubExpr(), State.Env);
}

SmallVector<PointerNullabilityDiagnostic> diagnoseArrow(
    const MemberExpr *MemberExpr, const MatchFinder::MatchResult &Result,
    const TransferStateForDiagnostics<PointerNullabilityLattice> &State) {
  return diagnoseNonnullExpected(MemberExpr->getBase(), State.Env);
}

// Diagnoses whether any of the arguments are incompatible with the
// corresponding type in the function prototype.
SmallVector<PointerNullabilityDiagnostic> diagnoseArgumentCompatibility(
    const FunctionProtoType &CalleeFPT, ArrayRef<const Expr *> Args,
    const Environment &Env, ASTContext &Ctx) {
  auto ParamTypes = CalleeFPT.getParamTypes();
  // C-style varargs cannot be annotated and therefore are unchecked.
  if (CalleeFPT.isVariadic()) {
    CHECK_GE(Args.size(), ParamTypes.size());
    Args = Args.take_front(ParamTypes.size());
  }
  CHECK_EQ(ParamTypes.size(), Args.size());
  SmallVector<PointerNullabilityDiagnostic> Diagnostics;
  for (unsigned int I = 0; I < Args.size(); ++I) {
    auto ParamType = ParamTypes[I].getNonReferenceType();
    if (ParamType->isAnyPointerType())
      Diagnostics.append(
          diagnoseTypeExprCompatibility(ParamType, Args[I], Env, Ctx));
  }
  return Diagnostics;
}

NullabilityKind parseNullabilityKind(StringRef EnumName) {
  return llvm::StringSwitch<NullabilityKind>(EnumName)
      .Case("NK_nonnull", NullabilityKind::NonNull)
      .Case("NK_nullable", NullabilityKind::Nullable)
      .Case("NK_unspecified", NullabilityKind::Unspecified)
      .Default(NullabilityKind::Unspecified);
}

/// Evaluates the `__assert_nullability` call by comparing the expected
/// nullability to the nullability computed by the dataflow analysis.
///
/// If the function being diagnosed is called `__assert_nullability`, we assume
/// it is a call of the shape __assert_nullability<a, b, c, ...>(p), where `p`
/// is an expression that contains pointers and a, b, c ... represent each of
/// the NullabilityKinds in `p`'s expected nullability. An expression's
/// nullability can be expressed as a vector of NullabilityKinds, where each
/// vector element corresponds to one of the pointers contained in the
/// expression.
///
/// For example:
/// \code
///    enum NullabilityKind {
///      NK_nonnull,
///      NK_nullable,
///      NK_unspecified,
///    };
///
///    template<NullabilityKind ...NK, typename T>
///    void __assert_nullability(T&);
///
///    template<typename T0, typename T1>
///    struct Struct2Arg {
///      T0 arg0;
///      T1 arg1;
///    };
///
///    void target(Struct2Arg<int *, int * _Nullable> p) {
///      __assert_nullability<NK_unspecified, NK_nullable>(p);
///    }
/// \endcode
SmallVector<PointerNullabilityDiagnostic> diagnoseAssertNullabilityCall(
    const CallExpr *CE,
    const TransferStateForDiagnostics<PointerNullabilityLattice> &State,
    ASTContext &Ctx) {
  auto *DRE = cast<DeclRefExpr>(CE->getCallee()->IgnoreImpCasts());

  // Extract the expected nullability from the template parameter pack.
  TypeNullability Expected;
  for (auto P : DRE->template_arguments()) {
    if (P.getArgument().getKind() == TemplateArgument::Expression) {
      if (auto *EnumDRE = dyn_cast<DeclRefExpr>(P.getSourceExpression())) {
        Expected.push_back(parseNullabilityKind(EnumDRE->getDecl()->getName()));
      }
    }
  }

  // Compare the nullability computed by nullability analysis with the
  // expected one.
  const Expr *GivenExpr = CE->getArg(0);
  const TypeNullability *MaybeComputed =
      State.Lattice.getExprNullability(GivenExpr);
  if (MaybeComputed == nullptr)
    return {{PointerNullabilityDiagnostic::ErrorCode::Untracked,
             CharSourceRange::getTokenRange(CE->getSourceRange())}};

  if (*MaybeComputed == Expected) return {};

  LLVM_DEBUG({
    // The computed and expected nullabilities differ. Print both to aid
    // debugging.
    llvm::dbgs() << "__assert_nullability failed at location: ";
    CE->getExprLoc().print(llvm::dbgs(), Ctx.getSourceManager());
    llvm::dbgs() << "\nExpression:\n";
    GivenExpr->dump();
    llvm::dbgs() << "Expected nullability: ";
    llvm::dbgs() << nullabilityToString(Expected) << "\n";
    llvm::dbgs() << "Computed nullability: ";
    llvm::dbgs() << nullabilityToString(*MaybeComputed) << "\n";
  });

  return {{PointerNullabilityDiagnostic::ErrorCode::AssertFailed,
           CharSourceRange::getTokenRange(CE->getSourceRange())}};
}

SmallVector<PointerNullabilityDiagnostic> diagnoseCallExpr(
    const CallExpr *CE, const MatchFinder::MatchResult &Result,
    const TransferStateForDiagnostics<PointerNullabilityLattice> &State) {
  // Check whether the callee is null.
  // - Skip direct callees to avoid handling builtin functions, which don't
  //   decay to pointer.
  // - Skip member callees, as they are not pointers at all (rather "bound
  //   member function type").
  //   Note that in `(obj.*nullable_pmf)()` the deref is *before* the call.
  if (!CE->getDirectCallee() && !isa<CXXMemberCallExpr>(CE)) {
    auto D = diagnoseNonnullExpected(CE->getCallee(), State.Env);
    if (!D.empty()) return D;
  }

  if (auto *FD = CE->getDirectCallee())
    if (FD->getDeclName().isIdentifier() &&
        FD->getName() == "__assert_nullability") {
      return diagnoseAssertNullabilityCall(CE, State, *Result.Context);
    }

  auto *Callee = CE->getCalleeDecl();
  // TODO(mboehme): Retrieve the nullability directly from the callee using
  // `getNullabilityForChild(CE->getCallee())`, as what we have here now
  // doesn't work for callees that don't have a decl.
  if (!Callee) return {};

  auto *CalleeType = Callee->getFunctionType();
  if (!CalleeType) return {};

  // TODO(mboehme): We're only looking at the nullability spelled on the
  // `FunctionProtoType`, but there could be extra information in the callee.
  // An example (due to sammccall@):
  //
  // template <typename T> struct Sink {
  //   static void eat(T) { ... }
  // }
  // void target(Sink<Nonnull<int*>> &S) {
  //   S<Nonnull<int*>>::eat(nullptr); // no warning
  //   // callee is instantiated Sink<int*>::eat(int*)
  //   // however nullability vector of DRE S::eat should be [Nonnull]
  //   // (not sure if it is today)
  // }
  auto *CalleeFPT = CalleeType->getAs<FunctionProtoType>();
  if (!CalleeFPT) return {};

  ArrayRef<const Expr *> Args(CE->getArgs(), CE->getNumArgs());
  // The first argument of an member operator call expression is the implicit
  // object argument, which does not appear in the list of parameter types.
  // Note that operator calls always have a direct callee.
  if (isa<CXXOperatorCallExpr>(CE) &&
      isa<CXXMethodDecl>(CE->getDirectCallee())) {
    Args = Args.drop_front();
  }
  return diagnoseArgumentCompatibility(*CalleeFPT, Args, State.Env,
                                       *Result.Context);
}

SmallVector<PointerNullabilityDiagnostic> diagnoseConstructExpr(
    const CXXConstructExpr *CE, const MatchFinder::MatchResult &Result,
    const TransferStateForDiagnostics<PointerNullabilityLattice> &State) {
  auto *CalleeFPT = CE->getConstructor()->getType()->getAs<FunctionProtoType>();
  if (!CalleeFPT) return {};
  ArrayRef<const Expr *> ConstructorArgs(CE->getArgs(), CE->getNumArgs());
  return diagnoseArgumentCompatibility(*CalleeFPT, ConstructorArgs, State.Env,
                                       *Result.Context);
}

SmallVector<PointerNullabilityDiagnostic> diagnoseReturn(
    const ReturnStmt *RS, const MatchFinder::MatchResult &Result,
    const TransferStateForDiagnostics<PointerNullabilityLattice> &State) {
  auto ReturnType = cast<FunctionDecl>(State.Env.getDeclCtx())->getReturnType();

  // TODO: Handle non-pointer return types.
  if (!ReturnType->isPointerType()) {
    return {};
  }

  auto *ReturnExpr = RS->getRetValue();
  CHECK(ReturnExpr->getType()->isPointerType());

  return diagnoseTypeExprCompatibility(ReturnType, ReturnExpr, State.Env,
                                       *Result.Context);
}

SmallVector<PointerNullabilityDiagnostic> diagnoseMemberInitializer(
    const CXXCtorInitializer *CI, const MatchFinder::MatchResult &Result,
    const TransferStateForDiagnostics<PointerNullabilityLattice> &State) {
  CHECK(CI->isAnyMemberInitializer());
  auto MemberType = CI->getAnyMember()->getType();
  if (!MemberType->isAnyPointerType()) return {};

  auto *MemberInitExpr = CI->getInit();
  return diagnoseTypeExprCompatibility(MemberType, MemberInitExpr, State.Env,
                                       *Result.Context);
}

}  // namespace

PointerNullabilityDiagnoser pointerNullabilityDiagnoser() {
  return CFGMatchSwitchBuilder<const dataflow::TransferStateForDiagnostics<
                                   PointerNullabilityLattice>,
                               SmallVector<PointerNullabilityDiagnostic>>()
      // (*)
      .CaseOfCFGStmt<UnaryOperator>(isPointerDereference(), diagnoseDereference)
      // (->)
      .CaseOfCFGStmt<MemberExpr>(isPointerArrow(), diagnoseArrow)
      // Check compatibility of parameter assignments
      .CaseOfCFGStmt<CallExpr>(isCallExpr(), diagnoseCallExpr)
      .CaseOfCFGStmt<ReturnStmt>(isPointerReturn(), diagnoseReturn)
      .CaseOfCFGStmt<CXXConstructExpr>(isConstructExpr(), diagnoseConstructExpr)
      .CaseOfCFGInit<CXXCtorInitializer>(isCtorMemberInitializer(),
                                         diagnoseMemberInitializer)
      .Build();
}

}  // namespace clang::tidy::nullability
