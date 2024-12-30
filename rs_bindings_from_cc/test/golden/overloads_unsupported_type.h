// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_OVERLOADS_UNSUPPORTED_TYPE_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_OVERLOADS_UNSUPPORTED_TYPE_H_

// Tests that no bindings are generated when an overload set includes
// any unsupported items.
//
// See http://b/251045039

class SomeClass {};

inline void Overload() {}
// Note: rvalue reference is not supported
inline void Overload(SomeClass&&) {}

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_OVERLOADS_UNSUPPORTED_TYPE_H_
