// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_HEADER_IN_MULTIPLE_TARGETS_USER_OF_SHARED_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_HEADER_IN_MULTIPLE_TARGETS_USER_OF_SHARED_H_

#include "rs_bindings_from_cc/test/bazel/header_in_multiple_targets/shared.h"

namespace user_ns {

inline int UseInfo(const shared_ns::Info& info) { return info.value(); }

}  // namespace user_ns

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_HEADER_IN_MULTIPLE_TARGETS_USER_OF_SHARED_H_
