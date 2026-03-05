// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importers/enum_constant.h"

#include <optional>
#include <string>
#include <utility>

#include "absl/status/statusor.h"
#include "rs_bindings_from_cc/ast_util.h"
#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/Decl.h"
#include "llvm/Support/Casting.h"

namespace crubit {

std::optional<IR::Item> EnumConstantDeclImporter::Import(
    clang::EnumConstantDecl* enum_constant_decl) {
  absl::StatusOr<TranslatedIdentifier> enumerator_name =
      ictx_.GetTranslatedIdentifier(enum_constant_decl);
  if (!enumerator_name.ok()) {
    return ictx_.ImportUnsupportedItem(
        *enum_constant_decl, std::nullopt,
        FormattedError::PrefixedStrCat("Enumerator name is not supported",
                                       enumerator_name.status().message()));
  }

  auto enclosing_item_id = ictx_.GetEnclosingItemId(enum_constant_decl);
  if (!enclosing_item_id.ok()) {
    return ictx_.ImportUnsupportedItem(
        *enum_constant_decl, std::nullopt,
        FormattedError::FromStatus(std::move(enclosing_item_id.status())));
  }

  const clang::EnumDecl* enum_decl =
      llvm::cast<clang::EnumDecl>(enum_constant_decl->getDeclContext());
  clang::QualType cpp_type = enum_decl->getIntegerType();
  // Anonymous enums can't be forward declared, so getIntegerType() should not
  // be null.
  if (cpp_type.isNull()) {
    return ictx_.ImportUnsupportedItem(
        *enum_constant_decl, std::nullopt,
        FormattedError::Static("Enumerator's enum has no underlying type"));
  }

  absl::StatusOr<CcType> type =
      ictx_.ConvertQualType(cpp_type, nullptr, /*nullable=*/true,
                            ictx_.AreAssumedLifetimesEnabledForTarget(
                                ictx_.GetOwningTarget(enum_constant_decl)));
  if (!type.ok()) {
    return ictx_.ImportUnsupportedItem(
        *enum_constant_decl, std::nullopt,
        FormattedError::FromStatus(std::move(type.status())));
  }

  absl::StatusOr<std::optional<std::string>> unknown_attr =
      CollectUnknownAttrs(*enum_constant_decl);
  if (!unknown_attr.ok()) {
    return ictx_.ImportUnsupportedItem(
        *enum_constant_decl, std::nullopt,
        FormattedError::FromStatus(std::move(unknown_attr.status())));
  }

  ictx_.MarkAsSuccessfullyImported(enum_constant_decl);
  absl::StatusOr<IntegerConstant> value =
      IntegerConstant::FromAPValue(enum_constant_decl->getInitVal());
  if (!value.ok()) {
    return ictx_.ImportUnsupportedItem(
        *enum_constant_decl, std::nullopt,
        FormattedError::FromStatus(std::move(value.status())));
  }
  return Constant{
      .value = std::move(*value),
      .cc_name = enumerator_name->cc_identifier,
      .rs_name = enumerator_name->rs_identifier(),
      .unique_name = ictx_.GetUniqueName(*enum_constant_decl),
      .id = ictx_.GenerateItemId(enum_constant_decl),
      .owning_target = ictx_.GetOwningTarget(enum_constant_decl),
      .source_loc = ictx_.ConvertSourceLocation(
          enum_constant_decl->getBeginLoc(), nullptr),
      .type = *std::move(type),
      .unknown_attr = std::move(*unknown_attr),
      .enclosing_item_id = *std::move(enclosing_item_id),
  };
}

}  // namespace crubit
