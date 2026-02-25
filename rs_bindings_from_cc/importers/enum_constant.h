// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_ENUM_CONSTANT_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_ENUM_CONSTANT_H_

#include <optional>

#include "rs_bindings_from_cc/decl_importer.h"
#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/Decl.h"

namespace crubit {

// A `DeclImporter` for `EnumConstantDecl`s.
//
// This importer manages imports of constants defined inside an anonymous enum.
class EnumConstantDeclImporter
    : public DeclImporterBase<clang::EnumConstantDecl> {
 public:
  explicit EnumConstantDeclImporter(ImportContext& context)
      : DeclImporterBase(context) {}
  std::optional<IR::Item> Import(clang::EnumConstantDecl*) override;
};

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_ENUM_CONSTANT_H_
