// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importers/cxx_record.h"

#include "rs_bindings_from_cc/ast_convert.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/RecordLayout.h"
#include "clang/Sema/Sema.h"

namespace crubit {

std::optional<IR::Item> CXXRecordDeclImporter::Import(
    clang::CXXRecordDecl* record_decl) {
  const clang::DeclContext* decl_context = record_decl->getDeclContext();
  if (decl_context->isFunctionOrMethod()) {
    return std::nullopt;
  }
  if (record_decl->isInjectedClassName()) {
    return std::nullopt;
  }
  if (decl_context->isRecord()) {
    return ictx_.ImportUnsupportedItem(record_decl,
                                       "Nested classes are not supported yet");
  }
  if (record_decl->isInvalidDecl()) {
    return std::nullopt;
  }

  std::optional<Identifier> record_name =
      ictx_.GetTranslatedIdentifier(record_decl);
  if (!record_name.has_value()) {
    return std::nullopt;
  }

  if (clang::CXXRecordDecl* complete = record_decl->getDefinition()) {
    record_decl = complete;
  } else {
    CRUBIT_CHECK(!record_decl->isCompleteDefinition());
    ictx_.type_mapper_.Insert(record_decl);
    return IncompleteRecord{
        .cc_name = std::string(record_name->Ident()),
        .id = GenerateItemId(record_decl),
        .owning_target = ictx_.GetOwningTarget(record_decl),
        .enclosing_namespace_id = GetEnclosingNamespaceId(record_decl)};
  }

  // To compute the memory layout of the record, it needs to be a concrete type,
  // not a template.
  if (record_decl->getDescribedClassTemplate() ||
      clang::isa<clang::ClassTemplateSpecializationDecl>(record_decl)) {
    return ictx_.ImportUnsupportedItem(record_decl,
                                       "Class templates are not supported yet");
  }

  ictx_.sema_.ForceDeclarationOfImplicitMembers(record_decl);

  const clang::ASTRecordLayout& layout =
      ictx_.ctx_.getASTRecordLayout(record_decl);

  bool is_derived_class = record_decl->getNumBases() != 0;
  bool override_alignment = record_decl->hasAttr<clang::AlignedAttr>() ||
                            is_derived_class || layout.hasOwnVFPtr();

  absl::StatusOr<std::vector<Field>> fields = ImportFields(record_decl);
  if (!fields.ok()) {
    return ictx_.ImportUnsupportedItem(record_decl, fields.status().ToString());
  }

  for (const Field& field : *fields) {
    if (field.is_no_unique_address) {
      override_alignment = true;
      break;
    }
  }

  ictx_.type_mapper_.Insert(record_decl);

  auto item_ids = ictx_.GetItemIdsInSourceOrder(record_decl);
  return Record{
      .rs_name = std::string(record_name->Ident()),
      .cc_name = std::string(record_name->Ident()),
      .id = GenerateItemId(record_decl),
      .owning_target = ictx_.GetOwningTarget(record_decl),
      .doc_comment = ictx_.GetComment(record_decl),
      .unambiguous_public_bases = GetUnambiguousPublicBases(*record_decl),
      .fields = *std::move(fields),
      .size = layout.getSize().getQuantity(),
      .alignment = layout.getAlignment().getQuantity(),
      .is_derived_class = is_derived_class,
      .override_alignment = override_alignment,
      .copy_constructor = GetCopyCtorSpecialMemberFunc(*record_decl),
      .move_constructor = GetMoveCtorSpecialMemberFunc(*record_decl),
      .destructor = GetDestructorSpecialMemberFunc(*record_decl),
      .is_trivial_abi = record_decl->canPassInRegisters(),
      .is_inheritable =
          !record_decl->isEffectivelyFinal() && !record_decl->isUnion(),
      .is_union = record_decl->isUnion(),
      .child_item_ids = std::move(item_ids),
      .enclosing_namespace_id = GetEnclosingNamespaceId(record_decl),
  };
}

}  // namespace crubit
