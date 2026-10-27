// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_WRAPPER_WRAPPED_LIBRARY_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_WRAPPER_WRAPPED_LIBRARY_H_

struct Nontrivial {
  int value;
  explicit Nontrivial(int x) : value(x) {}
  Nontrivial(Nontrivial&&) = default;
  Nontrivial& operator=(Nontrivial&&) = default;
  ~Nontrivial() { value = 1234; }
};

inline Nontrivial Create() { return Nontrivial(42); }

inline int Read(Nontrivial nontrivial) { return nontrivial.value; }

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_WRAPPER_WRAPPED_LIBRARY_H_
