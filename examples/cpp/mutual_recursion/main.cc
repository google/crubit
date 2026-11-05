// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <iostream>

#include "examples/cpp/mutual_recursion/parity.h"

int main(int argc, char* argv[]) {
  // `IsEven` is C++, and `IsOdd` is Rust; each calls the other.
  std::cout << "IsEven(10) = " << parity::IsEven(10) << std::endl;
  std::cout << "IsOdd(7) = " << parity::IsOdd(7) << std::endl;
  return 0;
}
