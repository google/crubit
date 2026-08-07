// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/bridging/shared_ptr/shared_ptr.h"

#include <cstdint>
#include <memory>
#include <utility>

#include "gtest/gtest.h"

namespace {

// Tests that a `shared_ptr` can be passed to Rust and returned back to
// C++ without being destroyed prematurely.
TEST(SharedPtrBridging, Roundtrip) {
  auto ptr = std::make_shared<int32_t>(42);
  std::weak_ptr<int32_t> weak = ptr;
  EXPECT_FALSE(weak.expired());
  EXPECT_EQ(*ptr, 42);

  auto ptr2 = shared_ptr::roundtrip_shared_ptr(std::move(ptr));
  EXPECT_FALSE(weak.expired());
  EXPECT_NE(ptr2, nullptr);
  EXPECT_EQ(*ptr2, 42);

  ptr2 = nullptr;
  EXPECT_TRUE(weak.expired());
}

// Tests that when a `shared_ptr` is cloned in Rust, the ref count increases
// and the underlying object is kept alive until both are destroyed.
TEST(SharedPtrBridging, CloneInRust) {
  auto ptr = std::make_shared<int32_t>(100);
  EXPECT_EQ(ptr.use_count(), 1);

  auto ptr2 = shared_ptr::clone_shared_ptr(ptr);
  EXPECT_EQ(ptr.use_count(), 2);
  EXPECT_EQ(ptr2.use_count(), 2);
  EXPECT_EQ(*ptr2, 100);

  ptr = nullptr;
  EXPECT_EQ(ptr2.use_count(), 1);

  ptr2 = nullptr;
}

// Tests that when a `shared_ptr` is passed to Rust by value and not
// returned, Rust takes ownership and correctly destroys the C++ object when
// it goes out of scope.
TEST(SharedPtrBridging, ConsumedByRust) {
  auto ptr = std::make_shared<int32_t>(7);
  std::weak_ptr<int32_t> weak = ptr;
  EXPECT_FALSE(weak.expired());

  shared_ptr::consume_shared_ptr(std::move(ptr));

  EXPECT_TRUE(weak.expired());
}

}  // namespace
