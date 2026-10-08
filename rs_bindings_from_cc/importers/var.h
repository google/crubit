// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_VAR_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_VAR_H_

#include <memory>
#include <optional>
#include <string>

#include "rs_bindings_from_cc/decl_importer.h"
#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/Decl.h"

namespace crubit {

// A `DeclImporter` for `VarDecl`s.
class VarDeclImporter : public DeclImporterBase<clang::VarDecl> {
 public:
  explicit VarDeclImporter(ImportContext& context)
      : DeclImporterBase(context) {}
  std::unique_ptr<ir_proto::Item> Import(clang::VarDecl*) override;

 private:
  // Imports a variable with a constant initializer as an `ir_proto::Constant`.
  //
  // Supported are boolean and integer scalars.
  std::unique_ptr<ir_proto::Item> ImportConstant(
      clang::VarDecl* var_decl, TranslatedIdentifier& var_name,
      std::optional<ItemId> enclosing_item_id,
      std::optional<std::string> unknown_attr,
      std::optional<std::string> deprecated);
};

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_VAR_H_
