// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/inferrable.h"

#include "nullability/type_nullability.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/Type.h"
#include "clang/Basic/LLVM.h"

namespace clang::tidy::nullability {

namespace {

bool isInferrable(QualType T) {
  return isSupportedPointerType(T.getNonReferenceType());
}

}  // namespace

int countInferrableSlots(const Decl& D) {
  const clang::FunctionDecl* Func = dyn_cast<clang::FunctionDecl>(&D);
  if (!Func) return 0;
  int Slots = 0;
  if (isInferrable(Func->getReturnType())) ++Slots;
  for (auto* P : Func->parameters())
    if (isInferrable(P->getType())) ++Slots;
  return Slots;
}

bool isInferenceTarget(const Decl& D) {
  // For now, only support inferring nullability of functions.
  const auto* FD = dyn_cast<FunctionDecl>(&D);
  if (!FD) return false;
  return
      // Function templates are in principle inferrable.
      // However since we don't analyze their bodies, and other implementations
      // cannot interact with them directly, we can't perform any nontrivial
      // inference, just propagate annotations across redecls.
      // For now, we don't do this as some infra (NullabilityWalker) doesn't
      // work on dependent code.
      !FD->isDependentContext() &&
      // Inferring properties of template instantiations isn't useful in
      // itself. We can't record them anywhere unless they apply to the
      // template in general.
      // TODO: work out in what circumstances that would be safe.
      !FD->getTemplateInstantiationPattern();
}

}  // namespace clang::tidy::nullability
