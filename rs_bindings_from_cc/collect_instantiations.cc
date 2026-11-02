// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/collect_instantiations.h"

#include <string>
#include <utility>
#include <vector>

#include "common/ffi_types.h"
#include "common/status.h"
#include "llvm/ADT/ArrayRef.h"
#include "llvm/Support/Error.h"
#include "llvm/Support/FormatVariadic.h"
#include "llvm/Support/JSON.h"

// This function is implemented in Rust.
extern "C" crubit::FfiU8SliceBox CollectInstantiationsImpl(
    crubit::FfiU8Slice json);

namespace crubit {

llvm::Expected<std::vector<std::string>> CollectInstantiations(
    llvm::ArrayRef<std::string> rust_sources) {
  llvm::json::Value rust_sources_json = llvm::json::Array(rust_sources);
  std::string json = llvm::formatv("{0}", rust_sources_json);
  FfiU8SliceBox result = CollectInstantiationsImpl(MakeFfiU8Slice(json));
  std::string result_string = std::string(result.ptr, result.size);
  llvm::Expected<llvm::json::Value> expected_instantiations =
      llvm::json::parse(result_string);
  if (auto error = expected_instantiations.takeError()) {
    return llvm::createStringError(llvm::inconvertibleErrorCode(),
                                   llvm::toString(std::move(error)).c_str());
  }

  llvm::json::Value instantiations = *expected_instantiations;
  FreeFfiU8SliceBox(result);
  std::vector<std::string> instantiations_vector;
  llvm::json::Path::Root root;
  if (llvm::json::fromJSON(instantiations, instantiations_vector, root)) {
    return instantiations_vector;
  }
  return llvm::createStringError(llvm::inconvertibleErrorCode(),
                                 llvm::toString(root.getError()).c_str());
}

}  // namespace crubit
