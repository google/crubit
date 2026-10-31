// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "common/file_io.h"

#include <memory>
#include <string>
#include <system_error>

#include "common/status.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/Error.h"
#include "llvm/Support/ErrorOr.h"
#include "llvm/Support/MemoryBuffer.h"
#include "llvm/Support/raw_ostream.h"

namespace crubit {

llvm::Expected<std::string> GetFileContents(llvm::StringRef path) {
  llvm::ErrorOr<std::unique_ptr<llvm::MemoryBuffer>> err_or_buffer =
      llvm::MemoryBuffer::getFileOrSTDIN(path, /* IsText= */ true);
  if (std::error_code err = err_or_buffer.getError()) {
    return crubit::InternalError(err.message());
  }

  return std::string((*err_or_buffer)->getBuffer());
}

llvm::Error SetFileContents(llvm::StringRef path, llvm::StringRef contents) {
  std::error_code error_code;
  llvm::raw_fd_ostream stream(path, error_code);
  if (error_code) {
    return crubit::InternalError(error_code.message());
  }
  stream << contents;
  stream.close();
  if (stream.has_error()) {
    return crubit::InternalError(stream.error().message());
  }
  return llvm::Error::success();
}

}  // namespace crubit
