// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_DECL_IMPORTER_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_DECL_IMPORTER_H_

#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/container/flat_hash_set.h"
#include "absl/log/check.h"
#include "absl/status/statusor.h"
#include "absl/types/span.h"
#include "lifetime_annotations/lifetime_annotations.h"
#include "lifetime_annotations/type_lifetimes.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/DeclTemplate.h"
#include "clang/AST/RawCommentList.h"
#include "clang/AST/Type.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Sema/Sema.h"

namespace crubit {

// Top-level parameters as well as return value of an importer invocation.
class Invocation {
 public:
  Invocation(
      BazelLabel target, absl::Span<const HeaderName> public_headers,
      const absl::flat_hash_map<HeaderName, BazelLabel>& header_targets,
      std::optional<absl::flat_hash_set<std::string>> do_not_bind_allowlist)
      : target_(target),
        public_headers_(public_headers),
        lifetime_context_(std::make_shared<
                          clang::tidy::lifetimes::LifetimeAnnotationContext>()),
        do_not_bind_allowlist_(std::move(do_not_bind_allowlist)),
        header_targets_(header_targets) {
    // Caller should verify that the inputs are non-empty.
    CHECK(!public_headers_.empty());
    CHECK(!header_targets_.empty());

    ir_.public_headers.insert(ir_.public_headers.end(), public_headers_.begin(),
                              public_headers.end());
    ir_.current_target = target_;
  }

  // Returns the target of a header, if any.
  std::optional<BazelLabel> header_target(const HeaderName header) const {
    auto it = header_targets_.find(header);
    return (it != header_targets_.end()) ? std::optional(it->second)
                                         : std::nullopt;
  }

  // The main target from which we are importing.
  const BazelLabel target_;

  // The headers from which the import starts.  See the doc comment of
  // `IR::public_headers` and `HeaderName` for more details.
  const absl::Span<const HeaderName> public_headers_;

  const std::shared_ptr<clang::tidy::lifetimes::LifetimeAnnotationContext>
      lifetime_context_;

  const std::optional<absl::flat_hash_set<std::string>> do_not_bind_allowlist_;

  // The main output of the import process
  IR ir_;

 private:
  const absl::flat_hash_map<HeaderName, BazelLabel>& header_targets_;
};

// Explicitly defined interface that defines how `DeclImporter`s are allowed to
// interface with the global state of the importer.
class ImportContext {
 public:
  ImportContext(Invocation& invocation, clang::ASTContext& ctx,
                clang::Sema& sema)
      : invocation_(invocation), ctx_(ctx), sema_(sema) {}
  virtual ~ImportContext() = default;

  // Imports all decls contained in a `DeclContext`.
  virtual void ImportDeclsFromDeclContext(
      const clang::DeclContext* decl_context) = 0;

  // Returns an unsupported item that will result in a hard error at binding
  // generation time.
  virtual IR::Item HardError(const clang::Decl& decl, FormattedError error) = 0;

  // Imports an unsupported struct/union/enum/class with a single formatted
  // error message. Delegates to ImportUnsupportedItem with inferred
  // UnsupportedItem::Kind from the decl parameter.
  virtual IR::Item ImportUnsupportedRecord(
      const clang::TagDecl& decl, std::optional<UnsupportedItem::Path> path,
      FormattedError error) = 0;

  virtual IR::Item ImportUnsupportedFunc(
      const clang::NamedDecl& decl, std::optional<UnsupportedItem::Path> path,
      FormattedError error) = 0;

  // Imports an unsupported function with a vector of formatted error messages.
  virtual IR::Item ImportUnsupportedFunc(
      const clang::NamedDecl& decl, std::optional<UnsupportedItem::Path> path,
      std::vector<FormattedError> error) = 0;

  // Imports an unsupported item with a single formatted error message.
  virtual IR::Item ImportUnsupportedItem(
      const clang::Decl* decl, UnsupportedItem::Kind kind,
      std::optional<UnsupportedItem::Path> path, FormattedError error) = 0;

  // Imports a decl and creates an IR item (or error messages). This allows
  // importers to recursively delegate to other importers.
  // Does not use or update the cache.
  virtual std::optional<IR::Item> ImportDecl(clang::Decl* decl) = 0;

  // Returns the Item of a Decl, importing it first if necessary.
  // Updates the cache.
  virtual std::optional<IR::Item> GetDeclItem(clang::Decl* decl) = 0;

  virtual std::optional<IR::Item> GetImportedItem(
      const clang::Decl* decl) const = 0;

  virtual ItemId GenerateItemId(const clang::Decl* decl) const = 0;
  virtual ItemId GenerateItemId(const clang::RawComment* comment) const = 0;
  // Checks if the given item is unsupported and not from the current target.
  virtual bool IsUnsupportedAndAlien(ItemId item_id) const = 0;
  // Returns the ID of the parent record or namespace, if it exists, and
  // `std::nullopt` for top level decls. We use this function to assign a parent
  // item to all the IR items.
  //
  // Imports the parent decl if it is not already imported, and returns a bad
  // status if the parent cannot be imported.
  virtual absl::StatusOr<std::optional<ItemId>> GetEnclosingItemId(
      clang::Decl* decl) = 0;

  // Returns a map of top level item ids in source order for each target.
  virtual absl::flat_hash_map<BazelLabel, std::vector<ItemId>>
  GetTopLevelItemIdsInSourceOrder(const clang::TranslationUnitDecl* decl) = 0;

