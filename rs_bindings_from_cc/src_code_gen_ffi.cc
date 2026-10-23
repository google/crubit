// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/src_code_gen_ffi.h"

#include <string>

#include "rs_bindings_from_cc/ir_ffi_support.h"
#include "rs_bindings_from_cc/ir_generated.h"

namespace crubit {

extern "C" void GenerateBindingsImpl(ffi::GenerateBindingsRequest* request,
                                     ffi::GenerateBindingsResponse* response);

extern "C" void FormatIrDebugImpl(const ffi::IRProto* ir, ffi::FfiStr* out);

ffi::GenerateBindingsResponse GenerateBindingsCall(
    ffi::GenerateBindingsRequest&& request) {
  ffi::GenerateBindingsResponse response;
  GenerateBindingsImpl(&request, &response);
  return response;
}

std::string FormatIrDebug(const ffi::IRProto& ir) {
  ffi::FfiStr out;
  FormatIrDebugImpl(&ir, &out);
  return out.ToString();
}

}  // namespace crubit
