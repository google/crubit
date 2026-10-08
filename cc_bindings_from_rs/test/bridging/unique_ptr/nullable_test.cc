// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/bridging/unique_ptr/nullable.h"

#include <cstdint>
#include <memory>
#include <utility>
#include <vector>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/bridging/unique_ptr/test_helpers.h"
#include "support/annotations.h"

namespace {

using ::testing::ElementsAre;
using ::testing::IsNull;
using ::testing::NotNull;
using ::testing::Pointee;
using ::unique_ptr_test::Base;
using ::unique_ptr_test::Target;

// The C++ signatures are plain smart pointers carrying the `crubit_nullable`
// attribute, so ownership transfer is unaffected.
TEST(NullableUniquePtrBridging, Roundtrip) {
  auto token = std::make_shared<int32_t>(0);

  {
    auto ptr = ::unique_ptr_test::create_target(token);

    crubit_nullable std::unique_ptr<Target> ptr2 =
        nullable::roundtrip_nullable_unique_ptr(std::move(ptr));
    EXPECT_NE(ptr2, nullptr);

    EXPECT_EQ(token.use_count(), 2);
  }
  EXPECT_EQ(token.use_count(), 1);
}

TEST(NullableUniquePtrBridging, RoundtripNull) {
  EXPECT_EQ(nullable::roundtrip_nullable_unique_ptr(nullptr), nullptr);
}

TEST(NullableUniquePtrBridging, IsNull) {
  crubit_nullable std::unique_ptr<Target> null;
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

    crubit_nullable std::unique_ptr<Base> ptr2 =
        nullable::roundtrip_nullable_virtual_unique_ptr(std::move(ptr));
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

  crubit_nullable std::shared_ptr<int32_t> ptr2 =
      nullable::roundtrip_nullable_shared_ptr(std::move(ptr));
  EXPECT_FALSE(weak.expired());
  ASSERT_NE(ptr2, nullptr);
  EXPECT_EQ(*ptr2, 42);

  ptr2 = nullptr;
  EXPECT_TRUE(weak.expired());
}

TEST(NullableSharedPtrBridging, RoundtripNull) {
  EXPECT_EQ(nullable::roundtrip_nullable_shared_ptr(nullptr), nullptr);
}

// `Nullable<Ptr>` as a template argument is bridged as
// `std::vector<crubit_nullable Ptr>`, which is the same type as
// `std::vector<Ptr>`.
TEST(NullableInVectorBridging, RoundtripUniquePtr) {
  auto token = std::make_shared<int32_t>(0);

  {
    std::vector<crubit_nullable std::unique_ptr<Target>> v;
    v.push_back(::unique_ptr_test::create_target(token));
    v.push_back(nullptr);

    std::vector<crubit_nullable std::unique_ptr<Target>> v2 =
        nullable::roundtrip_vector_of_nullable_unique_ptr(std::move(v));
    EXPECT_THAT(v2, ElementsAre(NotNull(), IsNull()));

    EXPECT_EQ(token.use_count(), 2);
  }
  EXPECT_EQ(token.use_count(), 1);
}

TEST(NullableInVectorBridging, CountNull) {
  std::vector<crubit_nullable std::unique_ptr<Target>> v;
  v.push_back(::unique_ptr_test::create_target(std::make_shared<int32_t>(0)));
  v.push_back(nullptr);
  v.push_back(nullptr);
  EXPECT_EQ(nullable::count_null_unique_ptrs(std::move(v)), 2);
}

TEST(NullableInVectorBridging, MakeWithNull) {
  EXPECT_THAT(nullable::make_vector_of_null_unique_ptr(),
              ElementsAre(IsNull()));
}

TEST(NullableInVectorBridging, RoundtripSharedPtr) {
  auto ptr = std::make_shared<int32_t>(42);
  std::weak_ptr<int32_t> weak = ptr;

  std::vector<crubit_nullable std::shared_ptr<int32_t>> v;
  v.push_back(std::move(ptr));
  v.push_back(nullptr);

  std::vector<crubit_nullable std::shared_ptr<int32_t>> v2 =
      nullable::roundtrip_vector_of_nullable_shared_ptr(std::move(v));
  EXPECT_THAT(v2, ElementsAre(Pointee(42), IsNull()));

  v2.clear();
  EXPECT_TRUE(weak.expired());
}

}  // namespace
