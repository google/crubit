// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BAZEL_MULTIPLE_CRATE_VERSIONS_CONSUMER_V2_H_
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BAZEL_MULTIPLE_CRATE_VERSIONS_CONSUMER_V2_H_

#include <string>

// Version 2 of the consumer of the Rust crate.
namespace consumer_v2 {

// Call Rust to_string implementation to trigger linking Display into the final
// binary.
std::string GetV2String();

// Call a Rust free function to trigger linking it into the final binary.
std::string GetV2FreeFunction();

// Call a Rust method to trigger linking it into the final binary.
std::string GetV2Method();

// Call a Rust associated function to trigger linking it into the final binary.
std::string GetV2AssocFunction();

// Call a Rust trait function, clone, to trigger linking it into the final
// binary.
std::string GetV2Clone();

}  // namespace consumer_v2

#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BAZEL_MULTIPLE_CRATE_VERSIONS_CONSUMER_V2_H_
