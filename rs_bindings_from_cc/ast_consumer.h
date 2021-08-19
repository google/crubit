// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_AST_CONSUMER_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_AST_CONSUMER_H_

#include <string>

#include "rs_bindings_from_cc/ast_visitor.h"
#include "rs_bindings_from_cc/ir.h"
#include "third_party/absl/types/span.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/ASTConsumer.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/ASTContext.h"

namespace rs_bindings_from_cc {

// Consumes the Clang AST created from `public_headers` (a collection of paths
// in the format suitable for a google3-relative quote include) and generates
// the intermediate representation (`IR`).
class AstConsumer : public clang::ASTConsumer {
 public:
  explicit AstConsumer(absl::Span<const std::string> public_headers, IR &ir)
      : ast_visitor_(public_headers, ir) {}

  void HandleTranslationUnit(clang::ASTContext &context) override;

 private:
  AstVisitor ast_visitor_;
};  // class AstConsumer

}  // namespace rs_bindings_from_cc

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_AST_CONSUMER_H_
