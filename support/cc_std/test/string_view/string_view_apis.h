// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_CC_STD_STRING_VIEW_STRING_VIEW_APIS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_CC_STD_STRING_VIEW_STRING_VIEW_APIS_H_

#include <string_view>
namespace crubit_string_view {

inline std::string_view GetHelloWorld() { return "Hello, world!"; }

inline std::string_view GetDefault() { return std::string_view(); }

}  // namespace crubit_string_view

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_CC_STD_STRING_VIEW_STRING_VIEW_APIS_H_
