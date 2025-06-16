// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/pointer_nullability_diagnosis.h"

#include <cassert>
#include <cstdint>
#include <iterator>
#include <memory>
#include <optional>
#include <string>

#include "absl/base/nullability.h"
#include "absl/log/check.h"
#include "nullability/forwarding_functions.h"
#include "nullability/pointer_nullability.h"
#include "nullability/pointer_nullability_analysis.h"
#include "nullability/pointer_nullability_lattice.h"
#include "nullability/pointer_nullability_matchers.h"
#include "nullability/pragma.h"
#include "nullability/type_nullability.h"
#include "clang/include/clang/AST/ASTContext.h"
#include "clang/include/clang/AST/Decl.h"
#include "clang/include/clang/AST/DeclCXX.h"
#include "clang/include/clang/AST/Expr.h"
#include "clang/include/clang/AST/ExprCXX.h"
#include "clang/include/clang/AST/Stmt.h"
#include "clang/include/clang/AST/TemplateBase.h"
#include "clang/include/clang/AST/Type.h"
#include "clang/include/clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/include/clang/ASTMatchers/ASTMatchers.h"
#include "clang/include/clang/Analysis/CFG.h"
#include "clang/include/clang/Analysis/FlowSensitive/ASTOps.h"
#include "clang/include/clang/Analysis/FlowSensitive/AdornedCFG.h"
#include "clang/include/clang/Analysis/FlowSensitive/CFGMatchSwitch.h"
#include "clang/include/clang/Analysis/FlowSensitive/DataflowAnalysis.h"
#include "clang/include/clang/Analysis/FlowSensitive/DataflowAnalysisContext.h"
#include "clang/include/clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/include/clang/Analysis/FlowSensitive/MatchSwitch.h"
#include "clang/include/clang/Analysis/FlowSensitive/Solver.h"
#include "clang/include/clang/Analysis/FlowSensitive/StorageLocation.h"
#include "clang/include/clang/Analysis/FlowSensitive/Value.h"
#include "clang/include/clang/Analysis/FlowSensitive/WatchedLiteralsSolver.h"
#include "clang/include/clang/Basic/LLVM.h"
#include "clang/include/clang/Basic/SourceLocation.h"
#include "clang/include/clang/Basic/Specifiers.h"
#include "clang/include/clang/Tooling/Transformer/SourceCode.h"
#include "llvm/include/llvm/ADT/DenseSet.h"
#include "llvm/include/llvm/ADT/STLExtras.h"
#include "llvm/include/llvm/ADT/SmallVector.h"
#include "llvm/include/llvm/Support/Debug.h"
#include "llvm/include/llvm/Support/Errc.h"
#include "llvm/include/llvm/Support/Error.h"
#include "llvm/include/llvm/Support/raw_ostream.h"

#define DEBUG_TYPE "nullability-diagnostic"

