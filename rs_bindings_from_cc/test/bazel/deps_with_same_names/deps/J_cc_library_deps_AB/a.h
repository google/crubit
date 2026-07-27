// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_CC_LIBRARY_13_DEPS_12_A_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_CC_LIBRARY_13_DEPS_12_A_H_

#include "rs_bindings_from_cc/test/bazel/deps_with_same_names/deps/A_cc_library/a.h"
#include "rs_bindings_from_cc/test/bazel/deps_with_same_names/deps/B_cc_library/a.h"

inline int f13() { return f1() + f2(); }

struct J_Struct {
  int value;
};

inline int f13_struct(J_Struct j) { return f13() + j.value; }

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_CC_LIBRARY_13_DEPS_12_A_H_
