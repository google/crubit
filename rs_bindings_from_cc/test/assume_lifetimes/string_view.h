// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ASSUME_LIFETIMES_STRING_VIEW_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ASSUME_LIFETIMES_STRING_VIEW_H_

#include <string_view>

#define $a [[clang::annotate_type("lifetime", "a")]]

void string_view_sink(std::string_view s);
std::string_view string_view_return(std::string_view s);
std::string_view ambiguous_string_view_return(std::string_view a,
                                              std::string_view b);
void explicit_lifetime_string_view(std::string_view $a x);
std::string_view unambiguous_string_view_return_annotated(
    std::string_view $a x, std::string_view $a y);

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ASSUME_LIFETIMES_STRING_VIEW_H_
