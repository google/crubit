// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#include <iostream>

#include "rust_lib.h"

int main() {
  // Test we can successfully use a type from a Rust dependency.
  base64::engine::GeneralPurpose engine = rust_lib::make_engine();
  std::cout << "Successfully linked crubit generated C++ api: "
            << rust_lib::add(1, 2) << std::endl;
  return 0;
}
