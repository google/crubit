// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_IMPORTER_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_IMPORTER_H_

#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "absl/base/nullability.h"
#include "absl/container/flat_hash_map.h"
#include "absl/container/flat_hash_set.h"
#include "absl/log/check.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "lifetime_annotations/type_lifetimes.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/decl_importer.h"
#include "rs_bindings_from_cc/ir.h"
#include "rs_bindings_from_cc/ir.pb.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/DeclTemplate.h"
#include "clang/AST/Mangle.h"
#include "clang/AST/RawCommentList.h"
#include "clang/AST/Type.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Sema/Sema.h"

namespace crubit {

namespace ir_proto = rs_bindings_from_cc::ir_proto::flat;

// Stateful entry to prevent re-entrant imports, and to track the underlying
// proto item we should store the AST node in.
struct ItemCacheEntry {
  enum class Status { kInProgress, kCompleted, kFailed, kUnsupported };
  Status status = Status::kCompleted;
  ItemId id = ItemId(0);
  std::unique_ptr<ir_proto::Item> proto_item;
};

// Iterates over the AST created from the invocation's entry headers and
// creates an intermediate representation of the import (`IR`) into the
// invocation object.
class Importer final : public ImportContext {
 public:
  explicit Importer(Invocation& invocation, clang::ASTContext& ctx,
                    clang::Sema& sema);

  // Import all visible declarations from a translation unit.
  void Import(clang::TranslationUnitDecl* absl_nonnull decl);

 protected:
  // Implementation of `ImportContext`
  void ImportDeclsFromDeclContext(
      const clang::DeclContext& decl_context) override;
  std::unique_ptr<ir_proto::Item> HardError(const clang::Decl& decl,
                                            FormattedError error) override;
  std::unique_ptr<ir_proto::Item> ImportUnsupportedItem(
      const clang::Decl& decl,
      std::optional<ir_proto::UnsupportedItem::Path> path,
      std::vector<FormattedError> errors, bool is_hard_error) override;
  absl_nullable std::unique_ptr<ir_proto::Item> ImportDecl(
      clang::Decl* absl_nonnull decl) override;
  const ir_proto::Item* absl_nullable GetImportedItem(
      const clang::Decl& decl) const override;

  ItemId GenerateItemId(const clang::Decl& decl) const override;
  ItemId GenerateItemId(const clang::RawComment& comment) const override;
  bool IsUnsupportedAndAlien(ItemId item_id) const override;
  absl::StatusOr<std::optional<ItemId>> GetEnclosingItemId(
      clang::Decl* absl_nonnull decl) override;

  // The canonical children and comments that are within a decl.
  // This is the return type of `GetDeclItems`, and is intended to only be used
  // to abstract shared behavior between GetTopLevelItemIdsInSourceOrder and
  // GetItemIdsInSourceOrder.
  struct DeclItems {
    std::vector<const clang::RawComment*> comments = {};
    std::vector<std::pair<clang::Decl*, ItemId>> canonical_children = {};
  };

  // This is intended to only be used to abstract shared behavior between
  // GetTopLevelItemIdsInSourceOrder and GetItemIdsInSourceOrder.
  DeclItems GetDeclItems(const clang::Decl& decl);

  absl::flat_hash_map<BazelLabel, std::vector<ItemId>>
  GetTopLevelItemIdsInSourceOrder(
      const clang::TranslationUnitDecl& decl) override;
  std::vector<ItemId> GetItemIdsInSourceOrder(
      clang::Decl* absl_nonnull decl) override;
  std::string GetMangledName(const clang::NamedDecl& named_decl) const override;
  std::optional<ir_proto::UnsupportedItem::Path>
  GetUnsupportedItemPathForTemplateDecl(
      clang::RedeclarableTemplateDecl* absl_nonnull template_decl) override;
  BazelLabel GetOwningTarget(const clang::Decl& decl) const override;
  bool IsFromCurrentTarget(const clang::Decl& decl) const override;
  bool RefersToOwnedDefinition(const clang::CXXRecordDecl& decl) const override;
  bool IsFromCurrentTargetAndNotUnderSpecialization(
      const clang::Decl& decl) const;
  bool IsFromProtoTarget(const clang::Decl& decl) const override;
  bool IsCrubitEnabledForTarget(const BazelLabel& label) const override;
  bool AreAssumedLifetimesEnabledForTarget(
      const BazelLabel& label) const override;
  std::optional<std::string> GetTemplateArgumentLifetimeParam(
      const clang::ClassTemplateSpecializationDecl& specialization_decl)
      const override;
  bool IsUnsafeViewEnabledForTarget(const BazelLabel& label) const override;
  bool IsRecordImplDebugEnabledForTarget(
      const BazelLabel& label) const override;
  absl::StatusOr<bool> DetectFormatter(
      const clang::TypeDecl& decl) const override;
  bool ImplementsCoreFmtDebug(const clang::TypeDecl& type) const override;
  absl::StatusOr<std::optional<bool>> GetCrubitOverrideDebugAnnotation(
      const clang::TypeDecl& type) const override;
  absl::StatusOr<TranslatedUnqualifiedIdentifier> GetTranslatedName(
      const clang::NamedDecl& named_decl) const override;
  absl::StatusOr<TranslatedIdentifier> GetTranslatedIdentifier(
      const clang::NamedDecl& named_decl) const override;
  std::optional<std::string> GetComment(const clang::Decl& decl) const override;
  std::string ConvertSourceLocation(
      clang::SourceLocation loc,
      clang::DeclarationNameInfo* absl_nullable name_info) const override;
  CcType ConvertQualType(
      clang::QualType qual_type,
      const clang::tidy::lifetimes::ValueLifetimes* absl_nullable lifetimes,
      bool nullable, bool assume_lifetimes) override;

