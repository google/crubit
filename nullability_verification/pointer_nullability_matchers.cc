// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability_verification/pointer_nullability_matchers.h"

#include "clang/ASTMatchers/ASTMatchers.h"
#include "llvm/ADT/StringRef.h"

namespace clang {
namespace tidy {
namespace nullability {

using ast_matchers::binaryOperator;
using ast_matchers::booleanType;
using ast_matchers::expr;
using ast_matchers::hasImplicitDestinationType;
using ast_matchers::hasOperands;
using ast_matchers::hasOperatorName;
using ast_matchers::hasSourceExpression;
using ast_matchers::hasType;
using ast_matchers::hasUnaryOperand;
using ast_matchers::ignoringImplicit;
using ast_matchers::implicitCastExpr;
using ast_matchers::isAnyPointer;
using ast_matchers::nullPointerConstant;
using ast_matchers::unaryOperator;
using ast_matchers::internal::Matcher;

Matcher<Stmt> isPointerExpr() { return expr(hasType(isAnyPointer())); }

Matcher<Stmt> isPointerDereference() {
  return unaryOperator(hasOperatorName("*"), hasUnaryOperand(isPointerExpr()));
}

Matcher<Stmt> isNEQNullBinOp(llvm::StringRef BindID) {
  return binaryOperator(
      hasOperatorName("!="),
      hasOperands(ignoringImplicit(nullPointerConstant()),
                  expr(hasType(isAnyPointer())).bind(BindID)));
}

Matcher<Stmt> isImplicitCastPtrToBool() {
  return implicitCastExpr(hasSourceExpression(isPointerExpr()),
                          hasImplicitDestinationType(booleanType()));
}
}  // namespace nullability
}  // namespace tidy
}  // namespace clang
