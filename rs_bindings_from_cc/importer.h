// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_IMPORTER_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_IMPORTER_H_

#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/container/flat_hash_set.h"
#include "absl/log/check.h"
#include "absl/log/die_if_null.h"
#include "absl/status/statusor.h"
#include "lifetime_annotations/type_lifetimes.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/decl_importer.h"
#include "rs_bindings_from_cc/importers/class_template.h"
#include "rs_bindings_from_cc/importers/cxx_record.h"
#include "rs_bindings_from_cc/importers/enum.h"
#include "rs_bindings_from_cc/importers/friend.h"
#include "rs_bindings_from_cc/importers/function.h"
#include "rs_bindings_from_cc/importers/function_template.h"
#include "rs_bindings_from_cc/importers/namespace.h"
#include "rs_bindings_from_cc/importers/type_alias.h"
#include "rs_bindings_from_cc/importers/type_map_override.h"
#include "rs_bindings_from_cc/importers/var.h"
#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/DeclTemplate.h"
#include "clang/AST/Mangle.h"
#include "clang/AST/RawCommentList.h"
#include "clang/AST/Type.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Sema/Sema.h"

namespace crubit {

// Iterates over the AST created from the invocation's entry headers and
// creates an intermediate representation of the import (`IR`) into the
// invocation object.
class Importer final : public ImportContext {
 public:
  explicit Importer(Invocation& invocation, clang::ASTContext& ctx,
                    clang::Sema& sema)
      : ImportContext(invocation, ctx, sema),
        mangler_(ABSL_DIE_IF_NULL(ctx_.createMangleContext())) {
    decl_importers_.push_back(std::make_unique<TypeMapOverrideImporter>(*this));
    decl_importers_.push_back(
        std::make_unique<ClassTemplateDeclImporter>(*this));
    decl_importers_.push_back(std::make_unique<CXXRecordDeclImporter>(*this));
    decl_importers_.push_back(std::make_unique<EnumDeclImporter>(*this));
    decl_importers_.push_back(std::make_unique<VarDeclImporter>(*this));
    decl_importers_.push_back(std::make_unique<FriendDeclImporter>(*this));
    decl_importers_.push_back(std::make_unique<FunctionDeclImporter>(*this));
    decl_importers_.push_back(
        std::make_unique<FunctionTemplateDeclImporter>(*this));
    decl_importers_.push_back(std::make_unique<NamespaceDeclImporter>(*this));
    decl_importers_.push_back(std::make_unique<TypeAliasImporter>(*this));
  }

  // Import all visible declarations from a translation unit.
  void Import(clang::TranslationUnitDecl* decl);

 protected:
  // Implementation of `ImportContext`
  void ImportDeclsFromDeclContext(
      const clang::DeclContext* decl_context) override;
  IR::Item HardError(const clang::Decl& decl, FormattedError error) override;
  IR::Item ImportUnsupportedRecord(const clang::TagDecl& decl,
                                   std::optional<UnsupportedItem::Path> path,
                                   FormattedError error) override;
  IR::Item ImportUnsupportedFunc(const clang::NamedDecl& decl,
                                 std::optional<UnsupportedItem::Path> path,
                                 FormattedError error) override;
  IR::Item ImportUnsupportedFunc(const clang::NamedDecl& decl,
                                 std::optional<UnsupportedItem::Path> path,
                                 std::vector<FormattedError> error) override;
  IR::Item ImportUnsupportedItem(const clang::Decl* decl,
                                 UnsupportedItem::Kind kind,
                                 std::optional<UnsupportedItem::Path> path,
                                 FormattedError error) override;
  IR::Item ImportUnsupportedItem(const clang::Decl* decl,
                                 UnsupportedItem::Kind kind,
                                 std::optional<UnsupportedItem::Path> path,
                                 std::vector<FormattedError> error,
                                 bool is_hard_error);
  std::optional<IR::Item> ImportDecl(clang::Decl* decl) override;
  std::optional<IR::Item> GetImportedItem(
      const clang::Decl* decl) const override;

  ItemId GenerateItemId(const clang::Decl* decl) const override;
  ItemId GenerateItemId(const clang::RawComment* comment) const override;
  bool IsUnsupportedAndAlien(ItemId item_id) const override;
  absl::StatusOr<std::optional<ItemId>> GetEnclosingItemId(
      clang::Decl* decl) override;

