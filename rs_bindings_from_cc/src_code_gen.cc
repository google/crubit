// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/src_code_gen.h"

#include <string>

#include "rs_bindings_from_cc/ffi_types.h"
#include "rs_bindings_from_cc/ir.h"
#include "third_party/json/src/json.hpp"

namespace rs_bindings_from_cc {

// This function is implemented in Rust.
extern "C" FfiU8SliceBox GenerateRustApiImpl(FfiU8Slice);

std::string GenerateRustApi(const IR& ir) {
  std::string json = ir.ToJson().dump();
  FfiU8SliceBox slice_box = GenerateRustApiImpl(MakeFfiU8Slice(json));
  std::string rs_api(slice_box.ptr, slice_box.size);
  FreeFfiU8SliceBox(slice_box);
  return rs_api;
}

}  // namespace rs_bindings_from_cc
