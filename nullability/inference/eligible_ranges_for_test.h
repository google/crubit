// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_INFERENCE_ELIGIBLE_RANGES_FOR_TEST_H_
#define CRUBIT_NULLABILITY_INFERENCE_ELIGIBLE_RANGES_FOR_TEST_H_

#include "nullability/inference/eligible_ranges.h"
#include "nullability/type_nullability.h"
#include "clang/AST/ASTContext.h"

namespace clang::tidy::nullability {
/// Collects the ranges of types written in the given AST that are eligible for
/// nullability annotations.
///
/// Note that for large ASTs, this function accumulates a large vector of
/// EligibleRanges, which is slow and memory-intensive. It is only suitable
/// for testing.
inline EligibleRanges getEligibleRanges(
    ASTContext& Ctx, const TypeNullabilityDefaults& Defaults) {
  EligibleRanges Ranges;
  forAllEligibleRanges(
      [&Ranges](const EligibleRange& Range) { Ranges.push_back(Range); }, Ctx,
      Defaults);
  return Ranges;
}
}  // namespace clang::tidy::nullability

#endif  // CRUBIT_NULLABILITY_INFERENCE_ELIGIBLE_RANGES_FOR_TEST_H_
