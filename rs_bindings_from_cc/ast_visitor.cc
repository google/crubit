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
#include "third_party/llvm/llvm-project/clang/include/clang/AST/DeclCXX.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Mangle.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Type.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Basic/Specifiers.h"
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
    ir_.used_headers.push_back(HeaderName(std::string(header_name)));
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

static AccessSpecifier TranslateAccessSpecifier(clang::AccessSpecifier access) {
  switch (access) {
    case clang::AS_public:
      return kPublic;
    case clang::AS_protected:
      return kProtected;
    case clang::AS_private:
      return kPrivate;
    case clang::AS_none:
      // We should never be encoding a "none" access specifier in IR.
      assert(false);
      // We have to return something. Conservatively return private so we don't
      // inadvertently make a private member variable accessible in Rust.
      return kPrivate;
  }
}

bool AstVisitor::VisitRecordDecl(clang::RecordDecl* record_decl) {
  std::vector<Field> fields;
  clang::AccessSpecifier default_access = clang::AS_public;
  if (const auto* cxx_record_decl =
          clang::dyn_cast<clang::CXXRecordDecl>(record_decl)) {
    if (cxx_record_decl->isClass()) {
      default_access = clang::AS_private;
    }
  }
  for (const clang::FieldDecl* field_decl : record_decl->fields()) {
    clang::AccessSpecifier access = field_decl->getAccess();
    if (access == clang::AS_none) {
      access = default_access;
    }
    fields.push_back({.identifier = GetTranslatedName(field_decl),
                      .type = ConvertType(field_decl->getType()),
                      .access = TranslateAccessSpecifier(access)});
  }
  ir_.records.push_back({GetTranslatedName(record_decl), std::move(fields)});
  return true;
}

Type AstVisitor::ConvertType(clang::QualType qual_type) const {
  if (const clang::PointerType* pointer_type =
          qual_type->getAs<clang::PointerType>()) {
    return Type::PointerTo(ConvertType(pointer_type->getPointeeType()));

  } else if (const clang::BuiltinType* builtin_type =
                 qual_type->getAs<clang::BuiltinType>()) {
    if (builtin_type->isIntegerType()) {
      return Type{"i32", "int"};
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
