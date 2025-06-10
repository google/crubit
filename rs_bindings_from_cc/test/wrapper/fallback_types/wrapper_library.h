// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_WRAPPER_FALLBACK_TYPES_WRAPPER_LIBRARY_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_WRAPPER_FALLBACK_TYPES_WRAPPER_LIBRARY_H_

#include "rs_bindings_from_cc/test/wrapper/fallback_types/unsupported_library.h"

inline UnsupportedType& GetGlobalUnsupportedType() {
  static UnsupportedType x{0};
  return x;
}

inline void SetValue(UnsupportedType& x, int value) { x.value = value; }

inline int GetValue(const UnsupportedType& x) { return x.value; }

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_WRAPPER_FALLBACK_TYPES_WRAPPER_LIBRARY_H_
