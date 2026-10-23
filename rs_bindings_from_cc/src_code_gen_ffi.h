// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_SRC_CODE_GEN_FFI_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_SRC_CODE_GEN_FFI_H_

#include <string>

#include "rs_bindings_from_cc/ir_generated.h"
#include "rs_bindings_from_cc/profiling.h"

namespace crubit {

// The public interface called by the rest of the generator.
CRUBIT_PROFILE_NOINLINE
ffi::GenerateBindingsResponse GenerateBindingsCall(
    ffi::GenerateBindingsRequest&& request);

// Formats an IRProto using its Rust Debug representation (for `--ir_out`).
std::string FormatIrDebug(const ffi::IRProto& ir);

}  // namespace crubit

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_SRC_CODE_GEN_FFI_H_
