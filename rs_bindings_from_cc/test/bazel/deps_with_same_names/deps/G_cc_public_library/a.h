// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_CC_PUBLIC_LIBRARY_10_A_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_CC_PUBLIC_LIBRARY_10_A_H_

#include "rs_bindings_from_cc/test/bazel/deps_with_same_names/deps/G_cc_public_library/private/a.h"

inline int f10() { return f10_private(); }

struct G_Struct {
  int x;
};

inline int f10_struct(G_Struct g) { return f10_private() + g.x; }

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_CC_PUBLIC_LIBRARY_10_A_H_
