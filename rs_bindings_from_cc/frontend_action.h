// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_FRONTEND_ACTION_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_FRONTEND_ACTION_H_

#include <memory>

#include "base/logging.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/ir.h"
#include "third_party/absl/container/flat_hash_map.h"
#include "third_party/absl/types/span.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/ASTConsumer.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Frontend/CompilerInstance.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Frontend/FrontendAction.h"

namespace rs_bindings_from_cc {

// Creates an `ASTConsumer` that generates the intermediate representation
// (`IR`) into the `ir` parameter.
class FrontendAction : public clang::ASTFrontendAction {
 public:
  explicit FrontendAction(
      Label current_target, absl::Span<const HeaderName> public_header_names,
      const absl::flat_hash_map<const HeaderName, const Label>*
          headers_to_targets,
      IR* ir)
      : current_target_(current_target),
        public_header_names_(public_header_names),
        headers_to_targets_(*ABSL_DIE_IF_NULL(headers_to_targets)),
        ir_(*ABSL_DIE_IF_NULL(ir)) {}

  std::unique_ptr<clang::ASTConsumer> CreateASTConsumer(
      clang::CompilerInstance& instance, llvm::StringRef) override;

 private:
  Label current_target_;
  absl::Span<const HeaderName> public_header_names_;
  const absl::flat_hash_map<const HeaderName, const Label>& headers_to_targets_;
  IR& ir_;
};

}  // namespace rs_bindings_from_cc

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_FRONTEND_ACTION_H_
