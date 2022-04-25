// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importers/function_template.h"

namespace crubit {

std::optional<IR::Item> FunctionTemplateDeclImporter::Import(
    clang::FunctionTemplateDecl* function_template_decl) {
  return ictx_.ImportUnsupportedItem(
      function_template_decl, "Function templates are not supported yet");
}

}  // namespace crubit
