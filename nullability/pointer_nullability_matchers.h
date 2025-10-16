// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_POINTER_NULLABILITY_MATCHERS_H_
#define CRUBIT_NULLABILITY_POINTER_NULLABILITY_MATCHERS_H_

#include "nullability/type_nullability.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/Expr.h"
#include "clang/AST/ExprCXX.h"
#include "clang/AST/Stmt.h"
#include "clang/AST/TypeBase.h"
#include "clang/ASTMatchers/ASTMatchersInternal.h"
#include "clang/ASTMatchers/ASTMatchersMacros.h"
#include "clang/Basic/LLVM.h"
#include "llvm/ADT/ArrayRef.h"
#include "llvm/ADT/StringRef.h"

namespace clang {
namespace tidy {
namespace nullability {

namespace internal {

ast_matchers::internal::Matcher<Stmt> isSmartPointerMethodCallFunc(
    llvm::ArrayRef<const llvm::StringRef*> NameRefs);

}  // namespace internal

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
extern const ast_matchers::internal::VariadicFunction<
    ast_matchers::internal::Matcher<Stmt>, llvm::StringRef,
    internal::isSmartPointerMethodCallFunc>
    isSmartPointerMethodCall;
ast_matchers::internal::Matcher<Stmt> isSmartPointerFreeSwapCall();
ast_matchers::internal::Matcher<Stmt> isSmartPointerBoolConversionCall();
ast_matchers::internal::Matcher<Stmt> isSmartPointerFactoryCall();
ast_matchers::internal::Matcher<Stmt> isWrapUniqueCall();
ast_matchers::internal::Matcher<Stmt> isSmartPointerComparisonOpCall();
ast_matchers::internal::Matcher<Stmt> isSharedPtrCastCall();
ast_matchers::internal::Matcher<Stmt> isWeakPtrLockCall();
ast_matchers::internal::Matcher<Stmt> isSupportedPointerAccessorCall();
ast_matchers::internal::Matcher<Stmt> isStatusOrValueOrCall();

AST_MATCHER(Stmt, isNullPointerDefaultInit) {
  const auto* DefaultInit = dyn_cast<CXXDefaultInitExpr>(&Node);
  return DefaultInit != nullptr &&
         isNullPointerLiteral().matches(*DefaultInit->getExpr(), Finder,
                                        Builder);
}

// Checks if the given declaration is within the `absl` namespace.
// Traverses the parent namespaces up to the top-level namespace.
// For example, `absl::nested::f()` is considered within the `absl` namespace.
// The logic is similar to `isDeclaredInAbseilOrUtil()` in the value transferer:
// https://github.com/google/crubit/blob/55767d191778d2a421a229d3fe446a65912c9865/nullability/value_transferer.cc#L882
// This is unlike `Decl::isInStdNamespace()`
// (https://clang.llvm.org/doxygen/classclang_1_1Decl.html#a066b012f94431b5bba21d19715a274f4),
// which only traverses up inline namespaces
// (https://en.cppreference.com/w/cpp/language/namespace.html#Inline_namespaces)
// and "transparent contexts" such as those induced by unscoped enums
// (https://clang.llvm.org/doxygen/classclang_1_1DeclContext.html#a1d3b0ef59e3e789890485aa141c4712e).
AST_MATCHER(Decl, isInAbslNamespace) {
  const DeclContext* DC = Node.getDeclContext();
  if (DC == nullptr || DC->isTranslationUnit()) {
    return false;
  }

  // Traverse the parent namespaces up to the top-level namespace.
  while (DC->getParent() != nullptr && !DC->getParent()->isTranslationUnit()) {
    DC = DC->getParent();
  }

  if (!DC->isNamespace()) {
    return false;
  }

  const NamespaceDecl* ND = cast<NamespaceDecl>(DC);
  const IdentifierInfo* II = ND->getIdentifier();
  return II != nullptr && II->isStr("absl");
}

}  // namespace nullability
}  // namespace tidy
}  // namespace clang

#endif  // CRUBIT_NULLABILITY_POINTER_NULLABILITY_MATCHERS_H_
