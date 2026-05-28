// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_BLOCKLIST_TEMPLATE_PATHS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_BLOCKLIST_TEMPLATE_PATHS_H_

template <typename T>
struct TS {
  void f();
};

TS<int> RTS();

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_BLOCKLIST_TEMPLATE_PATHS_H_
