// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/forwarding_functions.h"

#include <cassert>

#include "absl/base/nullability.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/ExprCXX.h"
#include "clang/AST/TemplateBase.h"
#include "clang/Basic/LLVM.h"
#include "llvm/Support/Debug.h"
#include "llvm/Support/ErrorHandling.h"
#include "llvm/Support/raw_ostream.h"

namespace clang::tidy::nullability {

namespace {

bool isStdMakeUniqueWithNontrivalConstructor(const FunctionDecl &FD) {
  if (!FD.getDeclName().isIdentifier()) return false;
  if (FD.getName() != "make_unique") return false;
  const auto *Namespace = dyn_cast_or_null<NamespaceDecl>(FD.getDeclContext());
  if (Namespace == nullptr) return false;
  if (!Namespace->isStdNamespace()) return false;

  // Check if it is (a) make_unique<T>(args...), or (b) make_unique<T[]>(size).
  // The array version would just call the 0-arg constructor, which isn't
  // very interesting to diagnose. We also want T in the first version to
  // be a record type so that there is a constructor call to analyze.
  const TemplateArgumentList *TemplateArgs = FD.getTemplateSpecializationArgs();
  if (!TemplateArgs) return false;
  ArrayRef<TemplateArgument> TemplateArgsArray = TemplateArgs->asArray();
  if (TemplateArgsArray.empty()) return false;
  const TemplateArgument &FirstArg = TemplateArgsArray.front();
  assert(FirstArg.getKind() == TemplateArgument::Type);
  if (FirstArg.getKind() != TemplateArgument::Type) return false;
  return FirstArg.getAsType()->isRecordType();
}

const Expr *findMakeUniqueNewExprInitializer(absl::Nonnull<const Stmt *> S) {
  // Do a simple walk over the children, which should be sufficient for
  // make_unique. RecursiveASTVisitor can also work but is supposed to be more
  // expensive to compile. make_unique should have a `new T( ... )` in its
  // body. Find that, and extract the Initializer.
  if (auto *NE = dyn_cast<CXXNewExpr>(S)) {
    return NE->getInitializer();
  }
  for (const Stmt *Child : S->children()) {
    if (auto *E = findMakeUniqueNewExprInitializer(Child)) return E;
  }
  // We expect to find a `new` in the body of make_unique (vs indirecting to
  // another function), since it's a fairly simple implementation.
  llvm::errs()
      << "Nullability: expected to find `new` in make_unique but did not\n";
  assert(false);
  return nullptr;
}

}  // namespace

absl::Nullable<const Expr *> getUnderlyingInitExprInStdMakeUnique(
    const FunctionDecl &Decl) {
  if (!isStdMakeUniqueWithNontrivalConstructor(Decl)) return nullptr;
  if (Decl.getBody() == nullptr) return nullptr;
  return findMakeUniqueNewExprInitializer(Decl.getBody());
}

absl::Nullable<const FunctionDecl *> getLastForwardingFunctionLayer(
    const FunctionDecl &Decl) {
  if (!isStdMakeUniqueWithNontrivalConstructor(Decl)) return nullptr;
  if (Decl.getBody() == nullptr) return nullptr;
  return &Decl;
}

}  // namespace clang::tidy::nullability
