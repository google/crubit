// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/forward_declaration/basic_references/forward_declaration.h"

struct A {};
A g_a;

A& fwd_source() { return g_a; }
A& fwd_ident(A& a) { return a; }
const A& fwd_ident_const(const A& a) { return a; }
