// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_METHODS_METHODS_QUALIFIERS_METHODS_QUALIFIERS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_METHODS_METHODS_QUALIFIERS_METHODS_QUALIFIERS_H_

#pragma clang lifetime_elision

struct UnpinStructWithRefQualifiedMethods final {
  int i;
  void increment_i();
  int unqualified_get_i();
  int const_qualified_get_i() const;
  int lvalue_ref_qualified_get_i() &;
  int const_lvalue_ref_qualified_get_i() const&;
  int rvalue_ref_qualified_get_i() &&;
  int const_rvalue_ref_qualified_get_i() const&&;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_METHODS_METHODS_QUALIFIERS_METHODS_QUALIFIERS_H_
