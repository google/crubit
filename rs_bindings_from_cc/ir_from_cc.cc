// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ir_from_cc.h"

#include <cstddef>
#include <cstdint>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/container/flat_hash_set.h"
#include "absl/log/check.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/match.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/str_split.h"
#include "absl/strings/string_view.h"
#include "absl/strings/substitute.h"
#include "absl/types/span.h"
#include "common/status_macros.h"
#include "common/string_view_conversion.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/decl_importer.h"
#include "rs_bindings_from_cc/frontend_action.h"
#include "rs_bindings_from_cc/ir.h"
#include "clang/Serialization/PCHContainerOperations.h"
#include "clang/Tooling/Tooling.h"

namespace crubit {

static constexpr absl::string_view kVirtualHeaderPath =
    "ir_from_cc_virtual_header.h";
static constexpr absl::string_view kVirtualInputPath =
    "ir_from_cc_virtual_input.cc";

namespace {

namespace ir_proto = ::crubit::rs_bindings_from_cc::ir_proto::flat;

struct UseModFromSrc {
  ir_proto::Item use_mod_item;
  // The namespace that this UseMod should be added to. If not set, the UseMod
  // is a top-level item.
  std::optional<ir_proto::Namespace*> enclosing_namespace;
};

absl::StatusOr<std::vector<UseModFromSrc>> CreateUseModsFromExtraRustSrcs(
    IR& ir, absl::Span<const std::string> extra_rs_srcs,
    const absl::flat_hash_map<BazelLabel, std::vector<ItemId>>&
        top_level_item_ids,
    const absl::flat_hash_map<ItemId, std::vector<ItemId>>& child_item_ids) {
  std::vector<ir_proto::Namespace*> all_namespaces =
      ir.get_items_if<ir_proto::Namespace>();
  absl::flat_hash_map<std::string, ItemId> name_to_top_level_ns;
  absl::flat_hash_set<ItemId> top_level_item_id_set;
  if (auto it = top_level_item_ids.find(ir.current_target);
      it != top_level_item_ids.end()) {
    top_level_item_id_set.insert(it->second.begin(), it->second.end());
  }
  absl::flat_hash_map<ItemId, ir_proto::Namespace*> id_to_namespace;
  for (auto ns : all_namespaces) {
    if (ns->owning_target() != ir.current_target.value()) {
      continue;
    }
    // If a namespace is open more than once, we pick the last one of them as
    // it will serve as the canonical namespace without any number suffix in
    // the name.
    if (top_level_item_id_set.contains(ItemId(ns->id()))) {
      name_to_top_level_ns[ns->cc_name().identifier()] = ItemId(ns->id());
    }
    id_to_namespace.insert({ItemId(ns->id()), ns});
  }

  auto follow_mod_path_to_ns =
      [&](absl::string_view mod_path) -> std::optional<ir_proto::Namespace*> {
    if (mod_path.empty()) {
      return std::nullopt;
    }
    std::vector<absl::string_view> parts = absl::StrSplit(mod_path, "::");
    // In case there are some inner namespaces with the same name, we need to
    // first find the top-level namespace and then follow its children to match
    // the full path.
    auto it = name_to_top_level_ns.find(parts[0]);
    if (it == name_to_top_level_ns.end()) {
      return std::nullopt;
    }
    ItemId ns_id = it->second;
    for (size_t i = 1; i < parts.size(); ++i) {
      const auto& part = parts[i];
      bool found = false;
      if (auto child_it = child_item_ids.find(ns_id);
          child_it != child_item_ids.end()) {
        for (auto child_id : child_it->second) {
          if (auto ns_it = id_to_namespace.find(child_id);
              ns_it != id_to_namespace.end() &&
              ns_it->second->cc_name().identifier() == part) {
            ns_id = child_id;
            found = true;
            break;
          }
        }
      }
      if (!found) {
        return std::nullopt;
      }
    }
    return id_to_namespace[ns_id];
  };

  int i = 0;
  std::vector<UseModFromSrc> use_mods;
  use_mods.reserve(extra_rs_srcs.size());
  for (const std::string& extra_source_info : extra_rs_srcs) {
    std::pair<absl::string_view, absl::string_view> parts =
        absl::StrSplit(extra_source_info, absl::MaxSplits('=', 1));
    absl::string_view extra_source_file_path = parts.first;
    absl::string_view mod_path = parts.second;
    if (absl::StrContains(mod_path, '=')) {
      return absl::InvalidArgumentError(
          absl::StrCat("Invalid extra_rs_srcs entry: ", extra_source_info));
    }
    // TODO(jeanpierreda): It'd be nice to give these human-readable names, e.g. the
    // name of the file without the `.rs`, but it's also annoying to handle name
    // collisions.
    ItemId id(reinterpret_cast<uintptr_t>(&extra_source_info));
    ir_proto::Item item;
    auto* use_mod = item.mutable_use_mod();
    use_mod->set_path(extra_source_file_path);
    use_mod->mutable_mod_name()->set_identifier(
        absl::StrCat("__crubit_mod_", i++));
    use_mod->set_id(id.value());

    std::optional<ir_proto::Namespace*> enclosing_namespace = std::nullopt;
    if (!mod_path.empty()) {
      if (auto ns = follow_mod_path_to_ns(mod_path); ns.has_value()) {
        enclosing_namespace = std::move(ns);
      } else {
        return absl::InvalidArgumentError(absl::Substitute(
            "Specified a namespace path '$0' that does not exist. If "
            "you want to create a new module, use pub mod.",
            mod_path));
      }
    }
    use_mods.push_back(UseModFromSrc{
        .use_mod_item = std::move(item),
        .enclosing_namespace = std::move(enclosing_namespace),
    });
  }
  return use_mods;
}

// Convert the extra_rs_srcs into UseMod items and add them to the IR.
absl::Status AddUseModToIr(
    IR& ir, absl::Span<const std::string> extra_rs_srcs,
    absl::flat_hash_map<BazelLabel, std::vector<ItemId>>& top_level_item_ids,
    absl::flat_hash_map<ItemId, std::vector<ItemId>>& child_item_ids) {
  CRUBIT_ASSIGN_OR_RETURN(
      std::vector<UseModFromSrc> use_mods,
      CreateUseModsFromExtraRustSrcs(ir, extra_rs_srcs, top_level_item_ids,
                                     child_item_ids));
  for (auto& use_mod_from_src : use_mods) {
    ItemId use_mod_id(use_mod_from_src.use_mod_item.use_mod().id());
    if (use_mod_from_src.enclosing_namespace.has_value()) {
      child_item_ids[ItemId(use_mod_from_src.enclosing_namespace.value()->id())]
          .push_back(use_mod_id);
      *use_mod_from_src.enclosing_namespace.value()->add_children() =
          std::move(use_mod_from_src.use_mod_item);
    } else {
      *(*ir.ir_proto.mutable_top_level_items())[ir.current_target.value()]
           .add_items() = std::move(use_mod_from_src.use_mod_item);
    }
  }
  return absl::OkStatus();
}
}  // namespace

absl::StatusOr<IR> IrFromCc(IrFromCcOptions options) {
  // Caller should verify that the inputs are not empty.
  CHECK(!options.extra_source_code_for_testing.empty() ||
        !options.public_headers.empty() ||
        !options.extra_instantiations.empty());

  clang::tooling::FileContentMappings file_contents;

  for (auto const& name_and_content :
       options.virtual_headers_contents_for_testing) {
    file_contents.push_back({std::string(name_and_content.first.IncludePath()),
                             name_and_content.second});
  }

  // We must parse `extra_cpp_srcs` (user-provided manual wiring C++ files) and
  // `extra_source_code_for_testing` (injected by tests) so that Clang includes
  // their AST nodes in the generated IR. We append them to `public_headers`.
  std::vector<HeaderName> augmented_public_headers(
      options.public_headers.begin(), options.public_headers.end());
  if (!options.extra_source_code_for_testing.empty()) {
    file_contents.push_back(
        {std::string(kVirtualHeaderPath),
         std::string(options.extra_source_code_for_testing)});
    HeaderName header_name = HeaderName(std::string(kVirtualHeaderPath));
    augmented_public_headers.push_back(header_name);
    options.headers_to_targets.insert({header_name, options.current_target});
  }

  std::string virtual_input_file_content;
  auto add_header = [&](absl::string_view header_path) {
    absl::SubstituteAndAppend(&virtual_input_file_content, "#include \"$0\"\n",
                              header_path);
  };
  for (const HeaderName& header_name : augmented_public_headers) {
    add_header(header_name.IncludePath());
  }
  for (const std::string& cc_src : options.extra_cpp_srcs) {
    add_header(cc_src);
    options.headers_to_targets.insert(
        {HeaderName(cc_src), options.current_target});
  }
  if (!options.extra_instantiations.empty()) {
    absl::SubstituteAndAppend(&virtual_input_file_content, "namespace $0 {\n",
                              kInstantiationsNamespaceName);
    int counter = 0;
    for (const std::string& extra_instantiation :
         options.extra_instantiations) {
      absl::SubstituteAndAppend(&virtual_input_file_content,
                                "using __cc_template_instantiation_$0 = $1;\n",
                                counter++, extra_instantiation);
    }
    absl::SubstituteAndAppend(&virtual_input_file_content,
                              "}  // namespace $0\n",
                              kInstantiationsNamespaceName);
  }
  std::vector<std::string> args_as_strings = {
      // Parse non-doc comments that are used as documentation
      "-fparse-all-comments"};
  args_as_strings.insert(args_as_strings.end(), options.clang_args.begin(),
                         options.clang_args.end());

  Invocation invocation(
      options.current_target, augmented_public_headers,
      options.headers_to_targets, std::move(options.do_not_bind_allowlist),
      std::move(options.crubit_features), std::move(options.crate_names),
      options.kythe_annotations, options.template_blocklist_path_regex,
      options.carcinize);
  if (!clang::tooling::runToolOnCodeWithArgs(
          std::make_unique<FrontendAction>(invocation),
          virtual_input_file_content, args_as_strings,
          StringRefFromStringView(kVirtualInputPath),
          // Passing the path to the driver script here allows Clang to find the
          // resource directory relative to this path.
          StringRefFromStringView(options.driver_path),
          std::make_shared<clang::PCHContainerOperations>(), file_contents)) {
    return absl::Status(absl::StatusCode::kInvalidArgument,
                        "Could not compile header contents");
  }

  invocation.ir_.ir_proto = std::move(invocation.ir_proto_);

  absl::flat_hash_map<BazelLabel, std::vector<ItemId>> top_level_item_ids =
      invocation.top_level_item_ids_;
  absl::flat_hash_map<ItemId, std::vector<ItemId>> child_item_ids =
      invocation.child_item_ids_;
  if (absl::Status status = AddUseModToIr(invocation.ir_, options.extra_rs_srcs,
                                          top_level_item_ids, child_item_ids);
      !status.ok()) {
    return status;
  }
  invocation.ir_.BuildTree(std::move(top_level_item_ids),
                           std::move(child_item_ids));
  invocation.ir_.reexported_namespaces =
      std::vector<std::string>(options.reexported_namespaces.begin(),
                               options.reexported_namespaces.end());
  invocation.ir_.unstable_rust_features.assign(
      options.unstable_rust_features.begin(),
      options.unstable_rust_features.end());
  return invocation.ir_;
}

}  // namespace crubit
