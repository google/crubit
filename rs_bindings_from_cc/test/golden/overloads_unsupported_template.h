// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_OVERLOADS_UNSUPPORTED_TEMPLATE_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_OVERLOADS_UNSUPPORTED_TEMPLATE_H_

// Tests that no bindings are generated when an overload set includes
// any unsupported items.
//
// See http://b/251045039

inline void Overload() {}

template <typename T>
void Overload(T) {}

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_OVERLOADS_UNSUPPORTED_TEMPLATE_H_
