// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/collect_instantiations.h"

#include <string>
#include <utility>
#include <vector>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/types/span.h"
#include "common/ffi_types.h"
#include "llvm/Support/Error.h"
#include "llvm/Support/FormatVariadic.h"
#include "llvm/Support/JSON.h"

// This function is implemented in Rust.
extern "C" crubit::FfiU8SliceBox CollectInstantiationsImpl(
    crubit::FfiU8Slice json);

namespace crubit {

absl::StatusOr<std::vector<std::string>> CollectInstantiations(
    absl::Span<const std::string> rust_sources) {
  llvm::json::Value rust_sources_json = llvm::json::Array(rust_sources);
  std::string json = llvm::formatv("{0}", rust_sources_json);
  FfiU8SliceBox result = CollectInstantiationsImpl(MakeFfiU8Slice(json));
  std::string result_string = std::string(result.ptr, result.size);
  llvm::Expected<llvm::json::Value> expected_instantiations =
      llvm::json::parse(result_string);
  if (auto error = expected_instantiations.takeError()) {
    return absl::InternalError(llvm::toString(std::move(error)));
  }

  llvm::json::Value instantiations = *expected_instantiations;
  FreeFfiU8SliceBox(result);
  std::vector<std::string> instantiations_vector;
  llvm::json::Path::Root root;
  if (llvm::json::fromJSON(instantiations, instantiations_vector, root)) {
    return instantiations_vector;
  }
  return absl::InternalError(llvm::toString(root.getError()));
}

}  // namespace crubit
