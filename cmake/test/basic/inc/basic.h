// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_CMAKE_TEST_BASIC_BASIC_H_
#define CRUBIT_CMAKE_TEST_BASIC_BASIC_H_

struct Point {
  int x;
  int y;
};

int Sum(Point p);

#ifdef FIXTURE_ALWAYS
inline int Always() { return 1; }
#endif

#endif  // CRUBIT_CMAKE_TEST_BASIC_BASIC_H_
