// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_DECL_IMPORTER_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_DECL_IMPORTER_H_

#include "absl/container/flat_hash_map.h"
#include "absl/status/statusor.h"
#include "lifetime_annotations/lifetime_annotations.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/ir.h"

namespace crubit {

// Top-level parameters as well as return value of an importer invocation.
class Invocation {
 public:
  Invocation(BazelLabel target, absl::Span<const HeaderName> entry_headers,
             const absl::flat_hash_map<const HeaderName, const BazelLabel>&
                 header_targets)
      : target_(target),
        entry_headers_(entry_headers),
        lifetime_context_(std::make_shared<
                          clang::tidy::lifetimes::LifetimeAnnotationContext>()),
        header_targets_(header_targets) {
    // Caller should verify that the inputs are non-empty.
    CRUBIT_CHECK(!entry_headers_.empty());
    CRUBIT_CHECK(!header_targets_.empty());

    ir_.used_headers.insert(ir_.used_headers.end(), entry_headers_.begin(),
                            entry_headers.end());
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

  // The headers from which the import starts (a collection of
  // paths in the format suitable for a google3-relative quote include).
  const absl::Span<const HeaderName> entry_headers_;

  const std::shared_ptr<clang::tidy::lifetimes::LifetimeAnnotationContext>
      lifetime_context_;

  // The main output of the import process
  IR ir_;

 private:
  const absl::flat_hash_map<const HeaderName, const BazelLabel>&
      header_targets_;
};

// The currently known canonical type decls that we know how to map into
// Rust.
class TypeMapper {
 public:
  TypeMapper(const clang::ASTContext* ctx) : ctx_(ctx) {}

  TypeMapper(const TypeMapper& other) = default;
  TypeMapper& operator=(const TypeMapper& other) = default;

  // Converts the Clang type `qual_type` into an equivalent `MappedType`.
  // Lifetimes for the type can optionally be specified using `lifetimes`.
  // If `qual_type` is a pointer type, `nullable` specifies whether the
  // pointer can be null.
  // TODO(b/209390498): Currently, we're able to specify nullability only for
  // top-level pointers. Extend this so that we can specify nullability for
  // all pointers contained in `qual_type`, in the same way that `lifetimes`
  // specifies lifetimes for all these pointers. Once this is done, make sure
  // that all callers pass in the appropriate information, derived from
  // nullability annotations.
  absl::StatusOr<MappedType> ConvertQualType(
      clang::QualType qual_type,
      std::optional<clang::tidy::lifetimes::ValueLifetimes>& lifetimes,
      bool nullable = true) const;
  absl::StatusOr<MappedType> ConvertType(
      const clang::Type* type,
      std::optional<clang::tidy::lifetimes::ValueLifetimes>& lifetimes,
      bool nullable) const;
  absl::StatusOr<MappedType> ConvertTypeDecl(const clang::TypeDecl* decl) const;
  std::optional<absl::string_view> MapKnownCcTypeToRsType(
      absl::string_view cc_type) const;

  bool Contains(const clang::TypeDecl* decl) const {
    return known_type_decls_.contains(
        clang::cast<clang::TypeDecl>(decl->getCanonicalDecl()));
  }

  void Insert(const clang::TypeDecl* decl) {
    known_type_decls_.insert(
        clang::cast<clang::TypeDecl>(decl->getCanonicalDecl()));
  }

 private:
  const clang::ASTContext* ctx_;
  absl::flat_hash_set<const clang::TypeDecl*> known_type_decls_;
};

// Explicitly defined interface that defines how `DeclImporter`s are allowed to
// interface with the global state of the importer.
class ImportContext {
 public:
  ImportContext(Invocation& invocation, clang::ASTContext& ctx,
                clang::Sema& sema)
      : invocation_(invocation), ctx_(ctx), sema_(sema), type_mapper_(&ctx){};
  virtual ~ImportContext(){};

  // Imports all decls contained in a `DeclContext`.
  virtual void ImportDeclsFromDeclContext(
      const clang::DeclContext* decl_context) = 0;

  // Imports an unsupported item with a single error message.
  virtual IR::Item ImportUnsupportedItem(const clang::Decl* decl,
                                         std::string error) = 0;

  // Imports an unsupported item with multiple error messages.
  virtual IR::Item ImportUnsupportedItem(const clang::Decl* decl,
                                         std::set<std::string> errors) = 0;

  // Returns the item ids of the children and comments of the given decl in
  // source order. This method assumes that the children decls have already been
  // imported.
  virtual std::vector<ItemId> GetItemIdsInSourceOrder(clang::Decl* decl) = 0;

  // Mangles the name of a named decl.
  virtual std::string GetMangledName(
      const clang::NamedDecl* named_decl) const = 0;

  // Returs the label of the target that contains a decl.
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
  // If the translated name is not yet implemented, this returns null.
  virtual std::optional<UnqualifiedIdentifier> GetTranslatedName(
      const clang::NamedDecl* named_decl) const = 0;

  // GetTranslatedName, but only for identifier names. This is the common case.
  virtual std::optional<Identifier> GetTranslatedIdentifier(
      const clang::NamedDecl* named_decl) const = 0;

  // Gets the doc comment of the declaration.
  virtual llvm::Optional<std::string> GetComment(
      const clang::Decl* decl) const = 0;

  // Converts a Clang source location to IR.
  virtual SourceLoc ConvertSourceLocation(clang::SourceLocation loc) const = 0;

  Invocation& invocation_;
  clang::ASTContext& ctx_;
  clang::Sema& sema_;
  TypeMapper type_mapper_;
};

// Interface for components that can import decls of a certain category.
class DeclImporter {
 public:
  DeclImporter(ImportContext& ictx) : ictx_(ictx){};
  virtual ~DeclImporter(){};

  // Determines whether this importer is autoritative for a decl. This does not
  // imply that the import will be succesful.
  virtual bool CanImport(clang::Decl*) = 0;

  // Returns an IR item for a decl, or `std::nullopt` if importing failed.
  // This member function may only be called after `CanImport` returned `true`.
  virtual std::optional<IR::Item> ImportDecl(clang::Decl*) = 0;

 protected:
  ImportContext& ictx_;
};

// Common implementation for defining `DeclImporter`s that determine their
// applicability by the dynamic type of the decl.
template <typename D>
class DeclImporterBase : public DeclImporter {
 public:
  DeclImporterBase(ImportContext& context) : DeclImporter(context) {}

 protected:
  bool CanImport(clang::Decl* decl) { return clang::isa<D>(decl); }
  std::optional<IR::Item> ImportDecl(clang::Decl* decl) {
    return Import(clang::cast<D>(decl));
  }
  virtual std::optional<IR::Item> Import(D*) = 0;
};

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_DECL_IMPORTER_H_
