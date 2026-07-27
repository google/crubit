// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_MIXED_DEPS_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_MIXED_DEPS_H_

#include "rs_bindings_from_cc/test/bazel/deps_with_same_names/deps/J_cc_library_deps_AB/a.h"

inline int f_mixed() { return f13(); }

inline int f_mixed_struct(J_Struct j) { return f13_struct(j); }

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_MIXED_DEPS_H_
