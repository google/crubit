// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ffi_types.h"
#include "rs_bindings_from_cc/ir_from_cc.h"

namespace rs_bindings_from_cc {

// This is intended to be called from Rust.
extern "C" FfiU8SliceBox json_from_cc(FfiU8Slice cc_source) {
  IR ir = IrFromCc({StringViewFromFfiU8Slice(cc_source)}, {});
  std::string json = ir.ToJson().dump();
  return AllocFfiU8SliceBox(MakeFfiU8Slice(json));
}

}  // namespace rs_bindings_from_cc
