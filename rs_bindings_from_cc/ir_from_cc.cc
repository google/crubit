// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ir_from_cc.h"

#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "absl/strings/substitute.h"
#include "absl/types/span.h"
#include "common/check.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/frontend_action.h"
#include "rs_bindings_from_cc/ir.h"
#include "clang/Frontend/FrontendAction.h"
#include "clang/Tooling/Tooling.h"

namespace crubit {

static constexpr absl::string_view kVirtualHeaderPath =
    "ir_from_cc_virtual_header.h";
static constexpr absl::string_view kVirtualInputPath =
    "ir_from_cc_virtual_input.cc";

absl::StatusOr<IR> IrFromCc(
    const absl::string_view extra_source_code, const BazelLabel current_target,
    absl::Span<const HeaderName> public_headers,
    absl::flat_hash_map<const HeaderName, const std::string>
        virtual_headers_contents,
    absl::flat_hash_map<const HeaderName, const BazelLabel> headers_to_targets,
    absl::Span<const absl::string_view> args,
    absl::Span<const std::string> extra_instantiations) {
  // Caller should verify that the inputs are not empty.
  // TODO(b/440066049): Generate a source file for requested instantiations once
  // cl/430823388 is submitted.
  CRUBIT_CHECK(!extra_source_code.empty() || !public_headers.empty() ||
               !extra_instantiations.empty());
  CRUBIT_CHECK(!extra_source_code.empty() || !headers_to_targets.empty() ||
               !extra_instantiations.empty());

  std::vector<HeaderName> entrypoint_headers(public_headers.begin(),
                                             public_headers.end());
  clang::tooling::FileContentMappings file_contents;

  for (auto const& name_and_content : virtual_headers_contents) {
    file_contents.push_back({std::string(name_and_content.first.IncludePath()),
                             name_and_content.second});
  }
  if (!extra_source_code.empty()) {
    file_contents.push_back(
        {std::string(kVirtualHeaderPath), std::string(extra_source_code)});
    HeaderName header_name = HeaderName(std::string(kVirtualHeaderPath));
    entrypoint_headers.push_back(header_name);
    headers_to_targets.insert({header_name, current_target});
  }

  std::string virtual_input_file_content;
  for (const HeaderName& header_name : entrypoint_headers) {
    absl::SubstituteAndAppend(&virtual_input_file_content, "#include \"$0\"\n",
                              header_name.IncludePath());
  }

  std::vector<std::string> args_as_strings{
      // Parse non-doc comments that are used as documention
      "-fparse-all-comments"};
  args_as_strings.insert(args_as_strings.end(), args.begin(), args.end());

  if (Invocation invocation(current_target, entrypoint_headers,
                            headers_to_targets);
      clang::tooling::runToolOnCodeWithArgs(
          std::make_unique<FrontendAction>(invocation),
          virtual_input_file_content, args_as_strings, kVirtualInputPath,
          "rs_bindings_from_cc",
          std::make_shared<clang::PCHContainerOperations>(), file_contents)) {
    return invocation.ir_;
  } else {
    return absl::Status(absl::StatusCode::kInvalidArgument,
                        "Could not compile header contents");
  }
}

}  // namespace crubit