namespace clang::tidy::nullability {

using ast_matchers::anyOf;
using ast_matchers::binaryOperator;
using ast_matchers::BoundNodes;
using ast_matchers::callExpr;
using ast_matchers::cxxConstructExpr;
using ast_matchers::cxxMemberCallExpr;
using ast_matchers::cxxOperatorCallExpr;
using ast_matchers::declStmt;
using ast_matchers::expr;
using ast_matchers::findAll;
using ast_matchers::hasArgument;
using ast_matchers::hasLHS;
using ast_matchers::hasOperands;
using ast_matchers::hasOperatorName;
using ast_matchers::hasType;
using ast_matchers::initListExpr;
using ast_matchers::isInteger;
using ast_matchers::match;
using ast_matchers::MatchFinder;
using ast_matchers::onImplicitObjectArgument;
using ast_matchers::unaryOperator;
using ast_matchers::unless;
using dataflow::CFGMatchSwitchBuilder;
using dataflow::DataflowAnalysisContext;
using dataflow::Environment;
using dataflow::FieldSet;
using dataflow::PointerValue;
using dataflow::RecordInitListHelper;
using dataflow::RecordStorageLocation;
using dataflow::StorageLocation;
using ::llvm::SmallVector;

namespace {

using DiagTransferState =
    dataflow::TransferStateForDiagnostics<PointerNullabilityLattice>;
using DiagTransferFunc =
    dataflow::CFGMatchSwitch<const DiagTransferState,
                             SmallVector<PointerNullabilityDiagnostic>>;

CharSourceRange getRangeModuloMacros(CharSourceRange Range,
                                     const ASTContext &Ctx) {
  if (auto RangeOpt =
          tooling::getFileRange(Range, Ctx, /*IncludeMacroExpansion=*/true))
    return RangeOpt.value();
  return Range;
}

SmallVector<PointerNullabilityDiagnostic> untrackedError(
    const Expr *E, const ASTContext &Ctx,
    PointerNullabilityDiagnostic::Context DiagCtx =
        PointerNullabilityDiagnostic::Context::Other) {
  return {{PointerNullabilityDiagnostic::ErrorCode::Untracked, DiagCtx,
           getRangeModuloMacros(
               CharSourceRange::getTokenRange(E->getSourceRange()), Ctx)}};
}

// Diagnoses whether `E` violates the expectation that it is nonnull.
SmallVector<PointerNullabilityDiagnostic> diagnoseNonnullExpected(
    const Expr *absl_nonnull E, const Environment &Env, const ASTContext &Ctx,
    PointerNullabilityDiagnostic::Context DiagCtx,
    const clang::NamedDecl *absl_nullable Callee = nullptr,
    const clang::IdentifierInfo *absl_nullable ParamName = nullptr,
    CharSourceRange Range = {}) {
  std::optional<bool> IsNullable;
  if (PointerValue *ActualVal = getPointerValue(E, Env))
    IsNullable = isNullable(*ActualVal, Env);
  else if (E->getType()->isNullPtrType())
    IsNullable = isReachableNullptrLiteral(Env);

  if (IsNullable.has_value()) {
    if (*IsNullable) {
      if (Range.isInvalid())
        Range = CharSourceRange::getTokenRange(E->getSourceRange());
      return {{PointerNullabilityDiagnostic::ErrorCode::ExpectedNonnull,
               DiagCtx, getRangeModuloMacros(Range, Ctx), Callee, ParamName}};
    }
    return {};
  }

  LLVM_DEBUG({
    llvm::dbgs()
        << "The dataflow analysis framework does not model a PointerValue "
           "for the following Expr, and thus its dereference is marked as "
           "unsafe:\n";
    E->dump();
  });
  return untrackedError(E, Ctx, DiagCtx);
}

// Diagnoses a conceptual assignment of LHS = RHS.
// LHS can be a variable, the return value of a function, a param etc.
SmallVector<PointerNullabilityDiagnostic> diagnoseAssignmentLike(
    QualType LHSType, ArrayRef<PointerTypeNullability> LHSNullability,
    const Expr *absl_nonnull RHS, const Environment &Env, ASTContext &Ctx,
    PointerNullabilityDiagnostic::Context DiagCtx,
    const clang::NamedDecl *absl_nullable Callee = nullptr,
    const clang::IdentifierInfo *absl_nullable ParamName = nullptr,
    CharSourceRange LHSRange = {}) {
  LHSType = LHSType.getNonReferenceType();
  // For now, we just check whether the top-level pointer type is compatible.
  // TODO: examine inner nullability too, considering variance.
  if (!isSupportedPointerType(LHSType)) return {};

  QualType RHSType = RHS->getType().getNonReferenceType();
  if (!RHSType->isNullPtrType() && !isSupportedPointerType(RHSType)) return {};

  return LHSNullability.front().concrete() == NullabilityKind::NonNull
             ? diagnoseNonnullExpected(RHS, Env, Ctx, DiagCtx, Callee,
                                       ParamName, LHSRange)
             : SmallVector<PointerNullabilityDiagnostic>{};
}

SmallVector<PointerNullabilityDiagnostic> diagnoseDereference(
    const UnaryOperator *absl_nonnull UnaryOp,
    const MatchFinder::MatchResult &Result, const DiagTransferState &State) {
  return diagnoseNonnullExpected(
      UnaryOp->getSubExpr(), State.Env, *Result.Context,
      PointerNullabilityDiagnostic::Context::NullableDereference);
}

SmallVector<PointerNullabilityDiagnostic> diagnoseSmartPointerDereference(
    const CXXOperatorCallExpr *absl_nonnull Op,
    const MatchFinder::MatchResult &Result, const DiagTransferState &State) {
  return diagnoseNonnullExpected(
      Op->getArg(0), State.Env, *Result.Context,
      PointerNullabilityDiagnostic::Context::NullableDereference);
}

SmallVector<PointerNullabilityDiagnostic> diagnoseSubscript(
    const ArraySubscriptExpr *absl_nonnull Subscript,
    const MatchFinder::MatchResult &Result, const DiagTransferState &State) {
  return diagnoseNonnullExpected(
      Subscript->getBase(), State.Env, *Result.Context,
      PointerNullabilityDiagnostic::Context::NullableDereference);
}

SmallVector<PointerNullabilityDiagnostic> diagnoseArrow(
    const MemberExpr *absl_nonnull MemberExpr,
    const MatchFinder::MatchResult &Result, const DiagTransferState &State) {
  return diagnoseNonnullExpected(
      MemberExpr->getBase(), State.Env, *Result.Context,
      PointerNullabilityDiagnostic::Context::NullableDereference,
      /*Callee=*/nullptr, /*ParamName=*/nullptr);
}

SmallVector<PointerNullabilityDiagnostic> diagnoseAssignment(
    const BinaryOperator *absl_nonnull Op,
    const MatchFinder::MatchResult &Result, const DiagTransferState &State) {
  const TypeNullability *LHSNullability =
      State.Lattice.getTypeNullability(Op->getLHS());
  if (!LHSNullability) return {};

  return diagnoseAssignmentLike(
      Op->getLHS()->getType(), *LHSNullability, Op->getRHS(), State.Env,
      *Result.Context, PointerNullabilityDiagnostic::Context::Assignment);
}

SmallVector<PointerNullabilityDiagnostic> diagnoseSmartPointerAssignment(
    const CXXOperatorCallExpr *absl_nonnull Op,
    const MatchFinder::MatchResult &Result, const DiagTransferState &State) {
  const TypeNullability *LHSNullability =
      State.Lattice.getTypeNullability(Op->getArg(0));
  if (!LHSNullability) return {};

  return diagnoseAssignmentLike(
      Op->getArg(0)->getType(), *LHSNullability, Op->getArg(1), State.Env,
      *Result.Context, PointerNullabilityDiagnostic::Context::Assignment);
}

SmallVector<PointerNullabilityDiagnostic> diagnoseSmartPointerReset(
    const CXXMemberCallExpr *absl_nonnull MCE,
    const MatchFinder::MatchResult &Result, const DiagTransferState &State) {
  const TypeNullability *ObjArgNullability =
      State.Lattice.getTypeNullability(MCE->getImplicitObjectArgument());
  if (!ObjArgNullability) return {};

  ArrayRef<PointerTypeNullability> ReceiverNullability = *ObjArgNullability;
  if (MCE->getImplicitObjectArgument()->getType()->isPointerType())
    ReceiverNullability = ReceiverNullability.drop_front();

  if (MCE->getNumArgs() == 0 ||
      (MCE->getNumArgs() == 1 && MCE->getArg(0)->getType()->isNullPtrType()) ||
      (MCE->getNumArgs() == 1 && MCE->getArg(0)->isDefaultArgument())) {
    if (ReceiverNullability.front().concrete() == NullabilityKind::NonNull)
      return {{PointerNullabilityDiagnostic::ErrorCode::ExpectedNonnull,
               PointerNullabilityDiagnostic::Context::Assignment,
               getRangeModuloMacros(
                   CharSourceRange::getTokenRange(MCE->getSourceRange()),
                   *Result.Context)}};
    return {};
  }

  return diagnoseAssignmentLike(
      MCE->getObjectType(), ReceiverNullability, MCE->getArg(0), State.Env,
      *Result.Context, PointerNullabilityDiagnostic::Context::Assignment);
}

// Diagnoses whether any of the arguments are incompatible with the
// corresponding type in the function prototype.
// ParmDecls is best-effort and used only for param names in diagnostics.
SmallVector<PointerNullabilityDiagnostic> diagnoseArgumentCompatibility(
    const FunctionProtoType &CalleeFPT,
    ArrayRef<PointerTypeNullability> ParamsNullability,
    ArrayRef<const ParmVarDecl *> ParmDecls, ArrayRef<const Expr *> Args,
    const clang::NamedDecl *absl_nullable Callee, const Environment &Env,
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
    unsigned Len = countPointersInType(ParamTypes[I]);
    auto ParamNullability = ParamsNullability.take_front(Len);
    ParamsNullability = ParamsNullability.drop_front(Len);

    const clang::IdentifierInfo *ParamName =
        (I < ParmDecls.size()) ? ParmDecls[I]->getIdentifier() : nullptr;
    Diagnostics.append(diagnoseAssignmentLike(
        ParamTypes[I], ParamNullability, Args[I], Env, Ctx,
        PointerNullabilityDiagnostic::Context::FunctionArgument, Callee,
        ParamName));
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
    const CallExpr *absl_nonnull CE, const DiagTransferState &State,
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
      State.Lattice.getTypeNullability(GivenExpr);
  if (MaybeComputed == nullptr) return untrackedError(CE, Ctx);

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
           getRangeModuloMacros(
               CharSourceRange::getTokenRange(CE->getSourceRange()), Ctx)}};
}

