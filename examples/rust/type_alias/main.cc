// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstdint>
#include <type_traits>

#include "examples/rust/type_alias/example_crate.h"

int main(int argc, char* argv[]) {
  static_assert(std::is_same_v<example_crate::TypeAlias, std::int32_t>);
  example_crate::TypeAlias x = 123;
  (void)x;

  return 0;
}
