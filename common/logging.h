// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_COMMON_LOGGING_H_
#define CRUBIT_COMMON_LOGGING_H_

#include <string>

#include "llvm/Support/raw_ostream.h"

namespace crubit {

enum class LogSeverity {
  kInfo = 0,
  kWarning = 1,
  kError = 2,
  kFatal = 3,
};

void SetMinLogSeverity(LogSeverity severity);
LogSeverity MinLogSeverity();
bool ShouldLog(LogSeverity severity);

namespace internal {
class LogStream {
 public:
  LogStream(const char* file, int line, LogSeverity severity) {
    llvm::errs() << file << ":" << line << ": ";
  }

  ~LogStream() { llvm::errs() << "\n"; }

  template <typename T>
  LogStream& operator<<(const T& value) {
    llvm::errs() << value;
    return *this;
  }
};
}  // namespace internal

#define CRUBIT_LOG(sev)                                      \
  if (!::crubit::ShouldLog(::crubit::LogSeverity::k##sev)) { \
  } else                                                     \
    ::crubit::internal::LogStream(__FILE__, __LINE__,        \
                                  ::crubit::LogSeverity::k##sev)

}  // namespace crubit

#endif  // CRUBIT_COMMON_LOGGING_H_
