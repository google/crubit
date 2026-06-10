// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/bridging/vector/vector.h"

#include <utility>
#include <vector>

#include "gtest/gtest.h"

namespace {

// Tests that a `vector` can be passed to Rust and returned back to
// C++ without being destroyed prematurely.
TEST(VectorBridging, Roundtrip) {
  vector::reset_destructor_count();
  int initial_count = vector::get_destructor_count();

  {
    auto vec = vector::create_vector(3);
    // Elements are constructed, but not destroyed yet.
    EXPECT_EQ(vector::get_destructor_count(), initial_count);

    auto vec2 = vector::roundtrip_vector(std::move(vec));
    EXPECT_EQ(vec2.size(), 3);
    EXPECT_EQ(vector::get_destructor_count(), initial_count);
  }
  // Now vec2 is destroyed, so elements should be destroyed.
  EXPECT_EQ(vector::get_destructor_count(), initial_count + 3);
}

// Tests that when a `vector` is passed to Rust by value and not
// returned, Rust takes ownership and correctly destroys the C++ object when
// it goes out of scope.
TEST(VectorBridging, ConsumedByRust) {
  vector::reset_destructor_count();
  int initial_count = vector::get_destructor_count();
  auto vec = vector::create_vector(3);

  vector::consume_vector(std::move(vec));

  EXPECT_EQ(vector::get_destructor_count(), initial_count + 3);
}

}  // namespace
