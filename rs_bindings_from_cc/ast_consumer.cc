// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ast_consumer.h"

#include "rs_bindings_from_cc/ast_visitor.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/ASTContext.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Decl.h"

namespace rs_bindings_from_cc {

void AstConsumer::HandleTranslationUnit(clang::ASTContext &ast_context) {
  ast_visitor_.TraverseDecl(ast_context.getTranslationUnitDecl());
}

}  // namespace rs_bindings_from_cc
