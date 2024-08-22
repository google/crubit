// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BRIDGE_BIRDGE_LIB_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BRIDGE_BIRDGE_LIB_H_
#include <cstddef>
#include <string>
#include <utility>

#include "support/internal/attribute_macros.h"

struct CRUBIT_INTERNAL_BRIDGE_SUPPORT("MyRustStruct", "rust_to_cpp_converter",
                                      "cpp_to_rust_converter") CppStruct {
  std::string s;

  explicit CppStruct(std::string s) : s(std::move(s)) {}
};

inline size_t CalStructSize(CppStruct x) { return x.s.size(); }
inline CppStruct ReturnHelloWorldStruct() { return CppStruct{"hello world"}; }

inline CppStruct PadADot(CppStruct x) {
  x.s += ".";
  return x;
}

inline CppStruct Concat(CppStruct x, CppStruct y) {
  x.s += y.s;
  return x;
}

inline void ffi_create_my_cpp_struct(const char* s, size_t len, void* output) {
  new (output) CppStruct(std::string(s, len));
}

inline const char* ffi_get_buffer(const void* input) {
  return reinterpret_cast<const CppStruct*>(input)->s.c_str();
}

inline size_t ffi_get_buffer_len(const void* input) {
  return reinterpret_cast<const CppStruct*>(input)->s.size();
}

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BRIDGE_BIRDGE_LIB_H_
