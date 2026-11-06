// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/golden/noescape_pointers.h"

void takes_noescape_nonnull(int* _Nonnull __attribute__((noescape)) p) {}
void takes_noescape_nullable(int* _Nullable __attribute__((noescape)) p) {}
void takes_nonnull_only(int* _Nonnull p) {}
void takes_noescape_void(void* _Nonnull __attribute__((noescape)) p) {}
void takes_noescape_incomplete(Incomplete* _Nonnull
                               __attribute__((noescape)) p) {}
void takes_noescape_pointer_to_pointer(int** _Nonnull
                                       __attribute__((noescape)) p) {}
void takes_noescape_const_pointee(const int* _Nonnull
                                  __attribute__((noescape)) p) {}
