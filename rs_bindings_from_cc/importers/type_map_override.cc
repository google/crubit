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
  if (auto override_type = GetTypeMapOverride(*type);
      override_type.ok() && override_type->has_value()) {
    return TypeMapOverride{
        .type = **std::move(override_type),
        .owning_target = ictx_.GetOwningTarget(type_decl),
        .id = GenerateItemId(type_decl),
    };
  }
  return std::nullopt;
}

}  // namespace crubit
