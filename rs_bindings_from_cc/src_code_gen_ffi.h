// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_SRC_CODE_GEN_FFI_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_SRC_CODE_GEN_FFI_H_

#include "rs_bindings_from_cc/generate_bindings/generate_bindings.pb.h"

namespace crubit {

rs_bindings_from_cc::generate_bindings::GenerateBindingsResponse
GenerateBindingsProtoCall(
    const rs_bindings_from_cc::generate_bindings::GenerateBindingsRequest&
        request);
}  // namespace crubit

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_SRC_CODE_GEN_FFI_H_
