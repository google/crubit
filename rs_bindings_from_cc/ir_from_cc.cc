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
    "ast_visitor_test_virtual_input.cc";

IR IrFromCc(absl::Span<const absl::string_view> header_files_contents,
            const std::vector<absl::string_view>& args) {
  std::vector<std::string> headers;
  absl::flat_hash_map<std::string, std::string> file_contents;
  std::string virtual_input_file_content;

  int counter = 0;
  for (const absl::string_view header_content : header_files_contents) {
    std::string filename(
        absl::Substitute("test/testing_header_$0.h", counter++));
    file_contents.insert({filename, std::string(header_content)});
    absl::SubstituteAndAppend(&virtual_input_file_content, "#include \"$0\"\n",
                              filename);
    headers.push_back(std::move(filename));
  }

  file_contents.insert(
      {std::string(kVirtualInputPath), virtual_input_file_content});

  std::vector<std::string> args_as_strings(args.begin(), args.end());
  args_as_strings.push_back("--syntax-only");
  // Needed, so that we can copy over non-doc comments that are used as
  // documention.
  args_as_strings.push_back("-fparse-all-comments");
  args_as_strings.push_back(std::string(kVirtualInputPath));

  IR ir;
  devtools::cymbal::RunToolWithClangFlagsOnCode(
      args_as_strings, file_contents,
      std::make_unique<rs_bindings_from_cc::FrontendAction>(
          std::vector<absl::string_view>(headers.begin(), headers.end()), ir));
  return ir;
}

}  // namespace rs_bindings_from_cc
