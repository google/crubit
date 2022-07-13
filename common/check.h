// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_COMMON_CHECK_H_
#define CRUBIT_COMMON_CHECK_H_

#include <string>
#include <utility>

#include "absl/base/attributes.h"
#include "absl/base/optimization.h"
#include "absl/strings/string_view.h"
#include "absl/types/source_location.h"
#include "llvm/Support/ErrorHandling.h"
#include "llvm/Support/FormatVariadic.h"

#define CRUBIT_CHECK(condition)                                           \
  do {                                                                    \
    if (ABSL_PREDICT_FALSE(!(condition))) {                               \
      ::llvm::report_fatal_error(                                         \
          ::llvm::formatv("CRUBIT_CHECK failure: {0}:{1}: {2}", __FILE__, \
                          __LINE__, #condition));                         \
    }                                                                     \
  } while (false)

namespace crubit {

// Terminate the binary.
//
// Similar to `llvm::report_fatal_error` but making it slightly easier to report
// the source location of the caller.
//
[[noreturn]] void inline ReportFatalError(
    absl::string_view error_message,
    absl::SourceLocation caller_location = absl::SourceLocation::current()) {
  auto full_message = llvm::formatv("Fatal Crubit failure: {0}:{1}: {2}",
                                    caller_location.file_name(),
                                    caller_location.line(), error_message);
   llvm::report_fatal_error(std::move(full_message));
}

// Helper for CRUBIT_DIE_IF_NULL.
//
template <typename T>
ABSL_MUST_USE_RESULT T DieIfNull(const char* file, int line,
                                 const char* exprtext, T&& t) {
  if (ABSL_PREDICT_FALSE(t == nullptr)) {
    ::llvm::report_fatal_error(llvm::formatv(
        "CRUBIT_DIE_IF_NULL failure: {0}:{1}: {2}", file, line, exprtext));
  }
  return std::forward<T>(t);
}

}  // namespace crubit

#define CRUBIT_DIE_IF_NULL(value) \
  ::crubit::DieIfNull(__FILE__, __LINE__, #value, (value))

#endif  // CRUBIT_COMMON_CHECK_H_
