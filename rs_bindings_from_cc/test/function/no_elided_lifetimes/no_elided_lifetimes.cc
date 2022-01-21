// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/function/no_elided_lifetimes/no_elided_lifetimes.h"

namespace {

const int* g_int_ptr = nullptr;

}  // namespace

void StorePointer(const int& int_ref) { g_int_ptr = &int_ref; }

int ReadStoredPointer() { return *g_int_ptr; }
