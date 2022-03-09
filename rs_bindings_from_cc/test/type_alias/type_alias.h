// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TYPE_ALIAS_TYPE_ALIAS_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TYPE_ALIAS_TYPE_ALIAS_H_

#pragma clang lifetime_elision

using Int = int;

// Use `inline` to force a thunk to be generated so we can test that it
// compiles.
inline int return_underlying(Int i) { return i; }

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TYPE_ALIAS_TYPE_ALIAS_H_