SmallVector<PointerNullabilityDiagnostic> diagnoseIncrementDecrement(
    const UnaryOperator *absl_nonnull UnaryOp,
    const MatchFinder::MatchResult &Result, const DiagTransferState &State) {
  return diagnoseNonnullExpected(UnaryOp->getSubExpr(), State.Env,
                                 *Result.Context,
                                 PointerNullabilityDiagnostic::Context::Other);
}

SmallVector<PointerNullabilityDiagnostic> diagnoseAddSubtract(
    Expr *PtrExpr, Expr *IntExpr, const Environment &Env,
    const ASTContext &Ctx) {
  // Adding or subtracting zero is allowed even if the pointer is null.
  if (auto *Lit = dyn_cast<IntegerLiteral>(IntExpr->IgnoreParenImpCasts())) {
    if (Lit->getValue().isZero()) return {};
  }

  return diagnoseNonnullExpected(PtrExpr, Env, Ctx,
                                 PointerNullabilityDiagnostic::Context::Other);
}

SmallVector<PointerNullabilityDiagnostic> diagnoseAddSubtractAssign(
    const BinaryOperator *absl_nonnull BinaryOp,
    const MatchFinder::MatchResult &Result, const DiagTransferState &State) {
  return diagnoseAddSubtract(BinaryOp->getLHS(), BinaryOp->getRHS(), State.Env,
                             *Result.Context);
}

SmallVector<PointerNullabilityDiagnostic> diagnoseAddSubtractInteger(
    const BinaryOperator *absl_nonnull BinaryOp,
    const MatchFinder::MatchResult &Result, const DiagTransferState &State) {
  if (BinaryOp->getLHS()->getType()->isIntegerType()) {
    return diagnoseAddSubtract(BinaryOp->getRHS(), BinaryOp->getLHS(),
                               State.Env, *Result.Context);
  }
  return diagnoseAddSubtract(BinaryOp->getLHS(), BinaryOp->getRHS(), State.Env,
                             *Result.Context);
}

SmallVector<PointerNullabilityDiagnostic> diagnosePointerDifference(
    const BinaryOperator *absl_nonnull BinaryOp,
    const MatchFinder::MatchResult &Result, const DiagTransferState &State) {
  SmallVector<PointerNullabilityDiagnostic> Diagnostics =
      diagnoseNonnullExpected(BinaryOp->getLHS(), State.Env, *Result.Context,
                              PointerNullabilityDiagnostic::Context::Other);
  Diagnostics.append(
      diagnoseNonnullExpected(BinaryOp->getRHS(), State.Env, *Result.Context,
                              PointerNullabilityDiagnostic::Context::Other));
  return Diagnostics;
}

