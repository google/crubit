// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BAZEL_MULTIPLE_CRATE_VERSIONS_CONSUMER_V1_H_
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BAZEL_MULTIPLE_CRATE_VERSIONS_CONSUMER_V1_H_

#include <string>

// Version 1 of the consumer of the Rust crate.
namespace consumer_v1 {

// Call Rust to_string implementation to trigger linking Display into the final
// binary.
std::string GetV1String();

// Call a Rust free function to trigger linking it into the final binary.
std::string GetV1FreeFunction();

// Call a Rust method to trigger linking it into the final binary.
std::string GetV1Method();

// Call a Rust associated function to trigger linking it into the final
// binary.
std::string GetV1AssocFunction();

// Call a Rust trait function, clone, to trigger linking it into the final
// binary.
std::string GetV1Clone();

}  // namespace consumer_v1

#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BAZEL_MULTIPLE_CRATE_VERSIONS_CONSUMER_V1_H_
