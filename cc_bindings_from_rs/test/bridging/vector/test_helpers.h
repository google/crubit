// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_VECTOR_TEST_HELPERS_H_
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_VECTOR_TEST_HELPERS_H_

#include <vector>

#include "support/annotations.h"

namespace vector_test {

struct Target {
  ~Target();
  CRUBIT_MUST_BIND static int get_destructor_count();
  CRUBIT_MUST_BIND static void reset_destructor_count();

 private:
  static int destructor_count;
};

CRUBIT_MUST_BIND std::vector<Target> create_vector(int size);

}  // namespace vector_test

#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_VECTOR_TEST_HELPERS_H_
