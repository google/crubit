// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstdint>
#include <iostream>

#include "examples/rust/library_config/example_crate.h"

int main(int argc, char* argv[]) {
  // The generated bindings are in `my::library`, not `example_crate`.
  int32_t sum = my::library::add_two_integers(2, 2);
  std::cout << "sum = " << sum << std::endl;
  return 0;
}
