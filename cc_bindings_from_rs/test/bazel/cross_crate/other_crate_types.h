// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BAZEL_CROSS_CRATE_OTHER_CRATE_TYPES_H_
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BAZEL_CROSS_CRATE_OTHER_CRATE_TYPES_H_

namespace crubit {
namespace test {

struct MyStatus {
  bool ok;
};

template <typename T>
struct MyStatusOr {
  bool has_value;
  T value;
};

}  // namespace test
}  // namespace crubit

#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BAZEL_CROSS_CRATE_OTHER_CRATE_TYPES_H_