  std::string GetUniqueName(const clang::Decl& decl) const override;

  void MarkAsSuccessfullyImported(const clang::NamedDecl& decl) override;
  bool HasBeenAlreadySuccessfullyImported(
      const clang::NamedDecl& decl) const override;
  void MarkAsInvalidTemplateSpecialization(
      const clang::ClassTemplateSpecializationDecl& decl,
      std::string reason) override;
  bool IsInvalidTemplateSpecialization(
      const clang::CXXRecordDecl& decl) const override;
  bool EnsureSuccessfullyImported(
      clang::NamedDecl* absl_nonnull decl) override {
    // First, return early so that we avoid re-entrant imports.
    if (HasBeenAlreadySuccessfullyImported(*decl)) return true;
    (void)GetDeclItem(CanonicalizeDecl(decl));
    return HasBeenAlreadySuccessfullyImported(*decl);
  }

  clang::TypedefNameDecl* absl_nullable GetTemplateSpecializationAlias(
      clang::Decl* absl_nonnull decl) const override;

 private:
  class SourceOrderKey;
  class SourceLocationComparator;

  // Returns a SourceOrderKey for the given `decl` that should be used for
  // ordering Items.
  SourceOrderKey GetSourceOrderKey(const clang::Decl& decl) const;
  // Returns a SourceOrderKey for the given `comment` that should be used for
  // ordering Items.
  SourceOrderKey GetSourceOrderKey(const clang::RawComment& comment) const;

  // Returns a name for `decl` that should be used for ordering declarations.
  std::string GetNameForSourceOrder(const clang::Decl& decl) const;

  // Returns the item ids of template instantiations that have been triggered
  // from the current target.  The returned items are in an arbitrary,
  // deterministic/reproducible order.
  std::vector<ItemId> GetOrderedItemIdsOfTemplateInstantiations() const;

  // Checks the invariants relied upon by `Import` in lazy alien-import mode:
  // every imported decl has all of its enclosing namespaces in the cache, and
  // every imported namespace has its canonical namespace in the cache (the Rust
  // side looks up `canonical_namespace_id`). This is expensive, so it is only
  // called when `--check_importer_invariants` is set.
  void CheckLazyImportInvariants() const;

  void FindAlwaysInstantiateSpecs(const clang::DeclContext& decl_context);
  bool IsAlwaysInstantiate(
      const clang::ClassTemplateSpecializationDecl& spec_decl) const;

  // Returns whether `type`, used as a template argument, carries exactly one
  // lifetime: see `GetTemplateArgumentLifetimeParam`.
  bool HasSingleLifetime(clang::QualType type) const;

  absl::flat_hash_set<const clang::ClassTemplateSpecializationDecl*>
      always_instantiate_specs_;

  const ir_proto::Item* absl_nullable GetDeclItem(
      clang::Decl* absl_nonnull decl) override;
  // Stores the comments of this target in source order.
  void ImportFreeComments();

  clang::Decl* absl_nullable CanonicalizeDecl(
      clang::Decl* absl_nonnull decl) const;
  const clang::Decl* absl_nullable CanonicalizeDecl(
      const clang::Decl& decl) const;

  std::vector<clang::Decl*> GetCanonicalChildren(
      const clang::DeclContext& decl_context) const;
  // Converts a type to a CcType.
  // TODO(b/251045039): Return a `CcType`.
  absl::StatusOr<CcType> ConvertType(
      const clang::Type& type,
      const clang::tidy::lifetimes::ValueLifetimes* absl_nullable lifetimes,
      bool nullable, bool assume_lifetimes);
  // Converts a type, without processing attributes.
  // TODO(b/251045039): Return a `CcType`.
  absl::StatusOr<CcType> ConvertUnattributedType(
      const clang::Type& type,
      const clang::tidy::lifetimes::ValueLifetimes* absl_nullable lifetimes,
      bool nullable, bool assume_lifetimes);
  // Adds the lifetime parameter that a specialization takes for its template
  // argument (see `GetTemplateArgumentLifetimeParam`) to `cpp_type`, the
  // conversion of `type`: either bound to the lifetime written on the
  // argument, if `type` is a use of the specialization, or unbound, if `type`
  // is the template parameter inside it. `has_written_lifetimes` is whether
  // `type` itself was written with lifetimes.
  absl::Status AddTemplateArgumentLifetime(const clang::Type& type,
                                           bool has_written_lifetimes,
                                           CcType& cpp_type);
  CcType ConvertTypeDecl(clang::NamedDecl* absl_nonnull decl);

