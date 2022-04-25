// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_CXX_RECORD_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_CXX_RECORD_H_

#include "rs_bindings_from_cc/decl_importer.h"
namespace crubit {

// A `DeclImporter` for `CXXRecordDecl`s.
class CXXRecordDeclImporter : public DeclImporterBase<clang::CXXRecordDecl> {
 public:
  CXXRecordDeclImporter(ImportContext& context) : DeclImporterBase(context) {}
  std::optional<IR::Item> Import(clang::CXXRecordDecl*);

 private:
  absl::StatusOr<std::vector<Field>> ImportFields(clang::CXXRecordDecl*);
  std::vector<BaseClass> GetUnambiguousPublicBases(
      const clang::CXXRecordDecl& record_decl) const;
};

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_CXX_RECORD_H_
