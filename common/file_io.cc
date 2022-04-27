// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "common/file_io.h"

#include "llvm/Support/raw_ostream.h"

namespace crubit {

absl::Status SetFileContents(absl::string_view path,
                             absl::string_view contents) {
  std::error_code error_code;
  llvm::raw_fd_ostream stream(path, error_code);
  if (error_code) {
    return absl::Status(absl::StatusCode::kInternal, error_code.message());
  }
  stream << contents;
  stream.close();
  if (stream.has_error()) {
    return absl::Status(absl::StatusCode::kInternal, stream.error().message());
  }
  return absl::OkStatus();
}

}  // namespace crubit
