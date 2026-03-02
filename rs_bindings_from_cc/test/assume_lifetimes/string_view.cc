// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/assume_lifetimes/string_view.h"

void string_view_sink(std::string_view s) {}
std::string_view string_view_return(std::string_view s) { return s; }
void explicit_lifetime_string_view(std::string_view x) {}
std::string_view ambiguous_string_view_return(std::string_view a,
                                              std::string_view b) {
  return a;
}
std::string_view unambiguous_string_view_return_annotated(std::string_view x,
                                                          std::string_view y) {
  return x;
}
