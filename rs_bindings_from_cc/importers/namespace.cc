// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importers/namespace.h"

#include "absl/log/check.h"
#include "absl/strings/match.h"

namespace crubit {

std::optional<IR::Item> NamespaceDeclImporter::Import(
    clang::NamespaceDecl* namespace_decl) {

  if (namespace_decl->isAnonymousNamespace()) {
    return ictx_.ImportUnsupportedItem(
        namespace_decl, "Anonymous namespaces are not supported yet");
  }

  ictx_.ImportDeclsFromDeclContext(namespace_decl);
  auto identifier = ictx_.GetTranslatedIdentifier(namespace_decl);
  CHECK(identifier.has_value());
  auto item_ids = ictx_.GetItemIdsInSourceOrder(namespace_decl);
  return Namespace{
      .name = *identifier,
      .id = GenerateItemId(namespace_decl),
      .canonical_namespace_id =
          GenerateItemId(namespace_decl->getCanonicalDecl()),
      .owning_target = ictx_.GetOwningTarget(namespace_decl),
      .child_item_ids = std::move(item_ids),
      .enclosing_namespace_id = GetEnclosingNamespaceId(namespace_decl),
  };
}

}  // namespace crubit