  // Converts `type` into a CcType, after first importing the Record behind
  // the template instantiation.
  CcType ConvertTemplateSpecializationType(
      const clang::TemplateSpecializationType& type, bool assume_lifetimes);

  // Attaches the template arguments of `type` as written at this use site to
  // `converted`, so that per-use information the shared specialization decl
  // cannot carry (e.g. nullability or lifetimes) reaches codegen.
  //
  // Only a single written argument, which must be a type, is recorded, and
  // only if it carries an explicit lifetime or a known nullability.
  // Records nothing if it fails to convert, except that it returns an error
  // type if the argument carries an explicit lifetime annotation, because the
  // lifetime was written down in the source, so dropping it silently would
  // produce bindings that disagree with the header.
  CcType WithAsWrittenTemplateArgs(
      CcType converted, const clang::TemplateSpecializationType& type,
      bool assume_lifetimes);

  // Eagerly instantiates `specialization_decl` (and everything its template
  // arguments name), and returns the diagnostics emitted while doing so, or
  // `std::nullopt` if everything could be instantiated.
  //
  // The result is memoized in `instantiation_attempts_`, which is required for
  // correctness rather than just for speed -- see the comment there.
  std::optional<std::string> CheckSpecializationInstantiable(
      clang::ClassTemplateSpecializationDecl* absl_nonnull specialization_decl);
  std::optional<std::string> CheckTemplateArgInstantiable(
      const clang::TemplateArgument& arg);

  bool RefersToOwnedDefinitionImpl(
      const clang::CXXRecordDecl& decl,
      absl::flat_hash_set<const clang::CXXRecordDecl*>& visited) const;

  bool IsFeatureEnabledForTarget(const BazelLabel& label,
                                 absl::string_view feature) const override;

  absl::StatusOr<std::optional<bool>> GetCrubitOverrideDisplayAnnotation(
      const clang::TypeDecl& decl) const;

  absl::StatusOr<std::optional<bool>> DetectFormatterForType(
      clang::CanQualType lookup, clang::CanQualType target) const;

  // The different decl importers. Note that order matters: the first importer
  // to successfully match a decl "wins", and no other importers are tried.
  std::vector<std::unique_ptr<DeclImporter>> decl_importers_;
  std::unique_ptr<clang::MangleContext> mangler_;
  // Mangler used *only* for mangling tag type names (see `GetMangledName`).
  //
  // This is always an Itanium mangler, even when the target platform uses
  // another C++ ABI (e.g. the Microsoft ABI).  See `GetMangledName` for why.
  std::unique_ptr<clang::MangleContext> itanium_tag_name_mangler_;
  absl::flat_hash_map<const clang::Decl*, ItemCacheEntry> import_cache_;
  absl::flat_hash_set<const clang::ClassTemplateSpecializationDecl*>
      class_template_instantiations_;
  // Memoizes the outcome of the eager instantiation attempts performed by
  // `CheckSpecializationInstantiable`, keyed by the canonical decl of the
  // specialization. The value holds the diagnostics emitted by the (first and
  // only) instantiation attempt, or `std::nullopt` if it succeeded.
  //
  // This cache is required for correctness, not just for speed: Clang reports
  // the errors of a failed instantiation only once, and leaves behind a
  // complete but ill-formed definition, so later attempts to complete the same
  // type silently succeed.
  absl::flat_hash_map<const clang::CXXRecordDecl*, std::optional<std::string>>
      instantiation_attempts_;

  std::vector<const clang::RawComment*> comments_;

  // Set of decls that have been successfully imported (i.e. that will be
  // present in the IR output / that will not produce dangling ItemIds in the IR
  // output).
  //
  // Note that this includes non-TypeDecls in the form of using decls.
  absl::flat_hash_set<const clang::NamedDecl*> known_type_decls_;

  clang::QualType rs_core_fmt_debug_;
  const clang::ClassTemplateDecl* absl_nullable rs_std_impl_;

  // Returns the nullability that `type` has in the absence of an explicit
  // annotation: the `#pragma nullability file_default` of the file in which
  // `type` was written, if any.
  //
  // This mirrors the "governing file" logic of the nullability library
  // (`getGoverningFile` in nullability/type_nullability.cc).
  clang::NullabilityKindOrNone GetDefaultNullability(
      const clang::Type& type) const;

  // Returns true if `type`, as written, contains a type whose nullability is
  // known: either annotated explicitly, or covered by a
  // `#pragma nullability file_default`. Without either, every nullability in
  // `type` is unspecified, which is exactly what the canonical type says.
  bool ContainsKnownNullability(clang::QualType type) const;

  // The file whose `#pragma nullability file_default` governs types that are
  // spelled directly in the decl currently being imported (i.e. not via a
  // typedef). Set by `ImportDecl`.
  clang::FileID governing_file_;
};  // class Importer

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IMPORTER_H_
