// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/pointer_nullability_diagnosis.h"

#include <cstdint>
#include <iterator>
#include <memory>
#include <optional>
#include <utility>

#include "absl/base/nullability.h"
#include "absl/log/check.h"
#include "nullability/pointer_nullability.h"
#include "nullability/pointer_nullability_analysis.h"
#include "nullability/pointer_nullability_lattice.h"
#include "nullability/pointer_nullability_matchers.h"
#include "nullability/pragma.h"
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
#include "clang/Analysis/FlowSensitive/AdornedCFG.h"
#include "clang/Analysis/FlowSensitive/CFGMatchSwitch.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysis.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysisContext.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/MatchSwitch.h"
#include "clang/Analysis/FlowSensitive/Solver.h"
#include "clang/Analysis/FlowSensitive/StorageLocation.h"
#include "clang/Analysis/FlowSensitive/Value.h"
#include "clang/Analysis/FlowSensitive/WatchedLiteralsSolver.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Basic/Specifiers.h"
#include "llvm/ADT/DenseSet.h"
#include "llvm/ADT/STLExtras.h"
#include "llvm/ADT/SmallVector.h"
#include "llvm/Support/Debug.h"
#include "llvm/Support/Errc.h"
#include "llvm/Support/Error.h"

#define DEBUG_TYPE "nullability-diagnostic"

