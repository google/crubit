// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_INTERNAL_RUST_TYPE_KEY_LIB_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_INTERNAL_RUST_TYPE_KEY_LIB_H_

#include "rs_bindings_from_cc/test/bazel/deps_with_same_names/internal_rust_type/deps/A_cc_library/a.h"
#include "rs_bindings_from_cc/test/bazel/deps_with_same_names/internal_rust_type/deps/B_cc_library/a.h"
#include "support/annotations.h"

// Map InternalTypeA to ::a::TypeA defined in A_cc_library's Rust crate.
// The hint prefix 'a' will be replaced by the mangled crate name of
// A_cc_library:a.
struct CRUBIT_INTERNAL_RUST_TYPE("::a::TypeA")
    CRUBIT_INTERNAL_RUST_TYPE_LABEL_HINT(
        "//rs_bindings_from_cc/test/bazel/"
        "deps_with_same_names/internal_rust_type/deps/A_cc_library:a")
        InternalTypeA {
  A_Struct a;
};

// Map InternalTypeB to ::a::TypeB defined in B_cc_library's Rust crate.
// The hint prefix 'a' will be replaced by the mangled crate name of
// B_cc_library:a.
struct CRUBIT_INTERNAL_RUST_TYPE("::a::TypeB")
    CRUBIT_INTERNAL_RUST_TYPE_LABEL_HINT(
        "//rs_bindings_from_cc/test/bazel/"
        "deps_with_same_names/internal_rust_type/deps/B_cc_library:a")
        InternalTypeB {
  B_Struct b;
};

inline InternalTypeA make_type_a(A_Struct a) { return {a}; }
inline InternalTypeB make_type_b(B_Struct b) { return {b}; }

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_INTERNAL_RUST_TYPE_KEY_LIB_H_