  std::vector<ItemId> GetItemIdsInSourceOrder(clang::Decl* decl) override;
  std::string GetMangledName(const clang::NamedDecl* named_decl) const override;
  std::optional<UnsupportedItem::Path> GetUnsupportedItemPathForTemplateDecl(
      clang::RedeclarableTemplateDecl* template_decl) override;
  BazelLabel GetOwningTarget(const clang::Decl* decl) const override;
  bool IsFromCurrentTarget(const clang::Decl* decl) const override;
  absl::StatusOr<TranslatedUnqualifiedIdentifier> GetTranslatedName(
      const clang::NamedDecl* named_decl) const override;
  absl::StatusOr<TranslatedIdentifier> GetTranslatedIdentifier(
      const clang::NamedDecl* named_decl) const override;
  std::optional<std::string> GetComment(const clang::Decl* decl) const override;
  std::string ConvertSourceLocation(clang::SourceLocation loc) const override;
  absl::StatusOr<CcType> ConvertQualType(
      clang::QualType qual_type,
      const clang::tidy::lifetimes::ValueLifetimes* lifetimes,
      bool nullable = true) override;

  void MarkAsSuccessfullyImported(const clang::NamedDecl* decl) override;
  bool HasBeenAlreadySuccessfullyImported(
      const clang::NamedDecl* decl) const override;
  bool EnsureSuccessfullyImported(clang::NamedDecl* decl) override {
    // First, return early so that we avoid re-entrant imports.
    if (HasBeenAlreadySuccessfullyImported(decl)) return true;
    (void)GetDeclItem(CanonicalizeDecl(decl));
    return HasBeenAlreadySuccessfullyImported(decl);
  }

 private:
  class SourceOrderKey;
  class SourceLocationComparator;

  // Returns a SourceOrderKey for the given `decl` that should be used for
  // ordering Items.
  SourceOrderKey GetSourceOrderKey(const clang::Decl* decl) const;
  // Returns a SourceOrderKey for the given `comment` that should be used for
  // ordering Items.
  SourceOrderKey GetSourceOrderKey(const clang::RawComment* comment) const;

  // Returns a name for `decl` that should be used for ordering declarations.
  std::string GetNameForSourceOrder(const clang::Decl* decl) const;

  // Returns the item ids of template instantiations that have been triggered
  // from the current target.  The returned items are in an arbitrary,
  // deterministic/reproducible order.
  std::vector<ItemId> GetOrderedItemIdsOfTemplateInstantiations() const;

  std::optional<IR::Item> GetDeclItem(clang::Decl* decl) override;
  // Stores the comments of this target in source order.
  void ImportFreeComments();

  clang::Decl* CanonicalizeDecl(clang::Decl* decl) const;
  const clang::Decl* CanonicalizeDecl(const clang::Decl* decl) const;

  std::vector<clang::Decl*> GetCanonicalChildren(
      const clang::DeclContext* decl_context) const;
  // Converts a type to a CcType.
  absl::StatusOr<CcType> ConvertType(
      const clang::Type* type,
      const clang::tidy::lifetimes::ValueLifetimes* lifetimes, bool nullable);
  // Converts a type, without processing attributes.
  absl::StatusOr<CcType> ConvertUnattributedType(
      const clang::Type* type,
      const clang::tidy::lifetimes::ValueLifetimes* lifetimes, bool nullable);
  absl::StatusOr<CcType> ConvertTypeDecl(clang::NamedDecl* decl);

  // Converts `type` into a CcType, after first importing the Record behind
  // the template instantiation.
  absl::StatusOr<CcType> ConvertTemplateSpecializationType(
      const clang::TemplateSpecializationType* type);

  // The different decl importers. Note that order matters: the first importer
  // to successfully match a decl "wins", and no other importers are tried.
  std::vector<std::unique_ptr<DeclImporter>> decl_importers_;
  std::unique_ptr<clang::MangleContext> mangler_;
  absl::flat_hash_map<const clang::Decl*, std::optional<IR::Item>>
      import_cache_;
  absl::flat_hash_set<const clang::ClassTemplateSpecializationDecl*>
      class_template_instantiations_;
  std::vector<const clang::RawComment*> comments_;

  // Set of decls that have been successfully imported (i.e. that will be
  // present in the IR output / that will not produce dangling ItemIds in the IR
  // output).
  //
  // Note that this includes non-TypeDecls in the form of using decls.
  absl::flat_hash_set<const clang::NamedDecl*> known_type_decls_;
};  // class Importer

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IMPORTER_H_
