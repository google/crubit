// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <iostream>

#include "examples/cpp/carcinize/math_utils_rs.h"
#include "examples/cpp/carcinize/point_rs.h"

int main(int argc, char* argv[]) {
  point_rs::geometry::Point p;
  p.x = 10;
  p.y = 20;
  std::cout << "point_rs GetX: " << point_rs::geometry::GetX(&p) << std::endl;

  math_utils_rs::math::Vector2 v1;
  v1.x = 1;
  v1.y = 2;
  math_utils_rs::math::Vector2 v2;
  v2.x = 3;
  v2.y = 4;
  std::cout << "DotProduct: " << math_utils_rs::math::DotProduct(&v1, &v2)
            << std::endl;

  std::cout << "clamp_i32: " << math_utils_rs::math::clamp_i32(15, 0, 10)
            << std::endl;

  // C++ callers also have access to C++ declarations from global_cpp!
  std::cout << "C++ Clamp template: " << math::Clamp(15, 0, 10) << std::endl;

  return 0;
}
