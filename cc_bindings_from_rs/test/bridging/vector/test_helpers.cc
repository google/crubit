// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/bridging/vector/test_helpers.h"

#include <vector>

namespace vector_test {

int Target::destructor_count = 0;

Target::~Target() { destructor_count++; }

int Target::get_destructor_count() { return destructor_count; }

void Target::reset_destructor_count() { destructor_count = 0; }

std::vector<Target> create_vector(int size) {
  return std::vector<Target>(size);
}

}  // namespace vector_test
