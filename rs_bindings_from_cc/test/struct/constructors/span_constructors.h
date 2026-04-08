// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_CONSTRUCTORS_SPAN_CONSTRUCTORS_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_CONSTRUCTORS_SPAN_CONSTRUCTORS_H_

#include "absl/types/span.h"

struct MyStruct {
  explicit MyStruct(absl::Span<const int> values) {}
  ~MyStruct() {
    // Nontrivial so that the constructor must return an `impl Ctor`.
  }
};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_CONSTRUCTORS_SPAN_CONSTRUCTORS_H_
