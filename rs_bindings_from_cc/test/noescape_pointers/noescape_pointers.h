// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wnullability-extension"
int takes_noescape_nonnull(int* _Nonnull __attribute__((noescape)) p);
#pragma clang diagnostic pop
