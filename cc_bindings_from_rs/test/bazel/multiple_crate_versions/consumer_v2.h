// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BAZEL_MULTIPLE_CRATE_VERSIONS_CONSUMER_V2_H_
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BAZEL_MULTIPLE_CRATE_VERSIONS_CONSUMER_V2_H_

#include <string>

namespace consumer_v2 {
std::string GetV2String();
std::string GetV2FreeFunction();
std::string GetV2Method();
std::string GetV2AssocFunction();
std::string GetV2Clone();
}  // namespace consumer_v2

#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BAZEL_MULTIPLE_CRATE_VERSIONS_CONSUMER_V2_H_
