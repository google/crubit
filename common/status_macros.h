// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Helper macros to return and propagate errors with `absl::Status`,
// `absl::StatusOr<T>`, `llvm::Error`, and `llvm::Expected<T>`.

#ifndef CRUBIT_COMMON_STATUS_MACROS_H_
#define CRUBIT_COMMON_STATUS_MACROS_H_

#include <utility>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "common/status_absl_compat.h"
#include "llvm/Support/Compiler.h"
#include "llvm/Support/Error.h"

namespace crubit::status_macros_internal {

inline bool IsOk(const absl::Status& status) { return status.ok(); }
template <typename T>
inline bool IsOk(const absl::StatusOr<T>& status_or) {
  return status_or.ok();
}
inline bool IsOk(llvm::Error& error) { return !error; }
template <typename T>
inline bool IsOk(llvm::Expected<T>& expected) {
  return static_cast<bool>(expected);
}

inline ErrorReturn TakeError(absl::Status& status) {
  return ErrorReturn(status);
}
template <typename T>
inline ErrorReturn TakeError(absl::StatusOr<T>& status_or) {
  return ErrorReturn(status_or.status());
}
inline ErrorReturn TakeError(llvm::Error& error) {
  return ErrorReturn(std::move(error));
}
template <typename T>
inline ErrorReturn TakeError(llvm::Expected<T>& expected) {
  return ErrorReturn(expected.takeError());
}

}  // namespace crubit::status_macros_internal

// Evaluates `expr`, which produces an `absl::Status` or `llvm::Error`, and
// returns the error from the current function if it is not OK. The enclosing
// function may return any of `absl::Status`, `absl::StatusOr<T>`,
// `llvm::Error`, or `llvm::Expected<T>`.
#define CRUBIT_RETURN_IF_ERROR(expr) \
  CRUBIT_RETURN_IF_ERROR_IMPL_(      \
      CRUBIT_STATUS_MACROS_CONCAT_(_status_macro_val, __COUNTER__), expr)

// Evaluates `rexpr`, which produces an `absl::StatusOr<T>` or
// `llvm::Expected<T>`. On success, moves its value into `lhs`; on error,
// returns the error from the current function.
//
// WARNING: Expands into multiple statements; cannot be used in an unbraced
// `if`/`else` body.
#define CRUBIT_ASSIGN_OR_RETURN(lhs, rexpr)                              \
  CRUBIT_ASSIGN_OR_RETURN_IMPL_(                                         \
      CRUBIT_STATUS_MACROS_CONCAT_(_status_macro_val, __COUNTER__), lhs, \
      rexpr)

#define CRUBIT_RETURN_IF_ERROR_IMPL_(val, expr)                        \
  do {                                                                 \
    auto val = (expr);                                                 \
    if (LLVM_UNLIKELY(!::crubit::status_macros_internal::IsOk(val))) { \
      return ::crubit::status_macros_internal::TakeError(val);         \
    }                                                                  \
  } while (false)

#define CRUBIT_ASSIGN_OR_RETURN_IMPL_(val, lhs, rexpr)               \
  auto val = (rexpr);                                                \
  if (LLVM_UNLIKELY(!::crubit::status_macros_internal::IsOk(val))) { \
    return ::crubit::status_macros_internal::TakeError(val);         \
  }                                                                  \
  lhs = std::move(*val)

#define CRUBIT_STATUS_MACROS_CONCAT_INNER_(x, y) x##y
#define CRUBIT_STATUS_MACROS_CONCAT_(x, y) \
  CRUBIT_STATUS_MACROS_CONCAT_INNER_(x, y)

#endif  // CRUBIT_COMMON_STATUS_MACROS_H_
