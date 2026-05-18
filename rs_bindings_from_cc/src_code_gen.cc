// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/src_code_gen.h"

#include <string>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "rs_bindings_from_cc/generate_bindings/generate_bindings.pb.h"
#include "rs_bindings_from_cc/ir.h"
#include "llvm/Support/FormatVariadic.h"

namespace crubit {

namespace {

using rs_bindings_from_cc::generate_bindings::GenerateBindingsRequest;
using rs_bindings_from_cc::generate_bindings::GenerateBindingsResponse;

}  // namespace

// This function is implemented in Rust.
extern "C" void GenerateBindingsImpl(const GenerateBindingsRequest* request,
                                     GenerateBindingsResponse* response);

absl::StatusOr<Bindings> GenerateBindings(
    const IR& ir, absl::string_view crubit_support_path_format,
    absl::string_view clang_format_exe_path, absl::string_view rustfmt_exe_path,
    absl::string_view rustfmt_config_path, bool generate_error_report,
    bool skip_source_location_in_doc_comments, bool kythe_annotations,
    absl::string_view kythe_default_corpus) {
  GenerateBindingsRequest request;
  request.set_json(llvm::formatv("{0}", ir.ToJson()));
  request.set_crubit_support_path_format(crubit_support_path_format);
  request.set_clang_format_exe_path(clang_format_exe_path);
  request.set_rustfmt_exe_path(rustfmt_exe_path);
  request.set_rustfmt_config_path(rustfmt_config_path);
  request.set_generate_error_report(generate_error_report);
  request.set_skip_source_location_in_doc_comments(
      skip_source_location_in_doc_comments);
  request.set_kythe_annotations(kythe_annotations);
  request.set_kythe_default_corpus(kythe_default_corpus);

  GenerateBindingsResponse response;
  GenerateBindingsImpl(&request, &response);

  if (!response.fatal_errors().empty()) {
    return absl::InvalidArgumentError(response.fatal_errors());
  }

  Bindings bindings;
  bindings.rs_api = response.rs_api();
  bindings.rs_api_impl = response.rs_api_impl();
  bindings.error_report = response.error_report();
  return bindings;
}

}  // namespace crubit
