// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/generate_bindings_and_metadata.h"

#include <optional>
#include <string>
#include <utility>

#include "absl/container/flat_hash_map.h"
#include "absl/container/flat_hash_set.h"
#include "absl/strings/string_view.h"
#include "common/status_macros.h"
#include "rs_bindings_from_cc/collect_instantiations.h"
#include "rs_bindings_from_cc/collect_namespaces.h"
#include "rs_bindings_from_cc/ir.h"
#include "rs_bindings_from_cc/ir_from_cc.h"
#include "rs_bindings_from_cc/src_code_gen.h"

namespace crubit {

std::optional<const Namespace*> FindNamespace(const IR& ir,
                                              absl::string_view name) {
  for (const auto* ns : ir.get_items_if<Namespace>()) {
    if (ns->name.Ident() == kInstantiationsNamespaceName) {
      return ns;
    }
  }
  return std::nullopt;
}

std::vector<const Record*> FindInstantiationsInNamespace(const IR& ir,
                                                         ItemId namespace_id) {
  absl::flat_hash_set<ItemId> record_ids;
  for (const auto* type_alias : ir.get_items_if<TypeAlias>()) {
    if (type_alias->enclosing_namespace_id.has_value() &&
        type_alias->enclosing_namespace_id == namespace_id) {
      const MappedType* mapped_type = &type_alias->underlying_type;
      CHECK(mapped_type->cc_type.decl_id.has_value());
      CHECK(mapped_type->rs_type.decl_id.has_value());
      CHECK(mapped_type->cc_type.decl_id.value() ==
            mapped_type->rs_type.decl_id.value());
      record_ids.insert(mapped_type->rs_type.decl_id.value());
    }
  }

  std::vector<const Record*> result;
  for (const auto* record : ir.get_items_if<Record>()) {
    if (record_ids.find(record->id) != record_ids.end()) {
      result.push_back(record);
    }
  }
  return result;
}

absl::StatusOr<BindingsAndMetadata> GenerateBindingsAndMetadata(
    Cmdline& cmdline, std::vector<std::string> clang_args,
    absl::flat_hash_map<const HeaderName, const std::string>
        virtual_headers_contents_for_testing) {
  std::vector<absl::string_view> clang_args_view;
  clang_args_view.insert(clang_args_view.end(), clang_args.begin(),
                         clang_args.end());

  CRUBIT_ASSIGN_OR_RETURN(
      std::vector<std::string> requested_instantiations,
      CollectInstantiations(cmdline.srcs_to_scan_for_instantiations()));

  CRUBIT_ASSIGN_OR_RETURN(
      IR ir,
      IrFromCc(
          /* extra_source_code_for_testing= */ "", cmdline.current_target(),
          cmdline.public_headers(), virtual_headers_contents_for_testing,
          cmdline.headers_to_targets(), cmdline.extra_rs_srcs(),
          clang_args_view, requested_instantiations,
          cmdline.target_to_features()));

  if (!cmdline.instantiations_out().empty()) {
    ir.crate_root_path = "__cc_template_instantiations_rs_api";
  }

  bool generate_error_report = !cmdline.error_report_out().empty();
  CRUBIT_ASSIGN_OR_RETURN(
      Bindings bindings,
      GenerateBindings(ir, cmdline.crubit_support_path(),
                       cmdline.clang_format_exe_path(),
                       cmdline.rustfmt_exe_path(),
                       cmdline.rustfmt_config_path(), generate_error_report));

  absl::flat_hash_map<std::string, std::string> instantiations;
  std::optional<const Namespace*> ns =
      FindNamespace(ir, kInstantiationsNamespaceName);
  if (ns.has_value()) {
    std::vector<const Record*> records =
        FindInstantiationsInNamespace(ir, ns.value()->id);
    for (const auto* record : records) {
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
      .error_report = bindings.error_report,
  };
}

}  // namespace crubit
