// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_AST_VISITOR_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_AST_VISITOR_H_

#include <string>

#include "third_party/llvm/llvm-project/clang/include/clang/AST/Decl.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/RecursiveASTVisitor.h"

namespace rs_bindings_from_cc {

// Iterates over the Clang AST nodes of the header and creates Rust bindings.
class AstVisitor : public clang::RecursiveASTVisitor<AstVisitor> {
 public:
  using Base = clang::RecursiveASTVisitor<AstVisitor>;

  explicit AstVisitor(std::string &rs_api, std::string &rs_api_impl)
      : rs_api_(rs_api), rs_api_impl_(rs_api_impl) {}

  bool TraverseDecl(clang::Decl *);

  bool VisitFunctionDecl(clang::FunctionDecl *);

 private:
  std::string &rs_api_;
  std::string &rs_api_impl_;
};  // class AstVisitor

}  // namespace rs_bindings_from_cc

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_AST_VISITOR_H_
