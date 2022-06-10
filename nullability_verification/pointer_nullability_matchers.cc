// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability_verification/pointer_nullability_matchers.h"

#include "clang/AST/OperationKinds.h"
#include "clang/ASTMatchers/ASTMatchers.h"

namespace clang {
namespace tidy {
namespace nullability {

using ast_matchers::anyOf;
using ast_matchers::binaryOperator;
using ast_matchers::declRefExpr;
using ast_matchers::expr;
using ast_matchers::hasAnyOperatorName;
using ast_matchers::hasCastKind;
using ast_matchers::hasOperands;
using ast_matchers::hasOperatorName;
using ast_matchers::hasType;
using ast_matchers::hasUnaryOperand;
using ast_matchers::ignoringImplicit;
using ast_matchers::implicitCastExpr;
using ast_matchers::isAnyPointer;
using ast_matchers::isArrow;
using ast_matchers::memberExpr;
using ast_matchers::nullPointerConstant;
using ast_matchers::unaryOperator;
using ast_matchers::internal::Matcher;

Matcher<Stmt> isPointerExpr() { return expr(hasType(isAnyPointer())); }
Matcher<Stmt> isPointerVariableReference() {
  return declRefExpr(hasType(isAnyPointer()));
}
Matcher<Stmt> isNullPointerLiteral() {
  return expr(ignoringImplicit(nullPointerConstant()));
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
Matcher<Stmt> isMemberExprInvolvingPointers() {
  return memberExpr(anyOf(isArrow(), hasType(isAnyPointer())));
}
}  // namespace nullability
}  // namespace tidy
}  // namespace clang
