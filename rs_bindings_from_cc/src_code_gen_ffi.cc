// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/src_code_gen_ffi.h"

#include <string>

#include "absl/strings/string_view.h"
#include "common/ffi_types.h"

namespace crubit {

using rs_bindings_from_cc::generate_bindings::GenerateBindingsRequest;
using rs_bindings_from_cc::generate_bindings::GenerateBindingsResponse;

extern "C" FfiU8SliceBox GenerateBindingsImpl(FfiU8Slice serialized_request);

GenerateBindingsResponse GenerateBindingsProtoCall(
    const GenerateBindingsRequest& request) {
  std::string serialized_request = request.SerializeAsString();
  FfiU8SliceBox serialized_response =
      GenerateBindingsImpl(MakeFfiU8Slice(serialized_request));

  GenerateBindingsResponse response;
  response.ParseFromString(
      absl::string_view(serialized_response.ptr, serialized_response.size));

  FreeFfiU8SliceBox(serialized_response);
  return response;
}

}  // namespace crubit