namespace clang::tidy::nullability {

using ast_matchers::anyOf;
using ast_matchers::BoundNodes;
using ast_matchers::callExpr;
using ast_matchers::cxxConstructExpr;
using ast_matchers::cxxMemberCallExpr;
using ast_matchers::cxxOperatorCallExpr;
using ast_matchers::expr;
using ast_matchers::findAll;
using ast_matchers::hasArgument;
using ast_matchers::hasType;
using ast_matchers::match;
using ast_matchers::MatchFinder;
using ast_matchers::onImplicitObjectArgument;
using ast_matchers::unless;
using dataflow::CFGMatchSwitchBuilder;
using dataflow::Environment;
using dataflow::PointerValue;
using dataflow::RecordStorageLocation;
using ::llvm::SmallVector;

namespace {

using DiagTransferState =
    dataflow::TransferStateForDiagnostics<PointerNullabilityLattice>;
using DiagTransferFunc =
    dataflow::CFGMatchSwitch<const DiagTransferState,
                             SmallVector<PointerNullabilityDiagnostic>>;

SmallVector<PointerNullabilityDiagnostic> untrackedError(
    const Expr *E, PointerNullabilityDiagnostic::Context DiagCtx =
                       PointerNullabilityDiagnostic::Context::Other) {
  return {{PointerNullabilityDiagnostic::ErrorCode::Untracked, DiagCtx,
           CharSourceRange::getTokenRange(E->getSourceRange())}};
}

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
  return untrackedError(E, DiagCtx);
}

// Diagnoses a conceptual assignment of LHS = RHS.
// LHS can be a variable, the return value of a function, a param etc.
SmallVector<PointerNullabilityDiagnostic> diagnoseAssignmentLike(
    QualType LHSType, ArrayRef<PointerTypeNullability> LHSNullability,
    absl::Nonnull<const Expr *> RHS, const Environment &Env, ASTContext &Ctx,
    PointerNullabilityDiagnostic::Context DiagCtx,
    std::optional<std::string> ParamName = std::nullopt) {
  LHSType = LHSType.getNonReferenceType();
  // For now, we just check whether the top-level pointer type is compatible.
  // TODO: examine inner nullability too, considering variance.
  if (!isSupportedPointerType(LHSType)) return {};
  return LHSNullability.front().concrete() == NullabilityKind::NonNull
             ? diagnoseNonnullExpected(RHS, Env, DiagCtx, ParamName)
             : SmallVector<PointerNullabilityDiagnostic>{};
}

SmallVector<PointerNullabilityDiagnostic> diagnoseDereference(
    absl::Nonnull<const UnaryOperator *> UnaryOp,
    const MatchFinder::MatchResult &, const DiagTransferState &State) {
  return diagnoseNonnullExpected(
      UnaryOp->getSubExpr(), State.Env,
      PointerNullabilityDiagnostic::Context::NullableDereference);
}

SmallVector<PointerNullabilityDiagnostic> diagnoseSmartPointerDereference(
    absl::Nonnull<const CXXOperatorCallExpr *> Op,
    const MatchFinder::MatchResult &, const DiagTransferState &State) {
  return diagnoseNonnullExpected(
      Op->getArg(0), State.Env,
      PointerNullabilityDiagnostic::Context::NullableDereference);
}

SmallVector<PointerNullabilityDiagnostic> diagnoseSubscript(
    absl::Nonnull<const ArraySubscriptExpr *> Subscript,
    const MatchFinder::MatchResult &, const DiagTransferState &State) {
  return diagnoseNonnullExpected(
      Subscript->getBase(), State.Env,
      PointerNullabilityDiagnostic::Context::NullableDereference);
}

SmallVector<PointerNullabilityDiagnostic> diagnoseArrow(
    absl::Nonnull<const MemberExpr *> MemberExpr,
    const MatchFinder::MatchResult &Result, const DiagTransferState &State) {
  return diagnoseNonnullExpected(
      MemberExpr->getBase(), State.Env,
      PointerNullabilityDiagnostic::Context::NullableDereference);
}

// Diagnoses whether any of the arguments are incompatible with the
// corresponding type in the function prototype.
// ParmDecls is best-effort and used only for param names in diagnostics.
SmallVector<PointerNullabilityDiagnostic> diagnoseArgumentCompatibility(
    const FunctionProtoType &CalleeFPT,
    ArrayRef<PointerTypeNullability> ParamsNullability,
    ArrayRef<const ParmVarDecl *> ParmDecls, ArrayRef<const Expr *> Args,
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
    unsigned Len = countPointersInType(ParamTypes[I]);
    auto ParamNullability = ParamsNullability.take_front(Len);
    ParamsNullability = ParamsNullability.drop_front(Len);

    std::string ParamName =
        (I < ParmDecls.size()) ? ParmDecls[I]->getDeclName().getAsString() : "";
    Diagnostics.append(diagnoseAssignmentLike(
        ParamTypes[I], ParamNullability, Args[I], Env, Ctx,
        PointerNullabilityDiagnostic::Context::FunctionArgument,
        std::move(ParamName)));
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
    absl::Nonnull<const CallExpr *> CE, const DiagTransferState &State,
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
  if (MaybeComputed == nullptr) return untrackedError(CE);

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
    const DiagTransferState &State) {
  // __assert_nullability is a special-case.
  if (auto *FD = CE->getDirectCallee()) {
    if (FD->getDeclName().isIdentifier() &&
        FD->getName() == "__assert_nullability") {
      return diagnoseAssertNullabilityCall(CE, State, *Result.Context);
    }
  }

  const Expr *Callee = CE->getCallee();
  auto *CalleeNullabilityPtr =
      State.Lattice.getExprNullability(CE->getCallee());
  if (!CalleeNullabilityPtr) return {};
  const FunctionProtoType *CalleeType;
  ArrayRef CalleeNullability = *CalleeNullabilityPtr;  // Matches CalleeType.

  // Callee is typically a function pointer (not for members or builtins).
  // Check it for null, and unwrap the pointer for the next step.
  if (Callee->getType()->isPointerType()) {
    auto D = diagnoseNonnullExpected(
        Callee, State.Env, PointerNullabilityDiagnostic::Context::Other);
    // TODO: should we continue to diagnose arguments?
    if (!D.empty()) return D;

    CalleeNullability = CalleeNullability.drop_front();
    CalleeType =
        Callee->getType()->getPointeeType()->getAs<FunctionProtoType>();
  } else {
    QualType ET = exprType(Callee);
    // pseudo-destructor exprs are callees with null types :-(
    CalleeType = ET.isNull() ? nullptr : ET->getAs<FunctionProtoType>();
  }
  if (!CalleeType) return {};
  // We should rely entirely on the callee's nullability vector, and not at all
  // on the FunctionProtoType's sugar. Throw it away to be sure!
  CalleeType = cast<FunctionProtoType>(
      CalleeType->getCanonicalTypeInternal().getTypePtr());

  // Now check the args against the parameter types.
  ArrayRef<const Expr *> Args(CE->getArgs(), CE->getNumArgs());
  // The first argument of an member operator call expression is the implicit
  // object argument, which does not appear in the list of parameter types.
  // Note that operator calls always have a direct callee.
  if (isa<CXXOperatorCallExpr>(CE) &&
      isa<CXXMethodDecl>(CE->getDirectCallee())) {
    Args = Args.drop_front();
  }
  auto ParamNullability = CalleeNullability.drop_front(
      countPointersInType(CalleeType->getReturnType()));

  ArrayRef<ParmVarDecl *> Params;
  if (auto *DC = CE->getDirectCallee()) Params = DC->parameters();
  return diagnoseArgumentCompatibility(*CalleeType, ParamNullability, Params,
                                       Args, State.Env, *Result.Context);
}

SmallVector<PointerNullabilityDiagnostic> diagnoseConstructExpr(
    absl::Nonnull<const CXXConstructExpr *> CE,
    const MatchFinder::MatchResult &Result, const DiagTransferState &State) {
  auto *CalleeFPT = CE->getConstructor()->getType()->getAs<FunctionProtoType>();
  if (!CalleeFPT) return {};
  ArrayRef<const Expr *> ConstructorArgs(CE->getArgs(), CE->getNumArgs());
  // ctor's type is void(Args), so its nullability == arg nullability.
  auto CtorNullability =
      getTypeNullability(*CE->getConstructor(), State.Lattice.defaults());

  return diagnoseArgumentCompatibility(
      *CalleeFPT, CtorNullability,
      CE->getConstructor()->getAsFunction()->parameters(), ConstructorArgs,
      State.Env, *Result.Context);
}

SmallVector<PointerNullabilityDiagnostic> diagnoseReturn(
    absl::Nonnull<const ReturnStmt *> RS,
    const MatchFinder::MatchResult &Result, const DiagTransferState &State) {
  if (!RS->getRetValue()) return {};

  auto *Function = State.Env.getCurrentFunc();
  CHECK(Function);
  auto FunctionNullability =
      getTypeNullability(*Function, State.Lattice.defaults());
  auto ReturnTypeNullability =
      ArrayRef(FunctionNullability)
          .take_front(countPointersInType(Function->getReturnType()));

  return diagnoseAssignmentLike(
      Function->getReturnType(), ReturnTypeNullability, RS->getRetValue(),
      State.Env, *Result.Context,
      PointerNullabilityDiagnostic::Context::ReturnValue);
}

SmallVector<PointerNullabilityDiagnostic> diagnoseMemberInitializer(
    absl::Nonnull<const CXXCtorInitializer *> CI,
    const MatchFinder::MatchResult &Result, const DiagTransferState &State) {
  CHECK(CI->isAnyMemberInitializer());
  auto *Member = CI->getAnyMember();
  return diagnoseAssignmentLike(
      Member->getType(), getTypeNullability(*Member, State.Lattice.defaults()),
      CI->getInit(), State.Env, *Result.Context,
      PointerNullabilityDiagnostic::Context::Initializer);
}

SmallVector<PointerNullabilityDiagnostic> diagnoseMovedFromNonnullSmartPointer(
    absl::Nonnull<const Expr *> E, const MatchFinder::MatchResult &,
    const DiagTransferState &State) {
  const TypeNullability *Nullability = State.Lattice.getExprNullability(E);
  if (Nullability == nullptr) return untrackedError(E);

  if (Nullability->front().concrete() != NullabilityKind::NonNull) return {};

  PointerValue *Val = getPointerValueFromSmartPointer(
      State.Env.get<RecordStorageLocation>(*E), State.Env);
  if (Val == nullptr) return untrackedError(E);

  if (isNullable(*Val, State.Env))
    return {{PointerNullabilityDiagnostic::ErrorCode::
                 AccessingMovedFromNonnullPointer,
             PointerNullabilityDiagnostic::Context::Other,
             CharSourceRange::getTokenRange(E->getSourceRange())}};

  return {};
}

/// Expressions of smart pointer type that are allowed to be in a moved-from
/// state even if the smart pointer is annotated nonnull.
class AllowedMovedFromNonnullSmartPointerExprs {
 public:
  explicit AllowedMovedFromNonnullSmartPointerExprs(const FunctionDecl *Func) {
    for (const BoundNodes &Node :
         match(findAll(expr(anyOf(
                   cxxMemberCallExpr(
                       isSmartPointerMethodCall("reset"),
                       unless(hasArgument(0, hasType(isNullPtrType()))),
                       onImplicitObjectArgument(expr().bind("e"))),
                   cxxOperatorCallExpr(isSmartPointerOperatorCall("="),
                                       hasArgument(0, expr().bind("e")))))),
               *Func->getBody(), Func->getASTContext())) {
      AllowedExprs.insert(normalize(Node.getNodeAs<Expr>("e")));
    }
  }

