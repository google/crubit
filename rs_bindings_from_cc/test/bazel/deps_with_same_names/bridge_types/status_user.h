// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_BRIDGE_TYPES_STATUS_USER_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_BRIDGE_TYPES_STATUS_USER_H_

#include "absl/status/statusor.h"

inline absl::StatusOr<int> make_status_or(int x) {
  if (x > 0) {
    return x;
  }
  return absl::InvalidArgumentError("must be positive");
}

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_DEPS_WITH_SAME_NAMES_BRIDGE_TYPES_STATUS_USER_H_
