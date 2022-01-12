// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/struct/methods/elided_lifetimes.h"

int ElidedLifetimes::get_int_field() const { return int_field; }

void ElidedLifetimes::set_int_field(int new_value) { int_field = new_value; }
