// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Parses C++ headers and generates:
// * a Rust source file with bindings for the C++ API
// * a C++ source file with the implementation of the bindings

#include <string>
#include <utility>
#include <vector>

#include "base/init_google.h"
#include "base/logging.h"
#include "third_party/absl/container/flat_hash_map.h"
#include "third_party/absl/status/status.h"
#include "third_party/absl/status/statusor.h"
#include "third_party/absl/strings/string_view.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/cmdline.h"
#include "rs_bindings_from_cc/ir.h"
#include "rs_bindings_from_cc/ir_from_cc.h"
#include "rs_bindings_from_cc/src_code_gen.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/Support/FormatVariadic.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/Support/JSON.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/Support/raw_ostream.h"
#include "util/task/status_macros.h"

namespace {

absl::Status SetFileContents(absl::string_view path,
                             absl::string_view contents) {
  std::error_code error_code;
  llvm::raw_fd_ostream stream(path, error_code);
  if (error_code) {
    return absl::Status(absl::StatusCode::kInternal, error_code.message());
  }
  stream << contents;
  stream.close();
  if (stream.has_error()) {
    return absl::Status(absl::StatusCode::kInternal, stream.error().message());
  }
  return absl::OkStatus();
}

absl::Status Main(int argc, char* argv[]) {
  using rs_bindings_from_cc::Cmdline;
  using rs_bindings_from_cc::IR;
  ASSIGN_OR_RETURN(Cmdline cmdline, Cmdline::Create());

  if (cmdline.do_nothing()) {
    RETURN_IF_ERROR(SetFileContents(
        cmdline.rs_out(),
        "// intentionally left empty because --do_nothing was passed."));
    RETURN_IF_ERROR(SetFileContents(
        cmdline.cc_out(),
        "// intentionally left empty because --do_nothing was passed."));
    return absl::OkStatus();
  }

  ASSIGN_OR_RETURN(
      IR ir,
      rs_bindings_from_cc::IrFromCc(
          /* extra_source_code= */ "", cmdline.current_target(),
          cmdline.public_headers(),
          /* virtual_headers_contents= */ {}, cmdline.headers_to_targets(),
          std::vector<absl::string_view>(argv, argv + argc)));

  if (!cmdline.ir_out().empty()) {
    RETURN_IF_ERROR(SetFileContents(
        cmdline.ir_out(), std::string(llvm::formatv("{0:2}", ir.ToJson()))));
  }

  rs_bindings_from_cc::Bindings bindings =
      rs_bindings_from_cc::GenerateBindings(ir);
  RETURN_IF_ERROR(SetFileContents(cmdline.rs_out(), bindings.rs_api));
  RETURN_IF_ERROR(SetFileContents(cmdline.cc_out(), bindings.rs_api_impl));

  return absl::OkStatus();
}

}  // namespace

int main(int argc, char* argv[]) {
  InitGoogle(argv[0], &argc, &argv, true);
  absl::Status status = Main(argc, argv);
  if (!status.ok()) {
    llvm::errs() << status.message() << "\n";
    return -1;
  }
  return 0;
}