  /// Returns whether `E` is allowed to be in a moved-from state even if the
  /// smart pointer is annotated nonnull.
  bool allowed(const Expr *E) const {
    return AllowedExprs.contains(normalize(E));
  }

 private:
  /// Normalizes `E` to ignore parentheses and casts.
  /// We wrap this in a function so that, if we need to change the
  /// normalization, all callers use consistent behavior.
  static const Expr *normalize(const Expr *E) {
    return E->IgnoreParenBaseCasts();
  }

  llvm::DenseSet<const Expr *> AllowedExprs;
};

bool shouldDiagnoseExpectedNonnullDefaultArgValue(
    clang::ASTContext &Ctx, const ParmVarDecl &Param,
    const TypeNullabilityDefaults &Defaults) {
  const Expr *Init = Param.getInit();
  if (!Init) return false;
  if (Init->isNullPointerConstant(Ctx, Expr::NPC_ValueDependentIsNotNull))
    return true;
  QualType InitTy = Init->getType();
  if (InitTy->isDependentType() || !isSupportedPointerType(InitTy))
    return false;
  if (TypeNullability DefaultValueAnnotation = getTypeNullability(
          exprType(Init), Ctx.getSourceManager().getFileID(Param.getLocation()),
          Defaults);
      !DefaultValueAnnotation.empty() &&
      DefaultValueAnnotation.front().concrete() == NullabilityKind::Nullable) {
    return true;
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
    llvm::SmallVector<PointerNullabilityDiagnostic> &Diags,
    const TypeNullabilityDefaults &Defaults) {
  if (Parm.getType()->isDependentType()) return;
  TypeNullability DeclAnnotation = getTypeNullability(Parm, Defaults);
  if (DeclAnnotation.empty() ||
      DeclAnnotation.front().concrete() != NullabilityKind::NonNull) {
    return;
  }

  const Expr *DefaultVal = Parm.getInit();
  if (!DefaultVal ||
      !shouldDiagnoseExpectedNonnullDefaultArgValue(Ctx, Parm, Defaults))
    return;

  Diags.push_back({PointerNullabilityDiagnostic::ErrorCode::ExpectedNonnull,
                   PointerNullabilityDiagnostic::Context::Initializer,
                   CharSourceRange::getTokenRange(DefaultVal->getSourceRange()),
                   Parm.getNameAsString()});
}

void checkAnnotationsConsistent(
    absl::Nonnull<const ValueDecl *> VD,
    llvm::SmallVector<PointerNullabilityDiagnostic> &Diags,
    const TypeNullabilityDefaults &Defaults) {
  auto *CanonicalDecl = cast<ValueDecl>(VD->getCanonicalDecl());

  // We check against the annotation on the canonical decl, so if this is the
  // canonical decl, there is nothing to do.
  if (VD == CanonicalDecl) return;

  TypeNullability Canonical = getTypeNullability(*CanonicalDecl, Defaults);
  TypeNullability Cur = getTypeNullability(*VD, Defaults);
  if (Cur != Canonical) {
    Diags.push_back(
        {PointerNullabilityDiagnostic::ErrorCode::InconsistentAnnotations,
         PointerNullabilityDiagnostic::Context::Other,
         CharSourceRange::getTokenRange(VD->getSourceRange()), std::nullopt,
         CharSourceRange::getTokenRange(CanonicalDecl->getSourceRange())});
  }
}

DiagTransferFunc pointerNullabilityDiagnoser(
    const AllowedMovedFromNonnullSmartPointerExprs &AllowedMovedFromNonnull) {
  DiagTransferFunc MainDiagnoser =
      CFGMatchSwitchBuilder<const DiagTransferState,
                            SmallVector<PointerNullabilityDiagnostic>>()
          // (*)
          .CaseOfCFGStmt<UnaryOperator>(isPointerDereference(),
                                        diagnoseDereference)
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
          .CaseOfCFGStmt<CallExpr>(callExpr(), diagnoseCallExpr)
          .CaseOfCFGStmt<CXXConstructExpr>(cxxConstructExpr(),
                                           diagnoseConstructExpr)
          .CaseOfCFGStmt<ReturnStmt>(isPointerReturn(), diagnoseReturn)
          // Check compatibility of member initializers.
          .CaseOfCFGInit<CXXCtorInitializer>(isCtorMemberInitializer(),
                                             diagnoseMemberInitializer)
          .Build();

  // We need a second transfer function to diagnose moved-from nonnull smart
  // pointers because it needs to look at all expressions of smart pointer type.
  // We can't do this in the existing transfer function because some of the
  // existing cases may have smart pointer type, and only one of the cases is
  // ever run.
  DiagTransferFunc MovedFromNonnullPointerDiagnoser =
      CFGMatchSwitchBuilder<const DiagTransferState,
                            SmallVector<PointerNullabilityDiagnostic>>()
          .CaseOfCFGStmt<Expr>(
              expr(hasType(isSupportedSmartPointer()), isGLValue()),
              [&AllowedMovedFromNonnull](absl::Nonnull<const Expr *> E,
                                         const MatchFinder::MatchResult &Result,
                                         const DiagTransferState &State)
                  -> SmallVector<PointerNullabilityDiagnostic> {
                if (AllowedMovedFromNonnull.allowed(E)) return {};
                return diagnoseMovedFromNonnullSmartPointer(E, Result, State);
              })
          .Build();

  return [MainDiagnoser = std::move(MainDiagnoser),
          MovedFromNonnullPointerDiagnoser =
              std::move(MovedFromNonnullPointerDiagnoser)](
             const CFGElement &Elt, ASTContext &ASTCtx,
             const DiagTransferState &State) {
    SmallVector<PointerNullabilityDiagnostic> Diags =
        MainDiagnoser(Elt, ASTCtx, State);
    Diags.append(MovedFromNonnullPointerDiagnoser(Elt, ASTCtx, State));
    return Diags;
  };
}

}  // namespace

std::unique_ptr<dataflow::Solver> makeDefaultSolverForDiagnosis() {
  // This limit is set based on empirical observations. Mostly, it is a rough
  // proxy for a line between "finite" and "effectively infinite", rather than a
  // strict limit on resource use.
  constexpr std::int64_t MaxSATIterations = 2'000'000;
  return std::make_unique<dataflow::WatchedLiteralsSolver>(MaxSATIterations);
}

llvm::Expected<llvm::SmallVector<PointerNullabilityDiagnostic>>
diagnosePointerNullability(const ValueDecl *VD,
                           const NullabilityPragmas &Pragmas,
                           const SolverFactory &MakeSolver) {
  // This limit is set based on empirical observations. Mostly, it is a rough
  // proxy for a line between "finite" and "effectively infinite", rather than a
  // strict limit on resource use.
  constexpr std::int32_t MaxBlockVisits = 20'000;

  llvm::SmallVector<PointerNullabilityDiagnostic> Diags;
  if (VD->isTemplated()) return Diags;

  ASTContext &Ctx = VD->getASTContext();
  TypeNullabilityDefaults Defaults{Ctx, Pragmas};

  checkAnnotationsConsistent(VD, Diags, Defaults);

  const auto *Func = dyn_cast<FunctionDecl>(VD);
  if (Func == nullptr) return Diags;

  for (const ParmVarDecl *Parm : Func->parameters())
    checkParmVarDeclWithPointerDefaultArg(Ctx, *Parm, Diags, Defaults);

  // Use `doesThisDeclarationHaveABody()` rather than `hasBody()` to ensure we
  // analyze forward-declared functions only once.
  if (!Func->doesThisDeclarationHaveABody()) return Diags;

  AllowedMovedFromNonnullSmartPointerExprs AllowedMovedFromNonnull(Func);

  // TODO(b/332565018): it would be nice to have some common pieces (limits,
  // adorning, error-handling) reused. diagnoseFunction() is too restrictive.
  auto CFG = dataflow::AdornedCFG::build(*Func);
  if (!CFG) return CFG.takeError();

  std::unique_ptr<dataflow::Solver> Solver = MakeSolver();
  dataflow::DataflowAnalysisContext AnalysisContext(*Solver);
  Environment Env(AnalysisContext, *Func);

  PointerNullabilityAnalysis Analysis(Ctx, Env, Pragmas);

  auto Result = dataflow::runDataflowAnalysis(
      *CFG, Analysis, Env,
      [&, Diagnoser(pointerNullabilityDiagnoser(AllowedMovedFromNonnull))](
          const CFGElement &Elt,
          const dataflow::DataflowAnalysisState<PointerNullabilityLattice>
              &State) mutable {
        auto EltDiagnostics = Diagnoser(Elt, Ctx, {State.Lattice, State.Env});
        llvm::move(EltDiagnostics, std::back_inserter(Diags));
      },
      MaxBlockVisits);
  if (!Result) return Result.takeError();
  if (Solver->reachedLimit())
    return llvm::createStringError(llvm::errc::interrupted,
                                   "SAT solver timed out");

  return Diags;
}

}  // namespace clang::tidy::nullability
