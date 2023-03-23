// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/struct/methods_qualifiers/methods_qualifiers.h"

void UnpinStructWithRefQualifiedMethods::increment_i() { i += 1; }
int UnpinStructWithRefQualifiedMethods::unqualified_get_i() { return i; }
int UnpinStructWithRefQualifiedMethods::const_qualified_get_i() const {
  return i;
}
int UnpinStructWithRefQualifiedMethods::lvalue_ref_qualified_get_i() & {
  return i;
}
int UnpinStructWithRefQualifiedMethods::const_lvalue_ref_qualified_get_i()
    const& {
  return i;
}
int UnpinStructWithRefQualifiedMethods::rvalue_ref_qualified_get_i() && {
  return i;
}
int UnpinStructWithRefQualifiedMethods::const_rvalue_ref_qualified_get_i()
    const&& {
  return i;
}