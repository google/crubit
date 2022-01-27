// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ast_consumer.h"

#include "base/logging.h"
#include "rs_bindings_from_cc/importer.h"
#include "rs_bindings_from_cc/ir.h"
#include "third_party/absl/container/flat_hash_map.h"
#include "third_party/absl/types/span.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/ASTContext.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Decl.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Frontend/CompilerInstance.h"

namespace rs_bindings_from_cc {

void AstConsumer::HandleTranslationUnit(clang::ASTContext& ast_context) {
  if (ast_context.getDiagnostics().hasErrorOccurred()) {
    // We do not need to process partially incorrect headers, we assume all
    // input is valid C++. If there is an error Clang already printed it to
    // stderr; the user will be informed about the cause of the failure.
    // There is nothing more for us to do here.
    return;
  }
  CHECK(instance_.hasSema());
  CHECK(!public_header_names_.empty());
  CHECK(!headers_to_targets_.empty());
  Importer importer(instance_.getSema(), current_target_, public_header_names_,
                    &headers_to_targets_, &ir_, *lifetime_context_);
  importer.Import(ast_context.getTranslationUnitDecl());
}

}  // namespace rs_bindings_from_cc
