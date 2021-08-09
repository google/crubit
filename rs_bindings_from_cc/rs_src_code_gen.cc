// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/rs_src_code_gen.h"

#include <stddef.h>

#include <string>

#include "rs_bindings_from_cc/ir.h"
#include "third_party/absl/strings/string_view.h"
#include "third_party/json/src/json.hpp"

namespace rs_bindings_from_cc {

struct FfiU8SliceBox {
  const char* ptr;
  size_t size;
};

struct FfiU8Slice {
  const char* ptr;
  size_t size;
};

static FfiU8Slice MakeFfiU8Slice(absl::string_view s) {
  FfiU8Slice result;
  result.ptr = s.data();
  result.size = s.size();
  return result;
}

// This function is implemented in Rust.
extern "C" FfiU8SliceBox GenerateRustApiImpl(FfiU8Slice);

// This function is implemented in Rust.
extern "C" void FreeFfiU8SliceBox(FfiU8SliceBox);

std::string GenerateRustApi(const IR& ir) {
  std::string json = ir.ToJson().dump();
  FfiU8SliceBox slice_box = GenerateRustApiImpl(MakeFfiU8Slice(json));
  std::string rs_api(slice_box.ptr, slice_box.size);
  FreeFfiU8SliceBox(slice_box);
  return rs_api;
}

}  // namespace rs_bindings_from_cc
