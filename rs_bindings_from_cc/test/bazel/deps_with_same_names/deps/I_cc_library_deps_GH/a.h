// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_CC_LIBRARY_12_DEPS_1011_A_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_CC_LIBRARY_12_DEPS_1011_A_H_

#include "rs_bindings_from_cc/test/bazel/deps_with_same_names/deps/G_cc_public_library/a.h"
#include "rs_bindings_from_cc/test/bazel/deps_with_same_names/deps/H_cc_public_library/a.h"

inline int f12() { return f10() + f11(); }

inline int f12_struct(G_Struct g, H_Struct h) {
  return f10_struct(g) + f11_struct(h);
}

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_CC_LIBRARY_12_DEPS_1011_A_H_
