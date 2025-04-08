// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_INFERENCE_ELIGIBLE_RANGES_H_
#define CRUBIT_NULLABILITY_INFERENCE_ELIGIBLE_RANGES_H_

#include <optional>
#include <ostream>
#include <string>
#include <vector>

#include "absl/base/nullability.h"
#include "nullability/inference/inference.proto.h"
#include "nullability/inference/usr_cache.h"
#include "nullability/type_nullability.h"
#include "clang/include/clang/AST/DeclBase.h"
#include "llvm/include/llvm/ADT/STLFunctionalExtras.h"
#include "llvm/include/llvm/ADT/StringRef.h"

namespace clang::tidy::nullability {

struct EligibleRange {
  // If empty, this range is not associated with an inferable slot.
  std::optional<Slot> Slot;
  SlotRange Range;
  // Only set when this range is collected by whole-AST `getEligibleRanges`. In
  // other cases, the caller already has the Decl and can compute the USR
  // themselves.
  std::optional<std::string> USR;

  // Enable GoogleTest to print EligibleRange to ease debugging of tests.
  // NOLINTNEXTLINE(readability-identifier-naming) must match GoogleTest naming.
  friend void PrintTo(const EligibleRange& Range, std::ostream* OS) {
    *OS << "Slot: ";
    if (Range.Slot)
      *OS << *Range.Slot << "\n";
    else
      *OS << "nullopt\n";
    *OS << "USR: ";
    if (Range.USR)
      *OS << *Range.USR << "\n";
    else
      *OS << "nullopt\n";
    *OS << "Range: {" << Range.Range.DebugString() << "}\n";
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

/// Runs `Func` with an EligibleRange for each type written in the given AST
/// that is eligible for a nullability annotation.
///
/// Includes the `USR` in each EligibleRange if `USRs` is not null and the USR
/// can be retrieved from `USRs` or be generated.
void forAllEligibleRanges(llvm::function_ref<void(const EligibleRange&)> Func,
                          ASTContext& Ctx,
                          const TypeNullabilityDefaults& Defaults,
                          USRCache* absl_nullable USRs = nullptr,
                          bool RestrictToMainFileOrHeader = false);

/// Return the given string ref without any escaped newline prefixes.
/// Does not support backslashes spelled with trigraphs.
llvm::StringRef skipEscapedNewLinePrefixes(llvm::StringRef Str);

}  // namespace clang::tidy::nullability

#endif  // CRUBIT_NULLABILITY_INFERENCE_ELIGIBLE_RANGES_H_
