// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ir_from_cc.h"

#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "devtools/cymbal/common/clang_tool.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/frontend_action.h"
#include "rs_bindings_from_cc/ir.h"
#include "third_party/absl/container/flat_hash_map.h"
#include "third_party/absl/status/status.h"
#include "third_party/absl/status/statusor.h"
#include "third_party/absl/strings/string_view.h"
#include "third_party/absl/strings/substitute.h"
#include "third_party/absl/types/span.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Frontend/FrontendAction.h"

namespace rs_bindings_from_cc {

static constexpr absl::string_view kVirtualHeaderPath =
    "ir_from_cc_virtual_header.h";
static constexpr absl::string_view kVirtualInputPath =
    "ir_from_cc_virtual_input.cc";

absl::StatusOr<IR> IrFromCc(
    const absl::string_view extra_source_code, const Label current_target,
    absl::Span<const HeaderName> public_headers,
    absl::flat_hash_map<const HeaderName, const std::string>
        virtual_headers_contents,
    absl::flat_hash_map<const HeaderName, const Label> headers_to_targets,
    absl::Span<const absl::string_view> args) {
  std::vector<const HeaderName> entrypoint_headers(public_headers.begin(),
                                                   public_headers.end());
  absl::flat_hash_map<std::string, std::string> file_contents;

  for (auto const& name_and_content : virtual_headers_contents) {
    file_contents.insert({std::string(name_and_content.first.IncludePath()),
                          name_and_content.second});
  }
  if (!extra_source_code.empty()) {
    file_contents.insert(
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
  file_contents.insert(
      {std::string(kVirtualInputPath), virtual_input_file_content});

  std::vector<std::string> args_as_strings{
      // This includes the path of the binary that we're pretending to be.
      // TODO(forster): Find out where to point this. Should we use the
      // production crosstool for this? See
      // http://google3/devtools/cymbal/common/clang_tool.cc;l=171;rcl=385188113
      "clang",
      // We only need the AST.
      "-fsyntax-only",
      // Parse non-doc comments that are used as documention.
      "-fparse-all-comments"};
  args_as_strings.insert(args_as_strings.end(), args.begin(), args.end());
  args_as_strings.push_back(std::string(kVirtualInputPath));

  if (IR ir; devtools::cymbal::RunToolWithClangFlagsOnCode(
          args_as_strings, file_contents,
          std::make_unique<FrontendAction>(current_target, entrypoint_headers,
                                           &headers_to_targets, &ir))) {
    return ir;
  } else {
    return absl::Status(absl::StatusCode::kInvalidArgument,
                        "Could not compile header contents");
  }
}

}  // namespace rs_bindings_from_cc
