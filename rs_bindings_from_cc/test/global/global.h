// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_EXTERN_BASIC_EXTERN_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_EXTERN_BASIC_EXTERN_H_

extern int extern_int;
extern const int kExternConstInt;

// Check that duplicate extern declarations are handled correctly.
extern int extern_int;

namespace foo {
extern int extern_int_namespaced;
extern "C" int extern_c_int_namespaced;
}  // namespace foo

// Make sure we don't choke on these, even though they aren't supported yet.
extern const inline int kInlineConstInt = 6;
constexpr int kConstexprInt = 7;
inline int inline_int = 5;
namespace foo {
inline int inline_int_namespaced = 5;
}  // namespace foo

// Also make sure we don't choke on templated variables.
template <typename T>
T templated_variable = {};
// instantiate templated_variable<int>
inline void Unused(int arg = templated_variable<int>) {}

int GetIntVal();
int GetNamespacedIntVal();
int GetCNamespacedIntVal();
int GetInlineIntVal();

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_EXTERN_BASIC_EXTERN_H_
