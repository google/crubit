// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/merge.h"

#include <array>
#include <utility>

#include "absl/log/check.h"
#include "nullability/inference/inference.proto.h"
#include "llvm/ADT/ArrayRef.h"
#include "llvm/ADT/STLExtras.h"

namespace clang::tidy::nullability {
namespace {

static void mergeSampleLocations(Partial::SampleLocations &LHS,
                                 const Partial::SampleLocations &RHS) {
  static constexpr unsigned Limit = 3;
  // We don't care which we pick, but they should be unique.
  // Multiple instantiations of the same template are not interesting.
  for (const auto &Loc : RHS.location()) {
    if (LHS.location_size() >= Limit) break;
    // Linear scan is fine because Limit is tiny.
    if (!llvm::is_contained(LHS.location(), Loc)) LHS.add_location(Loc);
  };
}

static void mergeSlotPartials(Partial::SlotPartial &LHS,
                              const Partial::SlotPartial &RHS) {
  for (auto [Kind, Count] : RHS.kind_count())
    (*LHS.mutable_kind_count())[Kind] += Count;
  for (const auto &[Kind, Samples] : RHS.kind_samples())
    mergeSampleLocations((*LHS.mutable_kind_samples())[Kind], Samples);
}

}  // namespace

Partial partialFromEvidence(const Evidence &E) {
  Partial P;
  *P.mutable_symbol() = E.symbol();
  // We want to set P.slot[E.slot], so populate previous slots.
  while (P.slot_size() < E.slot()) P.add_slot();
  auto *S = P.add_slot();
  ++(*S->mutable_kind_count())[E.kind()];
  if (E.has_location())
    (*S->mutable_kind_samples())[E.kind()].add_location(E.location());
  return P;
}

void mergePartials(Partial &LHS, const Partial &RHS) {
  CHECK_EQ(LHS.symbol().usr(), RHS.symbol().usr());
  auto *Slots = LHS.mutable_slot();
  while (RHS.slot_size() > Slots->size()) Slots->Add();
  for (unsigned I = 0; I < RHS.slot_size(); ++I)
    mergeSlotPartials(*LHS.mutable_slot(I), RHS.slot(I));
}

// Form nullability conclusions from a set of evidence.
Inference finalize(const Partial &P) {
  Inference Result;
  *Result.mutable_symbol() = P.symbol();
  for (unsigned I = 0; I < P.slot_size(); ++I) {
    if (P.slot(I).kind_count_size() == 0) continue;
    auto &Slot = *Result.add_slot_inference();
    Slot.set_slot(I);

    // Reconstitute samples, if we have them.
    for (const auto &[Kind, Samples] : P.slot(I).kind_samples()) {
      for (const auto &Loc : Samples.location()) {
        auto *Sample = Slot.add_sample_evidence();
        Sample->set_location(Loc);
        Sample->set_kind(static_cast<Evidence::Kind>(Kind));
      }
    }
    llvm::stable_sort(*Slot.mutable_sample_evidence(), [&](auto &L, auto &R) {
      return std::forward_as_tuple(L.kind(), L.location()) <
             std::forward_as_tuple(R.kind(), R.location());
    });

    std::array<unsigned, Evidence::Kind_MAX + 1> KindCounts = {};
    for (auto [Kind, Count] : P.slot(I).kind_count()) KindCounts[Kind] = Count;
    auto Result = infer(KindCounts);
    Slot.set_nullability(Result.Nullability);
    if (Result.Conflict) Slot.set_conflict(true);
  }
  return Result;
}

InferResult infer(llvm::ArrayRef<unsigned> Counts) {
  CHECK_EQ(Counts.size(), Evidence::Kind_MAX + 1);
  // Annotations take precedence over everything.
  // If some other evidence is incompatible with an annotation, that's not
  // an inference conflict, just an error to be caught by verification.
  if (Counts[Evidence::ANNOTATED_NONNULL] &&
      Counts[Evidence::ANNOTATED_NULLABLE]) {
    return {Inference::UNKNOWN, /*Conflict=*/true};
  }
  if (Counts[Evidence::ANNOTATED_NONNULL]) return {Inference::NONNULL};
  if (Counts[Evidence::ANNOTATED_NULLABLE]) return {Inference::NULLABLE};

  // Mandatory inference rules, required by type-checking.
  // TODO: report conflicts between these.
  if (Counts[Evidence::UNCHECKED_DEREFERENCE]) return {Inference::NONNULL};

  // TODO: Optional "soft" inference heuristics.
  // These do not report conflicts.

  return {Inference::UNKNOWN};
}

}  // namespace clang::tidy::nullability
