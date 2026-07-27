// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_INDIRECT_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_INDIRECT_H_

#include "rs_bindings_from_cc/test/bazel/deps_with_same_names/deps/C_cc_library_deps_DE/a.h"
#include "rs_bindings_from_cc/test/bazel/deps_with_same_names/deps/D_cc_library_deps_F/a.h"

inline int f_indirect() { return f3_helper(); }

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_INDIRECT_H_
