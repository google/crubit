// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_EXAMPLES_CPP_CARCINIZE_MATH_UTILS_H_
#define THIRD_PARTY_CRUBIT_EXAMPLES_CPP_CARCINIZE_MATH_UTILS_H_

namespace math {

struct Vector2 {
  int x;
  int y;
};

inline int DotProduct(const Vector2& a, const Vector2& b) {
  return a.x * b.x + a.y * b.y;
}

// C++ template (not bindable automatically by Crubit)
template <typename T>
T Clamp(T val, T min, T max) {
  return val < min ? min : (val > max ? max : val);
}

}  // namespace math

#endif  // THIRD_PARTY_CRUBIT_EXAMPLES_CPP_CARCINIZE_MATH_UTILS_H_
