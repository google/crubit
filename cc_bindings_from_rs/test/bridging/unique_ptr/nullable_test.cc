// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/bridging/unique_ptr/nullable.h"

#include <cstdint>
#include <memory>
#include <utility>

#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/bridging/unique_ptr/test_helpers.h"

namespace {

using ::unique_ptr_test::Target;

// The C++ signatures are plain smart pointers carrying the `crubit_nullable`
// attribute, so ownership transfer is unaffected.
TEST(NullableUniquePtrBridging, Roundtrip) {
  auto token = std::make_shared<int32_t>(0);

  {
    auto ptr = ::unique_ptr_test::create_target(token);

    auto ptr2 = nullable::roundtrip_nullable_unique_ptr(std::move(ptr));
    EXPECT_NE(ptr2, nullptr);

    EXPECT_EQ(token.use_count(), 2);
  }
  EXPECT_EQ(token.use_count(), 1);
}

TEST(NullableUniquePtrBridging, RoundtripNull) {
  EXPECT_EQ(nullable::roundtrip_nullable_unique_ptr(nullptr), nullptr);
}

TEST(NullableUniquePtrBridging, IsNull) {
  std::unique_ptr<Target> null;
  EXPECT_TRUE(nullable::is_null_unique_ptr(null));

  auto ptr = ::unique_ptr_test::create_target(std::make_shared<int32_t>(0));
  EXPECT_FALSE(nullable::is_null_unique_ptr(ptr));
}

TEST(NullableUniquePtrBridging, MakeNull) {
  EXPECT_EQ(nullable::make_null_unique_ptr(), nullptr);
}

TEST(NullableVirtualUniquePtrBridging, Roundtrip) {
  auto token = std::make_shared<int32_t>(0);

  {
    auto ptr = ::unique_ptr_test::create_virtual_base(token);

    auto ptr2 = nullable::roundtrip_nullable_virtual_unique_ptr(std::move(ptr));
    EXPECT_NE(ptr2, nullptr);

    EXPECT_EQ(token.use_count(), 2);
  }
  EXPECT_EQ(token.use_count(), 1);
}

TEST(NullableVirtualUniquePtrBridging, RoundtripNull) {
  EXPECT_EQ(nullable::roundtrip_nullable_virtual_unique_ptr(nullptr), nullptr);
}

TEST(NullableSharedPtrBridging, Roundtrip) {
  auto ptr = std::make_shared<int32_t>(42);
  std::weak_ptr<int32_t> weak = ptr;

  auto ptr2 = nullable::roundtrip_nullable_shared_ptr(std::move(ptr));
  EXPECT_FALSE(weak.expired());
  ASSERT_NE(ptr2, nullptr);
  EXPECT_EQ(*ptr2, 42);

  ptr2 = nullptr;
  EXPECT_TRUE(weak.expired());
}

TEST(NullableSharedPtrBridging, RoundtripNull) {
  EXPECT_EQ(nullable::roundtrip_nullable_shared_ptr(nullptr), nullptr);
}

}  // namespace
