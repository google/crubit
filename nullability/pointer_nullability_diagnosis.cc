// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/pointer_nullability_diagnosis.h"

#include <cstdint>
#include <optional>

#include "absl/base/nullability.h"
#include "absl/log/check.h"
#include "nullability/pointer_nullability.h"
#include "nullability/pointer_nullability_analysis.h"
#include "nullability/pointer_nullability_matchers.h"
#include "nullability/type_nullability.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/Expr.h"
#include "clang/AST/ExprCXX.h"
#include "clang/AST/Stmt.h"
#include "clang/AST/Type.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/ASTMatchers/ASTMatchers.h"
#include "clang/Analysis/CFG.h"
#include "clang/Analysis/FlowSensitive/CFGMatchSwitch.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysis.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/Value.h"
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
using dataflow::PointerValue;
using dataflow::TransferStateForDiagnostics;
using ::llvm::SmallVector;

namespace {

// Diagnoses whether `E` violates the expectation that it is nonnull.
SmallVector<PointerNullabilityDiagnostic> diagnoseNonnullExpected(
    absl::Nonnull<const Expr *> E, const Environment &Env,
    PointerNullabilityDiagnostic::Context DiagCtx,
    std::optional<std::string> ParamName = std::nullopt) {
  if (PointerValue *ActualVal = getPointerValue(E, Env)) {
    if (isNullable(*ActualVal, Env))
      return {{PointerNullabilityDiagnostic::ErrorCode::ExpectedNonnull,
               DiagCtx, CharSourceRange::getTokenRange(E->getSourceRange()),
               std::move(ParamName)}};
    return {};
  }

  LLVM_DEBUG({
    llvm::dbgs()
        << "The dataflow analysis framework does not model a PointerValue "
           "for the following Expr, and thus its dereference is marked as "
           "unsafe:\n";
    E->dump();
  });
  return {{PointerNullabilityDiagnostic::ErrorCode::Untracked, DiagCtx,
           CharSourceRange::getTokenRange(E->getSourceRange())}};
}

// Diagnoses whether the nullability of `E` is incompatible with the expectation
// set by `DeclaredType`.
SmallVector<PointerNullabilityDiagnostic> diagnoseTypeExprCompatibility(
    QualType DeclaredType, absl::Nonnull<const Expr *> E,
    const Environment &Env, ASTContext &Ctx,
    PointerNullabilityDiagnostic::Context DiagCtx,
    std::optional<std::string> ParamName = std::nullopt) {
  CHECK(isSupportedPointerType(DeclaredType));
  return getNullabilityAnnotationsFromType(DeclaredType).front().concrete() ==
                 NullabilityKind::NonNull
             ? diagnoseNonnullExpected(E, Env, DiagCtx, ParamName)
             : SmallVector<PointerNullabilityDiagnostic>{};
}

SmallVector<PointerNullabilityDiagnostic> diagnoseDereference(
    absl::Nonnull<const UnaryOperator *> UnaryOp,
    const MatchFinder::MatchResult &,
    const TransferStateForDiagnostics<PointerNullabilityLattice> &State) {
  return diagnoseNonnullExpected(
      UnaryOp->getSubExpr(), State.Env,
      PointerNullabilityDiagnostic::Context::NullableDereference);
}

SmallVector<PointerNullabilityDiagnostic> diagnoseSmartPointerDereference(
    absl::Nonnull<const CXXOperatorCallExpr *> Op,
    const MatchFinder::MatchResult &,
    const TransferStateForDiagnostics<PointerNullabilityLattice> &State) {
  return diagnoseNonnullExpected(
      Op->getArg(0), State.Env,
      PointerNullabilityDiagnostic::Context::NullableDereference);
}

SmallVector<PointerNullabilityDiagnostic> diagnoseSubscript(
    absl::Nonnull<const ArraySubscriptExpr *> Subscript,
    const MatchFinder::MatchResult &,
    const TransferStateForDiagnostics<PointerNullabilityLattice> &State) {
  return diagnoseNonnullExpected(
      Subscript->getBase(), State.Env,
      PointerNullabilityDiagnostic::Context::NullableDereference);
}

SmallVector<PointerNullabilityDiagnostic> diagnoseArrow(
    absl::Nonnull<const MemberExpr *> MemberExpr,
    const MatchFinder::MatchResult &Result,
    const TransferStateForDiagnostics<PointerNullabilityLattice> &State) {
  return diagnoseNonnullExpected(
      MemberExpr->getBase(), State.Env,
      PointerNullabilityDiagnostic::Context::NullableDereference);
}

// Diagnoses whether any of the arguments are incompatible with the
// corresponding type in the function prototype.
SmallVector<PointerNullabilityDiagnostic> diagnoseArgumentCompatibility(
    const FunctionProtoType &CalleeFPT, ArrayRef<const Expr *> Args,
    ArrayRef<const ParmVarDecl *> ParmDecls, const Environment &Env,
    ASTContext &Ctx) {
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
    if (isSupportedPointerType(ParamType)) {
      std::string ParamName = (I < ParmDecls.size())
                                  ? ParmDecls[I]->getDeclName().getAsString()
                                  : "";
      Diagnostics.append(diagnoseTypeExprCompatibility(
          ParamType, Args[I], Env, Ctx,
          PointerNullabilityDiagnostic::Context::FunctionArgument,
          std::move(ParamName)));
    }
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
    absl::Nonnull<const CallExpr *> CE,
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
             PointerNullabilityDiagnostic::Context::Other,
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
           PointerNullabilityDiagnostic::Context::Other,
           CharSourceRange::getTokenRange(CE->getSourceRange())}};
}

