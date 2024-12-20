// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BRIDGING_STRING_TEST_LIB_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BRIDGING_STRING_TEST_LIB_H_
#include <cstddef>
#include <string>
inline int GetStringSize(std::string s) { return s.size(); }
inline std::string CreateString(const void* ptr, size_t size) {
  return std::string(static_cast<const char*>(ptr), size);
}

inline std::basic_string<char> CreateStringAsBasicString(const void* ptr,
                                                         size_t size) {
  return std::basic_string<char>(static_cast<const char*>(ptr), size);
}

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BRIDGING_STRING_TEST_LIB_H_
