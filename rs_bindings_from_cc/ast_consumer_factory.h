// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_AST_CONSUMER_FACTORY_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_AST_CONSUMER_FACTORY_H_

#include <memory>
#include <string>

#include "third_party/llvm/llvm-project/clang/include/clang/AST/ASTConsumer.h"

namespace rs_bindings_from_cc {

// Creates an `ASTConsumer` that writes its outputs to `rs_api` and rs_api_impl`
// parameters.
class AstConsumerFactory {
 public:
  explicit AstConsumerFactory(std::string &rs_api, std::string &rs_api_impl)
      : rs_api_(rs_api), rs_api_impl_(rs_api_impl) {}
  std::unique_ptr<clang::ASTConsumer> newASTConsumer();

 private:
  std::string &rs_api_;
  std::string &rs_api_impl_;
};

}  // namespace rs_bindings_from_cc

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_AST_CONSUMER_FACTORY_H_
