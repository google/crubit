// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_EXAMPLES_CPP_MUTUAL_RECURSION_PARITY_H_
#define THIRD_PARTY_CRUBIT_EXAMPLES_CPP_MUTUAL_RECURSION_PARITY_H_

namespace parity {

// Returns true if `n` is even. Requires `n >= 0`.
//
// Implemented in C++ (parity.cc), by calling `IsOdd`.
bool IsEven(int n);

// Returns true if `n` is odd. Requires `n >= 0`.
//
// Implemented in C++ (parity.cc), by forwarding to the Rust function
// `is_odd::is_odd` (is_odd.rs), which calls `IsEven`. This declaration (and so
// the API and ABI seen by C++ callers) is unchanged from when this function was
// implemented entirely in C++.
bool IsOdd(int n);

}  // namespace parity

#endif  // THIRD_PARTY_CRUBIT_EXAMPLES_CPP_MUTUAL_RECURSION_PARITY_H_
