// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_MULTIPLE_TARGETS_DEPENDENCY_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_MULTIPLE_TARGETS_DEPENDENCY_H_

#pragma clang lifetime_elision

struct Dependency final {
  int magic;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_MULTIPLE_TARGETS_DEPENDENCY_H_
