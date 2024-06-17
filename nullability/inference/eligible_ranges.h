// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_INFERENCE_ELIGIBLE_RANGES_H_
#define CRUBIT_NULLABILITY_INFERENCE_ELIGIBLE_RANGES_H_

#include <optional>

#include "nullability/inference/inference.proto.h"
#include "nullability/type_nullability.h"
#include "clang/AST/DeclBase.h"

namespace clang::tidy::nullability {

/// Collects the ranges of types written in the given declaration that are
/// eligible for nullability annotations. Essentially, all pointer types. If the
/// return value is populated, none of its fields will be empty.
///
/// Fields in the return value account for the existing nullability annotations
/// including modification by file-level defaults as provided by `Defaults`.
std::optional<TypeLocRanges> getEligibleRanges(
    const Decl& D, const TypeNullabilityDefaults& Defaults);

/// Collect the ranges of types written in the given declaration that are
/// eligible for nullability annotations, if the given declaration is an
/// inference target.
std::optional<TypeLocRanges> getInferenceRanges(
    const Decl& D, const TypeNullabilityDefaults& Defaults);

}  // namespace clang::tidy::nullability

#endif  // CRUBIT_NULLABILITY_INFERENCE_ELIGIBLE_RANGES_H_
