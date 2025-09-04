// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_POINTER_NULLABILITY_MATCHERS_H_
#define CRUBIT_NULLABILITY_POINTER_NULLABILITY_MATCHERS_H_

#include "nullability/type_nullability.h"
#include "clang/AST/ExprCXX.h"
#include "clang/ASTMatchers/ASTMatchersInternal.h"
#include "clang/ASTMatchers/ASTMatchersMacros.h"

namespace clang {
namespace tidy {
namespace nullability {

AST_MATCHER(QualType, isSupportedPointer) {
  return isSupportedPointerType(Node);
}

AST_MATCHER(QualType, isSupportedRawPointer) {
  return isSupportedRawPointerType(Node);
}

AST_MATCHER(QualType, isSupportedSmartPointer) {
  return isSupportedSmartPointerType(Node);
}

AST_MATCHER(Expr, isGLValue) { return Node.isGLValue(); }

AST_MATCHER(Stmt, isRawPointerValueInit) {
  const auto* ValueInit = dyn_cast<CXXScalarValueInitExpr>(&Node);
  return ValueInit != nullptr &&
         isSupportedRawPointerType(ValueInit->getType());
}

AST_MATCHER(Stmt, isRawPointerImplicitValueInit) {
  const auto* ValueInit = dyn_cast<ImplicitValueInitExpr>(&Node);
  return ValueInit != nullptr &&
         isSupportedRawPointerType(ValueInit->getType());
}

AST_MATCHER(QualType, isNullPtrType) { return Node->isNullPtrType(); }

ast_matchers::internal::Matcher<Stmt> isPointerExpr();
ast_matchers::internal::Matcher<Stmt> isMemberOfPointerType();
ast_matchers::internal::Matcher<Stmt> isPointerArrow();
ast_matchers::internal::Matcher<Stmt> isNullPointerLiteral();
ast_matchers::internal::Matcher<Stmt> isAddrOf();
ast_matchers::internal::Matcher<Stmt> isPointerDereference();
ast_matchers::internal::Matcher<Stmt> isPointerSubscript();
ast_matchers::internal::Matcher<Stmt> isPointerCheckBinOp();
ast_matchers::internal::Matcher<Stmt> isPointerIncOrDec();
ast_matchers::internal::Matcher<Stmt> isPointerAddOrSubAssign();
ast_matchers::internal::Matcher<Stmt> isImplicitCastPointerToBool();
ast_matchers::internal::Matcher<Stmt> isPointerReturn();
ast_matchers::internal::Matcher<CXXCtorInitializer> isCtorMemberInitializer();
ast_matchers::internal::Matcher<Stmt> isZeroParamConstMemberCall();
ast_matchers::internal::Matcher<Stmt> isZeroParamConstMemberOperatorCall();
ast_matchers::internal::Matcher<Stmt> isNonConstMemberCall();
ast_matchers::internal::Matcher<Stmt> isNonConstMemberOperatorCall();
ast_matchers::internal::Matcher<Stmt> isSmartPointerArrowMemberExpr();
ast_matchers::internal::Matcher<Stmt> isSmartPointerConstructor();
ast_matchers::internal::Matcher<Stmt> isSmartPointerOperatorCall(
    llvm::StringRef Name, int NumArgs);
ast_matchers::internal::Matcher<Stmt> isSmartPointerMethodCall(
    llvm::StringRef Name);
ast_matchers::internal::Matcher<Stmt> isSmartPointerFreeSwapCall();
ast_matchers::internal::Matcher<Stmt> isSmartPointerBoolConversionCall();
ast_matchers::internal::Matcher<Stmt> isSmartPointerFactoryCall();
ast_matchers::internal::Matcher<Stmt> isSmartPointerComparisonOpCall();
ast_matchers::internal::Matcher<Stmt> isSharedPtrCastCall();
ast_matchers::internal::Matcher<Stmt> isWeakPtrLockCall();
ast_matchers::internal::Matcher<Stmt> isSupportedPointerAccessorCall();

AST_MATCHER(Stmt, isNullPointerDefaultInit) {
  const auto* DefaultInit = dyn_cast<CXXDefaultInitExpr>(&Node);
  return DefaultInit != nullptr &&
         isNullPointerLiteral().matches(*DefaultInit->getExpr(), Finder,
                                        Builder);
}

}  // namespace nullability
}  // namespace tidy
}  // namespace clang

#endif  // CRUBIT_NULLABILITY_POINTER_NULLABILITY_MATCHERS_H_
