// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_COMMON_STATUS_ABSL_COMPAT_H_
#define CRUBIT_COMMON_STATUS_ABSL_COMPAT_H_

#include <utility>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "common/status.h"
#include "llvm/Support/Error.h"

namespace crubit {

inline absl::Status ToAbslStatus(llvm::Error error) {
  if (!error) return absl::OkStatus();
  absl::Status result = absl::UnknownError("Unknown error");
  llvm::handleAllErrors(
      std::move(error),
      [&](const StatusError& err) {
        result = absl::Status(static_cast<absl::StatusCode>(err.code()),
                              err.message());
      },
      [&](const llvm::ErrorInfoBase& err) {
        result = absl::UnknownError(err.message());
      });
  return result;
}

inline llvm::Error FromAbslStatus(const absl::Status& s) {
  if (s.ok()) return llvm::Error::success();
  return MakeStatusError(static_cast<StatusCode>(s.code()), s.message());
}

class AbslErrorReturn {
 public:
  explicit AbslErrorReturn(absl::Status status) : status_(std::move(status)) {}
  AbslErrorReturn(AbslErrorReturn&&) = default;
  AbslErrorReturn& operator=(AbslErrorReturn&&) = default;

  operator absl::Status() && { return std::move(status_); }

  template <typename T>
  operator absl::StatusOr<T>() && {
    return std::move(status_);
  }

  operator llvm::Error() && { return FromAbslStatus(status_); }

  template <typename T>
  operator llvm::Expected<T>() && {
    return FromAbslStatus(status_);
  }

 private:
  absl::Status status_;
};

class LlvmErrorReturn {
 public:
  explicit LlvmErrorReturn(llvm::Error error) : error_(std::move(error)) {}
  LlvmErrorReturn(LlvmErrorReturn&&) = default;
  LlvmErrorReturn& operator=(LlvmErrorReturn&&) = default;

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
