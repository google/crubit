// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_AST_VISITOR_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_AST_VISITOR_H_

#include <memory>
#include <string>

#include "rs_bindings_from_cc/ir.h"
#include "third_party/absl/container/flat_hash_set.h"
#include "third_party/absl/status/statusor.h"
#include "third_party/absl/strings/string_view.h"
#include "third_party/absl/types/span.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Decl.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Mangle.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/RecursiveASTVisitor.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Type.h"

namespace rs_bindings_from_cc {

// Iterates over the AST created from `public_header_names` (a collection of
// paths in the format suitable for a google3-relative quote include) and
// creates an intermediate representation of the import (`IR`).
class AstVisitor : public clang::RecursiveASTVisitor<AstVisitor> {
 public:
  using Base = clang::RecursiveASTVisitor<AstVisitor>;

  explicit AstVisitor(absl::Span<const absl::string_view> public_header_names,
                      IR& ir)
      : public_header_names_(public_header_names), ir_(ir) {}

  // These functions are called by the base class while visiting the different
  // parts of the AST. The API follows the rules of the base class which is
  // responsible for the traversal of the AST.
  bool TraverseTranslationUnitDecl(
      clang::TranslationUnitDecl* translation_unit_decl);
  bool TraverseDecl(clang::Decl* decl);

  bool VisitFunctionDecl(clang::FunctionDecl* function_decl);
  bool VisitRecordDecl(clang::RecordDecl* record_decl);

 private:
  std::string GetMangledName(const clang::NamedDecl* named_decl) const;
  // Gets the identifier naming the symbol.
  // Returns nullopt for things with non-identifier names, such as the
  // destructor.
  std::optional<Identifier> GetTranslatedName(
      const clang::NamedDecl* named_decl) const;
  absl::StatusOr<MappedType> ConvertType(clang::QualType qual_type,
                                         const clang::ASTContext& ctx) const;

  absl::Span<const absl::string_view> public_header_names_;
  IR& ir_;
  std::unique_ptr<clang::MangleContext> mangler_;
  absl::flat_hash_set<const clang::Decl*> seen_decls_;
};  // class AstVisitor

}  // namespace rs_bindings_from_cc

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_AST_VISITOR_H_
