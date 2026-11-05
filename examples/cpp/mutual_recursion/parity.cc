// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "examples/cpp/mutual_recursion/parity.h"

#include "examples/cpp/mutual_recursion/is_odd.h"

namespace parity {

bool IsEven(int n) {
  if (n == 0) return true;
  return IsOdd(n - 1);
}

// The C++ implementation of `IsOdd` used to be:
//
//   if (n == 0) return false;
//   return IsEven(n - 1);
//
// It now forwards to the Rust implementation, through the Crubit-generated C++
// API for the `:is_odd` crate (`is_odd.h`). C++ callers still call
// `parity::IsOdd`, and don't need to change.
bool IsOdd(int n) { return is_odd::is_odd(n); }

}  // namespace parity
