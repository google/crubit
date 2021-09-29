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
#include "third_party/absl/strings/substitute.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/ASTContext.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Decl.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/DeclCXX.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Mangle.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/RecordLayout.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Type.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Basic/Specifiers.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/Support/Casting.h"

namespace rs_bindings_from_cc {

constexpr std::string_view kTypeStatusPayloadUrl =
    "type.googleapis.com/devtools.rust.cc_interop.rs_binding_from_cc.type";

bool AstVisitor::TraverseDecl(clang::Decl* decl) {
  if (seen_decls_.insert(decl->getCanonicalDecl()).second) {
    return Base::TraverseDecl(decl);
  }
  return true;
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
    auto param_type =
        ConvertType(param->getType(), function_decl->getASTContext());
    if (!param_type.ok()) {
      // TODO(b/200239975):  Add diagnostics for declarations we can't import
      return true;
    }
    params.push_back({*param_type, GetTranslatedName(param)});
  }

  auto return_type = ConvertType(function_decl->getReturnType(),
                                 function_decl->getASTContext());
  if (!return_type.ok()) {
    return true;
  }
  ir_.functions.push_back(Func{
      .identifier = GetTranslatedName(function_decl),
      .mangled_name = GetMangledName(function_decl),
      .return_type = *return_type,
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
  const clang::ASTRecordLayout& layout =
      record_decl->getASTContext().getASTRecordLayout(record_decl);
  for (const clang::FieldDecl* field_decl : record_decl->fields()) {
    auto type = ConvertType(field_decl->getType(), field_decl->getASTContext());
    if (!type.ok()) {
      // TODO(b/200239975):  Add diagnostics for declarations we can't import
      return true;
    }
    clang::AccessSpecifier access = field_decl->getAccess();
    if (access == clang::AS_none) {
      access = default_access;
    }
    fields.push_back(
        {.identifier = GetTranslatedName(field_decl),
         .type = *type,
         .access = TranslateAccessSpecifier(access),
         .offset = layout.getFieldOffset(field_decl->getFieldIndex())});
  }
  ir_.records.push_back({.identifier = GetTranslatedName(record_decl),
                         .fields = std::move(fields),
                         .size = layout.getSize().getQuantity(),
                         .alignment = layout.getAlignment().getQuantity()});
  return true;
}

absl::StatusOr<MappedType> AstVisitor::ConvertType(
    clang::QualType qual_type, const clang::ASTContext& ctx) const {
  std::optional<MappedType> type = std::nullopt;
  std::string type_string = qual_type.getAsString();

  if (const clang::PointerType* pointer_type =
          qual_type->getAs<clang::PointerType>()) {
    auto pointee_type = ConvertType(pointer_type->getPointeeType(), ctx);
    if (pointee_type.ok()) {
      type = MappedType::PointerTo(*pointee_type);
    }
  } else if (const clang::BuiltinType* builtin_type =
                 qual_type->getAs<clang::BuiltinType>()) {
    switch (builtin_type->getKind()) {
      case clang::BuiltinType::Bool:
        type = MappedType::Simple("bool", "bool");
        break;
      case clang::BuiltinType::Float:
        type = MappedType::Simple("float", "float");
        break;
      case clang::BuiltinType::Double:
        type = MappedType::Simple("double", "double");
        break;
      case clang::BuiltinType::Void:
        type = MappedType::Void();
        break;
      default:
        if (builtin_type->isIntegerType()) {
          auto size = ctx.getTypeSize(builtin_type);
          if (size == 8 || size == 16 || size == 32 || size == 64) {
            type = MappedType::Simple(
                absl::Substitute(
                    "$0$1", builtin_type->isSignedInteger() ? 'i' : 'u', size),
                type_string);
          }
        }
    }
  }

  if (!type.has_value()) {
    absl::Status error = absl::UnimplementedError(
        absl::Substitute("Unsupported type '$0'", type_string));
    error.SetPayload(kTypeStatusPayloadUrl, absl::Cord(type_string));
    return error;
  }

  // Add cv-qualification.
  type->cc_type.is_const = qual_type.isConstQualified();
  // Not doing volatile for now -- note that volatile pointers do not exist in
  // Rust, though volatile reads/writes still do.

  return *std::move(type);
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
