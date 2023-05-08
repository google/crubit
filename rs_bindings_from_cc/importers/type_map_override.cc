// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importers/type_map_override.h"

#include <optional>

#include "rs_bindings_from_cc/type_map.h"

namespace crubit {

std::optional<IR::Item> TypeMapOverrideImporter::Import(
    clang::TypeDecl* type_decl) {
  const clang::Type* type =
      type_decl->getASTContext().getTypeDeclType(type_decl).getTypePtr();
  if (type == nullptr) return std::nullopt;
  if (auto override_type = TypeMapOverride(*type);
      override_type.ok() && override_type->has_value()) {
    // TODO(b/274834739): emit size/align assertions for these mapped types.
    return ictx_.ImportUnsupportedItem(
        type_decl, absl::StrCat("Type bindings suppressed due to being "
                                "mapped to an existing Rust type (",
                                (**override_type).rs_type.name, ")"));
  }
  return std::nullopt;
}

}  // namespace crubit
