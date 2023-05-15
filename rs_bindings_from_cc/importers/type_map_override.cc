// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importers/type_map_override.h"

#include <optional>
#include <utility>

#include "rs_bindings_from_cc/type_map.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Type.h"
#include "llvm/Support/raw_ostream.h"

namespace crubit {

std::optional<IR::Item> TypeMapOverrideImporter::Import(
    clang::TypeDecl* type_decl) {
  clang::ASTContext& context = type_decl->getASTContext();
  const clang::Type* type = context.getTypeDeclType(type_decl).getTypePtr();
  if (type == nullptr) return std::nullopt;
  if (auto override_type = GetTypeMapOverride(*type);
      override_type.ok() && override_type->has_value()) {
    std::optional<SizeAlign> size_align;
    if (!type->isIncompleteType()) {
      size_align = SizeAlign{
          .size = context.getTypeSizeInChars(type).getQuantity(),
          .alignment = context.getTypeAlignInChars(type).getQuantity(),
      };
    }
    return TypeMapOverride{
        .type = **std::move(override_type),
        .owning_target = ictx_.GetOwningTarget(type_decl),
        .size_align = std::move(size_align),
        .id = GenerateItemId(type_decl),
    };
  }
  return std::nullopt;
}

}  // namespace crubit
