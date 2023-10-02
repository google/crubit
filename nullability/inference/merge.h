// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Combines evidence of the nullability of a symbol into inference conclusions.
//
// This evidence was gathered from local analysis across the codebase in a
// "map" phase, merging this evidence is conceptually the "reduce" phase.
//
// To allow this to be distributed efficiently for commonly-used symbols,
// merging is performed incrementally by repeatedly combining partial results.

#ifndef CRUBIT_NULLABILITY_INFERENCE_MERGE_H_
#define CRUBIT_NULLABILITY_INFERENCE_MERGE_H_

#include <vector>

#include "nullability/inference/inference.proto.h"
#include "llvm/ADT/ArrayRef.h"

namespace clang::tidy::nullability {

// Build a Partial representing a single piece of evidence.
Partial partialFromEvidence(const Evidence &);
// Update LHS to include the evidence from RHS.
// The two must describe the same symbol.
// The merging of partials is commutative and associative.
void mergePartials(Partial &LHS, const Partial &RHS);
// Form nullability conclusions from a set of evidence.
Inference finalize(const Partial &);

struct InferResult {
  Inference::Nullability Nullability;
  bool Conflict = false;
  bool Trivial = false;
};
// Final inference decisions, based on event counts.
// TODO: once this interface sticks, move to a dedicated file.
InferResult infer(llvm::ArrayRef<unsigned> EventCounts);

// Combines local evidence about symbol nullability to form a global conclusion.
// All evidence must for be the same symbol, and there must be some.
//
// This signature fundamentally limits the scalability of merging: we must see
// all the evidence for a symbol at once.
inline Inference mergeEvidence(llvm::ArrayRef<Evidence> Ev) {
  Partial P = partialFromEvidence(Ev.front());
  for (const auto &E : Ev.drop_front())
    mergePartials(P, partialFromEvidence(E));
  return finalize(P);
}

}  // namespace clang::tidy::nullability

#endif
