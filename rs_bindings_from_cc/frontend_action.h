// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_FRONTEND_ACTION_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_FRONTEND_ACTION_H_

#include <memory>
#include <string>
#include <utility>

#include "absl/container/flat_hash_set.h"
#include "rs_bindings_from_cc/decl_importer.h"
#include "clang/AST/ASTConsumer.h"
#include "clang/Frontend/CompilerInstance.h"
#include "clang/Frontend/FrontendAction.h"
#include "llvm/ADT/StringRef.h"

namespace crubit {

// Creates an `ASTConsumer` that generates the intermediate representation
// (`IR`) into the invocation object.
class FrontendAction : public clang::ASTFrontendAction {
 public:
  // `in_memory_files` are tracked to exclude them from the depfile requested
  // via `-MD`, `-MF`, etc.
  FrontendAction(Invocation& invocation,
                 absl::flat_hash_set<std::string> in_memory_files)
      : invocation_(invocation), in_memory_files_(std::move(in_memory_files)) {}

  bool BeginInvocation(clang::CompilerInstance& instance) override;

  std::unique_ptr<clang::ASTConsumer> CreateASTConsumer(
      clang::CompilerInstance& instance, llvm::StringRef) override;

 private:
  Invocation& invocation_;
  absl::flat_hash_set<std::string> in_memory_files_;
};

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_FRONTEND_ACTION_H_
