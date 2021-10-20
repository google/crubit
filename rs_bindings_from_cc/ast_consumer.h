// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_AST_CONSUMER_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_AST_CONSUMER_H_

#include "rs_bindings_from_cc/ir.h"
#include "third_party/absl/strings/string_view.h"
#include "third_party/absl/types/span.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/ASTConsumer.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/ASTContext.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Frontend/CompilerInstance.h"

namespace rs_bindings_from_cc {

// Consumes the Clang AST created from `public_header_names` (a collection of
// paths in the format suitable for a google3-relative quote include) and
// generates the intermediate representation (`IR`).
class AstConsumer : public clang::ASTConsumer {
 public:
  explicit AstConsumer(clang::CompilerInstance& instance,
                       absl::Span<const absl::string_view> public_header_names,
                       IR& ir)
      : instance_(instance),
        public_header_names_(public_header_names),
        ir_(ir) {}

  void HandleTranslationUnit(clang::ASTContext& context) override;

 private:
  clang::CompilerInstance& instance_;
  absl::Span<const absl::string_view> public_header_names_;
  IR& ir_;
};  // class AstConsumer

}  // namespace rs_bindings_from_cc

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_AST_CONSUMER_H_
