// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importers/var.h"

#include <optional>
#include <string>
#include <utility>

#include "absl/log/check.h"
#include "rs_bindings_from_cc/ast_util.h"
#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/DeclTemplate.h"
#include "llvm/Support/Casting.h"

namespace crubit {

std::optional<IR::Item> VarDeclImporter::Import(clang::VarDecl* var_decl) {
  // Most vars are not globals – fail fast if this is the case.
  clang::DeclContext* decl_context = var_decl->getDeclContext();
  if (!decl_context) {
    return ictx_.ImportUnsupportedItem(
        *var_decl, std::nullopt,
        FormattedError::Static("DeclContext was unexpectedly null"));
  }
  if (!decl_context->isTranslationUnit() && !decl_context->isExternCContext() &&
      !decl_context->isExternCXXContext() && !decl_context->isNamespace() &&
      !var_decl->isStaticDataMember()) {
    return std::nullopt;
  }

  if (var_decl->isStaticDataMember()) {
    return ictx_.ImportUnsupportedItem(
        *var_decl, std::nullopt,
        FormattedError::Static("static data members are not supported"));
  }

  // Note that `[const|inline] T x = /* constant initializer */;` acts like
  // constexpr and does not create an external symbol in Clang. This was
  // apparently done to support constexpr-like patterns before it existed in the
  // language.
  bool is_const_or_inline =
      var_decl->getType().isConstQualified() || var_decl->isInline();
  bool might_not_export =
      var_decl->isConstexpr() ||
      (is_const_or_inline && var_decl->hasConstantInitialization());
  // TODO(b/208945197): We don't support compile-time constants yet.
  if (might_not_export) {
    return ictx_.ImportUnsupportedItem(
        *var_decl, std::nullopt,
        FormattedError::Static(
            "compile-time and inline constants are not supported"));
  }

  if (!var_decl->hasExternalFormalLinkage()) {
    return std::nullopt;
  }

  if (llvm::isa<clang::VarTemplateSpecializationDecl>(var_decl)) {
    return ictx_.ImportUnsupportedItem(
        *var_decl, std::nullopt,
        FormattedError::Static("templated variables are not supported"));
  }

  absl::StatusOr<TranslatedIdentifier> var_name =
      ictx_.GetTranslatedIdentifier(var_decl);
  if (!var_name.ok()) {
    return ictx_.ImportUnsupportedItem(
        *var_decl, std::nullopt,
        FormattedError::PrefixedStrCat("variable name is not supported",
                                       var_name.status().message()));
  }

  auto enclosing_item_id = ictx_.GetEnclosingItemId(var_decl);
  if (!enclosing_item_id.ok()) {
    return ictx_.ImportUnsupportedItem(
        *var_decl, std::nullopt,
        FormattedError::FromStatus(std::move(enclosing_item_id.status())));
  }

  CcType type =
      ictx_.ConvertQualType(var_decl->getType(), nullptr, /*nullable=*/true,
                            ictx_.AreAssumedLifetimesEnabledForTarget(
                                ictx_.GetOwningTarget(var_decl)));

  // Global variables without extern "C" have different linkage, but in practice
  // all this means is that the name is mangled, not that the ABI is different.
  // So we can support directly binding to all variables as long as we use the
  // correct name.
  std::optional<std::string> mangled_name = std::nullopt;
  if (!var_decl->isExternC()) {
    mangled_name = ictx_.GetMangledName(var_decl);
  }
  if (mangled_name == var_name->rs_identifier().Ident()) {
    mangled_name = std::nullopt;
  }

  absl::StatusOr<std::optional<std::string>> unknown_attr =
      CollectUnknownAttrs(*var_decl);
  if (!unknown_attr.ok()) {
    return ictx_.ImportUnsupportedItem(
        *var_decl, std::nullopt,
        FormattedError::FromStatus(std::move(unknown_attr.status())));
  }

  ictx_.MarkAsSuccessfullyImported(var_decl);
  return GlobalVar{
      .cc_name = var_name->cc_identifier,
      .rs_name = var_name->rs_identifier(),
      .id = ictx_.GenerateItemId(var_decl),
      .owning_target = ictx_.GetOwningTarget(var_decl),
      .source_loc = ictx_.ConvertSourceLocation(var_decl->getBeginLoc()),
      .mangled_name = mangled_name,
      .type = std::move(type),
      .unknown_attr = std::move(*unknown_attr),
      .enclosing_item_id = *std::move(enclosing_item_id),
  };
}

}  // namespace crubit
