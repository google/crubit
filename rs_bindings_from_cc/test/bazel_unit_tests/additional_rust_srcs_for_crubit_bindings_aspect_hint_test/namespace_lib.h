// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_UNIT_TESTS_ADDITIONAL_RUST_SRCS_FOR_CRUBIT_BINDINGS_ASPECT_HINT_TEST_NAMESPACE_LIB_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_UNIT_TESTS_ADDITIONAL_RUST_SRCS_FOR_CRUBIT_BINDINGS_ASPECT_HINT_TEST_NAMESPACE_LIB_H_

namespace a {
// Open a.
namespace b {
inline int k() { return 42; }
}  // namespace b
}  // namespace a

namespace a {
// Reopen a.
namespace b {
namespace c {
namespace a {
namespace b {
namespace c {
inline int g() { return 53; }
}  // namespace c
}  // namespace b
}  // namespace a
}  // namespace c
}  // namespace b
}  // namespace a

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_UNIT_TESTS_ADDITIONAL_RUST_SRCS_FOR_CRUBIT_BINDINGS_ASPECT_HINT_TEST_NAMESPACE_LIB_H_
