// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_TYPEDEF_NAME_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_TYPEDEF_NAME_H_

#include "rs_bindings_from_cc/decl_importer.h"
#include "clang/AST/Decl.h"

namespace crubit {

// A `DeclImporter` for `TypedefNameDecl`s (including `TypeAliasDecl` and
// `TypedefDecl`).
class TypedefNameDeclImporter
    : public DeclImporterBase<clang::TypedefNameDecl> {
 public:
  TypedefNameDeclImporter(ImportContext& context) : DeclImporterBase(context) {}
  std::optional<IR::Item> Import(clang::TypedefNameDecl*);
};

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_TYPEDEF_NAME_H_
