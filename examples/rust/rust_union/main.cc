// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "examples/rust/rust_union/example_crate.h"

int main(int argc, char* argv[]) {
  example_crate::ReprRustUnion u1;
  // Because the union is not repr(C), we cannot set the fields directly.
  // u1.a = 100;
  // u1.b = 123.456;
  // But we can expose and use functions to do the same.
  u1.set_a(100);
  u1.set_b(123.456);
  return 0;
}
