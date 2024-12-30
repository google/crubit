// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importers/type_alias.h"

#include <optional>
#include <utility>

#include "absl/log/check.h"
#include "lifetime_annotations/type_lifetimes.h"
#include "rs_bindings_from_cc/ast_util.h"
#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/Type.h"
#include "clang/Basic/LLVM.h"

namespace crubit {

std::optional<IR::Item> crubit::TypeAliasImporter::Import(
    clang::NamedDecl* decl) {
  clang::DeclContext* decl_context = decl->getDeclContext();
  clang::QualType underlying_qualtype;
  if (auto* typedef_name_decl = clang::dyn_cast<clang::TypedefNameDecl>(decl)) {
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
          decl, UnsupportedItem::Kind::kType, std::nullopt,
          FormattedError::Static(
              "Typedef only used to introduce a name in C. Not importing."));
    }
  } else if (auto* using_decl = clang::dyn_cast<clang::UsingShadowDecl>(decl)) {
    clang::NamedDecl* target = using_decl->getTargetDecl();
    auto* target_type = clang::dyn_cast<clang::TypeDecl>(target);
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

  absl::StatusOr<Identifier> identifier = ictx_.GetTranslatedIdentifier(decl);
  if (!identifier.ok()) {
    return ictx_.ImportUnsupportedItem(
        decl, UnsupportedItem::Kind::kType, std::nullopt,
        FormattedError::PrefixedStrCat("Type alias name is not supported",
                                       identifier.status().message()));
  }

  auto enclosing_item_id = ictx_.GetEnclosingItemId(decl);
  if (!enclosing_item_id.ok()) {
    return ictx_.ImportUnsupportedItem(
        decl, UnsupportedItem::Kind::kType, std::nullopt,
        FormattedError::FromStatus(std::move(enclosing_item_id.status())));
  }

  clang::tidy::lifetimes::ValueLifetimes* no_lifetimes = nullptr;
  // TODO(mboehme): Once lifetime_annotations supports retrieving lifetimes in
  // type aliases, pass these to ConvertQualType().
  absl::StatusOr<MappedType> underlying_type =
      ictx_.ConvertQualType(underlying_qualtype, no_lifetimes, std::nullopt);

  if (!underlying_type.ok()) {
    return ictx_.ImportUnsupportedItem(
        decl, UnsupportedItem::Kind::kType,
        UnsupportedItem::Path{.ident = *identifier,
                              .enclosing_item_id = *enclosing_item_id},
        FormattedError::FromStatus(std::move(underlying_type.status())));
  }
  ictx_.MarkAsSuccessfullyImported(decl);
  return TypeAlias{
      .identifier = *identifier,
      .id = ictx_.GenerateItemId(decl),
      .owning_target = ictx_.GetOwningTarget(decl),
      .doc_comment = ictx_.GetComment(decl),
      .unknown_attr = CollectUnknownAttrs(*decl),
      .underlying_type = *underlying_type,
      .source_loc = ictx_.ConvertSourceLocation(decl->getBeginLoc()),
      .enclosing_item_id = *std::move(enclosing_item_id),
  };
}

}  // namespace crubit
