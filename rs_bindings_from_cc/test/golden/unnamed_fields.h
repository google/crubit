// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_UNNAMED_FIELDS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_UNNAMED_FIELDS_H_

struct WithUnnamedFields {
  int foo;
  int : 32;
  int bar;
  int : 3;
  int baz;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_UNNAMED_FIELDS_H_
