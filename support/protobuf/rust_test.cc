// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/protobuf/rust.h"

#include <cstddef>
#include <type_traits>

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
  static_assert(std::is_move_constructible_v<Rust<DummyMessage>>);
  static_assert(std::is_move_assignable_v<Rust<DummyMessage>>);
}

TEST(RustWrapperTest, MoveConstructionAndAssignment) {
  Rust<DummyMessage> msg;
  DummyMessage* orig_ptr = msg.get();
  EXPECT_NE(orig_ptr, nullptr);

  Rust<DummyMessage> moved_msg(std::move(msg));
  EXPECT_EQ(moved_msg.get(), orig_ptr);

  Rust<DummyMessage> msg2;
  DummyMessage* orig_ptr2 = msg2.get();
  moved_msg = std::move(msg2);
  EXPECT_EQ(moved_msg.get(), orig_ptr2);
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
