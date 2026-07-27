// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_PUBLIC_DEPS_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_PUBLIC_DEPS_H_

#include "rs_bindings_from_cc/test/bazel/deps_with_same_names/deps/I_cc_library_deps_GH/a.h"

inline int f_public() { return f12(); }

inline int f_public_struct(G_Struct g, H_Struct h) { return f12_struct(g, h); }

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_PUBLIC_DEPS_H_
