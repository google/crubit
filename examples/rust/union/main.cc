// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "examples/rust/union/example_crate.h"

int main(int argc, char* argv[]) {
  example_crate::ReprCUnion u1;
  u1.a = 100;
  u1.b = 123.456;
  return 0;
}