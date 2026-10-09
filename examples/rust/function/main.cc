// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstdint>
#include <iostream>

// The generated bindings are in a header at the same path as the
// `example_crate` rust_library, with a `.h` suffix.
#include "examples/rust/function/example_crate.h"

#include <unordered_map>
#include <vector>

void demonstrate_nondeterminism() {
  std::unordered_map<int*, int> pointer_map;
  int a = 1, b = 2, c = 3;
  pointer_map[&a] = 1;
  pointer_map[&b] = 2;
  pointer_map[&c] = 3;

  std::cout << "Iteration order: ";
  for (const auto& [key, value] : pointer_map) {
    std::cout << value << " ";
  }
  std::cout << std::endl;
}

int main(int argc, char* argv[]) {
  // The generated bindings are in a namespace with the same name as the
  // target crate:
    demonstrate_nondeterminism();
  int32_t sum = example_crate::add_two_integers(2, 2);
  std::cout << "sum = " << sum << std::endl;
  return 0;
}
