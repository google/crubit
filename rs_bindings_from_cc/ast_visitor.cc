// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ast_visitor.h"

#include <memory>
#include <string>
#include <vector>

#include "base/logging.h"
#include "rs_bindings_from_cc/ir.h"
#include "third_party/absl/container/flat_hash_set.h"
#include "third_party/absl/strings/string_view.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/ASTContext.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Decl.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Mangle.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Type.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/Support/Casting.h"

namespace rs_bindings_from_cc {

bool AstVisitor::TraverseDecl(clang::Decl* decl) {
  if (seen_decls_.insert(decl->getCanonicalDecl()).second) {
    return Base::TraverseDecl(decl);
  }
  return false;
}

bool AstVisitor::TraverseTranslationUnitDecl(
    clang::TranslationUnitDecl* translation_unit_decl) {
  mangler_.reset(translation_unit_decl->getASTContext().createMangleContext());

  for (const absl::string_view header_name : public_header_names_) {
    ir_.used_headers.emplace_back(HeaderName(std::string(header_name)));
  }

  return Base::TraverseTranslationUnitDecl(translation_unit_decl);
}

bool AstVisitor::VisitFunctionDecl(clang::FunctionDecl* function_decl) {
  std::vector<FuncParam> params;
  for (const clang::ParmVarDecl* param : function_decl->parameters()) {
    params.push_back({ConvertType(param->getType()), GetTranslatedName(param)});
  }

  ir_.functions.push_back(Func{
      .identifier = GetTranslatedName(function_decl),
      .mangled_name = GetMangledName(function_decl),
      .return_type = ConvertType(function_decl->getReturnType()),
      .params = std::move(params),
      .is_inline = function_decl->isInlined(),
  });
  return true;
}

bool AstVisitor::VisitRecordDecl(clang::RecordDecl* record_decl) {
  std::vector<Field> fields;
  for (const clang::FieldDecl* field_decl : record_decl->fields()) {
    fields.push_back({.identifier = GetTranslatedName(field_decl),
                      .type = ConvertType(field_decl->getType())});
  }
  ir_.records.emplace_back(GetTranslatedName(record_decl), std::move(fields));
  return true;
}

Type AstVisitor::ConvertType(clang::QualType qual_type) const {
  if (const clang::PointerType* pointer_type =
          qual_type->getAs<clang::PointerType>()) {
    return Type::PointerTo(ConvertType(pointer_type->getPointeeType()));

  } else if (const clang::BuiltinType* builtin_type =
                 qual_type->getAs<clang::BuiltinType>()) {
    if (builtin_type->isIntegerType()) {
      return Type{std::string("i32"), std::string("int")};
    }
    if (builtin_type->isVoidType()) {
      return Type::Void();
    }
  }
  LOG(FATAL) << "Unsupported type " << qual_type.getAsString() << "\n";
}

std::string AstVisitor::GetMangledName(
    const clang::NamedDecl* named_decl) const {
  std::string name;
  llvm::raw_string_ostream stream(name);
  mangler_->mangleName(named_decl, stream);
  stream.flush();
  return name;
}

Identifier AstVisitor::GetTranslatedName(
    const clang::NamedDecl* named_decl) const {
  return Identifier(std::string(named_decl->getName()));
}

}  // namespace rs_bindings_from_cc
