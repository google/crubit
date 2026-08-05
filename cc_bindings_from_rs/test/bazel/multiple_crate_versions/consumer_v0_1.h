// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BAZEL_MULTIPLE_CRATE_VERSIONS_CONSUMER_V0_1_H_
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BAZEL_MULTIPLE_CRATE_VERSIONS_CONSUMER_V0_1_H_

#include <string>

namespace consumer_v0_1 {
std::string GetV0_1String();
std::string GetV0_1FreeFunction();
std::string GetV0_1Method();
std::string GetV0_1AssocFunction();
std::string GetV0_1Clone();
}  // namespace consumer_v0_1

#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BAZEL_MULTIPLE_CRATE_VERSIONS_CONSUMER_V0_1_H_
