// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "examples/types/absl_status/cpp_api.h"

#include "absl/status/status.h"
#include "absl/status/statusor.h"

absl::Status ReturnsStatus(bool ok) {
  if (ok) {
    return absl::OkStatus();
  } else {
    return absl::InternalError("Something went wrong, oh no!");
  }
}

absl::StatusOr<int> ReturnsStatusOrInt(bool ok) {
  if (ok) {
    return 42;
  } else {
    return absl::InternalError("Something went wrong, oh no!");
  }
}
