// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/inferable.h"

#include "nullability/type_nullability.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/DeclTemplate.h"
#include "clang/AST/Type.h"
#include "clang/Basic/LLVM.h"

namespace clang::tidy::nullability {

bool hasInferable(QualType T) {
  return isSupportedPointerType(T.getNonReferenceType());
}

int countInferableSlots(const Decl& D) {
  if (const auto* Func = dyn_cast<FunctionDecl>(&D)) {
    int Slots = 0;
    if (hasInferable(Func->getReturnType())) ++Slots;
    for (auto* P : Func->parameters())
      if (hasInferable(P->getType())) ++Slots;
    return Slots;
  }
  if (const auto* Field = dyn_cast<FieldDecl>(&D)) {
    return hasInferable(Field->getType()) ? 1 : 0;
  }
  if (const auto* Var = dyn_cast<VarDecl>(&D)) {
    return hasInferable(Var->getType()) ? 1 : 0;
  }
  return 0;
}

static bool isAnyParentContextTemplateOrInstantiation(const Decl& D) {
  auto* Parent = D.getDeclContext();
  if (Parent->isDependentContext()) return true;
  while (Parent) {
    if (isa<ClassTemplateSpecializationDecl>(Parent)) return true;
    if (auto* FD = dyn_cast<FunctionDecl>(Parent);
        FD && FD->isTemplateInstantiation())
      return true;
    Parent = Parent->getParent();
  }
  return false;
}

bool isInferenceTarget(const Decl& D) {
  if (const auto* Func = dyn_cast<FunctionDecl>(&D)) {
    return
        // Function templates are in principle inferable.
        // However since we don't analyze their bodies, and other
        // implementations cannot interact with them directly, we can't perform
        // any nontrivial inference, just propagate annotations across redecls.
        // For now, we don't do this as some infra (NullabilityWalker) doesn't
        // work on dependent code.
        !Func->isDependentContext() &&
        // Inferring properties of template instantiations isn't useful in
        // itself. We can't record them anywhere unless they apply to the
        // template in general.
        // TODO: work out in what circumstances that would be safe.
        !Func->isTemplateInstantiation() &&
        // builtins can't be annotated and are irregular in their type checking
        // and in other ways, leading to violations of otherwise sound
        // assumptions.
        // If we find that their nullability is unexpectedly leaking into
        // programs under analysis in significant ways, we can hardcode this
        // small set of functions.
        Func->getBuiltinID() == 0 &&
        // Implicit functions cannot be annotated.
        !Func->isImplicit() && !isAnyParentContextTemplateOrInstantiation(D) &&
        // Do the most expensive check last.
        countPointersInType(Func->getType()) > 0;
  }
  if (const auto* Field = dyn_cast<FieldDecl>(&D)) {
    return
        // See comments above regarding dependent contexts and templates.
        !isAnyParentContextTemplateOrInstantiation(D) &&
        // Do the most expensive check last.
        countPointersInType(Field->getType()) > 0;
  }
  if (const auto* Var = dyn_cast<VarDecl>(&D)) {
    // Include static member variables, global variables, and local variables,
    // including static variables defined in a function.

    // Exclude parameters, which are handled as part of their enclosing function
    // declaration.
    if (isa<ParmVarDecl>(Var)) return false;
    return
        // Exclude variables in templates and dependent contexts as well as
        // variable templates. See comments above regarding similar restrictions
        // on functions.
        !Var->isTemplated() && !Var->getTemplateInstantiationPattern() &&
        !isAnyParentContextTemplateOrInstantiation(D) &&
        // Do the most expensive check last.
        countPointersInType(Var->getType()) > 0;
  }
  return false;
}

}  // namespace clang::tidy::nullability
