// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/bridging/protobuf_duplicate_crate/a/a_extension.pb.h"
#include "cc_bindings_from_rs/test/bridging/protobuf_duplicate_crate/b/b_extension.pb.h"
#include "cc_bindings_from_rs/test/bridging/protobuf_duplicate_crate/rust_lib.h"

namespace {

TEST(ProtobufDuplicateCrateTest, CanPassMessages) {
  a::MsgA msg_a;
  msg_a.set_x(42);
  EXPECT_EQ(rust_lib::take_msg_a(msg_a), 42);

  b::MsgB msg_b;
  msg_b.set_y(100);
  EXPECT_EQ(rust_lib::take_msg_b(msg_b), 100);
}

}  // namespace
