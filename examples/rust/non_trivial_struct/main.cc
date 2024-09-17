// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "examples/rust/non_trivial_struct/example_crate.h"

int main(int argc, char* argv[]) {
  example_crate::NonTrivialStruct s;
  s.a = 100;
  return 0;
  // Will call the `drop` method of `s1` when it goes out of scope.
  // And print "Dropping NonTrivialStruct".
}
