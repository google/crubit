// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_COMMON_STATUS_ABSL_COMPAT_H_
#define CRUBIT_COMMON_STATUS_ABSL_COMPAT_H_

#include <utility>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "common/status.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/Error.h"

namespace crubit {

inline absl::Status ToAbslStatus(llvm::Error error) {
  if (!error) return absl::OkStatus();
  absl::Status status(static_cast<absl::StatusCode>(GetStatusCode(error)),
                      GetStatusMessage(error));
  llvm::consumeError(std::move(error));
  return status;
}

inline llvm::Error FromAbslStatus(const absl::Status& s) {
  if (s.ok()) return llvm::Error::success();
  return MakeStatusError(
      static_cast<StatusCode>(s.code()),
      llvm::StringRef(s.message().data(), s.message().size()));
}

// Adapter returned on the error path of `CRUBIT_RETURN_IF_ERROR` and
// `CRUBIT_ASSIGN_OR_RETURN` that converts between `llvm::Error` /
// `llvm::Expected<T>` and `absl::Status` / `absl::StatusOr<T>`.
class ErrorReturn {
 public:
  explicit ErrorReturn(llvm::Error error) : error_(std::move(error)) {}
  explicit ErrorReturn(const absl::Status& status)
      : error_(FromAbslStatus(status)) {}

  operator llvm::Error() && { return std::move(error_); }

  template <typename T>
  operator llvm::Expected<T>() && {
    return std::move(error_);
  }

  operator absl::Status() && { return ToAbslStatus(std::move(error_)); }

  template <typename T>
  operator absl::StatusOr<T>() && {
    return ToAbslStatus(std::move(error_));
  }

 private:
  llvm::Error error_;
};

}  // namespace crubit

#endif  // CRUBIT_COMMON_STATUS_ABSL_COMPAT_H_
