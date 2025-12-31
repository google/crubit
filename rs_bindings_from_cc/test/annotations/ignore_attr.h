// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ANNOTATIONS_IGNORE_ATTR_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ANNOTATIONS_IGNORE_ATTR_H_

#include "support/annotations.h"

namespace crubit::test {

// This struct would not have bindings generated without the ignore annotation.
// `gnu::abi_tag` is an arbitrarily selected attribute that Crubit doesn't
// handle.
struct CRUBIT_UNSAFE_IGNORE_ATTR("gnu::abi_tag") [[gnu::abi_tag("foo")]]
MyStruct {};

}  // namespace crubit::test

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ANNOTATIONS_IGNORE_ATTR_H_
