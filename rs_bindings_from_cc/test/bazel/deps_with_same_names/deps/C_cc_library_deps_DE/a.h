// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_CC_LIBRARY_3_DEPS_45_A_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_CC_LIBRARY_3_DEPS_45_A_H_

#include "rs_bindings_from_cc/test/bazel/deps_with_same_names/deps/D_cc_library_deps_F/a.h"
#include "rs_bindings_from_cc/test/bazel/deps_with_same_names/deps/E_cc_library_deps_F/a.h"

inline int f3(D_Struct d, E_Struct e) { return d.s.value + e.s.value; }

inline int f3_helper() { return f3(f4(), f5()); }

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_CC_LIBRARY_3_DEPS_45_A_H_
