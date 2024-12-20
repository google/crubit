// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_CONVERSION_FUNCTION_HELPERS_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_CONVERSION_FUNCTION_HELPERS_H_

#include <string>

namespace crubit_cc_std_internal::conversion_function_helpers {

// std::string helpers start
inline size_t StringGetSize(const void* s) {
  return reinterpret_cast<const std::string*>(s)->size();
}

inline const char* StringGetData(const void* s) {
  return reinterpret_cast<const std::string*>(s)->data();
}

inline void StringCreateInPlace(void* s, const char* str, size_t len) {
  new (reinterpret_cast<std::string*>(s)) std::string(str, len);
}
// std::string helpers end

}  // namespace crubit_cc_std_internal::conversion_function_helpers

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_CONVERSION_FUNCTION_HELPERS_H_
