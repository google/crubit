// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Provides runtime severity-threshold logging (CRUBIT_LOG) backed by
// llvm::errs(). (LLVM Support LLVM_DEBUG is compiled out under NDEBUG and lacks
// runtime Info/Warning/Error levels).

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_LOGGING_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_LOGGING_H_

#include <atomic>
#include <cstdlib>
#include <iostream>
#include <sstream>
#include <string_view>
#include <type_traits>

namespace crubit {

enum class LogSeverity {
  kInfo = 0,
  kWarning = 1,
  kError = 2,
  kFatal = 3,
};

void SetStderrThreshold(LogSeverity severity);
LogSeverity StderrThreshold();
bool ShouldLog(LogSeverity severity);

namespace internal {
class LogMessage {
 public:
  LogMessage(std::string_view file, int line, LogSeverity severity);
  ~LogMessage();

  template <typename T>
    requires(!requires(std::ostream& os, const T& v) { os << v; } &&
             std::is_convertible_v<const T&, std::string_view>)
  LogMessage& operator<<(const T& value) {
    stream_ << std::string_view(value);
    return *this;
  }

  template <typename T>
  LogMessage& operator<<(const T& value) {
    stream_ << value;
    return *this;
  }

 private:
  std::ostringstream stream_;
  LogSeverity severity_;
};
}  // namespace internal

#define CRUBIT_LOG(sev)                                      \
  if (!::crubit::ShouldLog(::crubit::LogSeverity::k##sev)) { \
  } else                                                     \
    ::crubit::internal::LogMessage(__FILE__, __LINE__,       \
                                   ::crubit::LogSeverity::k##sev)

inline void LogAt(LogSeverity severity, std::string_view file, int line,
                  std::string_view message) {
  if (ShouldLog(severity)) {
    internal::LogMessage(file, line, severity) << message;
  }
}

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_LOGGING_H_
