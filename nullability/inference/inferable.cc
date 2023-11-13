// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/inferable.h"

#include "nullability/type_nullability.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/Type.h"
#include "clang/Basic/LLVM.h"

namespace clang::tidy::nullability {

namespace {

bool isInferable(QualType T) {
  return isSupportedRawPointerType(T.getNonReferenceType());
}

}  // namespace

int countInferableSlots(const Decl& D) {
  const clang::FunctionDecl* Func = dyn_cast<clang::FunctionDecl>(&D);
  if (!Func) return 0;
  int Slots = 0;
  if (isInferable(Func->getReturnType())) ++Slots;
  for (auto* P : Func->parameters())
    if (isInferable(P->getType())) ++Slots;
  return Slots;
}

bool isInferenceTarget(const Decl& D) {
  // For now, only support inferring nullability of functions.
  const auto* FD = dyn_cast<FunctionDecl>(&D);
  if (!FD) return false;
  return
      // Function templates are in principle inferable.
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
      !FD->getTemplateInstantiationPattern() &&
      // builtins can't be annotated and are irregular in their type checking
      // and in other ways, leading to violations of otherwise sound
      // assumptions.
      // If we find that their nullability is unexpectedly leaking into
      // programs under analysis in significant ways, we can hardcode this small
      // set of functions.
      FD->getBuiltinID() == 0;
}

}  // namespace clang::tidy::nullability
