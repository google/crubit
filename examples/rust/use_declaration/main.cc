// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstdint>

#include "examples/rust/use_declaration/example_crate.h"

int main(int argc, char* argv[]) {
  std::int32_t x = example_crate::Type().x;
  (void)x;

  example_crate::function();

  return 0;
}
