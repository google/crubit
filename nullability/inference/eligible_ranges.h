// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_INFERENCE_ELIGIBLE_RANGES_H_
#define CRUBIT_NULLABILITY_INFERENCE_ELIGIBLE_RANGES_H_

#include <optional>
#include <ostream>
#include <vector>

#include "nullability/inference/inference.proto.h"
#include "nullability/type_nullability.h"
#include "clang/AST/DeclBase.h"
#include "llvm/ADT/StringRef.h"

namespace clang::tidy::nullability {

struct EligibleRange {
  std::optional<Slot> Slot;
  SlotRange Range;

  // Enable GoogleTest to print EligibleRange to ease debugging of tests.
  // NOLINTNEXTLINE(readability-identifier-naming) must match GoogleTest naming.
  friend void PrintTo(const EligibleRange& Range, std::ostream* OS) {
    *OS << "Slot: ";
    if (Range.Slot)
      *OS << *Range.Slot;
    else
      *OS << "nullopt";
    *OS << "\nRange: {" << Range.Range.DebugString() << "}\n";
  }
};
using EligibleRanges = std::vector<EligibleRange>;

/// Collects the ranges of types written in the given declaration that are
/// eligible for nullability annotations. Essentially, all pointer types.
///
/// Fields in the return value account for the existing nullability annotations
/// including modification by file-level defaults as provided by `Defaults`.
EligibleRanges getEligibleRanges(const Decl& D,
                                 const TypeNullabilityDefaults& Defaults);

/// Collect the ranges of types written in the given declaration that are
/// eligible for nullability annotations, if the given declaration is an
/// inference target.
EligibleRanges getInferenceRanges(const Decl& D,
                                  const TypeNullabilityDefaults& Defaults);

/// Collects the ranges of types written in the given AST that are eligible for
/// nullability annotations.
EligibleRanges getEligibleRanges(ASTContext& Ctx,
                                 const TypeNullabilityDefaults& Defaults,
                                 bool RestrictToMainFileOrHeader = false);

/// Return the given string ref without any escaped newline prefixes.
/// Does not support backslashes spelled with trigraphs.
llvm::StringRef skipEscapedNewLinePrefixes(llvm::StringRef Str);

}  // namespace clang::tidy::nullability

#endif  // CRUBIT_NULLABILITY_INFERENCE_ELIGIBLE_RANGES_H_
