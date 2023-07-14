// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_CXX_RECORD_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_CXX_RECORD_H_

#include <optional>
#include <vector>

#include "rs_bindings_from_cc/decl_importer.h"
#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclCXX.h"

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
};

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_CXX_RECORD_H_
