// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/generate_bindings_and_metadata.h"

#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <variant>
#include <vector>

#include "absl/base/nullability.h"
#include "absl/container/flat_hash_map.h"
#include "absl/container/flat_hash_set.h"
#include "absl/log/check.h"
#include "absl/status/status.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "common/status_macros.h"
#include "rs_bindings_from_cc/cmdline.h"
#include "rs_bindings_from_cc/collect_instantiations.h"
#include "rs_bindings_from_cc/collect_namespaces.h"
#include "rs_bindings_from_cc/ir.h"
#include "rs_bindings_from_cc/ir_from_cc.h"
#include "rs_bindings_from_cc/src_code_gen.h"
#include "llvm/Support/Regex.h"

namespace crubit {

namespace ir_proto = ::crubit::rs_bindings_from_cc::ir_proto::flat;

const ir_proto::Namespace* absl_nullable FindNamespace(const IR& ir,
                                                       absl::string_view name) {
  for (const auto* ns : get_items_if<ir_proto::Namespace>(ir)) {
    if (ns->cc_name().identifier() == kInstantiationsNamespaceName) {
      return ns;
    }
  }
  return nullptr;
}

std::vector<const ir_proto::Record* absl_nonnull> FindInstantiationsInNamespace(
    const IR& ir, ItemId namespace_id) {
  absl::flat_hash_set<ItemId> record_ids;
  for (const auto* type_alias : get_items_if<ir_proto::TypeAlias>(ir)) {
    if (ItemId(type_alias->enclosing_item_id()) == namespace_id) {
      CHECK(type_alias->underlying_type().has_decl());
      record_ids.insert(ItemId(type_alias->underlying_type().decl()));
    }
  }

  std::vector<const ir_proto::Record* absl_nonnull> result;
  for (const auto* record : get_items_if<ir_proto::Record>(ir)) {
    if (record_ids.find(ItemId(record->id())) != record_ids.end()) {
      result.push_back(record);
    }
  }
  return result;
}

absl::StatusOr<BindingsAndMetadata> GenerateBindingsAndMetadata(
    Cmdline& cmdline, std::vector<std::string> clang_args,
    absl::flat_hash_map<HeaderName, std::string>
        virtual_headers_contents_for_testing) {
  std::vector<absl::string_view> clang_args_view;
  clang_args_view.insert(clang_args_view.end(), clang_args.begin(),
                         clang_args.end());
  const CmdlineArgs& args = cmdline.args();

  CRUBIT_ASSIGN_OR_RETURN(
      std::vector<std::string> requested_instantiations,
      CollectInstantiations(args.srcs_to_scan_for_instantiations));

  std::optional<absl::flat_hash_set<std::string>> do_not_bind_allowlist =
      std::nullopt;
  if (args.do_not_bind_allowlist.has_value()) {
    do_not_bind_allowlist = absl::flat_hash_set<std::string>();
    for (const std::string& decl : *args.do_not_bind_allowlist) {
      do_not_bind_allowlist->insert(decl);
    }
  }

  absl_nullable std::shared_ptr<const llvm::Regex>
      template_blocklist_path_regex = nullptr;
  if (!args.template_blocklist_path_regex.empty()) {
    template_blocklist_path_regex = std::make_shared<llvm::Regex>(
        args.template_blocklist_path_regex, llvm::Regex::RegexFlags::NoFlags);
    if (!template_blocklist_path_regex->isValid()) {
      return absl::InvalidArgumentError(
          absl::StrCat("Failed to parse template_blocklist_path_regex: ",
                       args.template_blocklist_path_regex, " as llvm::Regex"));
    }
  }

  CRUBIT_ASSIGN_OR_RETURN(
      IR ir, IrFromCc(IrFromCcOptions{
                 .current_target = args.current_target,
                 .public_headers = args.public_headers,
                 .virtual_headers_contents_for_testing =
                     std::move(virtual_headers_contents_for_testing),
                 .headers_to_targets = args.headers_to_targets,
                 .extra_rs_srcs = args.extra_rs_srcs,
                 .extra_cpp_srcs = args.extra_cpp_srcs,
                 .reexported_namespaces = args.reexported_namespaces,
                 .unstable_rust_features = args.unstable_rust_features,
                 .clang_args = clang_args_view,
                 .extra_instantiations = requested_instantiations,
                 .crubit_features = args.target_to_features,
                 .crate_names = args.target_to_crate_name,
                 .driver_path = args.driver_path,
                 .do_not_bind_allowlist = std::move(do_not_bind_allowlist),
                 .kythe_annotations = args.kythe_annotations,
                 .template_blocklist_path_regex = template_blocklist_path_regex,
                 .carcinize = args.carcinize}));

  if (!args.instantiations_out.empty()) {
    ir.set_crate_root_path("__cc_template_instantiations_rs_api");
  }

  bool generate_error_report = !args.error_report_out.empty();

  absl::flat_hash_map<Identifier, Identifier> instantiations;
  if (const auto* absl_nullable ns =
          FindNamespace(ir, kInstantiationsNamespaceName)) {
    std::vector<const ir_proto::Record* absl_nonnull> records =
        FindInstantiationsInNamespace(ir, ItemId(ns->id()));
    for (const auto* record : records) {
      instantiations.insert(
          {Identifier(std::string(record->cc_name().identifier())),
           Identifier(std::string(record->rs_name().identifier()))});
    }
  }

  NamespacesHierarchy top_level_namespaces;
  if (!args.namespaces_out.empty()) {
    top_level_namespaces = crubit::CollectNamespaces(ir);
  }

  IR ir_out;
  // In development builds, if `--ir-out` is specified, copy the IR to be
  // returned in the output.
  if (!args.ir_out.empty()) {
    ir_out = ir;
  }

  CRUBIT_ASSIGN_OR_RETURN(
      Bindings bindings,
      GenerateBindings(std::move(ir), args.crubit_support_path_format,
                       args.crubit_support_versioned_path_format,
                       args.clang_format_exe_path, args.rustfmt_exe_path,
                       args.rustfmt_config_path, generate_error_report,
                       args.is_golden_test, args.kythe_annotations,
                       args.kythe_default_corpus));

  return BindingsAndMetadata{
      .ir = std::move(ir_out),
      .rs_api = bindings.rs_api,
      .rs_api_impl = bindings.rs_api_impl,
      .namespaces = std::move(top_level_namespaces),
      .instantiations = std::move(instantiations),
      .error_report = bindings.error_report,
  };
}

}  // namespace crubit
