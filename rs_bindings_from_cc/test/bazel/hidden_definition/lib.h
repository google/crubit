// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_HIDDEN_DEFINITION_LIB_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_HIDDEN_DEFINITION_LIB_H_

#include "rs_bindings_from_cc/test/bazel/hidden_definition/fwd.h"

// This header doesn't actually include `hidden.h`, which defines
// `hidden_ns::Holder`. But the include scanner doesn't evaluate preprocessor
// conditions, so when header modules are used, the precompiled module of
// `:hidden` is loaded anyway. Its declarations are then reachable, but not
// visible.
#if 0
#include "rs_bindings_from_cc/test/bazel/hidden_definition/hidden.h"
#endif

namespace lib_ns {

// Like `fwd_ns::AcceptsHolder`, this is not bound, because `hidden_ns::Holder`
// is incomplete here.
inline int AcceptsHolder(hidden_ns::Holder* holder) {
  return fwd_ns::AcceptsHolder(holder);
}

inline int ReturnsOne() { return 1; }

}  // namespace lib_ns

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_HIDDEN_DEFINITION_LIB_H_
