// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/recording_diagnostic_consumer.h"

#include <memory>
#include <optional>
#include <string>
#include <utility>

#include "common/check.h"
#include "clang/Basic/Diagnostic.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Basic/SourceManager.h"
#include "llvm/ADT/ScopeExit.h"
#include "llvm/ADT/SmallString.h"
#include "llvm/ADT/StringExtras.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/FormatVariadic.h"

namespace crubit {

// Clang doesn't provide a public utility method to stringify the diagnostic
// level, so we roll our own here.
static ::llvm::StringRef GetDiagnosticLevelName(
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
      CRUBIT_CHECK(false) << "Unknown diagnostic level: "
                          << static_cast<int>(level);
      return "<unknown diagnostic level>";
  };
}

std::string RecordingDiagnosticConsumer::Diagnostic::FormattedDiagnostics()
    const {
  constexpr llvm::StringRef kDiagnosticFormat = "{0}:{1}:{2}: {3}: {4}";
  if (source_location.has_value()) {
    llvm::StringRef filename = source_location->getFilename();
    filename.consume_front("./");
    return llvm::formatv(
        kDiagnosticFormat.data(), filename, source_location->getLine(),
        source_location->getColumn(), GetDiagnosticLevelName(diagnostic_level),
        diagnostic.str());
  } else {
    return llvm::formatv("{0}: {1}", GetDiagnosticLevelName(diagnostic_level),
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

  if (on_error_.has_value() &&
      diagnostic_level >= clang::DiagnosticsEngine::Error) {
    (*on_error_)();
  }
}

void RecordingDiagnosticConsumer::clear() {
  clang::DiagnosticConsumer::clear();
  diagnostics_.clear();
}

std::string RecordingDiagnosticConsumer::ConcatenatedDiagnostics(
    llvm::StringRef prefix) const {
  // In reverse order, so that it's easier to backtrace.
  std::vector<std::string> formatted_diagnostics;
  formatted_diagnostics.reserve(diagnostics_.size());
  for (auto it = diagnostics_.rbegin(); it != diagnostics_.rend(); ++it) {
    formatted_diagnostics.push_back(it->FormattedDiagnostics());
  }
  std::string concatenated = llvm::join(formatted_diagnostics.begin(),
                                        formatted_diagnostics.end(), "\n");
  if (concatenated.empty()) {
    return concatenated;
  }
  return prefix.str() + concatenated;
}

RecordingDiagnosticConsumer RecordDiagnostics(
    clang::DiagnosticsEngine& diagnostic_engine,
    llvm::function_ref<void()> callback,
    std::optional<llvm::function_ref<void()>> on_error) {
  // Reset the diagnostic engine to a known state. In particular, if there were
  // too many diagnostics reported previously (even in sfinae contexts),
  // the diagnostic engine's fatal bit will get stuck on.
  diagnostic_engine.Reset(/*soft=*/true);
  RecordingDiagnosticConsumer diagnostic_recorder;
  diagnostic_recorder.on_error_ = on_error;
  // `takeClient` only transfers ownership (if any) - it returns null when
  // `diagnostic_engine` doesn't own its client (e.g. a consumer from
  // `ToolInvocation::setDiagnosticConsumer`), and `getClient` still returns
  // the original client afterwards.
  std::unique_ptr<clang::DiagnosticConsumer> owned_original_consumer =
      diagnostic_engine.takeClient();
  clang::DiagnosticConsumer* original_consumer = diagnostic_engine.getClient();
  diagnostic_engine.setClient(&diagnostic_recorder, /*ShouldOwnClient=*/false);
  auto restore_consumer = llvm::scope_exit([&] {
    diagnostic_engine.setClient(
        original_consumer,
        /*ShouldOwnClient=*/owned_original_consumer.release() != nullptr);
  });
  callback();
  // `on_error` refers to the caller's state; don't let it outlive this call.
  diagnostic_recorder.on_error_ = std::nullopt;
  return diagnostic_recorder;
}

}  // namespace crubit
