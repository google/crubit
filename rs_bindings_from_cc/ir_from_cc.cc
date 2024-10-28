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

struct UseModFromSrc {
  UseMod use_mod;
  // The namespace that this UseMod should be added to. If not set, the UseMod
  // is a top-level item.
  std::optional<Namespace*> enclosing_namespace;
};

absl::StatusOr<std::vector<UseModFromSrc>> CreateUseModsFromExtraRustSrcs(
    IR& ir, absl::Span<const std::string> extra_rs_srcs) {
  std::vector<Namespace*> all_namespaces = ir.get_items_if<Namespace>();
  absl::flat_hash_map<std::string, ItemId> name_to_top_level_ns;
  absl::flat_hash_set<ItemId> top_level_item_id_set =
      absl::flat_hash_set<ItemId>(ir.top_level_item_ids.begin(),
                                  ir.top_level_item_ids.end());
  absl::flat_hash_map<ItemId, Namespace*> id_to_namespace;
  for (auto ns : all_namespaces) {
    if (ns->owning_target != ir.current_target) {
      continue;
    }
    // If a namespace is open more than once, we pick the last one of them as
    // it will serve as the canonical namespace without any number suffix in
    // the name.
    if (top_level_item_id_set.contains(ns->canonical_namespace_id)) {
      name_to_top_level_ns[ns->name.Ident()] = ns->id;
    }
    id_to_namespace.insert({ns->id, ns});
  }

  auto follow_mod_path_to_ns =
      [&](absl::string_view mod_path) -> std::optional<Namespace*> {
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
      for (auto child_id : id_to_namespace[ns_id]->child_item_ids) {
        if (id_to_namespace.contains(child_id) &&
            id_to_namespace[child_id]->name.Ident() == part) {
          ns_id = child_id;
          found = true;
          break;
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
    UseMod use_mod = UseMod{
        .path = std::string(extra_source_file_path),
        .mod_name = Identifier(absl::StrCat("__crubit_mod_", i++)),
        .id = id,
    };
    std::optional<Namespace*> enclosing_namespace = std::nullopt;
    if (!mod_path.empty()) {
      if (auto ns = follow_mod_path_to_ns(mod_path); ns.has_value()) {
        enclosing_namespace = std::move(ns);
      } else {
        return absl::InvalidArgumentError(
            "Specified a namespace path that does not exist. If "
            "you want to create a new module, use pub mod.");
      }
    }
    use_mods.push_back(UseModFromSrc{
        .use_mod = std::move(use_mod),
        .enclosing_namespace = std::move(enclosing_namespace),
    });
  }
  return use_mods;
}

// Convert the extra_rs_srcs into UseMod items and add them to the IR.
absl::Status AddUseModToIr(IR& ir,
                           absl::Span<const std::string> extra_rs_srcs) {
  // We have to reserve the space for the new items here because below we store
  // pointers to the Namespace items in the `UseModFromSrc`. If we reserve the
  // space after creating the `UseModFromSrc`, the pointers might be
  // invalidated.
  ir.items.reserve(ir.items.size() + extra_rs_srcs.size());
  CRUBIT_ASSIGN_OR_RETURN(std::vector<UseModFromSrc> use_mods,
                          CreateUseModsFromExtraRustSrcs(ir, extra_rs_srcs));
  for (auto& use_mod_from_src : use_mods) {
    ir.items.push_back(std::move(use_mod_from_src.use_mod));
    if (use_mod_from_src.enclosing_namespace.has_value()) {
      use_mod_from_src.enclosing_namespace.value()->child_item_ids.push_back(
          use_mod_from_src.use_mod.id);
    } else {
      ir.top_level_item_ids.push_back(use_mod_from_src.use_mod.id);
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

  // Tests may inject `extra_source_code_for_testing` - it needs to be appended
  // to `public_headers` and exposed via `file_contents` virtual file system.
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
  for (const HeaderName& header_name : augmented_public_headers) {
    absl::SubstituteAndAppend(&virtual_input_file_content, "#include \"$0\"\n",
                              header_name.IncludePath());
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

  Invocation invocation(options.current_target, augmented_public_headers,
                        options.headers_to_targets);
  if (!clang::tooling::runToolOnCodeWithArgs(
          std::make_unique<FrontendAction>(invocation),
          virtual_input_file_content, args_as_strings, kVirtualInputPath,
          "rs_bindings_from_cc",
          std::make_shared<clang::PCHContainerOperations>(), file_contents)) {
    return absl::Status(absl::StatusCode::kInvalidArgument,
                        "Could not compile header contents");
  }

  if (absl::Status status =
          AddUseModToIr(invocation.ir_, options.extra_rs_srcs);
      !status.ok()) {
    return status;
  }
  invocation.ir_.crubit_features = std::move(options.crubit_features);
  return invocation.ir_;
}

}  // namespace crubit
