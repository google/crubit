// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BAZEL_MULTIPLE_CRATE_VERSIONS_CONSUMER_V1_H_
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BAZEL_MULTIPLE_CRATE_VERSIONS_CONSUMER_V1_H_

#include <string>

namespace consumer_v1 {
std::string GetV1String();
std::string GetV1FreeFunction();
std::string GetV1Method();
std::string GetV1AssocFunction();
std::string GetV1Clone();
}  // namespace consumer_v1

#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BAZEL_MULTIPLE_CRATE_VERSIONS_CONSUMER_V1_H_
