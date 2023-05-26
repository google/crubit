// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importers/type_alias.h"

#include <optional>
#include <string>

#include "absl/log/check.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Decl.h"

namespace crubit {

std::optional<IR::Item> crubit::TypeAliasImporter::Import(
    clang::NamedDecl* decl) {
  clang::DeclContext* decl_context = decl->getDeclContext();
  clang::QualType underlying_qualtype;
  if (auto* typedef_name_decl = llvm::dyn_cast<clang::TypedefNameDecl>(decl)) {
    if (typedef_name_decl->getAnonDeclWithTypedefName()) {
      // Anonymous declarations with typedefs just incorporate the typedef name
      // into their item, instead of having a separate TypeAlias item in
      // addition.
      return std::nullopt;
    }
    underlying_qualtype = typedef_name_decl->getUnderlyingType();
    clang::QualType type =
        decl->getASTContext().getTypedefType(typedef_name_decl);
    if (const auto* tag_decl = type->getAsTagDecl();
        tag_decl && tag_decl->getDeclContext() == decl_context &&
        tag_decl->getName() == decl->getName()) {
      return ictx_.ImportUnsupportedItem(
          decl, "Typedef only used to introduce a name in C. Not importing.");
    }
  } else if (auto* using_decl = llvm::dyn_cast<clang::UsingShadowDecl>(decl)) {
    clang::NamedDecl* target = using_decl->getTargetDecl();
    auto* target_type = llvm::dyn_cast<clang::TypeDecl>(target);
    if (target_type == nullptr) {
      // Not a type.
      return std::nullopt;
    }
    underlying_qualtype =
        target_type->getASTContext().getTypeDeclType(target_type);
    decl = using_decl;
  } else {
    // Neither a typedef nor a using decl.
    return std::nullopt;
  }

  std::optional<ItemId> enclosing_record_id = std::nullopt;
  if (decl_context) {
    if (decl_context->isFunctionOrMethod()) {
      return std::nullopt;
    }
    if (auto* record_decl = llvm::dyn_cast<clang::RecordDecl>(decl_context)) {
      if (!ictx_.EnsureSuccessfullyImported(record_decl)) {
        return ictx_.ImportUnsupportedItem(decl, "Couldn't import the parent");
      }
      enclosing_record_id = GenerateItemId(record_decl);
    }
  }

  absl::StatusOr<Identifier> identifier = ictx_.GetTranslatedIdentifier(decl);
  if (!identifier.ok()) {
    return ictx_.ImportUnsupportedItem(
        decl, absl::StrCat("Type alias name is not supported: ",
                           identifier.status().message()));
  }

  std::optional<clang::tidy::lifetimes::ValueLifetimes> no_lifetimes;
  // TODO(mboehme): Once lifetime_annotations supports retrieving lifetimes in
  // type aliases, pass these to ConvertQualType().
  absl::StatusOr<MappedType> underlying_type =
      ictx_.ConvertQualType(underlying_qualtype, no_lifetimes, std::nullopt);

  if (!underlying_type.ok()) {
    return ictx_.ImportUnsupportedItem(
        decl, std::string(underlying_type.status().message()));
  }

  ictx_.MarkAsSuccessfullyImported(decl);
  return TypeAlias{
      .identifier = *identifier,
      .id = GenerateItemId(decl),
      .owning_target = ictx_.GetOwningTarget(decl),
      .doc_comment = ictx_.GetComment(decl),
      .underlying_type = *underlying_type,
      .source_loc = ictx_.ConvertSourceLocation(decl->getBeginLoc()),
      .enclosing_record_id = enclosing_record_id,
      .enclosing_namespace_id = GetEnclosingNamespaceId(decl),
  };
}

}  // namespace crubit
