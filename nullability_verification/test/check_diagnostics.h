// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_NULLABILITY_VERIFICATION_TEST_CHECK_DIAGNOSTICS_H_
#define THIRD_PARTY_CRUBIT_NULLABILITY_VERIFICATION_TEST_CHECK_DIAGNOSTICS_H_

#include "llvm/ADT/StringRef.h"

namespace clang {
namespace tidy {
namespace nullability {

/// Runs nullability verification on `SourceCode` and returns whether
/// diagnostics are produced on those lines marked in the source code with
/// `llvm::Annotations` style annotations (and no other lines).
/// TODO(mboehme): So far, we only check the locations of the diagnostics; it
/// would be desirable to check their actual content too.
bool checkDiagnostics(llvm::StringRef SourceCode);

}  // namespace nullability
}  // namespace tidy
}  // namespace clang

#endif  // THIRD_PARTY_CRUBIT_NULLABILITY_VERIFICATION_TEST_CHECK_DIAGNOSTICS_H_
