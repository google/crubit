// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/pointer_nullability_matchers.h"

#include "clang/AST/DeclCXX.h"
#include "clang/AST/OperationKinds.h"
#include "clang/AST/Stmt.h"
#include "clang/AST/Type.h"
#include "clang/ASTMatchers/ASTMatchers.h"
#include "clang/ASTMatchers/ASTMatchersMacros.h"

namespace clang::tidy::nullability {

using ast_matchers::anyOf;
using ast_matchers::argumentCountIs;
using ast_matchers::arraySubscriptExpr;
using ast_matchers::binaryOperator;
using ast_matchers::booleanType;
using ast_matchers::callee;
using ast_matchers::callExpr;
using ast_matchers::compoundStmt;
using ast_matchers::cxxConstructExpr;
using ast_matchers::cxxConversionDecl;
using ast_matchers::cxxCtorInitializer;
using ast_matchers::cxxMemberCallExpr;
using ast_matchers::cxxMethodDecl;
using ast_matchers::cxxOperatorCallExpr;
using ast_matchers::cxxRecordDecl;
using ast_matchers::cxxThisExpr;
using ast_matchers::decl;
using ast_matchers::expr;
using ast_matchers::functionDecl;
using ast_matchers::has;
using ast_matchers::hasAnyName;
using ast_matchers::hasAnyOperatorName;
using ast_matchers::hasAnyOverloadedOperatorName;
using ast_matchers::hasArgument;
using ast_matchers::hasBase;
using ast_matchers::hasBody;
using ast_matchers::hasCanonicalType;
using ast_matchers::hasCastKind;
using ast_matchers::hasDeclaration;
using ast_matchers::hasName;
using ast_matchers::hasOperands;
using ast_matchers::hasOperatorName;
using ast_matchers::hasOverloadedOperatorName;
using ast_matchers::hasReturnValue;
using ast_matchers::hasType;
using ast_matchers::hasUnaryOperand;
using ast_matchers::ignoringParenImpCasts;
using ast_matchers::implicitCastExpr;
using ast_matchers::isArrow;
using ast_matchers::isConst;
using ast_matchers::isInStdNamespace;
using ast_matchers::isMemberInitializer;
using ast_matchers::memberExpr;
using ast_matchers::parameterCountIs;
using ast_matchers::returnStmt;
using ast_matchers::statementCountIs;
using ast_matchers::thisPointerType;
using ast_matchers::unaryOperator;
using ast_matchers::unless;
using ast_matchers::internal::Matcher;

namespace {

AST_MATCHER(QualType, isNullPtrType) { return Node->isNullPtrType(); }

}  // namespace

Matcher<Stmt> isPointerExpr() { return expr(hasType(isSupportedRawPointer())); }
Matcher<Stmt> isNullPointerLiteral() {
  return implicitCastExpr(anyOf(hasCastKind(CK_NullToPointer),
                                hasCastKind(CK_NullToMemberPointer)));
}
Matcher<Stmt> isAddrOf() { return unaryOperator(hasOperatorName("&")); }
Matcher<Stmt> isPointerDereference() {
  return unaryOperator(hasOperatorName("*"), hasUnaryOperand(isPointerExpr()));
}
Matcher<Stmt> isPointerSubscript() {
  return arraySubscriptExpr(hasBase(isPointerExpr()));
}
Matcher<Stmt> isPointerCheckBinOp() {
  return binaryOperator(hasAnyOperatorName("!=", "=="),
                        hasOperands(isPointerExpr(), isPointerExpr()));
}
Matcher<Stmt> isImplicitCastPointerToBool() {
  return implicitCastExpr(hasCastKind(CK_PointerToBoolean));
}
Matcher<Stmt> isMemberOfPointerType() {
  return memberExpr(hasType(isSupportedRawPointer()));
}
Matcher<Stmt> isPointerArrow() { return memberExpr(isArrow()); }
Matcher<Stmt> isPointerReturn() {
  return returnStmt(hasReturnValue(hasType(isSupportedPointer())));
}
Matcher<CXXCtorInitializer> isCtorMemberInitializer() {
  return cxxCtorInitializer(isMemberInitializer());
}

Matcher<Stmt> isZeroParamConstMemberCall() {
  return cxxMemberCallExpr(
      callee(cxxMethodDecl(parameterCountIs(0), isConst())));
}

Matcher<Stmt> isNonConstMemberCall() {
  return cxxMemberCallExpr(callee(cxxMethodDecl(unless(isConst()))));
}

Matcher<Stmt> isSmartPointerGlValue() {
  return expr(hasType(isSupportedSmartPointer()), isGLValue());
}

Matcher<Stmt> isSmartPointerConstructor() {
  return cxxConstructExpr(hasType(isSupportedSmartPointer()));
}

Matcher<Stmt> isSmartPointerOperatorCall(llvm::StringRef Name) {
  return cxxOperatorCallExpr(
      hasOverloadedOperatorName(Name),
      hasArgument(0, hasType(isSupportedSmartPointer())));
}

Matcher<Stmt> isSmartPointerMethodCall(llvm::StringRef Name) {
  return cxxMemberCallExpr(thisPointerType(isSupportedSmartPointer()),
                           callee(cxxMethodDecl(hasName(Name))));
}

Matcher<Stmt> isSmartPointerFreeSwapCall() {
  return callExpr(callee(functionDecl(isInStdNamespace(), hasName("swap"))),
                  argumentCountIs(2),
                  hasArgument(0, hasType(isSupportedSmartPointer())),
                  hasArgument(1, hasType(isSupportedSmartPointer())));
}

Matcher<Stmt> isSmartPointerBoolConversionCall() {
  return cxxMemberCallExpr(thisPointerType(isSupportedSmartPointer()),
                           callee(cxxConversionDecl()), hasType(booleanType()));
}

Matcher<Stmt> isSmartPointerFactoryCall() {
  return callExpr(
      hasType(isSupportedSmartPointer()),
      callee(functionDecl(
          isInStdNamespace(),
          hasAnyName("make_unique", "make_unique_for_overwrite", "make_shared",
                     "make_shared_for_overwrite", "allocate_shared",
                     "allocate_shared_for_overwrite"))));
}

Matcher<Stmt> isSmartPointerComparisonOpCall() {
  return cxxOperatorCallExpr(
      hasAnyOverloadedOperatorName("==", "!="), argumentCountIs(2),
      anyOf(hasArgument(0, hasType(isSupportedSmartPointer())),
            hasArgument(1, hasType(isSupportedSmartPointer()))),
      // If one of the arguments isn't a smart pointer, it has to be
      // `std::nullptr_t`.
      hasArgument(0, anyOf(hasType(isSupportedSmartPointer()),
                           hasType(isNullPtrType()))),
      hasArgument(1, anyOf(hasType(isSupportedSmartPointer()),
                           hasType(isNullPtrType()))));
}

Matcher<Stmt> isSharedPtrCastCall() {
  return callExpr(
      argumentCountIs(1),
      hasArgument(0, hasType(hasCanonicalType(hasDeclaration(cxxRecordDecl(
                         isInStdNamespace(), hasName("shared_ptr")))))),
      callee(functionDecl(
          isInStdNamespace(),
          hasAnyName("static_pointer_cast", "dynamic_pointer_cast",
                     "const_pointer_cast", "reinterpret_pointer_cast"))));
}

Matcher<Stmt> isWeakPtrLockCall() {
  return cxxMemberCallExpr(
      thisPointerType(cxxRecordDecl(isInStdNamespace(), hasName("weak_ptr"))),
      callee(cxxMethodDecl(hasName("lock"))));
}

Matcher<Stmt> isSupportedPointerAccessorCall() {
  return cxxMemberCallExpr(callee(cxxMethodDecl(hasBody(compoundStmt(
      statementCountIs(1),
      has(returnStmt(has(implicitCastExpr(
          hasCastKind(CK_LValueToRValue),
          has(ignoringParenImpCasts(
              memberExpr(has(ignoringParenImpCasts(cxxThisExpr())),
                         hasType(isSupportedRawPointer()),
                         hasDeclaration(decl().bind("member-decl"))))))))))))));
}

}  // namespace clang::tidy::nullability
