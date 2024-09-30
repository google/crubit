// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "examples/rust/enum/example_crate.h"

int main(int argc, char* argv[]) {
  example_crate::Color e1;
  example_crate::Color e2 = e1;
  (void)e2;
  return 0;
}
