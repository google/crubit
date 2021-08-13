// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_FRONTEND_ACTION_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_FRONTEND_ACTION_H_

#include <memory>

#include "rs_bindings_from_cc/ir.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/ASTConsumer.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Frontend/CompilerInstance.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Frontend/FrontendAction.h"

namespace rs_bindings_from_cc {

// Creates an `ASTConsumer` that generates the intermediate representation
// (`IR`) into the `ir` parameter.
class FrontendAction : public clang::ASTFrontendAction {
 public:
  explicit FrontendAction(IR &ir) : ir_(ir) {}

  std::unique_ptr<clang::ASTConsumer> CreateASTConsumer(
      clang::CompilerInstance &, llvm::StringRef) override;

 private:
  IR &ir_;
};

}  // namespace rs_bindings_from_cc

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_FRONTEND_ACTION_H_
