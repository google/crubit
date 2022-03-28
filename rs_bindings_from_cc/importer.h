// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_IMPORTER_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_IMPORTER_H_

#include <memory>
#include <optional>
#include <set>
#include <string>
#include <utility>
#include <variant>
#include <vector>

#include "third_party/absl/container/flat_hash_map.h"
#include "third_party/absl/container/flat_hash_set.h"
#include "third_party/absl/status/statusor.h"
#include "third_party/absl/types/span.h"
#include "lifetime_annotations/lifetime_annotations.h"
#include "lifetime_annotations/type_lifetimes.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/ir.h"
#include "rs_bindings_from_cc/util/check.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/ASTContext.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Decl.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Mangle.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/RawCommentList.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Type.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Basic/SourceLocation.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Basic/Specifiers.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Sema/Sema.h"

namespace rs_bindings_from_cc {

// Iterates over the AST created from the invocation's entry headers and
// creates an intermediate representation of the import (`IR`) into the
// invocation object.
class Importer {
 public:
  // Top-level parameters as well as return value of an importer invocation.
  class Invocation {
   public:
    Invocation(BazelLabel target, absl::Span<const HeaderName> entry_headers,
               const absl::flat_hash_map<const HeaderName, const BazelLabel>&
                   header_targets)
        : target_(target),
          entry_headers_(entry_headers),
          lifetime_context_(
              std::make_shared<devtools_rust::LifetimeAnnotationContext>()),
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

    const std::shared_ptr<devtools_rust::LifetimeAnnotationContext>
        lifetime_context_;

    // The main output of the import process
    IR ir_;

   private:
    const absl::flat_hash_map<const HeaderName, const BazelLabel>&
        header_targets_;
  };

  explicit Importer(Invocation& invocation, clang::ASTContext& ctx,
                    clang::Sema& sema)
      : invocation_(invocation),
        ctx_(ctx),
        sema_(sema),
        mangler_(CRUBIT_DIE_IF_NULL(ctx_.createMangleContext())) {}

  // Import all visible declarations from a translation unit.
  void Import(clang::TranslationUnitDecl* decl);

 private:
  // Imports all decls contained in a `DeclContext`.
  void ImportDeclsFromDeclContext(const clang::DeclContext* decl_context);

  // Imports a decl, but only if it hasn't been already imported earlier.
  void ImportDeclIfNeeded(clang::Decl* decl);

  // Imports a decl and creates an IR item (or error messages).
  // Does not use or update the cache.
  std::optional<IR::Item> ImportDecl(clang::Decl* decl);

  // These functions import specific `Decl` subtypes. They use `LookupDecl` to
  // lookup dependencies. They don't use or update the cache themselves.
  std::optional<IR::Item> ImportFunction(clang::FunctionDecl* function_decl);
  std::optional<IR::Item> ImportRecord(clang::CXXRecordDecl* record_decl);
  std::optional<IR::Item> ImportTypedefName(
      clang::TypedefNameDecl* typedef_name_decl);
  std::optional<IR::Item> ImportEnum(clang::EnumDecl* enum_decl);

  IR::Item ImportUnsupportedItem(const clang::Decl* decl, std::string error);
  IR::Item ImportUnsupportedItem(const clang::Decl* decl,
                                 std::set<std::string> errors);

  absl::StatusOr<std::vector<Field>> ImportFields(
      clang::CXXRecordDecl* record_decl);
  std::vector<clang::RawComment*> ImportFreeComments();

  std::string GetMangledName(const clang::NamedDecl* named_decl) const;
  BazelLabel GetOwningTarget(const clang::Decl* decl) const;

  // Checks if the given decl belongs to the current target. Does not look into
  // other redeclarations of the decl.
  bool IsFromCurrentTarget(const clang::Decl* decl) const;

  // Gets an IR UnqualifiedIdentifier for the named decl.
  //
  // If the decl's name is an identifier, this returns that identifier as-is.
  //
  // If the decl is a special member function or operator overload, this returns
  // a SpecialName.
  //
  // If the translated name is not yet implemented, this returns null.
  std::optional<UnqualifiedIdentifier> GetTranslatedName(
      const clang::NamedDecl* named_decl) const;

  // GetTranslatedName, but only for identifier names. This is the common case.
  std::optional<Identifier> GetTranslatedIdentifier(
      const clang::NamedDecl* named_decl) const {
    if (std::optional<UnqualifiedIdentifier> name =
            GetTranslatedName(named_decl)) {
      return std::move(*std::get_if<Identifier>(&*name));
    }
    return std::nullopt;
  }

  // Gets the doc comment of the declaration.
  llvm::Optional<std::string> GetComment(const clang::Decl* decl) const;

  // Converts the Clang type `qual_type` into an equivalent `MappedType`.
  // Lifetimes for the type can optionally be specified using `lifetimes`.
  // If `qual_type` is a pointer type, `nullable` specifies whether the pointer
  // can be null.
  // TODO(b/209390498): Currently, we're able to specify nullability only for
  // top-level pointers. Extend this so that we can specify nullability for all
  // pointers contained in `qual_type`, in the same way that `lifetimes`
  // specifies lifetimes for all these pointers. Once this is done, make sure
  // that all callers pass in the appropriate information, derived from
  // nullability annotations.
  absl::StatusOr<MappedType> ConvertQualType(
      clang::QualType qual_type,
      std::optional<devtools_rust::ValueLifetimes>& lifetimes,
      bool nullable = true) const;
  absl::StatusOr<MappedType> ConvertType(
      const clang::Type* type,
      std::optional<devtools_rust::ValueLifetimes>& lifetimes,
      bool nullable) const;
  absl::StatusOr<MappedType> ConvertTypeDecl(const clang::TypeDecl* decl) const;

  SourceLoc ConvertSourceLocation(clang::SourceLocation loc) const;

  std::vector<BaseClass> GetUnambiguousPublicBases(
      const clang::CXXRecordDecl& record_decl) const;

  Invocation& invocation_;

  clang::ASTContext& ctx_;
  clang::Sema& sema_;

  std::unique_ptr<clang::MangleContext> mangler_;
  absl::flat_hash_map<const clang::Decl*, std::optional<IR::Item>>
      import_cache_;
  absl::flat_hash_set<const clang::TypeDecl*> known_type_decls_;
};  // class Importer

}  // namespace rs_bindings_from_cc

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IMPORTER_H_
