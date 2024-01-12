// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <stdint.h>

#include <iostream>

// The generated bindings are in a header at the same path as the
// `example_crate` `rust_library` and with the name that follows the `<crate
// name>_cc_api.h` pattern:
#include "examples/rust/function/example_crate_cc_api.h"

int main(int argc, char* argv[]) {
  // The generated bindings are in a namespace with the same name as the
  // target crate:
  int32_t sum = example_crate::add_two_integers(2, 2);
  std::cout << "sum = " << sum << std::endl;
  return 0;
}
