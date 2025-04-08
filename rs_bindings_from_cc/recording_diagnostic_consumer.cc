// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/recording_diagnostic_consumer.h"

#include <cassert>
#include <functional>
#include <memory>
#include <optional>
#include <string>
#include <utility>

#include "absl/log/log.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/str_format.h"
#include "absl/strings/str_join.h"
#include "absl/strings/string_view.h"
#include "absl/strings/strip.h"
#include "clang/Basic/Diagnostic.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Basic/SourceManager.h"
#include "llvm/ADT/SmallString.h"

namespace crubit {

// Clang doesn't provide a public utility method to stringify the diagnostic
// level, so we roll our own here.
static ::absl::string_view GetDiagnosticLevelName(
    clang::DiagnosticsEngine::Level level) {
  switch (level) {
    case clang::DiagnosticsEngine::Ignored:
      return "ignored";
    case clang::DiagnosticsEngine::Remark:
      return "remark";
    case clang::DiagnosticsEngine::Note:
      return "note";
    case clang::DiagnosticsEngine::Warning:
      return "warning";
    case clang::DiagnosticsEngine::Error:
      return "error";
    case clang::DiagnosticsEngine::Fatal:
      return "fatal error";
    default:
      LOG(DFATAL) << "Unknown diagnostic level: " << static_cast<int>(level);
      return "<unknown diagnostic level>";
  };
}

std::string RecordingDiagnosticConsumer::Diagnostic::FormattedDiagnostics()
    const {
  constexpr absl::string_view kDiagnosticFormat = "%s:%u:%u: %s: %s";
  if (source_location.has_value()) {
    return absl::StrFormat(
        kDiagnosticFormat,
        // `getFileName` has the form `./relative/path.h`.
        absl::StripPrefix(source_location->getFilename(), "./"),
        source_location->getLine(), source_location->getColumn(),
        GetDiagnosticLevelName(diagnostic_level), diagnostic.str());
  } else {
    return absl::StrFormat("%s: %s", GetDiagnosticLevelName(diagnostic_level),
                           diagnostic.str());
  }
};

void RecordingDiagnosticConsumer::HandleDiagnostic(
    clang::DiagnosticsEngine::Level diagnostic_level,
    const clang::Diagnostic& info) {
  clang::DiagnosticConsumer::HandleDiagnostic(diagnostic_level, info);

  llvm::SmallString<64> diagnostic;
  info.FormatDiagnostic(diagnostic);
  auto source_loc = info.getLocation();
  std::optional<clang::PresumedLoc> presumed_source_loc;
  if (source_loc.isValid()) {
    presumed_source_loc = info.getSourceManager().getPresumedLoc(source_loc);
  }
  diagnostics_.push_back({info.getID(), diagnostic_level, presumed_source_loc,
                          std::move(diagnostic)});
}

void RecordingDiagnosticConsumer::clear() {
  clang::DiagnosticConsumer::clear();
  diagnostics_.clear();
}

std::string RecordingDiagnosticConsumer::ConcatenatedDiagnostics(
    absl::string_view prefix) const {
  // In reverse order, so that it's easier to backtrace.
  std::string concatenated =
      absl::StrJoin(diagnostics_.rbegin(), diagnostics_.rend(), "\n",
                    [](std::string* out, const Diagnostic diagnostic) {
                      absl::StrAppend(out, diagnostic.FormattedDiagnostics());
                    });
  if (concatenated.empty()) {
    return concatenated;
  }
  return absl::StrCat(prefix, concatenated);
}

RecordingDiagnosticConsumer RecordDiagnostics(
    clang::DiagnosticsEngine& diagnostic_engine,
    std::function<void(void)> callback) {
  RecordingDiagnosticConsumer diagnostic_recorder;
  std::unique_ptr<clang::DiagnosticConsumer> original_consumer =
      diagnostic_engine.takeClient();
  diagnostic_engine.setClient(&diagnostic_recorder, /*ShouldOwnClient=*/false);
  callback();
  diagnostic_engine.setClient(original_consumer.release(),
                              /*ShouldOwnClient=*/true);
  return diagnostic_recorder;
}

}  // namespace crubit
