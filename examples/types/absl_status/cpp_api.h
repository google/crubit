// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_EXAMPLES_TYPES_ABSL_STATUS_CPP_API_H_
#define THIRD_PARTY_CRUBIT_EXAMPLES_TYPES_ABSL_STATUS_CPP_API_H_

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "examples/types/absl_status/example.pb.h"

absl::Status ReturnsStatus(bool ok);

absl::StatusOr<int> ReturnsStatusOrInt(bool ok);

absl::Status ReturnsStatusWithProto(
    bool ok, crubit::absl_status_example::ExampleProto& out);

#endif  // THIRD_PARTY_CRUBIT_EXAMPLES_TYPES_ABSL_STATUS_CPP_API_H_
