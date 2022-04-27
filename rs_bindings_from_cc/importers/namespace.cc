// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importers/namespace.h"

#include "absl/strings/match.h"

namespace crubit {

std::optional<IR::Item> NamespaceDeclImporter::Import(
    clang::NamespaceDecl* namespace_decl) {
  if (!ictx_.IsFromCurrentTarget(namespace_decl)) return std::nullopt;

  // TODO(rosica) In order to fully enable namespaces we first need to ensure
  // that each decl Item contains information on its namespace parents.
  if (!absl::StrContains(namespace_decl->getQualifiedNameAsString(),
                         "test_namespace_bindings")) {
    return ictx_.ImportUnsupportedItem(namespace_decl,
                                       "Namespaces are not supported yet");
  }

  if (namespace_decl->isInline()) {
    return ictx_.ImportUnsupportedItem(
        namespace_decl, "Inline namespaces are not supported yet");
  }
  if (namespace_decl->isAnonymousNamespace()) {
    return ictx_.ImportUnsupportedItem(
        namespace_decl, "Anonymous namespaces are not supported yet");
  }

  ictx_.ImportDeclsFromDeclContext(namespace_decl);
  auto identifier = ictx_.GetTranslatedIdentifier(namespace_decl);
  CRUBIT_CHECK(identifier.has_value());
  auto item_ids = ictx_.GetItemIdsInSourceOrder(namespace_decl);
  return Namespace{
      .name = *identifier,
      .id = GenerateItemId(namespace_decl),
      .owning_target = ictx_.GetOwningTarget(namespace_decl),
      .child_item_ids = std::move(item_ids),
  };
}

}  // namespace crubit
