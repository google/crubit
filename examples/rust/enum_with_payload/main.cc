// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <utility>

#include "examples/rust/enum_with_payload/example_crate.h"

int main(int argc, char* argv[]) {
  example_crate::Color transparent_color =
      example_crate::Color::MakeTransparent();

  example_crate::Color red_color = example_crate::Color::MakeRgb(255, 0, 0);

  std::ignore = transparent_color;
  std::ignore = red_color;
  return 0;
}
