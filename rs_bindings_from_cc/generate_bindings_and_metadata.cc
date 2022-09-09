// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/generate_bindings_and_metadata.h"

#include <string>

#include "absl/strings/string_view.h"
#include "common/status_macros.h"
#include "rs_bindings_from_cc/collect_instantiations.h"
#include "rs_bindings_from_cc/collect_namespaces.h"
#include "rs_bindings_from_cc/ir.h"
#include "rs_bindings_from_cc/ir_from_cc.h"
#include "rs_bindings_from_cc/src_code_gen.h"

namespace crubit {

absl::StatusOr<BindingsAndMetadata> GenerateBindingsAndMetadata(
    Cmdline& cmdline, std::vector<std::string> clang_args,
    absl::flat_hash_map<const HeaderName, const std::string>
        virtual_headers_contents) {
  std::vector<absl::string_view> clang_args_view;
  clang_args_view.insert(clang_args_view.end(), clang_args.begin(),
                         clang_args.end());

  CRUBIT_ASSIGN_OR_RETURN(std::vector<std::string> requested_instantiations,
                          CollectInstantiations(cmdline.rust_sources()));

  CRUBIT_ASSIGN_OR_RETURN(
      IR ir, IrFromCc(
                 /* extra_source_code= */ "", cmdline.current_target(),
                 cmdline.public_headers(), virtual_headers_contents,
                 cmdline.headers_to_targets(), clang_args_view,
                 requested_instantiations));

  CRUBIT_ASSIGN_OR_RETURN(Bindings bindings,
                          GenerateBindings(ir, cmdline.crubit_support_path(),
                                           cmdline.rustfmt_exe_path(),
                                           cmdline.rustfmt_config_path()));

  absl::flat_hash_map<std::string, std::string> instantiations;
  for (const auto* record : ir.get_items_if<Record>()) {
    if (record->is_explicit_class_template_instantiation_definition) {
      instantiations.insert({record->cc_name, record->rs_name});
    }
  }

  auto top_level_namespaces = crubit::CollectNamespaces(ir);

  return BindingsAndMetadata{
      .ir = ir,
      .rs_api = bindings.rs_api,
      .rs_api_impl = bindings.rs_api_impl,
      .namespaces = std::move(top_level_namespaces),
      .instantiations = std::move(instantiations),
  };
}

}  // namespace crubit
