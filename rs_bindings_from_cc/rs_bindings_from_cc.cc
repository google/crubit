// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Parses C++ headers and generates:
// * a Rust source file with bindings for the C++ API
// * a C++ source file with the implementation of the bindings

#include <string>
#include <utility>
#include <vector>

#include "absl/flags/parse.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "common/file_io.h"
#include "common/status_macros.h"
#include "rs_bindings_from_cc/cmdline.h"
#include "rs_bindings_from_cc/collect_namespaces.h"
#include "rs_bindings_from_cc/generate_bindings_and_metadata.h"
#include "rs_bindings_from_cc/ir.h"
#include "llvm/Support/raw_ostream.h"

namespace crubit {

std::string InstantiationsAsJson(
    const BindingsAndMetadata& bindings_and_metadata) {
  llvm::json::Object obj;
  for (const auto& entry : bindings_and_metadata.instantiations) {
    obj[entry.first] = entry.second;
  }
  return std::string(llvm::formatv("{0:2}", llvm::json::Value(std::move(obj))));
}

absl::Status Main(absl::Span<char* const> args) {
  CRUBIT_ASSIGN_OR_RETURN(Cmdline cmdline, Cmdline::Create());

  if (cmdline.do_nothing()) {
    CRUBIT_RETURN_IF_ERROR(SetFileContents(
        cmdline.rs_out(),
        "// intentionally left empty because --do_nothing was passed."));
    CRUBIT_RETURN_IF_ERROR(SetFileContents(
        cmdline.cc_out(),
        "// intentionally left empty because --do_nothing was passed."));
    if (!cmdline.instantiations_out().empty()) {
      CRUBIT_RETURN_IF_ERROR(
          SetFileContents(cmdline.instantiations_out(), "[]"));
    }
    if (!cmdline.namespaces_out().empty()) {
      CRUBIT_RETURN_IF_ERROR(SetFileContents(cmdline.namespaces_out(), "[]"));
    }
    return absl::OkStatus();
  }

  std::vector<std::string> clang_args;
  clang_args.insert(clang_args.end(), args.begin(), args.end());

  CRUBIT_ASSIGN_OR_RETURN(
      BindingsAndMetadata bindings_and_metadata,
      GenerateBindingsAndMetadata(cmdline, std::move(clang_args)));

  if (!cmdline.ir_out().empty()) {
    CRUBIT_RETURN_IF_ERROR(
        SetFileContents(cmdline.ir_out(), IrToJson(bindings_and_metadata.ir)));
  }

  CRUBIT_RETURN_IF_ERROR(
      SetFileContents(cmdline.rs_out(), bindings_and_metadata.rs_api));
  CRUBIT_RETURN_IF_ERROR(
      SetFileContents(cmdline.cc_out(), bindings_and_metadata.rs_api_impl));

  if (!cmdline.instantiations_out().empty()) {
    CRUBIT_RETURN_IF_ERROR(
        SetFileContents(cmdline.instantiations_out(),
                        InstantiationsAsJson(bindings_and_metadata)));
  }

  if (!cmdline.namespaces_out().empty()) {
    CRUBIT_RETURN_IF_ERROR(SetFileContents(
        cmdline.namespaces_out(),
        crubit::NamespacesAsJson(bindings_and_metadata.namespaces)));
  }

  return absl::OkStatus();
}

}  // namespace crubit

int main(int argc, char* argv[]) {
  auto args = absl::ParseCommandLine(argc, argv);
  absl::Status status = crubit::Main(args);
  if (!status.ok()) {
    llvm::errs() << status.message() << "\n";
    return -1;
  }
  return 0;
}
