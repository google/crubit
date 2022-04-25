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
#include <vector>

#include "rs_bindings_from_cc/decl_importer.h"
#include "rs_bindings_from_cc/importers/class_template.h"
#include "rs_bindings_from_cc/importers/cxx_record.h"
#include "rs_bindings_from_cc/importers/enum.h"
#include "rs_bindings_from_cc/importers/function_template.h"
#include "rs_bindings_from_cc/ir.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/DeclCXX.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Mangle.h"

namespace crubit {

// TODO(forster): Move those implementations into separate files.

// A `DeclImporter` for `FunctionDecl`s.
class FunctionDeclImporter : public DeclImporterBase<clang::FunctionDecl> {
 public:
  FunctionDeclImporter(ImportContext& context) : DeclImporterBase(context){};
  std::optional<IR::Item> Import(clang::FunctionDecl*);
};

// A `DeclImporter` for `NamespaceDecl`s.
class NamespaceDeclImporter : public DeclImporterBase<clang::NamespaceDecl> {
 public:
  NamespaceDeclImporter(ImportContext& context) : DeclImporterBase(context){};
  std::optional<IR::Item> Import(clang::NamespaceDecl*);
};

// A `DeclImporter` for `TypedefNameDecl`s.
class TypedefNameDeclImporter
    : public DeclImporterBase<clang::TypedefNameDecl> {
 public:
  TypedefNameDeclImporter(ImportContext& context) : DeclImporterBase(context){};
  std::optional<IR::Item> Import(clang::TypedefNameDecl*);
};

// Iterates over the AST created from the invocation's entry headers and
// creates an intermediate representation of the import (`IR`) into the
// invocation object.
class Importer : public ImportContext {
 public:
  explicit Importer(Invocation& invocation, clang::ASTContext& ctx,
                    clang::Sema& sema)
      : ImportContext(invocation, ctx, sema),
        mangler_(CRUBIT_DIE_IF_NULL(ctx_.createMangleContext())) {
    decl_importers_.push_back(
        std::make_unique<ClassTemplateDeclImporter>(*this));
    decl_importers_.push_back(std::make_unique<CXXRecordDeclImporter>(*this));
    decl_importers_.push_back(std::make_unique<EnumDeclImporter>(*this));
    decl_importers_.push_back(std::make_unique<FunctionDeclImporter>(*this));
    decl_importers_.push_back(
        std::make_unique<FunctionTemplateDeclImporter>(*this));
    decl_importers_.push_back(std::make_unique<NamespaceDeclImporter>(*this));
    decl_importers_.push_back(std::make_unique<TypedefNameDeclImporter>(*this));
  }

  // Import all visible declarations from a translation unit.
  void Import(clang::TranslationUnitDecl* decl);

 protected:
  // Implementation of `ImportContext`
  void ImportDeclsFromDeclContext(
      const clang::DeclContext* decl_context) override;
  IR::Item ImportUnsupportedItem(const clang::Decl* decl,
                                 std::string error) override;
  IR::Item ImportUnsupportedItem(const clang::Decl* decl,
                                 std::set<std::string> errors) override;
  std::vector<ItemId> GetItemIdsInSourceOrder(clang::Decl* decl) override;
  std::string GetMangledName(const clang::NamedDecl* named_decl) const override;
  BazelLabel GetOwningTarget(const clang::Decl* decl) const override;
  bool IsFromCurrentTarget(const clang::Decl* decl) const override;
  std::optional<UnqualifiedIdentifier> GetTranslatedName(
      const clang::NamedDecl* named_decl) const override;
  std::optional<Identifier> GetTranslatedIdentifier(
      const clang::NamedDecl* named_decl) const override {
    if (std::optional<UnqualifiedIdentifier> name =
            GetTranslatedName(named_decl)) {
      return std::move(*std::get_if<Identifier>(&*name));
    }
    return std::nullopt;
  }
  llvm::Optional<std::string> GetComment(
      const clang::Decl* decl) const override;
  SourceLoc ConvertSourceLocation(clang::SourceLocation loc) const override;

 private:
  // Returns the Item of a Decl, importing it first if necessary.
  std::optional<IR::Item> GetDeclItem(clang::Decl* decl);

  // Imports a decl and creates an IR item (or error messages).
  // Does not use or update the cache.
  std::optional<IR::Item> ImportDecl(clang::Decl* decl);

  // Stores the comments of this target in source order.
  void ImportFreeComments();

  std::vector<std::unique_ptr<DeclImporter>> decl_importers_;
  std::unique_ptr<clang::MangleContext> mangler_;
  absl::flat_hash_map<const clang::Decl*, std::optional<IR::Item>>
      import_cache_;
  std::vector<const clang::RawComment*> comments_;
};  // class Importer

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IMPORTER_H_
