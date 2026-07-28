// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
// Force rebuild aspect

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_PROTO_DEPS_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_PROTO_DEPS_H_

#include "rs_bindings_from_cc/test/bazel/deps_with_same_names/deps/X_cc_library_deps_YZ/a.h"

inline int f_proto() { return f7_simple(); }

inline int f_proto_indirect(crubit::test::deps_with_same_names::proto8::M8 m8,
                            crubit::test::deps_with_same_names::proto9::M9 m9) {
  return f7(m8, m9);
}

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_PROTO_DEPS_H_
