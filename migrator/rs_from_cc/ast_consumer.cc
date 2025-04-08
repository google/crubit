// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "migrator/rs_from_cc/ast_consumer.h"

#include "absl/log/check.h"
#include "migrator/rs_from_cc/converter.h"
#include "clang/include/clang/AST/ASTContext.h"
#include "clang/include/clang/Frontend/CompilerInstance.h"

namespace crubit_rs_from_cc {

void AstConsumer::HandleTranslationUnit(clang::ASTContext& ast_context) {
  if (ast_context.getDiagnostics().hasErrorOccurred()) {
    // We do not need to process partially incorrect headers, we assume all
    // input is valid C++. If there is an error Clang already printed it to
    // stderr; the user will be informed about the cause of the failure.
    // There is nothing more for us to do here.
    return;
  }
  CHECK(instance_.hasSema());
  Converter converter(invocation_, ast_context);
  converter.Convert(ast_context.getTranslationUnitDecl());
}

}  // namespace crubit_rs_from_cc
