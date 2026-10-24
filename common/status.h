// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_COMMON_STATUS_H_
#define CRUBIT_COMMON_STATUS_H_

#include <string>

#include "llvm/ADT/StringRef.h"
#include "llvm/ADT/Twine.h"
#include "llvm/Support/Error.h"
#include "llvm/Support/raw_ostream.h"

namespace crubit {

enum class StatusCode {
  kOk = 0,
  kCancelled = 1,
  kUnknown = 2,
  kInvalidArgument = 3,
  kDeadlineExceeded = 4,
  kNotFound = 5,
  kAlreadyExists = 6,
  kPermissionDenied = 7,
  kResourceExhausted = 8,
  kFailedPrecondition = 9,
  kAborted = 10,
  kOutOfRange = 11,
  kUnimplemented = 12,
  kInternal = 13,
  kUnavailable = 14,
  kDataLoss = 15,
  kUnauthenticated = 16
};

llvm::StringRef StatusCodeToString(StatusCode code);

class StatusError final : public llvm::ErrorInfo<StatusError> {
 public:
  static char ID;

  StatusError(StatusCode code, std::string message)
      : code_(code), message_(std::move(message)) {}

  StatusCode code() const { return code_; }
  std::string message() const override { return message_; }

  void log(llvm::raw_ostream& os) const override { os << message_; }

  std::error_code convertToErrorCode() const override {
    return llvm::inconvertibleErrorCode();
  }

 private:
  StatusCode code_;
  std::string message_;
};

llvm::Error MakeStatusError(StatusCode code, const llvm::Twine& message);

inline llvm::Error InvalidArgumentError(const llvm::Twine& message) {
  return MakeStatusError(StatusCode::kInvalidArgument, message);
}
inline llvm::Error FailedPreconditionError(const llvm::Twine& message) {
  return MakeStatusError(StatusCode::kFailedPrecondition, message);
}
inline llvm::Error UnimplementedError(const llvm::Twine& message) {
  return MakeStatusError(StatusCode::kUnimplemented, message);
}
inline llvm::Error InternalError(const llvm::Twine& message) {
  return MakeStatusError(StatusCode::kInternal, message);
}
inline llvm::Error UnknownError(const llvm::Twine& message) {
  return MakeStatusError(StatusCode::kUnknown, message);
}
inline llvm::Error NotFoundError(const llvm::Twine& message) {
  return MakeStatusError(StatusCode::kNotFound, message);
}

StatusCode GetStatusCode(const llvm::Error& error);
std::string GetStatusMessage(const llvm::Error& error);

}  // namespace crubit

#endif  // CRUBIT_COMMON_STATUS_H_
