// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_AST_CONSUMER_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_AST_CONSUMER_H_

#include <string>

#include "rs_bindings_from_cc/ast_visitor.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/ASTConsumer.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/ASTContext.h"

namespace rs_bindings_from_cc {

// Consumes the Clang AST of the header and creates Rust bindings.
class AstConsumer : public clang::ASTConsumer {
 public:
  explicit AstConsumer(std::string &rs_api, std::string &rs_api_impl)
      : ast_visitor_(AstVisitor(rs_api, rs_api_impl)) {}

  void HandleTranslationUnit(clang::ASTContext &) override;

 private:
  AstVisitor ast_visitor_;
};  // class AstConsumer

}  // namespace rs_bindings_from_cc

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_AST_CONSUMER_H_
