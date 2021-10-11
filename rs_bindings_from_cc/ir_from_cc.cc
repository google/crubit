// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ir_from_cc.h"

#include "devtools/cymbal/common/clang_tool.h"
#include "rs_bindings_from_cc/frontend_action.h"
#include "third_party/absl/container/flat_hash_map.h"
#include "third_party/absl/strings/substitute.h"

namespace rs_bindings_from_cc {

static constexpr absl::string_view kVirtualInputPath =
    "ir_from_cc_virtual_input.cc";

absl::StatusOr<IR> IrFromCc(
    absl::Span<const absl::string_view> header_files_contents,
    absl::Span<const absl::string_view> header_names,
    absl::Span<const absl::string_view> args) {
  std::vector<std::string> headers{header_names.begin(), header_names.end()};
  absl::flat_hash_map<std::string, std::string> file_contents;
  std::string virtual_input_file_content;

  int counter = 0;
  for (const absl::string_view header_content : header_files_contents) {
    std::string filename(
        absl::Substitute("test/testing_header_$0.h", counter++));
    file_contents.insert({filename, std::string(header_content)});
    headers.push_back(std::move(filename));
  }

  for (const std::string& filename : headers) {
    absl::SubstituteAndAppend(&virtual_input_file_content, "#include \"$0\"\n",
                              filename);
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
          std::make_unique<FrontendAction>(
              std::vector<absl::string_view>(headers.begin(), headers.end()),
              ir))) {
    return ir;
  } else {
    return absl::Status(absl::StatusCode::kInvalidArgument,
                        "Could not compile header contents");
  }
}

}  // namespace rs_bindings_from_cc
