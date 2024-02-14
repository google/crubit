// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_EXAMPLES_CPP_TRIVIAL_ABI_STRUCT_EXAMPLE_H_
#define THIRD_PARTY_CRUBIT_EXAMPLES_CPP_TRIVIAL_ABI_STRUCT_EXAMPLE_H_

#include "absl/base/attributes.h"

// Because this class has a destructor, it will not receive Rust bindings
// without ABSL_ATTRIBUTE_TRIVIAL_ABI.
struct ABSL_ATTRIBUTE_TRIVIAL_ABI Position {
  int x;
  int y;

  ~Position() { x = y = 0; }
};

#endif  // THIRD_PARTY_CRUBIT_EXAMPLES_CPP_TRIVIAL_ABI_STRUCT_EXAMPLE_H_
