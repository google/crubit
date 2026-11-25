// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_PROTOBUF_CPP_HELPERS_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_PROTOBUF_CPP_HELPERS_H_

#include <cstddef>

#include "google/protobuf/message_lite.h"

namespace crubit::protobuf_internal {

extern "C" {

// Default-constructs a C++ protobuf message of the same dynamic type as
// `*default_instance` in-place at `dest` (with `arena = nullptr`), asserting
// that its C++ `allocation_size()` and `alignment()` match `expected_size` and
// `expected_align`.
void crubit_protobuf_placement_new(
    const ::google::protobuf::MessageLite* default_instance, void* dest,
    std::size_t expected_size, std::size_t expected_align);

// Destroys a C++ protobuf message in-place at `msg` without deallocating the
// storage pointed to by `msg`.
void crubit_protobuf_destroy(::google::protobuf::MessageLite* msg);

}  // extern "C"

}  // namespace crubit::protobuf_internal

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_PROTOBUF_CPP_HELPERS_H_
