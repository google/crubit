// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_CC_LIBRARY_4_DEPS_6_A_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_CC_LIBRARY_4_DEPS_6_A_H_

#include "rs_bindings_from_cc/test/bazel/deps_with_same_names/deps/F_cc_library/a.h"

struct D_Struct {
  S6 s;
};

inline D_Struct f4() { return D_Struct{make_s6()}; }

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_CC_LIBRARY_4_DEPS_6_A_H_
