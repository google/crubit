// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_EXAMPLES_TYPES_ABSL_STATUS_CPP_API_H_
#define THIRD_PARTY_CRUBIT_EXAMPLES_TYPES_ABSL_STATUS_CPP_API_H_

#include "absl/status/status.h"
#include "absl/status/statusor.h"

absl::Status ReturnsStatus(bool ok);

absl::StatusOr<int> ReturnsStatusOrInt(bool ok);

#endif  // THIRD_PARTY_CRUBIT_EXAMPLES_TYPES_ABSL_STATUS_CPP_API_H_
