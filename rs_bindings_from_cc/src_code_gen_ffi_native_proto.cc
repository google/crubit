// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <utility>

#include "rs_bindings_from_cc/src_code_gen_ffi.h"

namespace crubit {

using rs_bindings_from_cc::generate_bindings::GenerateBindingsRequest;
using rs_bindings_from_cc::generate_bindings::GenerateBindingsResponse;

extern "C" void GenerateBindingsImplFromNativeProto(
    GenerateBindingsRequest* request, GenerateBindingsResponse* response);

void GenerateBindingsProtoCallNative(GenerateBindingsRequest&& request,
                                     GenerateBindingsResponse* response) {
  // Move the request proto to the heap so Rust can own and safely destroy it.
  auto* owned_request = new GenerateBindingsRequest(std::move(request));
  GenerateBindingsImplFromNativeProto(owned_request, response);
}

}  // namespace crubit
