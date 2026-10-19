// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_HEADER_IN_MULTIPLE_TARGETS_USES_SHARED_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_HEADER_IN_MULTIPLE_TARGETS_USES_SHARED_H_

// Note: this header deliberately includes `shared.h` only indirectly, through
// `user_of_shared.h`, which belongs to another target. When header modules are
// used, that makes the declarations of `shared.h` reachable from the
// precompiled module of `:shared`, in addition to `shared.h` itself being
// parsed as a header of `:lib`.
#include "rs_bindings_from_cc/test/bazel/header_in_multiple_targets/user_of_shared.h"  // IWYU pragma: keep

namespace lib_ns {

inline int LibFunction(int x) { return x + 1; }

}  // namespace lib_ns

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_HEADER_IN_MULTIPLE_TARGETS_USES_SHARED_H_
