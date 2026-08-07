// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_BRIDGE_TYPES_CLIENT_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_BRIDGE_TYPES_CLIENT_H_

#include "rs_bindings_from_cc/test/bazel/deps_with_same_names/bridge_types/deps/A_cc_library/a.h"
#include "rs_bindings_from_cc/test/bazel/deps_with_same_names/bridge_types/deps/B_cc_library/a.h"
#include "rs_bindings_from_cc/test/bazel/deps_with_same_names/bridge_types/key_lib.h"

inline BridgeA client_make_bridge_a(A_Struct a) { return make_bridge_a(a); }

inline BridgeB client_make_bridge_b(B_Struct b) { return make_bridge_b(b); }

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_BRIDGE_TYPES_CLIENT_H_
