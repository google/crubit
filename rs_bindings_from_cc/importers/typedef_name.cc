// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importers/typedef_name.h"

#include "absl/log/check.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Attr.h"
#include "clang/AST/Decl.h"

namespace crubit {

std::optional<IR::Item> crubit::TypedefNameDeclImporter::Import(
    clang::TypedefNameDecl* typedef_name_decl) {
  const clang::DeclContext* decl_context = typedef_name_decl->getDeclContext();
  llvm::Optional<ItemId> enclosing_record_id = llvm::None;
  if (decl_context) {
    if (decl_context->isFunctionOrMethod()) {
      return std::nullopt;
    }
    if (auto* record_decl = llvm::dyn_cast<clang::RecordDecl>(decl_context)) {
      enclosing_record_id = GenerateItemId(record_decl);
    }
  }

  clang::QualType type =
      typedef_name_decl->getASTContext().getTypedefType(typedef_name_decl);
  if (MapKnownCcTypeToRsType(type.getAsString()).has_value()) {
    return std::nullopt;
  }

  std::optional<Identifier> identifier =
      ictx_.GetTranslatedIdentifier(typedef_name_decl);
  CHECK(identifier.has_value());  // This must always hold.

  std::optional<clang::tidy::lifetimes::ValueLifetimes> no_lifetimes;
  absl::StatusOr<MappedType> underlying_type = ictx_.ConvertQualType(
      typedef_name_decl->getUnderlyingType(), no_lifetimes);

  if (underlying_type.ok()) {
    if (const auto* tag_decl = type->getAsTagDecl();
        tag_decl && tag_decl->getDeclContext() == decl_context &&
        tag_decl->getName() == typedef_name_decl->getName()) {
      return ictx_.ImportUnsupportedItem(
          typedef_name_decl,
          "Typedef only used to introduce a name in C. Not importing.");
    }
    ictx_.MarkAsSuccessfullyImported(typedef_name_decl);
    return TypeAlias{
        .identifier = *identifier,
        .id = GenerateItemId(typedef_name_decl),
        .owning_target = ictx_.GetOwningTarget(typedef_name_decl),
        .doc_comment = ictx_.GetComment(typedef_name_decl),
        .underlying_type = *underlying_type,
        .source_loc =
            ictx_.ConvertSourceLocation(typedef_name_decl->getBeginLoc()),
        .enclosing_record_id = enclosing_record_id,
        .enclosing_namespace_id = GetEnclosingNamespaceId(typedef_name_decl),
    };
  } else if (typedef_name_decl->getAnonDeclWithTypedefName()) {
    auto* type = typedef_name_decl->getUnderlyingType().getTypePtrOrNull();
    if (type && type->getAsRecordDecl()) {
      // This is an anonymous declaration with a typedef name. (e.g. `typedef
      // struct {} Foo;` cases)
      ictx_.AddAnonDeclTypedefName(type->getAsRecordDecl(),
                                   identifier->Ident());
      auto item = ictx_.ImportDecl(type->getAsRecordDecl());
      if (item.has_value()) {
        // If the align attribute was attached to the typedef decl, we should
        // apply it to the generated record.
        if (auto* record = std::get_if<Record>(&item.value())) {
          auto* aligned = typedef_name_decl->getAttr<clang::AlignedAttr>();
          if (aligned) {
            record->alignment =
                ictx_.ctx_
                    .toCharUnitsFromBits(aligned->getAlignment(ictx_.ctx_))
                    .getQuantity();
            record->override_alignment = true;

            // If it has alignment, update the `record->size` to the aligned
            // one, because that size is going to be used as this record's
            // canonical size in IR and in the binding code.

            // Make sure that `alignment` is a power of 2.
            CHECK(!(record->alignment & (record->alignment - 1)));

            // Given that `alignment` is a power of 2, we can round it up by
            // a bit arithmetic: `alignment - 1` clears the single bit of it
            // while turning all the zeros in the right to 1s. Adding
            // `alignment - 1` and doing &~ with it effectively rounds it up
            // to the next multiple of the alignment.
            record->size = (record->size + record->alignment - 1) &
                           ~(record->alignment - 1);
          }
          record->is_anon_record_with_typedef = true;
        }
        return item;
      }
    }
  }
  return ictx_.ImportUnsupportedItem(
      typedef_name_decl, std::string(underlying_type.status().message()));
}

}  // namespace crubit
