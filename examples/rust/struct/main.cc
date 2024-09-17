// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <utility>

#include "examples/rust/struct/example_crate_cc_api.h"

int main(int argc, char* argv[]) {
  // Does not compile: Struct doesn't implement the `Default` trait.
  // example_crate::Struct s0;

  example_crate::StructWithDefault s1;
  s1.a = 100;
  // Do not compile: StructWithDefault doesn't implement the `Clone`
  // trait.
  // example_crate::StructWithDefault s2(s1);

  // Structs are movable.
  example_crate::StructWithDefault s2(std::move(s1));
  s2.a = 200;

  // Structs that implement the `Clone` and `Default` traits without custom
  // `Drop` impls are trivial.
  example_crate::StructWithClone s3;
  s3.a = 100;
  example_crate::StructWithClone s4(s3);
  s4.a = 200;

  return 0;
}