SmallVector<PointerNullabilityDiagnostic> diagnoseConstructorCall(
    ArrayRef<const Expr *> ConstructorArgs, const CXXConstructorDecl *CtorDecl,
    const MatchFinder::MatchResult &Result, const DiagTransferState &State) {
  auto *CalleeFPT = CtorDecl->getType()->getAs<FunctionProtoType>();
  if (!CalleeFPT) return {};
  // ctor's type is void(Args), so its nullability == arg nullability.
  auto CtorNullability =
      getTypeNullability(*CtorDecl, State.Lattice.defaults());
  return diagnoseArgumentCompatibility(
      *CalleeFPT, CtorNullability, CtorDecl->getAsFunction()->parameters(),
      ConstructorArgs, CtorDecl, State.Env, *Result.Context);
}

SmallVector<PointerNullabilityDiagnostic> diagnoseConstructExpr(
    const CXXConstructExpr *absl_nonnull CE,
    const MatchFinder::MatchResult &Result, const DiagTransferState &State) {
  const CXXConstructorDecl *CtorDecl = CE->getConstructor();
  ArrayRef<const Expr *> ConstructorArgs(CE->getArgs(), CE->getNumArgs());
  return diagnoseConstructorCall(ConstructorArgs, CtorDecl, Result, State);
}

SmallVector<PointerNullabilityDiagnostic> diagnoseMakeUniqueConstructExpr(
    const CallExpr *absl_nonnull MakeUniqueCall,
    const CXXConstructExpr *absl_nonnull CEInMakeUnique,
    const MatchFinder::MatchResult &Result, const DiagTransferState &State) {
  CXXConstructorDecl *CtorDecl = CEInMakeUnique->getConstructor();
  // Use the arguments from the `MakeUniqueCall`, which has the
  // call-site nullability information from the `State`, instead of the
  // arguments from `CEInMakeUnique`.
  ArrayRef<const Expr *> ConstructorArgs(MakeUniqueCall->getArgs(),
                                         MakeUniqueCall->getNumArgs());
  SmallVector<const Expr *> CopyOfArgs;
  if (CEInMakeUnique->getNumArgs() > MakeUniqueCall->getNumArgs()) {
    // Perhaps there are default arguments. Append them to the end of the
    // argument list, so that we get the same number of arguments as a
    // real constructor call.
    CopyOfArgs.insert(CopyOfArgs.end(), ConstructorArgs.begin(),
                      ConstructorArgs.end());
    for (unsigned I = MakeUniqueCall->getNumArgs();
         I < CEInMakeUnique->getNumArgs(); ++I) {
      CHECK(CEInMakeUnique->getArg(I)->isDefaultArgument());
      CopyOfArgs.push_back(CEInMakeUnique->getArg(I));
    }
    ConstructorArgs = CopyOfArgs;
  }
  return diagnoseConstructorCall(ConstructorArgs, CtorDecl, Result, State);
}

SmallVector<PointerNullabilityDiagnostic> diagnoseMakeUniqueParenInitListExpr(
    const CallExpr *absl_nonnull MakeUniqueCall,
    const CXXParenListInitExpr *absl_nonnull InitListInMakeUnique,
    const MatchFinder::MatchResult &Result, const DiagTransferState &State) {
  if (!InitListInMakeUnique->getType()->isRecordType()) return {};

  RecordInitListHelper InitListHelper(InitListInMakeUnique);
  SmallVector<PointerNullabilityDiagnostic> Diagnostics;
  // Skip through the base inits to get to the field inits. Any initialization
  // of base classes/fields will be collected from the InitListExpr for the
  // base initialization.
  int I = InitListHelper.base_inits().size();
  // Use the arguments from the `MakeUniqueCall`, which has the
  // call-site nullability information from the `State`, instead of the
  // `field_inits` from `InitListInMakeUnique`.
  int NumArgs = MakeUniqueCall->getNumArgs();
  for (auto [Field, Init] : InitListHelper.field_inits()) {
    // The make_unique call can have fewer arguments than fields in the struct.
    // The rest should be various kinds of default initializers.
    const Expr *Arg;
    if (I < NumArgs) {
      Arg = MakeUniqueCall->getArg(I);
    } else {
      assert(isa<ImplicitValueInitExpr>(Init) || isa<CXXConstructExpr>(Init) ||
             isa<CXXDefaultInitExpr>(Init));
      Arg = Init;
    }
    Diagnostics.append(diagnoseAssignmentLike(
        Field->getType(), getTypeNullability(*Field, State.Lattice.defaults()),
        Arg, State.Env, *Result.Context,
        PointerNullabilityDiagnostic::Context::Initializer));
    ++I;
  }
  return Diagnostics;
}

