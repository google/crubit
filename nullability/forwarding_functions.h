// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// This file provides utilities to detect forwarding functions like
// `std::make_unique` and find the underlying function/constructor call.
// The underlying function has more interesting nullability annotations or
// allows inference to be more precise than analyzing the forwarding function
// itself.

#ifndef THIRD_PARTY_CRUBIT_NULLABILITY_FORWARDING_FUNCTIONS_H_
#define THIRD_PARTY_CRUBIT_NULLABILITY_FORWARDING_FUNCTIONS_H_

#include "absl/base/nullability.h"
#include "clang/AST/Decl.h"

namespace clang::tidy::nullability {

// Returns the Initializer from the underlying `new` expression in
// `std::make_unique`. E.g., the initializer is often a `CXXConstructExpr` (but
// can be other expressions), given that `Decl` is an instantiation of
// `std::make_unique` with an interesting template type parameter. Primitives,
// and array template type parameter are not interesting because: (a) there is
// no constructor call to analyze or (b) it is just a 0-arg constructor call.
// The 0-arg constructor call may leave Nonnull fields uninitialized (with
// indeterminate values). We can try to diagnose later during initialization.
// Returns `nullptr` otherwise.
const Expr* absl_nullable getUnderlyingInitExprInStdMakeUnique(
    const FunctionDecl& Decl);

// Returns the last forwarding function layer in the call chain starting
// with `FD`, if `FD` is considered a forwarding function like
// `std::make_unique`.
const FunctionDecl* absl_nullable getLastForwardingFunctionLayer(
    const FunctionDecl& Decl);

}  // namespace clang::tidy::nullability

#endif  // THIRD_PARTY_CRUBIT_NULLABILITY_FORWARDING_FUNCTIONS_H_
