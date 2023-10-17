// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/pointer_nullability_matchers.h"

#include "clang/AST/DeclCXX.h"
#include "clang/AST/OperationKinds.h"
#include "clang/AST/Stmt.h"
#include "clang/ASTMatchers/ASTMatchers.h"

namespace clang::tidy::nullability {

using ast_matchers::anyOf;
using ast_matchers::binaryOperator;
using ast_matchers::callee;
using ast_matchers::callExpr;
using ast_matchers::compoundStmt;
using ast_matchers::cxxConstructExpr;
using ast_matchers::cxxCtorInitializer;
using ast_matchers::cxxMemberCallExpr;
using ast_matchers::cxxMethodDecl;
using ast_matchers::cxxThisExpr;
using ast_matchers::decl;
using ast_matchers::expr;
using ast_matchers::has;
using ast_matchers::hasAnyOperatorName;
using ast_matchers::hasBody;
using ast_matchers::hasCastKind;
using ast_matchers::hasDeclaration;
using ast_matchers::hasOperands;
using ast_matchers::hasOperatorName;
using ast_matchers::hasReturnValue;
using ast_matchers::hasType;
using ast_matchers::hasUnaryOperand;
using ast_matchers::ignoringParenImpCasts;
using ast_matchers::implicitCastExpr;
using ast_matchers::isArrow;
using ast_matchers::isConst;
using ast_matchers::isMemberInitializer;
using ast_matchers::memberExpr;
using ast_matchers::returnStmt;
using ast_matchers::statementCountIs;
using ast_matchers::unaryOperator;
using ast_matchers::unless;
using ast_matchers::internal::Matcher;

Matcher<Stmt> isPointerExpr() { return expr(hasType(isSupportedPointer())); }
Matcher<Stmt> isNullPointerLiteral() {
  return implicitCastExpr(anyOf(hasCastKind(CK_NullToPointer),
                                hasCastKind(CK_NullToMemberPointer)));
}
Matcher<Stmt> isAddrOf() { return unaryOperator(hasOperatorName("&")); }
Matcher<Stmt> isPointerDereference() {
  return unaryOperator(hasOperatorName("*"), hasUnaryOperand(isPointerExpr()));
}
Matcher<Stmt> isPointerCheckBinOp() {
  return binaryOperator(hasAnyOperatorName("!=", "=="),
                        hasOperands(isPointerExpr(), isPointerExpr()));
}
Matcher<Stmt> isImplicitCastPointerToBool() {
  return implicitCastExpr(hasCastKind(CK_PointerToBoolean));
}
Matcher<Stmt> isMemberOfPointerType() {
  return memberExpr(hasType(isSupportedPointer()));
}
Matcher<Stmt> isPointerArrow() { return memberExpr(isArrow()); }
Matcher<Stmt> isCXXThisExpr() { return cxxThisExpr(); }
Matcher<Stmt> isCallExpr() { return callExpr(); }
Matcher<Stmt> isPointerReturn() {
  return returnStmt(hasReturnValue(hasType(isSupportedPointer())));
}
Matcher<Stmt> isConstructExpr() { return cxxConstructExpr(); }
Matcher<CXXCtorInitializer> isCtorMemberInitializer() {
  return cxxCtorInitializer(isMemberInitializer());
}

Matcher<Stmt> isNonConstMemberCall() {
  return cxxMemberCallExpr(callee(cxxMethodDecl(unless(isConst()))));
}

Matcher<Stmt> isSupportedPointerAccessorCall() {
  return cxxMemberCallExpr(callee(cxxMethodDecl(hasBody(compoundStmt(
      statementCountIs(1),
      has(returnStmt(has(implicitCastExpr(
          hasCastKind(CK_LValueToRValue),
          has(ignoringParenImpCasts(
              memberExpr(has(ignoringParenImpCasts(cxxThisExpr())),
                         hasType(isSupportedPointer()),
                         hasDeclaration(decl().bind("member-decl"))))))))))))));
}

}  // namespace clang::tidy::nullability
