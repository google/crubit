// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "examples/rust/cpp_enum/example_crate.h"

int main(int argc, char* argv[]) {
  const example_crate::Color e1 = example_crate::Color::Green;
  const example_crate::Color e2 = example_crate::Color::Gray;
  static_assert(e1 == e2);
  return 0;
}
