// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ast_util.h"

#include "clang/AST/DeclCXX.h"
#include "clang/AST/DeclTemplate.h"

namespace crubit {

bool IsFullClassTemplateSpecializationOrChild(const clang::Decl* decl) {
  if (clang::isa<clang::ClassTemplatePartialSpecializationDecl>(decl)) {
    return false;
  }
  if (clang::isa<clang::ClassTemplateSpecializationDecl>(decl)) {
    return true;
  }

  if (const auto* decl_context = decl->getDeclContext()) {
    return IsFullClassTemplateSpecializationOrChild(
        clang::dyn_cast<clang::Decl>(decl_context));
  }

  return false;
}

}  // namespace crubit
