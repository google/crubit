// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/googlesql_value_nullability_analysis.h"

#include <cassert>

#include "nullability/googlesql_value_nullability.h"
#include "nullability/googlesql_value_nullability_lattice.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Expr.h"
#include "clang/AST/ExprCXX.h"
#include "clang/AST/Stmt.h"
#include "clang/AST/Type.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/ASTMatchers/ASTMatchers.h"
#include "clang/Analysis/CFG.h"
#include "clang/Analysis/FlowSensitive/CFGMatchSwitch.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/MatchSwitch.h"
#include "clang/Analysis/FlowSensitive/StorageLocation.h"
#include "clang/Basic/LLVM.h"
#include "llvm/ADT/SmallVector.h"

namespace clang {
namespace tidy {
namespace nullability {

using ::clang::ast_matchers::anyOf;
using ::clang::ast_matchers::callee;
using ::clang::ast_matchers::callExpr;
using ::clang::ast_matchers::cxxConstructExpr;
using ::clang::ast_matchers::cxxConstructorDecl;
using ::clang::ast_matchers::cxxMemberCallExpr;
using ::clang::ast_matchers::cxxMethodDecl;
using ::clang::ast_matchers::cxxOperatorCallExpr;
using ::clang::ast_matchers::cxxRecordDecl;
using ::clang::ast_matchers::hasAnyName;
using ::clang::ast_matchers::hasArgument;
using ::clang::ast_matchers::hasCanonicalType;
using ::clang::ast_matchers::hasDeclaration;
using ::clang::ast_matchers::hasName;
using ::clang::ast_matchers::hasOverloadedOperatorName;
using ::clang::ast_matchers::hasType;
using ::clang::ast_matchers::isCopyConstructor;
using ::clang::ast_matchers::isDefaultConstructor;
using ::clang::ast_matchers::isMoveConstructor;
using ::clang::ast_matchers::isStaticStorageClass;
using ::clang::ast_matchers::matchesName;
using ::clang::ast_matchers::MatchFinder;
using ::clang::ast_matchers::ofClass;
using ::clang::ast_matchers::recordType;
using ::clang::ast_matchers::unless;
using dataflow::CFGMatchSwitchBuilder;
using dataflow::RecordStorageLocation;
using dataflow::TransferState;

// Helper to ensure a googlesql::Value has a mapped null state.
static void ensureGoogleSqlValueInitialized(RecordStorageLocation& Loc,
                                            dataflow::Environment& Env) {
  if (!hasGoogleSqlValueNullState(Loc, Env)) {
    auto& A = Env.arena();
    initGoogleSqlValueNullState(Loc, Env, &A.makeAtomRef(A.makeAtom()));
  }
}

// Transfer function for default constructor: Value() -> Null.
static void transferDefaultConstructor(
    const CXXConstructExpr* Ctor, const MatchFinder::MatchResult& Result,
    TransferState<GoogleSqlValueNullabilityLattice>& State) {
  auto& A = State.Env.arena();
  RecordStorageLocation& Loc = State.Env.getResultObjectLocation(*Ctor);

  // Overwrite null state to definitely null (default ctor is null).
  setGoogleSqlValueNullState(Loc, State.Env, &A.makeLiteral(true));
}

// Transfer function for copy/move constructors: inherit state.
static void transferCopyOrMoveConstructor(
    const CXXConstructExpr* Ctor, const MatchFinder::MatchResult& Result,
    TransferState<GoogleSqlValueNullabilityLattice>& State) {
  RecordStorageLocation& Loc = State.Env.getResultObjectLocation(*Ctor);

  const Expr* Arg = Ctor->getArg(0);
  RecordStorageLocation* SrcLoc = nullptr;
  if (Arg->isGLValue()) {
    SrcLoc = State.Env.get<RecordStorageLocation>(*Arg);
  } else {
    SrcLoc = &State.Env.getResultObjectLocation(*Arg);
  }

  if (SrcLoc) {
    ensureGoogleSqlValueInitialized(*SrcLoc, State.Env);
    auto SrcState = getGoogleSqlValueNullState(*SrcLoc, State.Env);
    setGoogleSqlValueNullState(Loc, State.Env, SrcState.IsNull);
  }
}

// Transfer function for factory methods that create non-null values
// (e.g. Value::Int64(...)) -> Non-Null.
static void transferFactoryMethod(
    const CallExpr* CE, const MatchFinder::MatchResult& Result,
    TransferState<GoogleSqlValueNullabilityLattice>& State) {
  auto& A = State.Env.arena();
  RecordStorageLocation& Loc = State.Env.getResultObjectLocation(*CE);

  // Overwrite null state to definitely non-null.
  setGoogleSqlValueNullState(Loc, State.Env, &A.makeLiteral(false));
}

// Transfer function for factory methods that create null values
// (e.g. Value::NullInt64()) -> Null.
// These methods explicitly create a null value, so we mark the state as null.
static void transferNullFactoryMethod(
    const CallExpr* CE, const MatchFinder::MatchResult& Result,
    TransferState<GoogleSqlValueNullabilityLattice>& State) {
  auto& A = State.Env.arena();
  RecordStorageLocation& Loc = State.Env.getResultObjectLocation(*CE);

  // Overwrite null state to definitely null.
  setGoogleSqlValueNullState(Loc, State.Env, &A.makeLiteral(true));
}

// Transfer function for .is_null() call: tie result to internal IsNull formula.
static void transferIsNullCall(
    const CXXMemberCallExpr* MCE, const MatchFinder::MatchResult& Result,
    TransferState<GoogleSqlValueNullabilityLattice>& State) {
  auto& A = State.Env.arena();

  RecordStorageLocation* RecordLoc =
      dataflow::getImplicitObjectLocation(*MCE, State.Env);
  if (!RecordLoc) return;

  ensureGoogleSqlValueInitialized(*RecordLoc, State.Env);
  auto NullState = getGoogleSqlValueNullState(*RecordLoc, State.Env);

  if (NullState.IsNull != nullptr) {
    State.Env.setValue(*MCE, A.makeBoolValue(*NullState.IsNull));
  } else {
    State.Env.setValue(*MCE, A.makeTopValue());
  }
}

// Transfer function for assignment operator: v1 = v2.
static void transferAssignment(
    const CXXOperatorCallExpr* OCE, const MatchFinder::MatchResult& Result,
    TransferState<GoogleSqlValueNullabilityLattice>& State) {
  auto* DestLoc = cast_or_null<RecordStorageLocation>(
      State.Env.getStorageLocation(*OCE->getArg(0)));
  if (!DestLoc) return;

  const Expr* SrcExpr = OCE->getArg(1);
  RecordStorageLocation* SrcLoc = nullptr;
  if (SrcExpr->isGLValue()) {
    SrcLoc = State.Env.get<RecordStorageLocation>(*SrcExpr);
  } else {
    SrcLoc = &State.Env.getResultObjectLocation(*SrcExpr);
  }

  if (SrcLoc) {
    ensureGoogleSqlValueInitialized(*SrcLoc, State.Env);
    auto SrcState = getGoogleSqlValueNullState(*SrcLoc, State.Env);
    setGoogleSqlValueNullState(*DestLoc, State.Env, SrcState.IsNull);
  }
}

// Matcher for googlesql::Value type.
static auto isGoogleSqlValue() {
  return hasType(hasCanonicalType(
      recordType(hasDeclaration(cxxRecordDecl(hasName("googlesql::Value"))))));
}

// Build the transferer CFGMatchSwitch.
static dataflow::CFGMatchSwitch<TransferState<GoogleSqlValueNullabilityLattice>>
buildTransferer() {
  return CFGMatchSwitchBuilder<
             TransferState<GoogleSqlValueNullabilityLattice>>()
      // Default constructor: Value().
      .CaseOfCFGStmt<CXXConstructExpr>(
          cxxConstructExpr(
              hasDeclaration(cxxConstructorDecl(isDefaultConstructor())),
              isGoogleSqlValue()),
          transferDefaultConstructor)
      // Copy/Move constructor.
      .CaseOfCFGStmt<CXXConstructExpr>(
          cxxConstructExpr(hasDeclaration(cxxConstructorDecl(anyOf(
                               isCopyConstructor(), isMoveConstructor()))),
                           isGoogleSqlValue()),
          transferCopyOrMoveConstructor)
      // Null factory methods: e.g. Value::NullInt64().
      // Explicitly creates a null value.
      .CaseOfCFGStmt<CallExpr>(
          callExpr(
              callee(cxxMethodDecl(isStaticStorageClass(),
                                   ofClass(hasName("googlesql::Value")),
                                   matchesName("::googlesql::Value::Null"))),
              isGoogleSqlValue()),
          transferNullFactoryMethod)
      // Factory methods: e.g. Value::Int64(...). Creates a non-null value.
      // We exclude the Null* methods handled above.
      .CaseOfCFGStmt<CallExpr>(
          callExpr(
              callee(cxxMethodDecl(
                  isStaticStorageClass(), ofClass(hasName("googlesql::Value")),
                  unless(matchesName("::googlesql::Value::Null")))),
              isGoogleSqlValue()),
          transferFactoryMethod)
      // .is_null() call.
      .CaseOfCFGStmt<CXXMemberCallExpr>(
          cxxMemberCallExpr(callee(cxxMethodDecl(
              hasName("is_null"), ofClass(hasName("googlesql::Value"))))),
          transferIsNullCall)
      // operator=.
      .CaseOfCFGStmt<CXXOperatorCallExpr>(
          cxxOperatorCallExpr(hasOverloadedOperatorName("="),
                              hasArgument(0, isGoogleSqlValue())),
          transferAssignment)
      .Build();
}

// Diagnosis lambda for accessor calls.
static llvm::SmallVector<const Stmt*, 1> diagnoseAccessorCall(
    const CXXMemberCallExpr* MCE, const MatchFinder::MatchResult& Result,
    const dataflow::Environment& Env) {
  auto& A = Env.arena();

  RecordStorageLocation* RecordLoc =
      dataflow::getImplicitObjectLocation(*MCE, Env);
  if (!RecordLoc) return {};

  if (!hasGoogleSqlValueNullState(*RecordLoc, Env)) {
    // Unmodeled value, conservatively assume unsafe.
    return {MCE};
  }

  auto NullState = getGoogleSqlValueNullState(*RecordLoc, Env);
  if (NullState.IsNull == nullptr) {
    // Top (unknown), conservatively assume unsafe.
    return {MCE};
  }

  // Verify that we can prove the value is NOT null.
  if (!Env.proves(A.makeNot(*NullState.IsNull))) {
    return {MCE};  // Unsafe access!
  }

  return {};  // Safe access.
}

// Build the diagnoser CFGMatchSwitch.
static dataflow::CFGMatchSwitch<const dataflow::Environment,
                                llvm::SmallVector<const Stmt*, 1>>
buildDiagnoser() {
  return CFGMatchSwitchBuilder<const dataflow::Environment,
                               llvm::SmallVector<const Stmt*, 1>>()
      .CaseOfCFGStmt<CXXMemberCallExpr>(
          cxxMemberCallExpr(callee(cxxMethodDecl(
              ofClass(hasName("googlesql::Value")),
              // Catch-all for the entire googlesql::Value class, except for the
              // two methods we know are safe to call on a null object!
              unless(hasAnyName("is_null", "type"))))),
          diagnoseAccessorCall)
      .Build();
}

// Dispatches to specific transfer functions based on the CFG element type.
// This method is called by the Clang Dataflow framework for each element in the
// CFG.
void GoogleSqlValueNullabilityAnalysis::transfer(
    const CFGElement& Elt, GoogleSqlValueNullabilityLattice& Lattice,
    dataflow::Environment& Env) {
  static auto* Switch = new dataflow::CFGMatchSwitch<
      TransferState<GoogleSqlValueNullabilityLattice>>(buildTransferer());
  TransferState<GoogleSqlValueNullabilityLattice> State{Lattice, Env};
  (*Switch)(Elt, getASTContext(), State);
}

// The diagnosis function for the analysis.
// Checks for unsafe accesses to googlesql::Value objects and returns the
// statements that violate null safety.
llvm::SmallVector<const Stmt*, 1> diagnoseGoogleSqlValueNullability(
    const CFGElement& Elt, ASTContext& ASTCtx,
    const dataflow::Environment& Env) {
  static auto* Switch =
      new dataflow::CFGMatchSwitch<const dataflow::Environment,
                                   llvm::SmallVector<const Stmt*, 1>>(
          buildDiagnoser());
  return (*Switch)(Elt, ASTCtx, Env);
}

}  // namespace nullability
}  // namespace tidy
}  // namespace clang
