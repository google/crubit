// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_INLINE_HELLO_WORLD_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_INLINE_HELLO_WORLD_H_

inline int hello_world_inline() { return 42; }

// This testcase helps verify that thunks correctly work with const-ref
// parameters. Using an 'inline' method forces generation of a C++ thunk.
struct SomeStruct {
  int int_field;
};
inline int take_struct_by_const_ref(const SomeStruct& s) { return s.int_field; }

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_INLINE_HELLO_WORLD_H_
