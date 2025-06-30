// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "migrator/rs_from_cc/frontend_action.h"

#include <memory>

#include "migrator/rs_from_cc/ast_consumer.h"
#include "clang/AST/ASTConsumer.h"
#include "clang/Frontend/CompilerInstance.h"

namespace crubit_rs_from_cc {

std::unique_ptr<clang::ASTConsumer> FrontendAction::CreateASTConsumer(
    clang::CompilerInstance& instance, llvm::StringRef) {
  return std::make_unique<AstConsumer>(instance, invocation_);
}

}  // namespace crubit_rs_from_cc
