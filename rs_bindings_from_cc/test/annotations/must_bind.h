// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ANNOTATIONS_MUST_BIND_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ANNOTATIONS_MUST_BIND_H_

#include "support/annotations.h"

namespace crubit::test {

// Uncomment this template declaration in order to observe the build-time error
// during bindings generation. There should be no attempt to compile the
// dependent Rust code.
//
// TODO: b/402989591 - Use compile-fail UI test to check these outputs.
//

// template<typename T>
CRUBIT_MUST_BIND inline void MyFn() {}

// template<typename T>
struct CRUBIT_MUST_BIND MyStruct {};

}  // namespace crubit::test

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ANNOTATIONS_MUST_BIND_H_
