// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Parses C++ code and generates an equivalent Rust source file.

#include <utility>
#include <vector>

#include "absl/flags/flag.h"
#include "absl/flags/parse.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "common/check.h"
#include "common/file_io.h"
#include "migrator/rs_from_cc/rs_from_cc_lib.h"
#include "llvm/Support/FileSystem.h"

ABSL_FLAG(std::string, cc_in, "",
          "input path for the C++ source file (it may or may not be a header)");
ABSL_FLAG(std::string, rs_out, "",
          "output path for the Rust source file; will be overwritten if it "
          "already exists");

int main(int argc, char* argv[]) {
  auto args = absl::ParseCommandLine(argc, argv);

  auto cc_in = absl::GetFlag(FLAGS_cc_in);
  if (cc_in.empty()) {
    std::cerr << "please specify --cc_in" << std::endl;
    return 1;
  }
  auto rs_out = absl::GetFlag(FLAGS_rs_out);
  if (rs_out.empty()) {
    std::cerr << "please specify --rs_out" << std::endl;
    return 1;
  }

  auto status_or_cc_file_content = crubit::GetFileContents(cc_in);
  CRUBIT_CHECK(status_or_cc_file_content.ok());
  std::string cc_file_content = std::move(*status_or_cc_file_content);

  // Skip $0.
  ++argv;

  absl::StatusOr<std::string> rs_code = crubit_rs_from_cc::RsFromCc(
      cc_file_content, cc_in,
      std::vector<absl::string_view>(argv, argv + argc));
  if (!rs_code.ok()) {
    CRUBIT_CHECK(rs_code.ok());
  }

  CRUBIT_CHECK(crubit::SetFileContents(rs_out, *rs_code).ok());
  return 0;
}