SmallVector<PointerNullabilityDiagnostic> diagnoseCallExpr(
    absl::Nonnull<const CallExpr *> CE, const MatchFinder::MatchResult &Result,
    const TransferStateForDiagnostics<PointerNullabilityLattice> &State) {
  // Check whether the callee is null.
  // - Skip direct callees to avoid handling builtin functions, which don't
  //   decay to pointer.
  // - Skip member callees, as they are not pointers at all (rather "bound
  //   member function type").
  //   Note that in `(obj.*nullable_pmf)()` the deref is *before* the call.
  if (!CE->getDirectCallee() &&
      !CE->getCallee()->hasPlaceholderType(BuiltinType::BoundMember)) {
    auto D =
        diagnoseNonnullExpected(CE->getCallee(), State.Env,
                                PointerNullabilityDiagnostic::Context::Other);
    if (!D.empty()) return D;
  }

  if (auto *FD = CE->getDirectCallee()) {
    if (FD->getDeclName().isIdentifier() &&
        FD->getName() == "__assert_nullability") {
      return diagnoseAssertNullabilityCall(CE, State, *Result.Context);
    }
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
  ArrayRef<const ParmVarDecl *> ParmDecls = {};
  if (Callee->getAsFunction())
    ParmDecls = Callee->getAsFunction()->parameters();
  return diagnoseArgumentCompatibility(*CalleeFPT, Args, ParmDecls, State.Env,
                                       *Result.Context);
}

SmallVector<PointerNullabilityDiagnostic> diagnoseConstructExpr(
    absl::Nonnull<const CXXConstructExpr *> CE,
    const MatchFinder::MatchResult &Result,
    const TransferStateForDiagnostics<PointerNullabilityLattice> &State) {
  auto *CalleeFPT = CE->getConstructor()->getType()->getAs<FunctionProtoType>();
  if (!CalleeFPT) return {};
  ArrayRef<const Expr *> ConstructorArgs(CE->getArgs(), CE->getNumArgs());

  return diagnoseArgumentCompatibility(
      *CalleeFPT, ConstructorArgs,
      CE->getConstructor()->getAsFunction()->parameters(), State.Env,
      *Result.Context);
}

SmallVector<PointerNullabilityDiagnostic> diagnoseReturn(
    absl::Nonnull<const ReturnStmt *> RS,
    const MatchFinder::MatchResult &Result,
    const TransferStateForDiagnostics<PointerNullabilityLattice> &State) {
  auto ReturnType = cast<FunctionDecl>(State.Env.getDeclCtx())->getReturnType();

  // TODO: Handle non-pointer return types.
  if (!isSupportedPointerType(ReturnType)) {
    return {};
  }

  auto *ReturnExpr = RS->getRetValue();
  CHECK(isSupportedPointerType(ReturnExpr->getType()));

  return diagnoseTypeExprCompatibility(
      ReturnType, ReturnExpr, State.Env, *Result.Context,
      PointerNullabilityDiagnostic::Context::ReturnValue);
}

SmallVector<PointerNullabilityDiagnostic> diagnoseMemberInitializer(
    absl::Nonnull<const CXXCtorInitializer *> CI,
    const MatchFinder::MatchResult &Result,
    const TransferStateForDiagnostics<PointerNullabilityLattice> &State) {
  CHECK(CI->isAnyMemberInitializer());
  auto MemberType = CI->getAnyMember()->getType();
  if (!isSupportedPointerType(MemberType)) return {};

  auto *MemberInitExpr = CI->getInit();
  return diagnoseTypeExprCompatibility(
      MemberType, MemberInitExpr, State.Env, *Result.Context,
      PointerNullabilityDiagnostic::Context::Initializer);
}

bool shouldDiagnoseExpectedNonnullDefaultArgValue(clang::ASTContext &Ctx,
                                                  const Expr &DefaultArg) {
  if (DefaultArg.isNullPointerConstant(Ctx,
                                       Expr::NPC_ValueDependentIsNotNull)) {
    return true;
  } else if (isSupportedPointerType(DefaultArg.getType()) &&
             !DefaultArg.getType()->isDependentType()) {
    if (TypeNullability DefaultValueAnnotation =
            getNullabilityAnnotationsFromType(DefaultArg.getType());
        !DefaultValueAnnotation.empty() &&
        DefaultValueAnnotation.front().concrete() ==
            NullabilityKind::Nullable) {
      return true;
    }
  }
  return false;
}

// Checks for simple cases of default arguments that conflict with annotations
// on the parameter declaration.
//
// Default argument values are missing from the CFG at callsites, so they can't
// be analyzed in the same way as other function arguments. And the
// PointerNullabilityDiagnoser is only run over the CFG (not the entire AST),
// which doesn't really include elements of function declarations, only their
// bodies. Therefore, these initializations must be checked separately to ensure
// diagnostics are produced exactly once per invalid default argument
// declaration, regardless of how many times the function is called (including
// not called at all).
void checkParmVarDeclWithPointerDefaultArg(
    clang::ASTContext &Ctx, const clang::ParmVarDecl &Parm,
    llvm::SmallVector<PointerNullabilityDiagnostic> &Diags) {
  QualType ParmType = Parm.getType();
  if (ParmType->isDependentType()) return;
  TypeNullability DeclAnnotation = getNullabilityAnnotationsFromType(ParmType);
  if (DeclAnnotation.empty() ||
      DeclAnnotation.front().concrete() != NullabilityKind::NonNull) {
    return;
  }

  const Expr *DefaultVal = Parm.getInit();
  if (!DefaultVal ||
      !shouldDiagnoseExpectedNonnullDefaultArgValue(Ctx, *DefaultVal))
    return;

  Diags.push_back({PointerNullabilityDiagnostic::ErrorCode::ExpectedNonnull,
                   PointerNullabilityDiagnostic::Context::Initializer,
                   CharSourceRange::getTokenRange(DefaultVal->getSourceRange()),
                   Parm.getNameAsString()});
}

auto pointerNullabilityDiagnoser() {
  return CFGMatchSwitchBuilder<const dataflow::TransferStateForDiagnostics<
                                   PointerNullabilityLattice>,
                               SmallVector<PointerNullabilityDiagnostic>>()
      // (*)
      .CaseOfCFGStmt<UnaryOperator>(isPointerDereference(), diagnoseDereference)
      .CaseOfCFGStmt<CXXOperatorCallExpr>(isSmartPointerOperatorCall("*"),
                                          diagnoseSmartPointerDereference)
      // ([])
      .CaseOfCFGStmt<ArraySubscriptExpr>(isPointerSubscript(),
                                         diagnoseSubscript)
      .CaseOfCFGStmt<CXXOperatorCallExpr>(isSmartPointerOperatorCall("[]"),
                                          diagnoseSmartPointerDereference)
      // (->)
      .CaseOfCFGStmt<MemberExpr>(isPointerArrow(), diagnoseArrow)
      .CaseOfCFGStmt<CXXOperatorCallExpr>(isSmartPointerOperatorCall("->"),
                                          diagnoseSmartPointerDereference)
      // Check compatibility of parameter assignments and return values.
      .CaseOfCFGStmt<CallExpr>(ast_matchers::callExpr(), diagnoseCallExpr)
      .CaseOfCFGStmt<CXXConstructExpr>(ast_matchers::cxxConstructExpr(),
                                       diagnoseConstructExpr)
      .CaseOfCFGStmt<ReturnStmt>(isPointerReturn(), diagnoseReturn)
      // Check compatibility of member initializers.
      .CaseOfCFGInit<CXXCtorInitializer>(isCtorMemberInitializer(),
                                         diagnoseMemberInitializer)
      .Build();
}

}  // namespace

