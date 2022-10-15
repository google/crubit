// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/src_code_gen.h"

#include <string>

#include "common/ffi_types.h"
#include "common/status_macros.h"
#include "rs_bindings_from_cc/ir.h"
#include "llvm/Support/FormatVariadic.h"
#include "llvm/Support/JSON.h"

namespace crubit {

// FFI equivalent of `Bindings`.
struct FfiBindings {
  FfiU8SliceBox rs_api;
  FfiU8SliceBox rs_api_impl;
  FfiU8SliceBox error_report;
};

// This function is implemented in Rust.
extern "C" FfiBindings GenerateBindingsImpl(FfiU8Slice json,
                                            FfiU8Slice crubit_support_path,
                                            FfiU8Slice rustfmt_exe_path,
                                            FfiU8Slice rustfmt_config_path,
                                            bool generate_error_report);

// Creates `Bindings` instance from copied data from `ffi_bindings`.
static absl::StatusOr<Bindings> MakeBindingsFromFfiBindings(
    const FfiBindings& ffi_bindings) {
  Bindings bindings;

  const FfiU8SliceBox& rs_api = ffi_bindings.rs_api;
  const FfiU8SliceBox& rs_api_impl = ffi_bindings.rs_api_impl;
  const FfiU8SliceBox& error_report = ffi_bindings.error_report;

  bindings.rs_api = std::string(rs_api.ptr, rs_api.size);
  bindings.rs_api_impl = std::string(rs_api_impl.ptr, rs_api_impl.size);
  bindings.error_report = std::string(error_report.ptr, error_report.size);
  return bindings;
}

// Deallocates given `ffi_bindings` instance that was created in Rust.
static void FreeFfiBindings(FfiBindings ffi_bindings) {
  FreeFfiU8SliceBox(ffi_bindings.rs_api);
  FreeFfiU8SliceBox(ffi_bindings.rs_api_impl);
  FreeFfiU8SliceBox(ffi_bindings.error_report);
}

absl::StatusOr<Bindings> GenerateBindings(const IR& ir,
                                          absl::string_view crubit_support_path,
                                          absl::string_view rustfmt_exe_path,
                                          absl::string_view rustfmt_config_path,
                                          bool generate_error_report) {
  std::string json = llvm::formatv("{0}", ir.ToJson());

  FfiBindings ffi_bindings = GenerateBindingsImpl(
      MakeFfiU8Slice(json), MakeFfiU8Slice(crubit_support_path),
      MakeFfiU8Slice(rustfmt_exe_path), MakeFfiU8Slice(rustfmt_config_path),
      generate_error_report);
  CRUBIT_ASSIGN_OR_RETURN(Bindings bindings,
                          MakeBindingsFromFfiBindings(ffi_bindings));
  FreeFfiBindings(ffi_bindings);
  return bindings;
}

}  // namespace crubit
