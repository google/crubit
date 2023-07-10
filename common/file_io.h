// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_COMMON_FILE_IO_H_
#define CRUBIT_COMMON_FILE_IO_H_

#include <string>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"

namespace crubit {

absl::StatusOr<std::string> GetFileContents(absl::string_view path);

absl::Status SetFileContents(absl::string_view path,
                             absl::string_view contents);

}  // namespace crubit

#endif  // CRUBIT_COMMON_FILE_IO_H_
