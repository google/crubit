// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/generate_bindings_and_metadata.h"

#include "common/status_macros.h"
#include "rs_bindings_from_cc/ir_from_cc.h"
#include "rs_bindings_from_cc/src_code_gen.h"

namespace crubit {

absl::StatusOr<BindingsAndMetadata> GenerateBindingsAndMetadata(
    Cmdline& cmdline, std::vector<std::string> clang_args) {
  std::vector<absl::string_view> clang_args_view;
  clang_args_view.insert(clang_args_view.end(), clang_args.begin(),
                         clang_args.end());

  CRUBIT_ASSIGN_OR_RETURN(
      IR ir, crubit::IrFromCc(
                 /* extra_source_code= */ "", cmdline.current_target(),
                 cmdline.public_headers(),
                 /* virtual_headers_contents= */ {},
                 cmdline.headers_to_targets(), clang_args_view));

  crubit::Bindings bindings = crubit::GenerateBindings(
      ir, cmdline.crubit_support_path(), cmdline.rustfmt_config_path());

  return BindingsAndMetadata{
      .ir = ir,
      .rs_api = bindings.rs_api,
      .rs_api_impl = bindings.rs_api_impl,
  };
}

}  // namespace crubit