llvm::Expected<llvm::SmallVector<PointerNullabilityDiagnostic>>
diagnosePointerNullability(const FunctionDecl *Func) {
  llvm::SmallVector<PointerNullabilityDiagnostic> Diags;

  if (Func->isTemplated()) return Diags;

  ASTContext &Ctx = Func->getASTContext();

  for (const ParmVarDecl *Parm : Func->parameters())
    checkParmVarDeclWithPointerDefaultArg(Ctx, *Parm, Diags);

  // Use `doesThisDeclarationHaveABody()` rather than `hasBody()` to ensure we
  // analyze forward-declared functions only once.
  if (!Func->doesThisDeclarationHaveABody()) return Diags;

  auto Diagnoser = pointerNullabilityDiagnoser();
  // These limits are set based on empirical observations. Mostly, they are a
  // rough proxy for a line between "finite" and "effectively infinite", rather
  // than strict limits on resource use.
  constexpr std::int64_t MaxSATIterations = 2'000'000;
  constexpr std::int32_t MaxBlockVisits = 20'000;

  if (auto CfgDiags = dataflow::diagnoseFunction<PointerNullabilityAnalysis,
                                                 PointerNullabilityDiagnostic>(
          *Func, Ctx, Diagnoser, MaxSATIterations, MaxBlockVisits)) {
    Diags.insert(Diags.end(), CfgDiags->begin(), CfgDiags->end());
    return Diags;
  } else {
    return CfgDiags.takeError();
  }
}

}  // namespace clang::tidy::nullability
