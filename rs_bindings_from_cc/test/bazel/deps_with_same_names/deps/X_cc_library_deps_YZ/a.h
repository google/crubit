// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_CC_LIBRARY_7_DEPS_89_A_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_CC_LIBRARY_7_DEPS_89_A_H_

#include "rs_bindings_from_cc/test/bazel/deps_with_same_names/deps/Y_proto_library/proto.pb.h"
#include "rs_bindings_from_cc/test/bazel/deps_with_same_names/deps/Z_proto_library/proto.pb.h"

inline int f7(crubit::test::deps_with_same_names::proto8::M8 m8,
              crubit::test::deps_with_same_names::proto9::M9 m9) {
  return m8.value() + m9.value();
}

inline int f7_simple() {
  crubit::test::deps_with_same_names::proto8::M8 m8;
  crubit::test::deps_with_same_names::proto9::M9 m9;
  m8.set_value(8);
  m9.set_value(9);
  return f7(m8, m9);
}

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_CC_LIBRARY_7_DEPS_89_A_H_
// Force rebuild comment
