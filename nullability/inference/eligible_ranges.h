// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_INFERENCE_ELIGIBLE_RANGES_H_
#define CRUBIT_NULLABILITY_INFERENCE_ELIGIBLE_RANGES_H_

#include <vector>

#include "nullability/inference/inference.proto.h"
#include "nullability/type_nullability.h"
#include "clang/AST/DeclBase.h"

namespace clang::tidy::nullability {

/// Collects the ranges of types written in the given declaration that are
/// eligible for nullability annotations. Essentially, all pointer types.
///
/// Fields in the return value account for the existing nullability annotations
/// including modification by file-level defaults as provided by `Defaults`.
std::vector<SlotRange> getEligibleRanges(
    const Decl& D, const TypeNullabilityDefaults& Defaults);

/// Collect the ranges of types written in the given declaration that are
/// eligible for nullability annotations, if the given declaration is an
/// inference target.
std::vector<SlotRange> getInferenceRanges(
    const Decl& D, const TypeNullabilityDefaults& Defaults);

/// Collects the ranges of types written in the given AST that are eligible for
/// nullability annotations.
std::vector<SlotRange> getEligibleRanges(
    ASTContext& Ctx, const TypeNullabilityDefaults& Defaults,
    bool RestrictToMainFileOrHeader = false);

}  // namespace clang::tidy::nullability

#endif  // CRUBIT_NULLABILITY_INFERENCE_ELIGIBLE_RANGES_H_
