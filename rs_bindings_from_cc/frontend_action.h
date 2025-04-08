// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_FRONTEND_ACTION_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_FRONTEND_ACTION_H_

#include <memory>

#include "rs_bindings_from_cc/decl_importer.h"
#include "clang/include/clang/AST/ASTConsumer.h"
#include "clang/include/clang/Frontend/CompilerInstance.h"
#include "clang/include/clang/Frontend/FrontendAction.h"
#include "llvm/include/llvm/ADT/StringRef.h"

namespace crubit {

// Creates an `ASTConsumer` that generates the intermediate representation
// (`IR`) into the invocation object.
class FrontendAction : public clang::ASTFrontendAction {
 public:
  explicit FrontendAction(Invocation& invocation) : invocation_(invocation) {}

  std::unique_ptr<clang::ASTConsumer> CreateASTConsumer(
      clang::CompilerInstance& instance, llvm::StringRef) override;

 private:
  Invocation& invocation_;
};

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_FRONTEND_ACTION_H_
