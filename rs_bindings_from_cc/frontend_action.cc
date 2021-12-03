// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/frontend_action.h"

#include <memory>

#include "rs_bindings_from_cc/ast_consumer.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/ASTConsumer.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Frontend/CompilerInstance.h"

namespace rs_bindings_from_cc {

std::unique_ptr<clang::ASTConsumer> FrontendAction::CreateASTConsumer(
    clang::CompilerInstance& instance, llvm::StringRef) {
  AddLifetimeAnnotationHandlers(instance, lifetime_context_);
  return std::make_unique<AstConsumer>(
      instance, current_target_, public_header_names_, &headers_to_targets_,
      &ir_, lifetime_context_);
}

}  // namespace rs_bindings_from_cc
