// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/golden/noescape_pointers_feature_off.h"

void takes_noescape_nonnull_feature_off(int* _Nonnull
                                        __attribute__((noescape)) p) {}
