// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ALLOWLIST_TEMPLATE_INSTANSIATION_STRING_VIEW_TEST_LIB_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ALLOWLIST_TEMPLATE_INSTANSIATION_STRING_VIEW_TEST_LIB_H_

#include <string_view>

inline std::string_view PopACharFromStringView(std::string_view s) {
  return s.substr(1);
}

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ALLOWLIST_TEMPLATE_INSTANSIATION_STRING_VIEW_TEST_LIB_H_
