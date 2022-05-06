// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Parses C++ headers and generates:
// * a Rust source file with bindings for the C++ API
// * a C++ source file with the implementation of the bindings

#include <string>
#include <utility>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/flags/parse.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "common/file_io.h"
#include "common/status_macros.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/cmdline.h"
#include "rs_bindings_from_cc/ir.h"
#include "rs_bindings_from_cc/ir_from_cc.h"
#include "rs_bindings_from_cc/src_code_gen.h"
#include "llvm/Support/FormatVariadic.h"
#include "llvm/Support/JSON.h"
#include "llvm/Support/raw_ostream.h"

namespace crubit {

absl::Status Main(std::vector<char*> args) {
  using crubit::Cmdline;
  using crubit::IR;
  CRUBIT_ASSIGN_OR_RETURN(Cmdline cmdline, Cmdline::Create());

  if (cmdline.do_nothing()) {
    CRUBIT_RETURN_IF_ERROR(SetFileContents(
        cmdline.rs_out(),
        "// intentionally left empty because --do_nothing was passed."));
    CRUBIT_RETURN_IF_ERROR(SetFileContents(
        cmdline.cc_out(),
        "// intentionally left empty because --do_nothing was passed."));
    return absl::OkStatus();
  }

  std::vector<absl::string_view> args_as_string_views;
  args_as_string_views.insert(args_as_string_views.end(), args.begin(),
                              args.end());

  CRUBIT_ASSIGN_OR_RETURN(
      IR ir, crubit::IrFromCc(
                 /* extra_source_code= */ "", cmdline.current_target(),
                 cmdline.public_headers(),
                 /* virtual_headers_contents= */ {},
                 cmdline.headers_to_targets(), args_as_string_views));

  if (!cmdline.ir_out().empty()) {
    CRUBIT_RETURN_IF_ERROR(SetFileContents(
        cmdline.ir_out(), std::string(llvm::formatv("{0:2}", ir.ToJson()))));
  }

  crubit::Bindings bindings = crubit::GenerateBindings(
      ir, cmdline.crubit_support_path(), cmdline.rustfmt_config_path());
  CRUBIT_RETURN_IF_ERROR(SetFileContents(cmdline.rs_out(), bindings.rs_api));
  CRUBIT_RETURN_IF_ERROR(
      SetFileContents(cmdline.cc_out(), bindings.rs_api_impl));

  return absl::OkStatus();
}

}  // namespace crubit

int main(int argc, char* argv[]) {
  auto args = absl::ParseCommandLine(argc, argv);
  absl::Status status = crubit::Main(std::move(args));
  if (!status.ok()) {
    llvm::errs() << status.message() << "\n";
    return -1;
  }
  return 0;
}
