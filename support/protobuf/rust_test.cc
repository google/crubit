// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/protobuf/rust.h"

#include <cstddef>
#include <type_traits>
#include <utility>

#include "support/movable.h"
#include "gtest/gtest.h"
#include "support/protobuf/test.pb.h"

namespace proto2 {
namespace {

using ::proto2::crubit_test::DummyMessage;

TEST(RustWrapperTest, SizeAndAlignment) {
  static_assert(sizeof(Rust<DummyMessage>) == sizeof(void*));
  static_assert(alignof(Rust<DummyMessage>) == alignof(void*));
  static_assert(std::is_default_constructible_v<Rust<DummyMessage>>);
  static_assert(!std::is_constructible_v<Rust<DummyMessage>, std::nullptr_t>);
  static_assert(!std::is_copy_constructible_v<Rust<DummyMessage>>);
  static_assert(!std::is_copy_assignable_v<Rust<DummyMessage>>);
  static_assert(!std::is_move_constructible_v<Rust<DummyMessage>>);
  static_assert(!std::is_move_assignable_v<Rust<DummyMessage>>);
  static_assert(
      std::is_constructible_v<Rust<DummyMessage>, ::crubit::UnsafeRelocateTag,
                              Rust<DummyMessage>&&>);
}

TEST(RustWrapperTest, RelocatesObject) {
  rs::Movable<Rust<DummyMessage>> src(std::in_place);
  DummyMessage* orig_ptr = src->get();
  EXPECT_NE(orig_ptr, nullptr);

  rs::Movable<Rust<DummyMessage>> relocated = std::move(src);
  EXPECT_TRUE(src.valueless_after_move());
  EXPECT_FALSE(relocated.valueless_after_move());
  EXPECT_EQ(relocated->get(), orig_ptr);
}

TEST(RustWrapperTest, DefaultConstructsObject) {
  Rust<DummyMessage> msg;
  EXPECT_NE(msg.get(), nullptr);
}

TEST(RustWrapperTest, OwnsAndAccessesObject) {
  Rust<DummyMessage> msg;
  EXPECT_NE(msg.get(), nullptr);
  EXPECT_NE(&*msg, nullptr);
  EXPECT_NE(msg.operator->(), nullptr);

  // Const access
  const auto& const_msg = msg;
  EXPECT_NE(const_msg.get(), nullptr);
  EXPECT_NE(&*const_msg, nullptr);
  EXPECT_NE(const_msg.operator->(), nullptr);
}

TEST(RustWrapperTest, ProtoNamespaceAlias) {
  ::proto::Rust<DummyMessage> msg;
  static_assert(std::is_same_v<::proto::Rust<DummyMessage>,
                               ::proto2::Rust<DummyMessage>>);
  EXPECT_NE(msg.get(), nullptr);
}

}  // namespace
}  // namespace proto2
