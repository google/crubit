// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_INTERNAL_RUST_TYPE_CLIENT_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_INTERNAL_RUST_TYPE_CLIENT_H_

#include "rs_bindings_from_cc/test/bazel/deps_with_same_names/internal_rust_type/deps/A_cc_library/a.h"
#include "rs_bindings_from_cc/test/bazel/deps_with_same_names/internal_rust_type/deps/B_cc_library/a.h"
#include "rs_bindings_from_cc/test/bazel/deps_with_same_names/internal_rust_type/key_lib.h"

inline InternalTypeA client_make_type_a(A_Struct a) { return make_type_a(a); }
inline InternalTypeB client_make_type_b(B_Struct b) { return make_type_b(b); }

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_INTERNAL_RUST_TYPE_CLIENT_H_
