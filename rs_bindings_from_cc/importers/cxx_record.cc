// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importers/cxx_record.h"

#include "rs_bindings_from_cc/ast_convert.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/ASTContext.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/RecordLayout.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Sema/Sema.h"

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
        .owning_target = ictx_.GetOwningTarget(record_decl)};
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

  llvm::Optional<size_t> base_size;
  bool override_alignment = record_decl->hasAttr<clang::AlignedAttr>();
  if (record_decl->getNumBases() != 0) {
    // The size of the base class subobjects is easy to compute, so long as we
    // know that fields start after the base class subobjects. (This is not
    // guaranteed by the standard, but is true on the ABIs we work with.)
    base_size = layout.getFieldCount() == 0
                    ? static_cast<size_t>(layout.getDataSize().getQuantity())
                    : layout.getFieldOffset(0) / 8;
    // Ideally, we'd only include an alignment adjustment if one of the base
    // classes is more-aligned than any of the fields, but it is simpler do it
    // whenever there are any base classes at all.
    override_alignment = true;
  }

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
      .base_size = base_size,
      .override_alignment = override_alignment,
      .copy_constructor = GetCopyCtorSpecialMemberFunc(*record_decl),
      .move_constructor = GetMoveCtorSpecialMemberFunc(*record_decl),
      .destructor = GetDestructorSpecialMemberFunc(*record_decl),
      .is_trivial_abi = record_decl->canPassInRegisters(),
      .is_inheritable =
          !record_decl->isEffectivelyFinal() && !record_decl->isUnion(),
      .is_union = record_decl->isUnion(),
      .child_item_ids = std::move(item_ids)};
}

}  // namespace crubit
