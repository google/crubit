// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_INFERENCE_ELIGIBLE_RANGES_H_
#define CRUBIT_NULLABILITY_INFERENCE_ELIGIBLE_RANGES_H_

#include <optional>

#include "nullability/inference/inference.proto.h"
#include "clang/AST/DeclBase.h"

namespace clang::tidy::nullability {

// Collects the ranges of types written in the given function declaration that
// are eligible for nullability annotations. Essentially, all pointer types.  If
// the return value is populated, none of its fields will be empty.
std::optional<TypeLocRanges> getEligibleRanges(const Decl& D);

}  // namespace clang::tidy::nullability

#endif  // CRUBIT_NULLABILITY_INFERENCE_ELIGIBLE_RANGES_H_
