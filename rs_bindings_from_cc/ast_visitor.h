// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_AST_VISITOR_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_AST_VISITOR_H_

#include <memory>

#include "rs_bindings_from_cc/ir.h"
#include "third_party/absl/container/flat_hash_set.h"
#include "third_party/absl/strings/cord.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Decl.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Mangle.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/RecursiveASTVisitor.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Type.h"

namespace rs_bindings_from_cc {

// Iterates over the AST nodes of the header and creates intermediate
// representation of the import (`IR`).
class AstVisitor : public clang::RecursiveASTVisitor<AstVisitor> {
 public:
  using Base = clang::RecursiveASTVisitor<AstVisitor>;

  explicit AstVisitor(IR &ir) : ir_(ir) {}

  // These functions are called by the base class while visiting the different
  // parts of the AST. The API follows the rules of the base class which is
  // responsible for the traversal of the AST.
  bool TraverseTranslationUnitDecl(
      clang::TranslationUnitDecl *translation_unit_decl);
  bool TraverseDecl(clang::Decl *decl);

  bool VisitFunctionDecl(clang::FunctionDecl *function_decl);

 private:
  absl::Cord GetMangledName(const clang::NamedDecl *named_decl) const;
  Identifier GetTranslatedName(const clang::NamedDecl *named_decl) const;
  Type ConvertType(clang::QualType qual_type) const;

  IR &ir_;
  std::unique_ptr<clang::MangleContext> mangler_;
  absl::flat_hash_set<const clang::Decl *> seen_decls_;
};  // class AstVisitor

}  // namespace rs_bindings_from_cc

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_AST_VISITOR_H_
