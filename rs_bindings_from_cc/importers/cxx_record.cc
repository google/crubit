// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importers/cxx_record.h"

#include "absl/strings/substitute.h"
#include "rs_bindings_from_cc/ast_convert.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/CXXInheritance.h"
#include "clang/AST/RecordLayout.h"
#include "clang/Sema/Sema.h"

namespace crubit {

namespace {

std::string GetClassTemplateSpecializationCcName(
    const clang::ASTContext& ast_context,
    const clang::ClassTemplateSpecializationDecl* specialization_decl) {
  clang::PrintingPolicy policy(ast_context.getLangOpts());
  policy.IncludeTagDefinition = false;
  return clang::QualType(specialization_decl->getTypeForDecl(), 0)
      .getAsString(policy);
}

}  // namespace

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
  if (clang::isa<clang::ClassTemplatePartialSpecializationDecl>(record_decl)) {
    return ictx_.ImportUnsupportedItem(
        record_decl, "Partially-specialized class templates are not supported");
  }
  if (record_decl->isInvalidDecl()) {
    return std::nullopt;
  }

  std::string rs_name, cc_name;
  llvm::Optional<std::string> doc_comment;
  if (auto* specialization_decl =
          clang::dyn_cast<clang::ClassTemplateSpecializationDecl>(
              record_decl)) {
    rs_name = ictx_.GetMangledName(specialization_decl);
    cc_name =
        GetClassTemplateSpecializationCcName(ictx_.ctx_, specialization_decl);
    doc_comment = ictx_.GetComment(specialization_decl);
    if (!doc_comment.hasValue()) {
      doc_comment =
          ictx_.GetComment(specialization_decl->getSpecializedTemplate());
    }
  } else {
    std::optional<Identifier> record_name =
        ictx_.GetTranslatedIdentifier(record_decl);
    if (!record_name.has_value()) {
      return std::nullopt;
    }
    rs_name = cc_name = record_name->Ident();
    doc_comment = ictx_.GetComment(record_decl);
  }

