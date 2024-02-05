// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_TYPE_ALIAS_TYPE_ALIAS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_TYPE_ALIAS_TYPE_ALIAS_H_

#pragma clang lifetime_elision

namespace ns {
using Int = int;
}
using ns::Int;

// Use `inline` to force a thunk to be generated so we can test that it
// compiles.
inline int return_underlying(Int i) { return i; }

// This one, however, should NOT receive bindings.
using MyVector __attribute__((vector_size(16))) = int;
inline void VectorFunction(MyVector v) {}

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_TYPE_ALIAS_TYPE_ALIAS_H_
