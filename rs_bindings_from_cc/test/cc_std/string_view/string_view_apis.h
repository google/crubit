// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CC_STD_STRING_VIEW_STRING_VIEW_APIS_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CC_STD_STRING_VIEW_STRING_VIEW_APIS_H_

#include <string_view>
namespace crubit_string_view {

inline std::string_view GetHelloWorld() { return "Hello, world!"; }

inline std::string_view GetInvalidUtf8() { return "Not a UTF-8 byte: \xff"; }

}  // namespace crubit_string_view

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CC_STD_STRING_VIEW_STRING_VIEW_APIS_H_
