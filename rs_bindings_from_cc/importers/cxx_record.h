// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_CXX_RECORD_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_CXX_RECORD_H_

#include <optional>
#include <vector>

#include "absl/status/statusor.h"
#include "rs_bindings_from_cc/decl_importer.h"
#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/DeclTemplate.h"

namespace crubit {

// A `DeclImporter` for `CXXRecordDecl`s.
class CXXRecordDeclImporter : public DeclImporterBase<clang::CXXRecordDecl> {
 public:
  explicit CXXRecordDeclImporter(ImportContext& context)
      : DeclImporterBase(context) {}
  std::optional<IR::Item> Import(clang::CXXRecordDecl*) override;

 private:
  std::vector<Field> ImportFields(clang::CXXRecordDecl*);
  std::vector<BaseClass> GetUnambiguousPublicBases(
      const clang::CXXRecordDecl& record_decl) const;
  std::optional<Identifier> GetTranslatedFieldName(
      const clang::FieldDecl* field);

  // Returns the bridge type of the given `decl` if it is a builtin bridge type
  // (e.g., `std::optional`). Otherwise, returns `std::nullopt`.
  absl::StatusOr<std::optional<BridgeType>> GetBuiltinBridgeType(
      const clang::ClassTemplateSpecializationDecl* decl);
};

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_CXX_RECORD_H_
