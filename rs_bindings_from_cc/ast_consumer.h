// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_AST_CONSUMER_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_AST_CONSUMER_H_

#include <memory>

#include "rs_bindings_from_cc/decl_importer.h"
#include "clang/AST/ASTConsumer.h"
#include "clang/AST/ASTContext.h"
#include "clang/Frontend/CompilerInstance.h"
#include "clang/Parse/Parser.h"

namespace crubit {

// Consumes the Clang AST created from the invocation's entry header and
// generates the intermediate representation (`IR`) in the invocation object.
class AstConsumer : public clang::ASTConsumer {
 public:
  // `parser` holds the parser that parsed the translation unit, which is used
  // to parse macro constants after its end. It must be non-null while
  // `HandleTranslationUnit` is called, and outlive this consumer.
  explicit AstConsumer(clang::CompilerInstance& instance,
                       Invocation& invocation,
                       const std::unique_ptr<clang::Parser>& parser)
      : instance_(instance), invocation_(invocation), parser_(parser) {}

  void HandleTranslationUnit(clang::ASTContext& context) override;

 private:
  clang::CompilerInstance& instance_;
  Invocation& invocation_;
  const std::unique_ptr<clang::Parser>& parser_;
};  // class AstConsumer

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_AST_CONSUMER_H_
