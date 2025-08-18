// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importers/function_template.h"

#include <optional>

#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/DeclTemplate.h"

namespace crubit {

std::optional<IR::Item> FunctionTemplateDeclImporter::Import(
    clang::FunctionTemplateDecl* function_template_decl) {
  if (!ictx_.IsFromCurrentTarget(function_template_decl)) return std::nullopt;
  return ictx_.ImportUnsupportedItem(
      *function_template_decl,
      ictx_.GetUnsupportedItemPathForTemplateDecl(function_template_decl),
      FormattedError::Static("Function templates are not supported yet"));
}

}  // namespace crubit
