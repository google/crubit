// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/bridging/virtual_unique_ptr/virtual_unique_ptr.h"

#include "gtest/gtest.h"

namespace crubit {
namespace {

// Tests that a `virtual_unique_ptr` can be passed to Rust and returned back to
// C++ without being destroyed prematurely.
TEST(VirtualUniquePtrBridging, Roundtrip) {
  int initial_count = ::virtual_unique_ptr::get_derived_destructor_count();

  {
    auto ptr = virtual_unique_ptr::create_virtual_unique_ptr();

    auto ptr2 =
        virtual_unique_ptr::roundtrip_virtual_unique_ptr(std::move(ptr));
    EXPECT_NE(ptr2, nullptr);

    EXPECT_EQ(::virtual_unique_ptr::get_derived_destructor_count(),
              initial_count);
  }
  EXPECT_EQ(::virtual_unique_ptr::get_derived_destructor_count(),
            initial_count + 1);
}

// Tests that when a `virtual_unique_ptr` is passed to Rust by value and not
// returned, Rust rakes ownership and correctly destroys the C++ object when
// it goes out of scope.
TEST(VirtualUniquePtrBridging, ConsumedByRust) {
  int initial_count = ::virtual_unique_ptr::get_derived_destructor_count();
  auto ptr = virtual_unique_ptr::create_virtual_unique_ptr();

  virtual_unique_ptr::consume_virtual_unique_ptr(std::move(ptr));

  EXPECT_EQ(::virtual_unique_ptr::get_derived_destructor_count(),
            initial_count + 1);
}

}  // namespace
}  // namespace crubit