  // Imports children of `decl`.
  //
  // Returns item ids of the children that belong to the current target.  This
  // includes ids of comments within `decl`.  The returned ids are ordered by
  // their source order.
  virtual std::vector<ItemId> GetItemIdsInSourceOrder(clang::Decl* decl) = 0;

  // Mangles the name of a named decl.
  virtual std::string GetMangledName(
      const clang::NamedDecl* named_decl) const = 0;

  // Gets the path of an unsupported item by mangling its name and importing
  // its enclosing item. Returns `std::nullopt` if the enclosing item cannot be
  // imported.
  virtual std::optional<UnsupportedItem::Path>
  GetUnsupportedItemPathForTemplateDecl(
      clang::RedeclarableTemplateDecl* template_decl) = 0;

  // Returns the label of the target that contains a decl.
  virtual BazelLabel GetOwningTarget(const clang::Decl* decl) const = 0;

  // Checks if the given decl belongs to the current target. Does not look into
  // other redeclarations of the decl.
  virtual bool IsFromCurrentTarget(const clang::Decl* decl) const = 0;

  // Gets an IR UnqualifiedIdentifier for the named decl.
  //
  // If the decl's name is an identifier, this returns that identifier as-is.
  //
  // If the decl is a special member function or operator overload, this returns
  // a SpecialName.
  //
  // If the name can't be translated (or is empty), this returns an error.
  virtual absl::StatusOr<TranslatedUnqualifiedIdentifier> GetTranslatedName(
      const clang::NamedDecl* named_decl) const = 0;

  // GetTranslatedName, but only for identifier names. This is the common case.
  // If the name can't be translated (or is empty), this returns an error.
  virtual absl::StatusOr<TranslatedIdentifier> GetTranslatedIdentifier(
      const clang::NamedDecl* named_decl) const = 0;

  // Gets the doc comment of the declaration.
  virtual std::optional<std::string> GetComment(
      const clang::Decl* decl) const = 0;

  // Converts a Clang source location to IR.
  virtual std::string ConvertSourceLocation(
      clang::SourceLocation loc) const = 0;

  // Converts the Clang type `qual_type` into an equivalent `CcType`.
  // Lifetimes for the type can optionally be specified using `lifetimes` (pass
  // null otherwise).
  // If `qual_type` is a pointer type, `nullable` specifies whether the
  // pointer can be null.
  // TODO(b/209390498): Currently, we're able to specify nullability only for
  // top-level pointers. Extend this so that we can specify nullability for
  // all pointers contained in `qual_type`, in the same way that `lifetimes`
  // specifies lifetimes for all these pointers. Once this is done, make sure
  // that all callers pass in the appropriate information, derived from
  // nullability annotations. Today, the only caller that passes in
  // `nullable=false` is the code that handles the `this` parameter type for
  // methods, which is always a pointer that cannot be null.
  virtual absl::StatusOr<CcType> ConvertQualType(
      clang::QualType qual_type,
      const clang::tidy::lifetimes::ValueLifetimes* lifetimes,
      bool nullable = true) = 0;

  // Marks `decl` as successfully imported.  Other pieces of code can check
  // HasBeenAlreadySuccessfullyImported to avoid introducing dangling ItemIds
  // that refer to an unimportable `decl`.
  virtual void MarkAsSuccessfullyImported(const clang::NamedDecl* decl) = 0;

  // Returns whether the `decl` has been already successfully imported (maybe
  // partially - e.g. CXXRecordDeclImporter::Import marks the import as success
  // before importing the fields, because the latter cannot fail).  See also
  // MarkAsSuccessfullyImported.
  virtual bool HasBeenAlreadySuccessfullyImported(
      const clang::NamedDecl* decl) const = 0;

  // Returns whether the `decl` will be successfully imported. If it hasn't been
  // imported yet, attempts to import it now, calling
  // MarkAsSuccessfullyImported.
  virtual bool EnsureSuccessfullyImported(clang::NamedDecl* decl) = 0;

  Invocation& invocation_;
  clang::ASTContext& ctx_;
  clang::Sema& sema_;
};

// Interface for components that can import decls of a certain category.
class DeclImporter {
 public:
  explicit DeclImporter(ImportContext& ictx) : ictx_(ictx) {}
  virtual ~DeclImporter() = default;

  // Returns an IR item for a decl, or `std::nullopt` if it could not be
  // imported.
  // If it can't be imported, other DeclImporters may be attempted.
  // To indicate that an item can't be imported, and no other importers should
  // be attempted, return UnsupportedItem.
  virtual std::optional<IR::Item> ImportDecl(clang::Decl*) = 0;

 protected:
  ImportContext& ictx_;
};

// Common implementation for defining `DeclImporter`s that determine their
// applicability by the dynamic type of the decl.
template <typename D>
class DeclImporterBase : public DeclImporter {
 public:
  explicit DeclImporterBase(ImportContext& context) : DeclImporter(context) {}

 protected:
  std::optional<IR::Item> ImportDecl(clang::Decl* decl) override {
    auto* typed_decl = clang::dyn_cast<D>(decl);
    if (typed_decl == nullptr) return std::nullopt;
    return Import(typed_decl);
  }
  virtual std::optional<IR::Item> Import(D*) = 0;
};

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_DECL_IMPORTER_H_
