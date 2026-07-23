// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importers/namespace.h"

#include <algorithm>
#include <memory>
#include <optional>
#include <string>
#include <utility>

#include "absl/log/check.h"
#include "absl/status/statusor.h"
#include "rs_bindings_from_cc/ast_util.h"
#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/Attr.h"
#include "clang/AST/Decl.h"

namespace crubit {

std::optional<IR::Item> NamespaceDeclImporter::Import(
    clang::NamespaceDecl* namespace_decl) {
  if (namespace_decl->isAnonymousNamespace()) {
    return ictx_.ImportUnsupportedItem(
        *namespace_decl, std::nullopt,
        {FormattedError::Static("Anonymous namespaces are not yet supported")});
  }

  absl::StatusOr<TranslatedIdentifier> identifier =
      ictx_.GetTranslatedIdentifier(namespace_decl);
  if (!identifier.ok()) {
    return ictx_.ImportUnsupportedItem(
        *namespace_decl, std::nullopt,
        {FormattedError::PrefixedStrCat("Namespace name is not supported",
                                        identifier.status().message())});
  }

  ictx_.ImportDeclsFromDeclContext(namespace_decl);
  auto item_ids = ictx_.GetItemIdsInSourceOrder(namespace_decl);
  auto enclosing_item_id = ictx_.GetEnclosingItemId(namespace_decl);
  if (!enclosing_item_id.ok()) {
    return ictx_.ImportUnsupportedItem(
        *namespace_decl, std::nullopt,
        {FormattedError::FromStatus(std::move(enclosing_item_id.status()))});
  }
  // Renames are not currently supported for namespaces.
  // TODO - b/399487279: Support namespace renames using CRUBIT_RUST_NAME.
  // if (identifier->crubit_rust_name.has_value()) {
  //   return ictx_.ImportUnsupportedItem(
  //       *namespace_decl, std::nullopt,
  //       {FormattedError::Static("Namespace renames are not yet supported")});
  // }

  std::optional<std::string> deprecated;
  absl::StatusOr<std::optional<std::string>> unknown_attr =
      CollectUnknownAttrs(*namespace_decl, [&](const clang::Attr& attr) {
        if (auto* deprecated_attr =
                clang::dyn_cast<clang::DeprecatedAttr>(&attr)) {
          deprecated.emplace(deprecated_attr->getMessage());
          return true;
        }
        return false;
      });
  if (!unknown_attr.ok()) {
    return ictx_.ImportUnsupportedItem(
        *namespace_decl, std::nullopt,
        {FormattedError::FromStatus(std::move(unknown_attr.status()))});
  }

  ItemId id = ictx_.GenerateItemId(namespace_decl);
  ictx_.invocation_.child_item_ids_[id] = std::move(item_ids);

  return Namespace{.cc_name = identifier->cc_identifier,
                   .rs_name = identifier->cc_identifier,
                   .unique_name = ictx_.GetUniqueName(*namespace_decl),
                   .id = id,
                   .canonical_namespace_id =
                       ictx_.GenerateItemId(namespace_decl->getCanonicalDecl()),
                   .unknown_attr = std::move(*unknown_attr),
                   .owning_target = ictx_.GetOwningTarget(namespace_decl),
                   .enclosing_item_id = *std::move(enclosing_item_id),
                   .is_inline = namespace_decl->isInline(),
                   .deprecated = std::move(deprecated),
                   .doc_comment = ictx_.GetComment(namespace_decl)};
}

absl::StatusOr<std::unique_ptr<ir_proto::Item>>
NamespaceDeclImporter::ImportToProto(clang::NamespaceDecl* namespace_decl) {
  if (namespace_decl->isAnonymousNamespace()) {
    return ictx_.ImportUnsupportedItemToProto(
        *namespace_decl, std::nullopt,
        {FormattedError::Static("Anonymous namespaces are not yet supported")});
  }

  absl::StatusOr<TranslatedIdentifier> identifier =
      ictx_.GetTranslatedIdentifier(namespace_decl);
  if (!identifier.ok()) {
    return ictx_.ImportUnsupportedItemToProto(
        *namespace_decl, std::nullopt,
        {FormattedError::PrefixedStrCat("Namespace name is not supported",
                                        identifier.status().message())});
  }

  ictx_.ImportDeclsFromDeclContext(namespace_decl);
  auto item_ids = ictx_.GetItemIdsInSourceOrder(namespace_decl);
  auto enclosing_item_id = ictx_.GetEnclosingItemId(namespace_decl);
  if (!enclosing_item_id.ok()) {
    return ictx_.ImportUnsupportedItemToProto(
        *namespace_decl, std::nullopt,
        {FormattedError::FromStatus(std::move(enclosing_item_id.status()))});
  }

  std::optional<std::string> deprecated;
  absl::StatusOr<std::optional<std::string>> unknown_attr =
      CollectUnknownAttrs(*namespace_decl, [&](const clang::Attr& attr) {
        if (auto* deprecated_attr =
                clang::dyn_cast<clang::DeprecatedAttr>(&attr)) {
          deprecated.emplace(deprecated_attr->getMessage());
          return true;
        }
        return false;
      });
  if (!unknown_attr.ok()) {
    return ictx_.ImportUnsupportedItemToProto(
        *namespace_decl, std::nullopt,
        {FormattedError::FromStatus(std::move(unknown_attr.status()))});
  }

  ItemId id = ictx_.GenerateItemId(namespace_decl);
  ictx_.invocation_.child_item_ids_[id] = std::move(item_ids);

  auto item = std::make_unique<ir_proto::Item>();
  ir_proto::Namespace* ns = item->mutable_namespace_decl();

  *ns->mutable_cc_name() = identifier->cc_identifier.ToFlatProto();
  *ns->mutable_rs_name() = identifier->cc_identifier.ToFlatProto();
  ns->set_unique_name(ictx_.GetUniqueName(*namespace_decl));
  ns->set_id(id.value());
  ns->set_canonical_namespace_id(
      ictx_.GenerateItemId(namespace_decl->getCanonicalDecl()).value());
  if (unknown_attr->has_value()) {
    ns->set_unknown_attr(**unknown_attr);
  }
  ns->set_owning_target(ictx_.GetOwningTarget(namespace_decl).value());
  if (enclosing_item_id->has_value()) {
    ns->set_enclosing_item_id((*enclosing_item_id)->value());
  }
  ns->set_is_inline(namespace_decl->isInline());
  ns->set_must_bind(must_bind_);
  if (deprecated) {
    ns->set_deprecated(*deprecated);
  }
  if (auto doc_comment = ictx_.GetComment(namespace_decl)) {
    ns->set_doc_comment(*doc_comment);
  }

  return item;
}

}  // namespace crubit
