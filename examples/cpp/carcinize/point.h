// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_EXAMPLES_CPP_CARCINIZE_POINT_H_
#define THIRD_PARTY_CRUBIT_EXAMPLES_CPP_CARCINIZE_POINT_H_

namespace geometry {

struct Point {
  int x;
  int y;
};

inline int GetX(const Point& p) { return p.x; }

}  // namespace geometry

#endif  // THIRD_PARTY_CRUBIT_EXAMPLES_CPP_CARCINIZE_POINT_H_
