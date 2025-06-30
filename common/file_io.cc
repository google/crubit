// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "common/file_io.h"

#include <memory>
#include <string>
#include <system_error>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "common/string_view_conversion.h"
#include "llvm/Support/ErrorOr.h"
#include "llvm/Support/MemoryBuffer.h"
#include "llvm/Support/raw_ostream.h"

namespace crubit {

absl::StatusOr<std::string> GetFileContents(absl::string_view path) {
  llvm::ErrorOr<std::unique_ptr<llvm::MemoryBuffer>> err_or_buffer =
      llvm::MemoryBuffer::getFileOrSTDIN(path.data(), /* IsText= */ true);
  if (std::error_code err = err_or_buffer.getError()) {
    return absl::Status(absl::StatusCode::kInternal, err.message());
  }

  return std::string((*err_or_buffer)->getBuffer());
}

absl::Status SetFileContents(absl::string_view path,
                             absl::string_view contents) {
  std::error_code error_code;
  llvm::raw_fd_ostream stream(StringRefFromStringView(path), error_code);
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
