// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/src_code_gen.h"

#include <string>
#include <utility>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "rs_bindings_from_cc/ir.h"
#include "rs_bindings_from_cc/ir_generated.h"
#include "rs_bindings_from_cc/src_code_gen_ffi.h"

namespace crubit {

using ffi::GenerateBindingsRequest;
using ffi::GenerateBindingsResponse;

absl::StatusOr<Bindings> GenerateBindings(
    IR ir, llvm::StringRef crubit_support_path_format,
    llvm::StringRef crubit_support_versioned_path_format,
    llvm::StringRef clang_format_exe_path, llvm::StringRef rustfmt_exe_path,
    llvm::StringRef rustfmt_config_path, bool generate_error_report,
    bool is_golden_test, bool kythe_annotations,
    llvm::StringRef kythe_default_corpus) {
  GenerateBindingsRequest request;

  *request.mutable_ir() = std::move(ir);
  request.set_crubit_support_path_format(crubit_support_path_format);
  request.set_crubit_support_versioned_path_format(
      crubit_support_versioned_path_format);
  request.set_clang_format_exe_path(clang_format_exe_path);
  request.set_rustfmt_exe_path(rustfmt_exe_path);
  request.set_rustfmt_config_path(rustfmt_config_path);
  request.set_generate_error_report(generate_error_report);
  request.set_is_golden_test(is_golden_test);
  request.set_kythe_annotations(kythe_annotations);
  request.set_kythe_default_corpus(kythe_default_corpus);

  GenerateBindingsResponse response = GenerateBindingsCall(std::move(request));

  if (!response.fatal_errors().empty()) {
    return absl::InvalidArgumentError(response.fatal_errors());
  }

  Bindings bindings;
  bindings.rs_api = std::string(response.rs_api());
  bindings.rs_api_impl = std::string(response.rs_api_impl());
  bindings.error_report = std::string(response.error_report());
  return bindings;
}

}  // namespace crubit
