// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_ENUM_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_ENUM_H_

#include <optional>

#include "rs_bindings_from_cc/decl_importer.h"
#include "clang/AST/Decl.h"

namespace crubit {

// A `DeclImporter` for `EnumDecl`s.
class EnumDeclImporter : public DeclImporterBase<clang::EnumDecl> {
 public:
  explicit EnumDeclImporter(ImportContext& context)
      : DeclImporterBase(context) {}
  std::optional<IR::Item> Import(clang::EnumDecl*) override;
};

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_ENUM_H_
