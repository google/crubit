// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_TYPE_DECL_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_TYPE_DECL_H_

#include "rs_bindings_from_cc/decl_importer.h"
namespace crubit {

// Imports type decls which have an overridden type.
class TypeMapOverrideImporter final : public DeclImporterBase<clang::TypeDecl> {
 public:
  TypeMapOverrideImporter(ImportContext& context) : DeclImporterBase(context) {}
  std::optional<IR::Item> Import(clang::TypeDecl*);
};

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_TYPE_DECL_H_
