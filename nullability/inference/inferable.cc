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

bool isInferenceTarget(const Decl& D) {
  if (const auto* Func = dyn_cast<FunctionDecl>(&D)) {
    return
        // Function templates are in principle inferable.
        // However since we don't analyze their bodies, and other
        // implementations cannot interact with them directly, we can't
        // perform any nontrivial inference, just propagate annotations
        // across redecls. For now, we don't do this as some infra
        // (NullabilityWalker) doesn't work on dependent code. Instead, we infer
        // for the instantiations and the decls inside the instantiations, and
        // can use matching source ranges (the ranges inside the template) to
        // merge evidence and make an inference when all instantiations are
        // consistent enough.
        !Func->isDependentContext() &&
        // Same treatment for functions in templates as for function templates.
        !Func->getDeclContext()->isDependentContext() &&
        // builtins can't be annotated and are irregular in their type checking
        // and in other ways, leading to violations of otherwise sound
        // assumptions.
        // If we find that their nullability is unexpectedly leaking into
        // programs under analysis in significant ways, we can hardcode this
        // small set of functions.
        Func->getBuiltinID() == 0 &&
        // Implicit functions cannot be annotated.
        !Func->isImplicit() &&
        // Do the most expensive check last.
        countPointersInType(Func->getType()) > 0;
  }
  if (const auto* Field = dyn_cast<FieldDecl>(&D)) {
    return
        // See comments above regarding templates.
        !Field->getDeclContext()->isDependentContext() &&
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
        // Exclude variables inside templates as well as variable templates. See
        // comments above regarding similar restrictions on functions. As with
        // functions, the analogous variables inside instantiations and variable
        // template instantiations are not excluded.
        !Var->getDeclContext()->isDependentContext() && !Var->isTemplated() &&
        // Do the most expensive check last.
        countPointersInType(Var->getType()) > 0;
  }
  return false;
}

}  // namespace clang::tidy::nullability
