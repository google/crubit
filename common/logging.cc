// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "common/logging.h"

namespace crubit {

static LogSeverity g_min_log_severity = LogSeverity::kError;

void SetMinLogSeverity(LogSeverity severity) { g_min_log_severity = severity; }

LogSeverity MinLogSeverity() { return g_min_log_severity; }

bool ShouldLog(LogSeverity severity) {
  return static_cast<int>(severity) >= static_cast<int>(g_min_log_severity);
}

}  // namespace crubit
