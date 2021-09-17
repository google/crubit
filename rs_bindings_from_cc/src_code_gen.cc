// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/src_code_gen.h"

#include <string>

#include "rs_bindings_from_cc/ffi_types.h"
#include "rs_bindings_from_cc/ir.h"
#include "third_party/json/src/json.hpp"
#include "third_party/llvm/llvm-project/clang/include/clang/Format/Format.h"

namespace rs_bindings_from_cc {

// FFI equivalent of `Bindings`.
struct FfiBindings {
  FfiU8SliceBox rs_api;
  FfiU8SliceBox rs_api_impl;
};

// This function is implemented in Rust.
extern "C" FfiBindings GenerateBindingsImpl(FfiU8Slice json);

// Creates `Bindings` instance from copied data from `ffi_bindings`.
static Bindings MakeBindingsFromFfiBindings(const FfiBindings& ffi_bindings) {
  Bindings bindings;

  const FfiU8SliceBox& rs_api = ffi_bindings.rs_api;
  const FfiU8SliceBox& rs_api_impl = ffi_bindings.rs_api_impl;

  bindings.rs_api = std::string(rs_api.ptr, rs_api.size);

  std::string impl{rs_api_impl.ptr, rs_api_impl.size};
  bindings.rs_api_impl = *clang::tooling::applyAllReplacements(
      impl,
      clang::format::reformat(
          clang::format::getGoogleStyle(clang::format::FormatStyle::LK_Cpp),
          impl, clang::tooling::Range(0, impl.size()), "<stdin>"));

  return bindings;
}

// Deallocates given `ffi_bindings` instance that was created in Rust.
static void FreeFfiBindings(FfiBindings ffi_bindings) {
  FreeFfiU8SliceBox(ffi_bindings.rs_api);
  FreeFfiU8SliceBox(ffi_bindings.rs_api_impl);
}

Bindings GenerateBindings(const IR& ir) {
  std::string json = ir.ToJson().dump();
  FfiBindings ffi_bindings = GenerateBindingsImpl(MakeFfiU8Slice(json));
  Bindings bindings = MakeBindingsFromFfiBindings(ffi_bindings);
  FreeFfiBindings(ffi_bindings);
  return bindings;
}

}  // namespace rs_bindings_from_cc
