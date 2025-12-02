// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TRY_FORCE_TEMPLATE_INST_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TRY_FORCE_TEMPLATE_INST_H_

template <typename T>
struct Foo {};

template <typename T>
struct Bar {};

// Should fail because Foo<Bar<int>> is a template instantiation and needs
// wrapper mode, not because Bar<int> is an incomplete type (it's right there!)
void Baz(Foo<Bar<int>>);

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TRY_FORCE_TEMPLATE_INST_H_
