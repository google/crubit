// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importers/class_template.h"

#include <optional>

#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/DeclTemplate.h"

namespace crubit {

std::optional<IR::Item> ClassTemplateDeclImporter::Import(
    clang::ClassTemplateDecl* class_template_decl) {
  return ictx_.ImportUnsupportedItem(
      *class_template_decl,
      ictx_.GetUnsupportedItemPathForTemplateDecl(class_template_decl),
      FormattedError::Static("Class templates are not supported yet"));
}

}  // namespace crubit
