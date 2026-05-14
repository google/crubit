// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// This file contains definitions for a very simple named implicit template
// instantiation test.

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_NAMED_INSTANTIATION_NAMED_INSTANTIATION_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_NAMED_INSTANTIATION_NAMED_INSTANTIATION_H_

template <typename T, typename S>
struct Ni {
  Ni(T t, S s) {}
};

using NiIF = Ni<int, float>;
void SomeApi(const NiIF& i);

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_NAMED_INSTANTIATION_NAMED_INSTANTIATION_H_
