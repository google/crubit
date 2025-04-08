// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_TYPEDEF_NAME_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_TYPEDEF_NAME_H_

#include <optional>

#include "rs_bindings_from_cc/decl_importer.h"
#include "rs_bindings_from_cc/ir.h"
#include "clang/include/clang/AST/Decl.h"

namespace crubit {

// A `DeclImporter` for `TypedefNameDecl`s (e.g. `using x = y`, or
// `typedef y x`) and `UsingShadowDecl`s (`using x::y`).
class TypeAliasImporter : public DeclImporterBase<clang::NamedDecl> {
 public:
  explicit TypeAliasImporter(ImportContext& context)
      : DeclImporterBase(context) {}
  std::optional<IR::Item> Import(clang::NamedDecl* decl) override;
};

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_TYPEDEF_NAME_H_
