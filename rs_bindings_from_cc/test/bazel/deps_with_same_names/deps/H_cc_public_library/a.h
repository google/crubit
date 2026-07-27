// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_CC_PUBLIC_LIBRARY_11_A_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_CC_PUBLIC_LIBRARY_11_A_H_

#include "rs_bindings_from_cc/test/bazel/deps_with_same_names/deps/H_cc_public_library/private/a.h"

inline int f11() { return f11_private(); }

struct H_Struct {
  int x;
};

inline int f11_struct(H_Struct h) { return f11_private() + h.x; }

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_CC_PUBLIC_LIBRARY_11_A_H_
