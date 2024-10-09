// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstdint>
#include <iostream>

// The generated bindings are in a header at the same path as the
// `example_crate` rust_library, with a `.h` suffix.
#include "examples/rust/function/example_crate.h"

int main(int argc, char* argv[]) {
  // The generated bindings are in a namespace with the same name as the
  // target crate:
  int32_t sum = example_crate::add_two_integers(2, 2);
  std::cout << "sum = " << sum << std::endl;
  return 0;
}
