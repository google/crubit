// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_EXAMPLES_CPP_REFERENCES_AND_LIFETIMES_EXAMPLE_H_
#define THIRD_PARTY_CRUBIT_EXAMPLES_CPP_REFERENCES_AND_LIFETIMES_EXAMPLE_H_

#include "support/lifetime_annotations.h"

namespace example {

struct Counter final {
  int Get() const { return value; }
  void Increment() { value++; }

  const int& GetRef() const { return value; }
  int& GetMutRef() { return value; }

  int value;
};

inline void AddOne(int& x) { x++; }

// Lifetime elision doesn't apply: there are two input references and no `this`.
inline const int& Smaller(const int& x, const int& y) { return x < y ? x : y; }

// Explicit lifetime annotations: The result may refer to `x` or to `y`.
inline const int& $a SmallerWithLifetimes(const int& $a x, const int& $a y) {
  return x < y ? x : y;
}

struct Number final {
  // The result may refer to `this` or to `other`, but lifetime elision ties the
  // result only to `this`.
  const int& GetLarger(const int& other) const {
    return value > other ? value : other;
  }

  // The result may refer to `this` or to `other`.
  const int& $a GetLargerWithLifetimes(const int& $a other) const $a {
    return value > other ? value : other;
  }

  int value;
};

}  // namespace example

#endif  // THIRD_PARTY_CRUBIT_EXAMPLES_CPP_REFERENCES_AND_LIFETIMES_EXAMPLE_H_
