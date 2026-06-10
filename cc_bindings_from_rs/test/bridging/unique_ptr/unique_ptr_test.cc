// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/bridging/unique_ptr/unique_ptr.h"

#include <utility>

#include "gtest/gtest.h"

namespace {

// Tests that a `unique_ptr` can be passed to Rust and returned back to
// C++ without being destroyed prematurely.
TEST(UniquePtrBridging, Roundtrip) {
  int initial_count = ::unique_ptr::get_destructor_count();

  {
    auto ptr = unique_ptr::create_unique_ptr();

    auto ptr2 = unique_ptr::roundtrip_unique_ptr(std::move(ptr));
    EXPECT_NE(ptr2, nullptr);

    EXPECT_EQ(::unique_ptr::get_destructor_count(), initial_count);
  }
  EXPECT_EQ(::unique_ptr::get_destructor_count(), initial_count + 1);
}

// Tests that when a `unique_ptr` is passed to Rust by value and not
// returned, Rust takes ownership and correctly destroys the C++ object when
// it goes out of scope.
TEST(UniquePtrBridging, ConsumedByRust) {
  int initial_count = ::unique_ptr::get_destructor_count();
  auto ptr = unique_ptr::create_unique_ptr();

  unique_ptr::consume_unique_ptr(std::move(ptr));

  EXPECT_EQ(::unique_ptr::get_destructor_count(), initial_count + 1);
}

// Tests that a `virtual_unique_ptr` can be passed to Rust and returned back to
// C++ without being destroyed prematurely.
TEST(VirtualUniquePtrBridging, Roundtrip) {
  int initial_count = ::unique_ptr::get_derived_destructor_count();

  {
    auto ptr = unique_ptr::create_virtual_unique_ptr();

    auto ptr2 = unique_ptr::roundtrip_virtual_unique_ptr(std::move(ptr));
    EXPECT_NE(ptr2, nullptr);

    EXPECT_EQ(::unique_ptr::get_derived_destructor_count(), initial_count);
  }
  EXPECT_EQ(::unique_ptr::get_derived_destructor_count(), initial_count + 1);
}

// Tests that when a `virtual_unique_ptr` is passed to Rust by value and not
// returned, Rust takes ownership and correctly destroys the C++ object when
// it goes out of scope.
TEST(VirtualUniquePtrBridging, ConsumedByRust) {
  int initial_count = ::unique_ptr::get_derived_destructor_count();
  auto ptr = unique_ptr::create_virtual_unique_ptr();

  unique_ptr::consume_virtual_unique_ptr(std::move(ptr));

  EXPECT_EQ(::unique_ptr::get_derived_destructor_count(), initial_count + 1);
}

}  // namespace
