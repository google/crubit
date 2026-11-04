// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "common/status.h"

#include <string>

#include "llvm/ADT/StringRef.h"
#include "llvm/ADT/Twine.h"
#include "llvm/Support/Error.h"

namespace crubit {

char StatusError::ID = 0;

llvm::StringRef StatusCodeToString(StatusCode code) {
  switch (code) {
    case StatusCode::kOk:
      return "OK";
    case StatusCode::kCancelled:
      return "CANCELLED";
    case StatusCode::kUnknown:
      return "UNKNOWN";
    case StatusCode::kInvalidArgument:
      return "INVALID_ARGUMENT";
    case StatusCode::kDeadlineExceeded:
      return "DEADLINE_EXCEEDED";
    case StatusCode::kNotFound:
      return "NOT_FOUND";
    case StatusCode::kAlreadyExists:
      return "ALREADY_EXISTS";
    case StatusCode::kPermissionDenied:
      return "PERMISSION_DENIED";
    case StatusCode::kResourceExhausted:
      return "RESOURCE_EXHAUSTED";
    case StatusCode::kFailedPrecondition:
      return "FAILED_PRECONDITION";
    case StatusCode::kAborted:
      return "ABORTED";
    case StatusCode::kOutOfRange:
      return "OUT_OF_RANGE";
    case StatusCode::kUnimplemented:
      return "UNIMPLEMENTED";
    case StatusCode::kInternal:
      return "INTERNAL";
    case StatusCode::kUnavailable:
      return "UNAVAILABLE";
    case StatusCode::kDataLoss:
      return "DATA_LOSS";
    case StatusCode::kUnauthenticated:
      return "UNAUTHENTICATED";
  }
  return "UNKNOWN";
}

llvm::Error MakeStatusError(StatusCode code, const llvm::Twine& message) {
  return llvm::make_error<StatusError>(code, message.str());
}

StatusCode GetStatusCode(const llvm::Error& error) {
  StatusCode result = StatusCode::kOk;
  bool found_any_error = false;
  llvm::visitErrors(error, [&](const llvm::ErrorInfoBase& err) {
    found_any_error = true;
    if (err.isA<StatusError>()) {
      result = static_cast<const StatusError&>(err).code();
    }
  });
  if (found_any_error && result == StatusCode::kOk) {
    result = StatusCode::kUnknown;
  }
  return result;
}

std::string GetStatusMessage(const llvm::Error& error) {
  bool has_error = false;
  llvm::visitErrors(error,
                    [&](const llvm::ErrorInfoBase& err) { has_error = true; });
  if (!has_error) return "OK";
  return llvm::toStringWithoutConsuming(error);
}

}  // namespace crubit
