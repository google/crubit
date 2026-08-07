// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_BRIDGE_TYPES_KEY_LIB_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_BRIDGE_TYPES_KEY_LIB_H_

#include "rs_bindings_from_cc/test/bazel/deps_with_same_names/bridge_types/deps/A_cc_library/a.h"
#include "rs_bindings_from_cc/test/bazel/deps_with_same_names/bridge_types/deps/B_cc_library/a.h"
#include "support/annotations.h"

// We bridge BridgeA to a::TypeA defined in A_cc_library's Rust crate.
// The hint prefix 'a' will be replaced by the mangled crate name of
// A_cc_library:a.
struct CRUBIT_BRIDGE("::a::TypeA", "::a::TypeAAbi", "::a::TypeAAbi",
                     "//rs_bindings_from_cc/test/bazel/"
                     "deps_with_same_names/bridge_types/deps/A_cc_library:a")
    BridgeA {
  A_Struct a;
};

// We bridge BridgeB to a::TypeB defined in B_cc_library's Rust crate.
// The hint prefix 'a' will be replaced by the mangled crate name of
// B_cc_library:a.
struct CRUBIT_BRIDGE("::a::TypeB", "::a::TypeBAbi", "::a::TypeBAbi",
                     "//rs_bindings_from_cc/test/bazel/"
                     "deps_with_same_names/bridge_types/deps/B_cc_library:a")
    BridgeB {
  B_Struct b;
};

inline BridgeA make_bridge_a(A_Struct a) { return {a}; }
inline BridgeB make_bridge_b(B_Struct b) { return {b}; }

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_BRIDGE_TYPES_KEY_LIB_H_
