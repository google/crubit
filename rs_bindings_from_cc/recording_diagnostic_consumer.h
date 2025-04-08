// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_RECORDING_DIAGNOSTIC_CONSUMER_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_RECORDING_DIAGNOSTIC_CONSUMER_H_

#include <functional>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "clang/include/clang/Basic/Diagnostic.h"
#include "clang/include/clang/Basic/LLVM.h"
#include "clang/include/clang/Basic/SourceLocation.h"
#include "llvm/include/llvm/ADT/SmallString.h"

namespace crubit {

class RecordingDiagnosticConsumer final : public clang::DiagnosticConsumer {
 public:
  void HandleDiagnostic(clang::DiagnosticsEngine::Level diagnostic_level,
                        const clang::Diagnostic& info) override;
  void clear() override;

  struct Diagnostic {
    unsigned diagnostic_id;
    clang::DiagnosticsEngine::Level diagnostic_level;
    std::optional<clang::PresumedLoc> source_location;
    // The size of 64 is chosen based on just a few diagnostics; adjust as
    // needed.
    llvm::SmallString<64> diagnostic;
    std::string FormattedDiagnostics() const;
  };

  const std::vector<Diagnostic>& GetDiagnostics() const {
    return diagnostics_;
  };
  std::string ConcatenatedDiagnostics(absl::string_view prefix = "") const;

 private:
  std::vector<Diagnostic> diagnostics_;
};

/// A function that records the diagnostics emitted by `diagnostic_engine` while
/// running the `callback`, by swapping `diagonostic_engine`'s original
/// diagnostic client with a newly-created `RecordingDiagnosticConsumer` before
/// invoking the callback and restoring the original diagnostic consumer when
/// done.
/// Why? `clang::Sema` methods may fail (e.g., `IsCompleteType`) and the
/// emitted (fatal) diagnostic is sent to the original diagnostic consumer,
/// which causes `clang::tooling::runToolOnCodeWithArgs` to return an error
/// status, which then causes Crubit to exit with failure. In some cases,
/// it's OK for these methods to fail (e.g., to find out if a template
/// specialization can be instantiated and should thus be imported), so it
/// would be helpful to temporarily avoid sending the diagnostics for these
/// fallable attempts to the original diagnostic consumer, and this is where
/// this 'trap' becomes useful.
RecordingDiagnosticConsumer RecordDiagnostics(
    clang::DiagnosticsEngine& diagnostic_engine,
    std::function<void(void)> callback);
}  // namespace crubit

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_RECORDING_DIAGNOSTIC_CONSUMER_H_
