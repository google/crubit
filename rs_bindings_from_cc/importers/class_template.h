// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_CLASS_TEMPLATE_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_CLASS_TEMPLATE_H_

#include "rs_bindings_from_cc/decl_importer.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/DeclTemplate.h"

namespace crubit {

// A `DeclImporter` for `ClassTemplateDecl`s.
class ClassTemplateDeclImporter
    : public DeclImporterBase<clang::ClassTemplateDecl> {
 public:
  ClassTemplateDeclImporter(ImportContext& context)
      : DeclImporterBase(context) {}
  std::optional<IR::Item> Import(clang::ClassTemplateDecl*);
};

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_CLASS_TEMPLATE_H_
