// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importers/typedef_name.h"

#include "clang/AST/ASTContext.h"

namespace crubit {

std::optional<IR::Item> crubit::TypedefNameDeclImporter::Import(
    clang::TypedefNameDecl* typedef_name_decl) {
  const clang::DeclContext* decl_context = typedef_name_decl->getDeclContext();
  if (decl_context) {
    if (decl_context->isFunctionOrMethod()) {
      return std::nullopt;
    }
    if (decl_context->isRecord()) {
      return ictx_.ImportUnsupportedItem(
          typedef_name_decl,
          "Typedefs nested in classes are not supported yet");
    }
  }

  clang::QualType type =
      typedef_name_decl->getASTContext().getTypedefType(typedef_name_decl);
  if (ictx_.type_mapper_.MapKnownCcTypeToRsType(type.getAsString())
          .has_value()) {
    return std::nullopt;
  }

  std::optional<Identifier> identifier =
      ictx_.GetTranslatedIdentifier(typedef_name_decl);
  CRUBIT_CHECK(identifier.has_value());  // This should never happen.
  std::optional<clang::tidy::lifetimes::ValueLifetimes> no_lifetimes;
  absl::StatusOr<MappedType> underlying_type =
      ictx_.type_mapper_.ConvertQualType(typedef_name_decl->getUnderlyingType(),
                                         no_lifetimes);
  if (underlying_type.ok()) {
    ictx_.type_mapper_.Insert(typedef_name_decl);
    return TypeAlias{.identifier = *identifier,
                     .id = GenerateItemId(typedef_name_decl),
                     .owning_target = ictx_.GetOwningTarget(typedef_name_decl),
                     .doc_comment = ictx_.GetComment(typedef_name_decl),
                     .underlying_type = *underlying_type};
  } else {
    return ictx_.ImportUnsupportedItem(
        typedef_name_decl, std::string(underlying_type.status().message()));
  }
}

}  // namespace crubit
