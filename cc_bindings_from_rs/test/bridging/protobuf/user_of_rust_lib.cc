// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/bridging/protobuf/rust_lib.h"

#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/bridging/protobuf/foo.pb.h"

namespace crubit {
namespace {

TEST(ProtoBridging, ViewAndMutTypes) {
  foo_service::FooRequest req;
  req.set_input("hello world");
  foo_service::FooResponse rsp;

  rust_lib::FooService service;
  EXPECT_TRUE(service.handle_request(&req, &rsp));

  EXPECT_EQ(rsp.output(), req.input());
  EXPECT_EQ(rsp.output(), "hello world");

  const foo_service::FooRequestStats* req_stats = service.request_stats();
  EXPECT_EQ(req_stats->num_requests(), 1);
}

TEST(ProtoBridging, OwnedMessages) {
  rust_lib::FooService service;

  foo_service::FooRequestStats cloned_req_stats = service.clone_request_stats();
  EXPECT_EQ(cloned_req_stats.num_requests(), 0);

  // Update the local message and check that the service's stats are not
  // updated i.e. it was actually cloned.
  cloned_req_stats.set_num_requests(2);
  EXPECT_EQ(service.request_stats()->num_requests(), 0);

  service.update_request_stats(cloned_req_stats);
  EXPECT_EQ(service.request_stats()->num_requests(), 2);

  // Update the local message again and check that the service's stats are
  // not updated.
  cloned_req_stats.set_num_requests(3);
  EXPECT_EQ(service.request_stats()->num_requests(), 2);
}

TEST(ProtoBridging, StructWithProtoField) {
  rust_lib::StructWithProto s = rust_lib::create_struct_with_proto(42);
  EXPECT_EQ(s.stats->num_requests(), 42);
}

TEST(ProtoBridging, ProtoPointerAndReference) {
  rust_lib::StructWithProto s = rust_lib::create_struct_with_proto(42);
  EXPECT_EQ(rust_lib::read_proto_pointer(&s.stats), 42);
  EXPECT_EQ(rust_lib::read_proto_ref(s.stats), 42);
}

TEST(ProtoBridging, ProtoVec) {
  rs_std::Vec<proto::Rust<foo_service::FooRequestStats>> v =
      rust_lib::create_proto_vec(42);
  EXPECT_EQ(v.size(), 1);
  EXPECT_EQ(v[0]->num_requests(), 42);
}

TEST(ProtoBridging, ProtoStatusOr) {
  absl::StatusOr<proto::Rust<foo_service::FooRequestStats>> status_or =
      rust_lib::create_proto_status_or(42);
  EXPECT_TRUE(status_or.ok());
  EXPECT_EQ((*status_or)->num_requests(), 42);
}

}  // namespace
}  // namespace crubit
