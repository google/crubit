// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ast_visitor.h"

#include <string>

#include "third_party/llvm/llvm-project/clang/include/clang/AST/Decl.h"

namespace rs_bindings_from_cc {

bool AstVisitor::TraverseDecl(clang::Decl* decl) {
  Base::TraverseDecl(decl);
  rs_api_ = "// rs api";
  rs_api_impl_ = "// rs api impl";
  return true;
}

bool AstVisitor::VisitFunctionDecl(clang::FunctionDecl* decl) {
  decl->dump();
  return true;
}

}  // namespace rs_bindings_from_cc
