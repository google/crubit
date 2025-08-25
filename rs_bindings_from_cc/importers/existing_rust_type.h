// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_TYPE_DECL_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_TYPE_DECL_H_

#include <optional>

#include "rs_bindings_from_cc/decl_importer.h"
#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/Decl.h"
namespace crubit {

// Imports type decls which have an overridden type.
class ExistingRustTypeImporter final
    : public DeclImporterBase<clang::TypeDecl> {
 public:
  explicit ExistingRustTypeImporter(ImportContext& context)
      : DeclImporterBase(context) {}
  std::optional<IR::Item> Import(clang::TypeDecl*) override;
};

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_TYPE_DECL_H_
