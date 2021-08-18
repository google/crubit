// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Parses C++ headers and generates:
// * a Rust source file with bindings for the C++ API
// * a C++ source file with the implementation of the bindings

#include <memory>
#include <string>
#include <vector>

#include "base/init_google.h"
#include "base/logging.h"
#include "devtools/cymbal/common/clang_tool.h"
#include "rs_bindings_from_cc/frontend_action.h"
#include "rs_bindings_from_cc/ir.h"
#include "rs_bindings_from_cc/rs_src_code_gen.h"
#include "file/base/filesystem.h"
#include "file/base/helpers.h"
#include "file/base/options.h"
#include "third_party/absl/container/flat_hash_map.h"
#include "third_party/absl/flags/flag.h"
#include "third_party/absl/strings/string_view.h"
#include "third_party/absl/strings/substitute.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Frontend/FrontendAction.h"
#include "util/task/status.h"

ABSL_FLAG(std::string, rs_out, "",
          "output path for the Rust source file with bindings");
ABSL_FLAG(std::string, cc_out, "",
          "output path for the C++ source file with bindings implementation");
ABSL_FLAG(std::vector<std::string>, public_headers, std::vector<std::string>(),
          "public headers of the cc_library this tool should generate bindings "
          "for, in a format suitable for usage in #include \"\".");

constexpr absl::string_view kVirtualInputPath =
    "rs_bindings_from_cc_virtual_input.cc";

int main(int argc, char *argv[]) {
  InitGoogle(argv[0], &argc, &argv, true);

  auto rs_out = absl::GetFlag(FLAGS_rs_out);
  QCHECK(!rs_out.empty()) << "please specify --rs_out";
  auto cc_out = absl::GetFlag(FLAGS_cc_out);
  QCHECK(!cc_out.empty()) << "please specify --cc_out";
  auto public_headers = absl::GetFlag(FLAGS_public_headers);
  QCHECK(!public_headers.empty())
      << "please specify at least one header in --public_headers";

  std::vector<std::string> command_line(argv, argv + argc);
  command_line.push_back(std::string(kVirtualInputPath));

  std::string virtual_input_file_content;
  for (const std::string &header : public_headers) {
    absl::SubstituteAndAppend(&virtual_input_file_content, "#include \"$0\"\n",
                              header);
  }

  absl::flat_hash_map<std::string, std::string> file_contents{
      {std::string(kVirtualInputPath), virtual_input_file_content}};

  rs_bindings_from_cc::IR ir;
  if (devtools::cymbal::RunToolWithClangFlagsOnCode(
          command_line, file_contents,
          std::make_unique<rs_bindings_from_cc::FrontendAction>(ir))) {
    std::string rs_api = rs_bindings_from_cc::GenerateRustApi(ir);
    std::string rs_api_impl = "// No bindings implementation code was needed.";
    CHECK_OK(file::SetContents(rs_out, rs_api, file::Defaults()));
    CHECK_OK(file::SetContents(cc_out, rs_api_impl, file::Defaults()));
    return 0;
  } else {
    CHECK_OK(file::Delete(rs_out, file::Defaults()));
    CHECK_OK(file::Delete(cc_out, file::Defaults()));
    return 1;
  }
}
