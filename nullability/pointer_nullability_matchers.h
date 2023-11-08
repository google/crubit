// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_POINTER_NULLABILITY_MATCHERS_H_
#define CRUBIT_NULLABILITY_POINTER_NULLABILITY_MATCHERS_H_

#include "nullability/type_nullability.h"
#include "clang/ASTMatchers/ASTMatchersInternal.h"
#include "clang/ASTMatchers/ASTMatchersMacros.h"

namespace clang {
namespace tidy {
namespace nullability {

AST_MATCHER(QualType, isSupportedPointer) {
  return isSupportedPointerType(Node);
}

ast_matchers::internal::Matcher<Stmt> isPointerExpr();
ast_matchers::internal::Matcher<Stmt> isMemberOfPointerType();
ast_matchers::internal::Matcher<Stmt> isPointerArrow();
ast_matchers::internal::Matcher<Stmt> isCXXThisExpr();
ast_matchers::internal::Matcher<Stmt> isNullPointerLiteral();
ast_matchers::internal::Matcher<Stmt> isAddrOf();
ast_matchers::internal::Matcher<Stmt> isPointerDereference();
ast_matchers::internal::Matcher<Stmt> isPointerCheckBinOp();
ast_matchers::internal::Matcher<Stmt> isImplicitCastPointerToBool();
ast_matchers::internal::Matcher<Stmt> isCallExpr();
ast_matchers::internal::Matcher<Stmt> isPointerReturn();
ast_matchers::internal::Matcher<Stmt> isConstructExpr();
ast_matchers::internal::Matcher<CXXCtorInitializer> isCtorMemberInitializer();
ast_matchers::internal::Matcher<Stmt> isZeroParamConstMemberCall();
ast_matchers::internal::Matcher<Stmt> isNonConstMemberCall();
ast_matchers::internal::Matcher<Stmt> isSupportedPointerAccessorCall();

}  // namespace nullability
}  // namespace tidy
}  // namespace clang

#endif  // CRUBIT_NULLABILITY_POINTER_NULLABILITY_MATCHERS_H_
