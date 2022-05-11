// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_MIGRATOR_RS_FROM_CC_FRONTEND_ACTION_H_
#define CRUBIT_MIGRATOR_RS_FROM_CC_FRONTEND_ACTION_H_

#include <memory>

#include "lifetime_annotations/lifetime_annotations.h"
#include "migrator/rs_from_cc/converter.h"
#include "clang/AST/ASTConsumer.h"
#include "clang/Frontend/CompilerInstance.h"
#include "clang/Frontend/FrontendAction.h"

namespace crubit_rs_from_cc {

// Creates an `ASTConsumer` that generates the Rust code in the invocation
// object.
class FrontendAction : public clang::ASTFrontendAction {
 public:
  explicit FrontendAction(Converter::Invocation& invocation)
      : invocation_(invocation) {}

  std::unique_ptr<clang::ASTConsumer> CreateASTConsumer(
      clang::CompilerInstance& instance, llvm::StringRef) override;

 private:
  Converter::Invocation& invocation_;
};

}  // namespace crubit_rs_from_cc

#endif  // CRUBIT_MIGRATOR_RS_FROM_CC_FRONTEND_ACTION_H_
