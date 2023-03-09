// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/struct/methods_qualifiers/methods_qualifiers.h"

void ClassWithQualifiedMethods::increment_i() { i += 1; }
int ClassWithQualifiedMethods::unqualified_get_i() { return i; }
int ClassWithQualifiedMethods::const_qualified_get_i() const { return i; }
int ClassWithQualifiedMethods::lvalue_ref_qualified_get_i() & { return i; }
int ClassWithQualifiedMethods::const_lvalue_ref_qualified_get_i() const& {
  return i;
}
int ClassWithQualifiedMethods::rvalue_ref_qualified_get_i() && { return i; }
int ClassWithQualifiedMethods::const_rvalue_ref_qualified_get_i() const&& {
  return i;
}