  if (clang::CXXRecordDecl* complete = record_decl->getDefinition()) {
    record_decl = complete;
  } else {
    CRUBIT_CHECK(!record_decl->isCompleteDefinition());
    ictx_.type_mapper_.Insert(record_decl);
    return IncompleteRecord{
        .cc_name = std::move(cc_name),
        .id = GenerateItemId(record_decl),
        .owning_target = ictx_.GetOwningTarget(record_decl),
        .enclosing_namespace_id = GetEnclosingNamespaceId(record_decl)};
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
      .rs_name = std::move(rs_name),
      .cc_name = std::move(cc_name),
      .id = GenerateItemId(record_decl),
      .owning_target = ictx_.GetOwningTarget(record_decl),
      .doc_comment = std::move(doc_comment),
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

absl::StatusOr<std::vector<Field>> CXXRecordDeclImporter::ImportFields(
    clang::CXXRecordDecl* record_decl) {
  // Provisionally assume that we know this RecordDecl so that we'll be able
  // to import fields whose type contains the record itself.
  TypeMapper temp_import_mapper(ictx_.type_mapper_);
  temp_import_mapper.Insert(record_decl);

  clang::AccessSpecifier default_access =
      record_decl->isClass() ? clang::AS_private : clang::AS_public;
  std::vector<Field> fields;
  const clang::ASTRecordLayout& layout =
      ictx_.ctx_.getASTRecordLayout(record_decl);
  for (const clang::FieldDecl* field_decl : record_decl->fields()) {
    std::optional<clang::tidy::lifetimes::ValueLifetimes> no_lifetimes;
    auto type =
        temp_import_mapper.ConvertQualType(field_decl->getType(), no_lifetimes);
    if (!type.ok()) {
      return absl::UnimplementedError(absl::Substitute(
          "Type of field '$0' is not supported: $1",
          field_decl->getNameAsString(), type.status().message()));
    }
    clang::AccessSpecifier access = field_decl->getAccess();
    if (access == clang::AS_none) {
      access = default_access;
    }

    std::optional<Identifier> field_name =
        ictx_.GetTranslatedIdentifier(field_decl);
    CRUBIT_CHECK(
        field_name ||
        !field_decl->hasAttr<clang::NoUniqueAddressAttr>() &&
            "Unnamed fields can't be annotated with [[no_unique_address]]");
    fields.push_back(
        {.identifier = field_name ? *std::move(field_name)
                                  : llvm::Optional<Identifier>(llvm::None),
         .doc_comment = ictx_.GetComment(field_decl),
         .type = *type,
         .access = TranslateAccessSpecifier(access),
         .offset = layout.getFieldOffset(field_decl->getFieldIndex()),
         .is_no_unique_address =
             field_decl->hasAttr<clang::NoUniqueAddressAttr>()});
  }
  return fields;
}

std::vector<BaseClass> CXXRecordDeclImporter::GetUnambiguousPublicBases(
    const clang::CXXRecordDecl& record_decl) const {
  // This function is unfortunate: the only way to correctly get information
  // about the bases is lookupInBases. It runs a complex O(N^3) algorithm for
  // e.g. correctly determining virtual base paths, etc.
  //
  // However, lookupInBases does not recurse into a class once it's found.
  // So we need to call lookupInBases once per class, making this O(N^4).

  llvm::SmallPtrSet<const clang::CXXRecordDecl*, 4> seen;
  std::vector<BaseClass> bases;
  clang::CXXBasePaths paths;
  // the const cast is a common pattern, apparently, see e.g.
  // https://clang.llvm.org/doxygen/CXXInheritance_8cpp_source.html#l00074
  paths.setOrigin(const_cast<clang::CXXRecordDecl*>(&record_decl));

  auto next_class = [&]() {
    const clang::CXXRecordDecl* found = nullptr;

    // Matches the first new class it encounters (and adds it to `seen`, so
    // that future runs don't rediscover it.)
    auto is_new_class = [&](const clang::CXXBaseSpecifier* base_specifier,
                            clang::CXXBasePath&) {
      const auto* record_decl = base_specifier->getType()->getAsCXXRecordDecl();
      if (found) {
        return record_decl == found;
      }

      if (record_decl && seen.insert(record_decl).second) {
        found = record_decl;
        return true;
      }
      return false;
    };
    return record_decl.lookupInBases(is_new_class, paths);
  };

  for (; next_class(); paths.clear()) {
    for (const clang::CXXBasePath& path : paths) {
      if (path.Access != clang::AS_public) {
        continue;
      }
      const clang::CXXBaseSpecifier& base_specifier =
          *path[path.size() - 1].Base;
      const clang::QualType& base = base_specifier.getType();
      if (paths.isAmbiguous(ictx_.ctx_.getCanonicalType(base))) {
        continue;
      }

      clang::CXXRecordDecl* base_record_decl =
          CRUBIT_DIE_IF_NULL(base_specifier.getType()->getAsCXXRecordDecl());
      if (!ictx_.type_mapper_.ConvertTypeDecl(base_record_decl).status().ok()) {
        continue;
      }

      llvm::Optional<int64_t> offset = {0};
      for (const clang::CXXBasePathElement& base_path_element : path) {
        if (base_path_element.Base->isVirtual()) {
          offset.reset();
          break;
        }
        *offset +=
            {ictx_.ctx_.getASTRecordLayout(base_path_element.Class)
                 .getBaseClassOffset(CRUBIT_DIE_IF_NULL(
                     base_path_element.Base->getType()->getAsCXXRecordDecl()))
                 .getQuantity()};
      }
      CRUBIT_CHECK((!offset.hasValue() || *offset >= 0) &&
                   "Concrete base classes should have non-negative offsets.");
      bases.push_back(
          BaseClass{.base_record_id = GenerateItemId(base_record_decl),
                    .offset = offset});
      break;
    }
  }
  return bases;
}

}  // namespace crubit
