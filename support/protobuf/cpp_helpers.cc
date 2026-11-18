// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/protobuf/cpp_helpers.h"

#include <cstddef>

#include "absl/log/absl_check.h"
#include "google/protobuf/message_lite.h"

namespace crubit::protobuf_internal {

extern "C" {

void crubit_protobuf_placement_new(
    const ::google::protobuf::MessageLite* default_instance, void* dest,
    std::size_t expected_size, std::size_t expected_align) {
  ABSL_CHECK_NE(default_instance, nullptr);
  ABSL_CHECK_NE(dest, nullptr);
  const ::google::protobuf::internal::ClassData* data =
      ::google::protobuf::internal::GetClassData(*default_instance);
  ABSL_CHECK_NE(data, nullptr);
  ABSL_CHECK_EQ(expected_size, data->allocation_size());
  ABSL_CHECK_EQ(expected_align, data->alignment());
  data->PlacementNew(dest, /*arena=*/nullptr);
}

void crubit_protobuf_destroy(::google::protobuf::MessageLite* msg) {
  ABSL_CHECK_NE(msg, nullptr);
  ABSL_CHECK_EQ(msg->GetArena(), nullptr);
  ::google::protobuf::internal::GetClassData(*msg)->destroy_message(*msg);
}

}  // extern "C"

}  // namespace crubit::protobuf_internal
