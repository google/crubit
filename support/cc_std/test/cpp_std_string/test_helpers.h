// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_TEST_CPP_STD_STRING_CPP_STD_STRING_TEST_LIB_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_TEST_CPP_STD_STRING_CPP_STD_STRING_TEST_LIB_H_

#include "support/cc_std/cpp_std_string.h"

namespace cpp_std_string_test {

inline StdString RoundTrip(StdString s) {
  return StdString::FromStringView(s.value());
}

}  // namespace cpp_std_string_test

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_CC_STD_TEST_CPP_STD_STRING_CPP_STD_STRING_TEST_LIB_H_
