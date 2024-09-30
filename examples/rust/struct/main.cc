// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "examples/rust/struct/example_crate.h"

int main(int argc, char* argv[]) {
  example_crate::Struct s1;
  s1.a = 100;
  example_crate::Struct s2(s1);
  s2.a = 200;

  return 0;
}