SmallVector<PointerNullabilityDiagnostic> diagnoseCallExpr(
    const CallExpr *absl_nonnull CE, const MatchFinder::MatchResult &Result,
    const DiagTransferState &State) {
  // Handle some special cases first.
  if (auto *FD = CE->getDirectCallee()) {
    // __assert_nullability is a special-case (for testing)
    if (FD->getDeclName().isIdentifier() &&
        FD->getName() == "__assert_nullability") {
      return diagnoseAssertNullabilityCall(CE, State, *Result.Context);
    }
    // std::make_unique is a special-case: we want to verify the underlying
    // constructor call or initializer list.
    if (const Expr *Initializer = getUnderlyingInitExprInStdMakeUnique(*FD)) {
      if (const auto *ConstructExpr = dyn_cast<CXXConstructExpr>(Initializer)) {
        return diagnoseMakeUniqueConstructExpr(CE, ConstructExpr, Result,
                                               State);
      }
      if (const auto *ParenInitList =
              dyn_cast<CXXParenListInitExpr>(Initializer)) {
        return diagnoseMakeUniqueParenInitListExpr(CE, ParenInitList, Result,
                                                   State);
      }
      llvm::errs()
          << "Nullability: Unexpected initializer expression in make_unique: "
          << Initializer->getStmtClassName() << " in "
          << CE->getBeginLoc().printToString(Result.Context->getSourceManager())
          << "\n";
      assert(false);
    }
  }

  const Expr *Callee = CE->getCallee();
  auto *CalleeNullabilityPtr =
      State.Lattice.getTypeNullability(CE->getCallee());
  if (!CalleeNullabilityPtr) return {};
  const FunctionProtoType *CalleeType;
  ArrayRef CalleeNullability = *CalleeNullabilityPtr;  // Matches CalleeType.

  // Callee is typically a function pointer (not for members or builtins).
  // Check it for null, and unwrap the pointer for the next step.
  if (Callee->getType()->isPointerType()) {
    auto D =
        diagnoseNonnullExpected(Callee, State.Env, *Result.Context,
                                PointerNullabilityDiagnostic::Context::Other);
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

  return diagnoseArgumentCompatibility(
      *CalleeType, ParamNullability, Params, Args,
      dyn_cast_or_null<FunctionDecl>(CE->getCalleeDecl()), State.Env,
      *Result.Context);
}

SmallVector<PointerNullabilityDiagnostic> diagnoseReturn(
    const ReturnStmt *absl_nonnull RS, const MatchFinder::MatchResult &Result,
    const DiagTransferState &State) {
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
    const CXXCtorInitializer *absl_nonnull CI,
    const MatchFinder::MatchResult &Result, const DiagTransferState &State) {
  CHECK(CI->isAnyMemberInitializer());
  auto *Member = CI->getAnyMember();
  const auto *InitExpr = CI->getInit();
  if (!CI->isWritten()) {
    // Don't warn if a nonnull pointer field is merely default-initialized to
    // null. (If the constructor doesn't initialize the field to nonnull, we'll
    // catch this in diagnoseNonnullSmartPointerFieldMovedFromAtExit.)
    if (!isa<CXXDefaultInitExpr>(InitExpr)) return {};
    // Do warn if a nonnull pointer field has an in-class initializer that
    // explicitly initializes it to null.
    return diagnoseAssignmentLike(
        Member->getType(),
        getTypeNullability(*Member, State.Lattice.defaults()), InitExpr,
        State.Env, *Result.Context,
        PointerNullabilityDiagnostic::Context::Initializer, nullptr, nullptr,
        CharSourceRange::getTokenRange(Member->getSourceRange()));
  }
  return diagnoseAssignmentLike(
      Member->getType(), getTypeNullability(*Member, State.Lattice.defaults()),
      InitExpr, State.Env, *Result.Context,
      PointerNullabilityDiagnostic::Context::Initializer);
}

SmallVector<PointerNullabilityDiagnostic> diagnoseInitListExpr(
    const InitListExpr *absl_nonnull ILE,
    const MatchFinder::MatchResult &Result, const DiagTransferState &State) {
  if (!ILE->getType()->isRecordType()) return {};

  if (ILE->isSemanticForm() && ILE->isTransparent()) return {};

  RecordInitListHelper InitListHelper(ILE);
  SmallVector<PointerNullabilityDiagnostic> Diagnostics;
  for (auto [Field, Init] : InitListHelper.field_inits()) {
    auto Range = Init->getSourceRange();
    // ImplicitValueInitExprs do not have a source range, so we use the end
    // of the initializer list.
    if (isa<ImplicitValueInitExpr>(Init)) {
      assert(Init->getSourceRange().isInvalid());
      Range = ILE->getRBraceLoc();
    }
    Diagnostics.append(diagnoseAssignmentLike(
        Field->getType(), getTypeNullability(*Field, State.Lattice.defaults()),
        Init, State.Env, *Result.Context,
        PointerNullabilityDiagnostic::Context::Initializer, nullptr, nullptr,
        CharSourceRange::getTokenRange(Range)));
  }

  return Diagnostics;
}

SmallVector<PointerNullabilityDiagnostic> diagnoseDeclStmt(
    const DeclStmt *absl_nonnull DS, const MatchFinder::MatchResult &Result,
    const DiagTransferState &State) {
  SmallVector<PointerNullabilityDiagnostic> Diagnostics;
  for (const Decl *D : DS->decls()) {
    if (auto *VD = dyn_cast<VarDecl>(D); VD && VD->hasInit()) {
      Diagnostics.append(diagnoseAssignmentLike(
          VD->getType(), getTypeNullability(*VD, State.Lattice.defaults()),
          VD->getInit(), State.Env, *Result.Context,
          PointerNullabilityDiagnostic::Context::Initializer));
    }
  }
  return Diagnostics;
}

SmallVector<PointerNullabilityDiagnostic> diagnoseMovedFromNonnullSmartPointer(
    const Expr *absl_nonnull E, const MatchFinder::MatchResult &Result,
    const DiagTransferState &State) {
  const TypeNullability *Nullability = State.Lattice.getTypeNullability(E);
  const auto &Ctx = *Result.Context;
  if (Nullability == nullptr) return untrackedError(E, Ctx);

  if (Nullability->front().concrete() != NullabilityKind::NonNull) return {};

  PointerValue *Val = getPointerValueFromSmartPointer(
      State.Env.get<RecordStorageLocation>(*E), State.Env);
  if (Val == nullptr) return untrackedError(E, Ctx);

  if (isNullable(*Val, State.Env))
    return {{PointerNullabilityDiagnostic::ErrorCode::
                 AccessingMovedFromNonnullPointer,
             PointerNullabilityDiagnostic::Context::Other,
             getRangeModuloMacros(
                 CharSourceRange::getTokenRange(E->getSourceRange()), Ctx)}};

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
                   cxxOperatorCallExpr(isSmartPointerOperatorCall("=", 2),
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
    const TypeNullabilityDefaults &Defaults,
    const clang::NamedDecl *absl_nullable Callee = nullptr) {
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

  Diags.push_back(
      {PointerNullabilityDiagnostic::ErrorCode::ExpectedNonnull,
       PointerNullabilityDiagnostic::Context::Initializer,
       getRangeModuloMacros(
           CharSourceRange::getTokenRange(DefaultVal->getSourceRange()), Ctx),
       Callee, Parm.getIdentifier()});
}

void checkAnnotationsConsistentHelper(
    QualType T, QualType CanonicalT, const FileID &File,
    const FileID &CanonicalFile, SourceRange Range, SourceRange CanonicalRange,
    llvm::SmallVector<PointerNullabilityDiagnostic> &Diags,
    const TypeNullabilityDefaults &Defaults,
    PointerNullabilityDiagnostic::ErrorCode ErrorCode) {
  TypeNullability Nullability = getTypeNullability(T, File, Defaults);
  TypeNullability CanonicalNullability =
      getTypeNullability(CanonicalT, CanonicalFile, Defaults);
  if (Nullability != CanonicalNullability) {
    Diags.push_back({ErrorCode, PointerNullabilityDiagnostic::Context::Other,
                     CharSourceRange::getTokenRange(Range), nullptr, nullptr,
                     CharSourceRange::getTokenRange(CanonicalRange)});
  }
}

void checkAnnotationsConsistent(
    const ValueDecl *absl_nonnull VD,
    llvm::SmallVector<PointerNullabilityDiagnostic> &Diags,
    const TypeNullabilityDefaults &Defaults) {
  auto *CanonicalDecl = cast<ValueDecl>(VD->getCanonicalDecl());

  // We check against the annotation on the canonical decl, so if this is the
  // canonical decl, there is nothing to do.
  if (VD == CanonicalDecl) return;

  auto FileID =
      VD->getASTContext().getSourceManager().getFileID(VD->getLocation());
  auto CanonicalFileID =
      CanonicalDecl->getASTContext().getSourceManager().getFileID(
          CanonicalDecl->getLocation());
  const auto *Func = dyn_cast<FunctionDecl>(VD);
  if (Func != nullptr) {
    const auto *FuncCanonical = cast<FunctionDecl>(CanonicalDecl);
    unsigned int NumParams = Func->getNumParams();
    CHECK(NumParams <= FuncCanonical->getNumParams());
    for (unsigned int I = 0; I < NumParams; ++I) {
      const auto *Parm = Func->getParamDecl(I);
      const auto *ParmCanonical = FuncCanonical->getParamDecl(I);
      checkAnnotationsConsistentHelper(
          Parm->getType(), ParmCanonical->getType(), FileID, CanonicalFileID,
          Parm->getSourceRange(), ParmCanonical->getSourceRange(), Diags,
          Defaults,
          PointerNullabilityDiagnostic::ErrorCode::
              InconsistentAnnotationsForParameter);
    }
    checkAnnotationsConsistentHelper(
        Func->getReturnType(), FuncCanonical->getReturnType(), FileID,
        CanonicalFileID, Func->getReturnTypeSourceRange(),
        FuncCanonical->getReturnTypeSourceRange(), Diags, Defaults,
        PointerNullabilityDiagnostic::ErrorCode::
            InconsistentAnnotationsForReturn);
  } else {
    checkAnnotationsConsistentHelper(
        VD->getType(), CanonicalDecl->getType(), FileID, CanonicalFileID,
        VD->getSourceRange(), CanonicalDecl->getSourceRange(), Diags, Defaults,
        PointerNullabilityDiagnostic::ErrorCode::InconsistentAnnotations);
  }
}

CharSourceRange getMethodClosingBraceRange(const CXXMethodDecl &Method) {
  if (!Method.hasBody()) {
    // If the method doesn't have a body, fall back to using the entire method
    // source range (which should be the source range for the declaration).
    CharSourceRange::getTokenRange(Method.getSourceRange());
  }

  auto *Body = dyn_cast<CompoundStmt>(Method.getBody());
  if (Body == nullptr) {
    return CharSourceRange::getTokenRange(Method.getBody()->getSourceRange());
  }

  return CharSourceRange::getTokenRange(Body->getRBracLoc(),
                                        Body->getRBracLoc());
}

void diagnoseNonnullPointerFieldNullableAtExit(
    const FunctionDecl &Func,
    const dataflow::DataflowAnalysisState<PointerNullabilityLattice>
        &StateAtExit,
    DataflowAnalysisContext &AnalysisContext,
    llvm::SmallVector<PointerNullabilityDiagnostic> &Diags) {
  const auto *Method = dyn_cast<CXXMethodDecl>(&Func);
  if (Method == nullptr) return;

  // It isn't possible to access fields after the destructor exits, so don't
  // analyze destructors.
  if (isa<CXXDestructorDecl>(Method)) return;

  RecordStorageLocation *RecordLoc =
      StateAtExit.Env.getThisPointeeStorageLocation();
  if (RecordLoc == nullptr) return;

  const CXXRecordDecl *RD = Method->getParent();
  FieldSet ModeledFields =
      AnalysisContext.getModeledFields(RecordLoc->getType());
  for (const FieldDecl *Field : RD->fields()) {
    // If the field isn't modeled, we can't access it below -- but it also can't
    // be moved from because the method obviously doesn't refer to it.
    if (!ModeledFields.contains(Field)) continue;

    bool SmartPointer;
    if (isSupportedRawPointerType(Field->getType())) {
      SmartPointer = false;
    } else if (isSupportedSmartPointerType(Field->getType())) {
      SmartPointer = true;
    } else {
      continue;
    }

    TypeNullability FieldNullability =
        getTypeNullability(*Field, StateAtExit.Lattice.defaults());
    if (FieldNullability.empty() ||
        FieldNullability.front().concrete() != NullabilityKind::NonNull) {
      continue;
    }

    StorageLocation *FieldLoc = RecordLoc->getChild(*Field);
    if (FieldLoc == nullptr) continue;

    PointerValue *Val;
    if (SmartPointer) {
      Val = getPointerValueFromSmartPointer(
          cast<RecordStorageLocation>(FieldLoc), StateAtExit.Env);
    } else {
      Val = StateAtExit.Env.get<PointerValue>(*FieldLoc);
    }
    if (Val == nullptr) {
      Diags.push_back(
          {PointerNullabilityDiagnostic::ErrorCode::Untracked,
           PointerNullabilityDiagnostic::Context::Other,
           CharSourceRange::getTokenRange(Field->getSourceRange())});
      continue;
    }

    // We may see a value without nullability properties here, so guard against
    // that. This can happen, for example, if the field was initialized by the
    // framework on function entry but not accessed on some paths, hence not
    // giving us an opportunity to initialize the nullability properties on
    // those paths.
    if (!hasPointerNullState(*Val)) return;

    if (isNullable(*Val, StateAtExit.Env)) {
      Diags.push_back(
          {PointerNullabilityDiagnostic::ErrorCode::
               NonnullPointerFieldNullableAtExit,
           PointerNullabilityDiagnostic::Context::Other,
           getMethodClosingBraceRange(*Method), nullptr, nullptr,
           CharSourceRange::getTokenRange(Field->getSourceRange())});
    }
  }
}

DiagTransferFunc pointerNullabilityDiagnoserBefore() {
  // Almost all diagnosis callbacks should be run before the transfer function
  // has been applied because we want to check preconditions for the operation
  // performed by the `CFGElement`.
  return CFGMatchSwitchBuilder<const DiagTransferState,
                               SmallVector<PointerNullabilityDiagnostic>>()
      // `*`
      .CaseOfCFGStmt<UnaryOperator>(isPointerDereference(), diagnoseDereference)
      .CaseOfCFGStmt<CXXOperatorCallExpr>(isSmartPointerOperatorCall("*", 1),
                                          diagnoseSmartPointerDereference)
      // `[]`
      .CaseOfCFGStmt<ArraySubscriptExpr>(isPointerSubscript(),
                                         diagnoseSubscript)
      .CaseOfCFGStmt<CXXOperatorCallExpr>(isSmartPointerOperatorCall("[]", 2),
                                          diagnoseSmartPointerDereference)
      // `->`. Covers raw and smart pointers, because smart-pointer
      // `operator->` doesn't dereference. It just returns a pointer from which
      // a MemberExpr is built (with `->`), which does the actual dereference.
      .CaseOfCFGStmt<MemberExpr>(isPointerArrow(), diagnoseArrow)
      // `=` / `reset()`
      .CaseOfCFGStmt<BinaryOperator>(
          binaryOperator(hasOperatorName("="), hasLHS(isPointerExpr())),
          diagnoseAssignment)
      .CaseOfCFGStmt<CXXOperatorCallExpr>(isSmartPointerOperatorCall("=", 2),
                                          diagnoseSmartPointerAssignment)
      .CaseOfCFGStmt<CXXMemberCallExpr>(isSmartPointerMethodCall("reset"),
                                        diagnoseSmartPointerReset)
      // `--` / `++`
      .CaseOfCFGStmt<UnaryOperator>(
          unaryOperator(hasType(isSupportedRawPointer()),
                        anyOf(hasOperatorName("++"), hasOperatorName("--"))),
          diagnoseIncrementDecrement)
      // `+=` / `-=`
      .CaseOfCFGStmt<BinaryOperator>(
          binaryOperator(anyOf(hasOperatorName("+="), hasOperatorName("-=")),
                         hasOperands(isPointerExpr(), hasType(isInteger()))),
          diagnoseAddSubtractAssign)
      // `+` / `-`
      .CaseOfCFGStmt<BinaryOperator>(
          binaryOperator(
              anyOf(hasOperatorName("+"), hasOperatorName("-")),
              anyOf(hasOperands(isPointerExpr(), hasType(isInteger())),
                    hasOperands(hasType(isInteger()), isPointerExpr()))),
          diagnoseAddSubtractInteger)
      .CaseOfCFGStmt<BinaryOperator>(
          binaryOperator(hasOperatorName("-"),
                         hasOperands(isPointerExpr(), isPointerExpr())),
          diagnosePointerDifference)
      // Check compatibility of parameter assignments and return values.
      .CaseOfCFGStmt<CallExpr>(callExpr(), diagnoseCallExpr)
      .CaseOfCFGStmt<CXXConstructExpr>(cxxConstructExpr(),
                                       diagnoseConstructExpr)
      .CaseOfCFGStmt<ReturnStmt>(isPointerReturn(), diagnoseReturn)
      // Check compatibility of member initializers.
      .CaseOfCFGInit<CXXCtorInitializer>(isCtorMemberInitializer(),
                                         diagnoseMemberInitializer)
      // Check compatibility of initializer lists.
      .CaseOfCFGStmt<InitListExpr>(initListExpr(), diagnoseInitListExpr)
      .CaseOfCFGStmt<DeclStmt>(declStmt(), diagnoseDeclStmt)
      .Build();
}

DiagTransferFunc pointerNullabilityDiagnoserAfter(
    const AllowedMovedFromNonnullSmartPointerExprs &AllowedMovedFromNonnull) {
  return CFGMatchSwitchBuilder<const DiagTransferState,
                               SmallVector<PointerNullabilityDiagnostic>>()
      // `diagnoseMovedFromNonnullSmartPointer` needs to be run after the
      // transfer function has been applied so that the pointer and its
      // nullability properties are guaranteed be initialized (through
      // `ensureSmartPointerInitialized()`).
      .CaseOfCFGStmt<Expr>(
          expr(hasType(isSupportedSmartPointer()), isGLValue()),
          [&AllowedMovedFromNonnull](const Expr *absl_nonnull E,
                                     const MatchFinder::MatchResult &Result,
                                     const DiagTransferState &State)
              -> SmallVector<PointerNullabilityDiagnostic> {
            if (AllowedMovedFromNonnull.allowed(E)) return {};
            return diagnoseMovedFromNonnullSmartPointer(E, Result, State);
          })
      .Build();
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
  // Skip templated functions where we don't have full type information, but
  // allow template instantiations (not isTemplated()).
  if (VD->isTemplated()) return Diags;

  ASTContext &Ctx = VD->getASTContext();
  TypeNullabilityDefaults Defaults{Ctx, Pragmas};

  checkAnnotationsConsistent(VD, Diags, Defaults);

  const auto *Func = dyn_cast<FunctionDecl>(VD);
  if (Func == nullptr) return Diags;

  for (const ParmVarDecl *Parm : Func->parameters())
    checkParmVarDeclWithPointerDefaultArg(Ctx, *Parm, Diags, Defaults, Func);

  // Use `doesThisDeclarationHaveABody()` rather than `hasBody()` to ensure we
  // analyze forward-declared functions only once.
  if (!Func->doesThisDeclarationHaveABody()) return Diags;

  AllowedMovedFromNonnullSmartPointerExprs AllowedMovedFromNonnull(Func);

  // TODO(b/332565018): it would be nice to have some common pieces (limits,
  // adorning, error-handling) reused. diagnoseFunction() is too restrictive.
  auto CFG = dataflow::AdornedCFG::build(*Func);
  if (!CFG) return CFG.takeError();

  std::unique_ptr<dataflow::Solver> Solver = MakeSolver();
  DataflowAnalysisContext AnalysisContext(*Solver);
  Environment Env(AnalysisContext, *Func);

  PointerNullabilityAnalysis Analysis(Ctx, Env, Pragmas);

  dataflow::CFGEltCallbacks<PointerNullabilityAnalysis> PostAnalysisCallbacks;
  PostAnalysisCallbacks.Before =
      [&, Diagnoser = pointerNullabilityDiagnoserBefore()](
          const CFGElement &Elt,
          const dataflow::DataflowAnalysisState<PointerNullabilityLattice>
              &State) mutable {
        auto EltDiagnostics = Diagnoser(Elt, Ctx, {State.Lattice, State.Env});
        llvm::move(EltDiagnostics, std::back_inserter(Diags));
      };
  PostAnalysisCallbacks.After =
      [&,
       Diagnoser = pointerNullabilityDiagnoserAfter(AllowedMovedFromNonnull)](
          const CFGElement &Elt,
          const dataflow::DataflowAnalysisState<PointerNullabilityLattice>
              &State) mutable {
        auto EltDiagnostics = Diagnoser(Elt, Ctx, {State.Lattice, State.Env});
        llvm::move(EltDiagnostics, std::back_inserter(Diags));
      };
  auto Result = dataflow::runDataflowAnalysis(
      *CFG, Analysis, Env, PostAnalysisCallbacks, MaxBlockVisits);
  if (!Result) return Result.takeError();
  if (Solver->reachedLimit())
    return llvm::createStringError(llvm::errc::interrupted,
                                   "SAT solver timed out");

  const std::optional<
      dataflow::DataflowAnalysisState<PointerNullabilityLattice>>
      &ExitBlockState = (*Result)[CFG->getCFG().getExit().getBlockID()];
  if (ExitBlockState.has_value()) {
    diagnoseNonnullPointerFieldNullableAtExit(*Func, *ExitBlockState,
                                              AnalysisContext, Diags);
  }

  return Diags;
}

}  // namespace clang::tidy::nullability
