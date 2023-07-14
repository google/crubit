// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ir_from_cc.h"

#include <cstdint>
#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/log/check.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "absl/strings/substitute.h"
#include "absl/types/span.h"
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
      "-std=gnu++17",
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

  invocation.ir_.items.reserve(invocation.ir_.items.size() +
                               options.extra_rs_srcs.size());
  int i = 0;
  for (const std::string& extra_source : options.extra_rs_srcs) {
    // TODO(jeanpierreda): It'd be nice to give these human-readable names, e.g. the
    // name of the file without the `.rs`, but it's also annoying to handle name
    // collisions.
    ItemId id(reinterpret_cast<uintptr_t>(&extra_source));
    invocation.ir_.items.push_back(UseMod{
        .path = extra_source,
        .mod_name = Identifier(absl::StrCat("__crubit_mod_", i)),
        .id = id,
    });
    invocation.ir_.top_level_item_ids.push_back(id);
    ++i;
  }
  invocation.ir_.crubit_features = std::move(options.crubit_features);
  return invocation.ir_;
}

}  // namespace crubit
