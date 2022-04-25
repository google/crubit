// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_FUNCTION_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_FUNCTION_H_

#include "rs_bindings_from_cc/decl_importer.h"

namespace crubit {

// A `DeclImporter` for `FunctionDecl`s.
class FunctionDeclImporter : public DeclImporterBase<clang::FunctionDecl> {
 public:
  FunctionDeclImporter(ImportContext& context) : DeclImporterBase(context) {}
  std::optional<IR::Item> Import(clang::FunctionDecl*);
};

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_FUNCTION_H_
