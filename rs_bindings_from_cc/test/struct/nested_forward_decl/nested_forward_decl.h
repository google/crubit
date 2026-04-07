// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_NESTED_FORWARD_DECL_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_NESTED_FORWARD_DECL_H_

struct OuterType {
  struct Inner;
};

struct OuterType::Inner {
  int x;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_NESTED_FORWARD_DECL_H_
