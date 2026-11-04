// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/logging.h"

#include <atomic>
#include <cstdlib>
#include <string_view>

#include "llvm/Support/raw_ostream.h"

namespace crubit {

static std::atomic<LogSeverity> g_stderr_threshold{LogSeverity::kError};

void SetStderrThreshold(LogSeverity severity) {
  g_stderr_threshold.store(severity, std::memory_order_relaxed);
}

LogSeverity StderrThreshold() {
  return g_stderr_threshold.load(std::memory_order_relaxed);
}

bool ShouldLog(LogSeverity severity) {
  return static_cast<int>(severity) >= static_cast<int>(StderrThreshold());
}

namespace internal {

LogMessage::LogMessage(std::string_view file, int line, LogSeverity severity)
    : severity_(severity) {
  const char* sev_str = "UNKNOWN";
  switch (severity) {
    case LogSeverity::kInfo:
      sev_str = "INFO";
      break;
    case LogSeverity::kWarning:
      sev_str = "WARNING";
      break;
    case LogSeverity::kError:
      sev_str = "ERROR";
      break;
    case LogSeverity::kFatal:
      sev_str = "FATAL";
      break;
  }
  stream_ << "[" << sev_str << " " << file << ":" << line << "] ";
}

LogMessage::~LogMessage() {
  stream_ << "\n";
  llvm::errs() << stream_.str();
  llvm::errs().flush();
  if (severity_ == LogSeverity::kFatal) {
    std::abort();
  }
}

}  // namespace internal
}  // namespace crubit
