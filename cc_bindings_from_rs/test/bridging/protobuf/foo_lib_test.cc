// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/bridging/protobuf/foo_lib.h"

#include <memory>

#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/bridging/protobuf/foo.proto.h"

namespace crubit {
namespace {

TEST(ProtoBridging, Basic) {
  foo_service::FooRequest req;
  req.set_input("hello world");
  foo_service::FooResponse rsp;

  foo_lib::FooService service;
  EXPECT_TRUE(service.handle_request(&req, &rsp));

  EXPECT_EQ(rsp.output(), req.input());
  EXPECT_EQ(rsp.output(), "hello world");

  const foo_service::FooRequestStats* req_stats = service.request_stats();
  EXPECT_EQ(req_stats->num_requests(), 1);
}

}  // namespace
}  // namespace crubit