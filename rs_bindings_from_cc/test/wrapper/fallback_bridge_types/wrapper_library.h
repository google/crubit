// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_WRAPPER_FALLBACK_TYPES_WRAPPER_LIBRARY_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_WRAPPER_FALLBACK_TYPES_WRAPPER_LIBRARY_H_

#include <string>

inline const std::string& GetGlobalString() {
  static std::string* x = new std::string("hello, world!");
  return *x;
}

inline std::string CopyString(const std::string& x) { return x; }

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_WRAPPER_FALLBACK_TYPES_WRAPPER_LIBRARY_H_
