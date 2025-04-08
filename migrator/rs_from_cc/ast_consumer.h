// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_MIGRATOR_RS_FROM_CC_AST_CONSUMER_H_
#define CRUBIT_MIGRATOR_RS_FROM_CC_AST_CONSUMER_H_

#include "migrator/rs_from_cc/converter.h"
#include "clang/include/clang/AST/ASTConsumer.h"
#include "clang/include/clang/AST/ASTContext.h"
#include "clang/include/clang/Frontend/CompilerInstance.h"

namespace crubit_rs_from_cc {

// Consumes the Clang AST created from the invocation's entry header and
// generates the intermediate representation (`IR`) in the invocation object.
class AstConsumer : public clang::ASTConsumer {
 public:
  explicit AstConsumer(clang::CompilerInstance& instance,
                       Converter::Invocation& invocation)
      : instance_(instance), invocation_(invocation) {}

  void HandleTranslationUnit(clang::ASTContext& context) override;

 private:
  clang::CompilerInstance& instance_;
  Converter::Invocation& invocation_;
};  // class AstConsumer

}  // namespace crubit_rs_from_cc

#endif  // CRUBIT_MIGRATOR_RS_FROM_CC_AST_CONSUMER_H_
