// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importers/namespace.h"

#include <optional>
#include <utility>

#include "absl/log/check.h"
#include "absl/strings/str_cat.h"
#include "rs_bindings_from_cc/ast_util.h"
#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/Decl.h"

namespace crubit {

std::optional<IR::Item> NamespaceDeclImporter::Import(
    clang::NamespaceDecl* namespace_decl) {
  if (namespace_decl->isAnonymousNamespace()) {
    return ictx_.ImportUnsupportedItem(
        namespace_decl, "Anonymous namespaces are not supported yet");
  }

  absl::StatusOr<Identifier> identifier =
      ictx_.GetTranslatedIdentifier(namespace_decl);
  if (!identifier.ok()) {
    return ictx_.ImportUnsupportedItem(
        namespace_decl, absl::StrCat("Namespace name is not supported: ",
                                     identifier.status().message()));
  }

  ictx_.ImportDeclsFromDeclContext(namespace_decl);
  auto item_ids = ictx_.GetItemIdsInSourceOrder(namespace_decl);
  return Namespace{
      .name = *identifier,
      .id = ictx_.GenerateItemId(namespace_decl),
      .canonical_namespace_id =
          ictx_.GenerateItemId(namespace_decl->getCanonicalDecl()),
      .unknown_attr = CollectUnknownAttrs(*namespace_decl),
      .owning_target = ictx_.GetOwningTarget(namespace_decl),
      .child_item_ids = std::move(item_ids),
      .enclosing_namespace_id = ictx_.GetEnclosingNamespaceId(namespace_decl),
      .is_inline = namespace_decl->isInline()};
}

}  // namespace crubit
