// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/noescape_pointers/noescape_pointers.h"
int takes_noescape_nonnull(int* _Nonnull __attribute__((noescape)) p) {
  *p += 1;
  return *p;
}
