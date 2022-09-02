// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_INLINE_HELLO_WORLD_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_INLINE_HELLO_WORLD_H_

#pragma clang lifetime_elision

inline int hello_world_inline() { return 42; }

// This testcase helps verify that thunks correctly work with const-ref
// parameters. Using an 'inline' method forces generation of a C++ thunk.
struct SomeStruct final {
  int int_field;
};
inline int take_struct_by_const_ref(const SomeStruct& s) { return s.int_field; }

// This testcase helps verify that thunks correctly work with primitive types
// that have multi-word type names (e.g. `unsigned int`). Using an 'inline'
// method forces generation of a C++ thunk.
inline unsigned int double_unsigned_int(unsigned int i) { return 2 * i; }

// This is a regression test for b/244630626.  This mimics the standard library
// that forward-declares and then defines an inline `get_id` function in a
// `this_thread` namespace.
namespace foo {
inline int forward_declared_doubler(int x);
}  // namespace foo
namespace foo {
inline int forward_declared_doubler(int x) { return x * 2; }
}  // namespace foo

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_INLINE_HELLO_WORLD_H_
