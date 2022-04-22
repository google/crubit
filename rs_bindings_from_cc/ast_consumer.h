// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_AST_CONSUMER_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_AST_CONSUMER_H_

#include "rs_bindings_from_cc/importer.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/ASTConsumer.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/ASTContext.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Frontend/CompilerInstance.h"

namespace crubit {

// Consumes the Clang AST created from the invocation's entry header and
// generates the intermediate representation (`IR`) in the invocation object.
class AstConsumer : public clang::ASTConsumer {
 public:
  explicit AstConsumer(clang::CompilerInstance& instance,
                       Invocation& invocation)
      : instance_(instance), invocation_(invocation) {}

  void HandleTranslationUnit(clang::ASTContext& context) override;

 private:
  clang::CompilerInstance& instance_;
  Invocation& invocation_;
};  // class AstConsumer

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_AST_CONSUMER_H_
