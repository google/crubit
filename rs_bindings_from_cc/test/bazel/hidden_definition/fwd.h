// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_HIDDEN_DEFINITION_FWD_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_HIDDEN_DEFINITION_FWD_H_

// Note: this header deliberately does not include `hidden.h`.
namespace hidden_ns {
struct Holder;
}  // namespace hidden_ns

namespace fwd_ns {

inline int AcceptsHolder(hidden_ns::Holder* holder) {
  return holder == nullptr ? 1 : 2;
}

}  // namespace fwd_ns

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_HIDDEN_DEFINITION_FWD_H_
