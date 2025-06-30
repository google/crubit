// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/merge.h"

#include <array>
#include <optional>
#include <utility>

#include "absl/log/check.h"
#include "nullability/inference/inference.proto.h"
#include "llvm/ADT/ArrayRef.h"
#include "llvm/ADT/STLExtras.h"

namespace clang::tidy::nullability {
namespace {

static void mergeSampleLocations(SlotPartial::SampleLocations &LHS,
                                 const SlotPartial::SampleLocations &RHS) {
  static constexpr unsigned Limit = 3;
  // We don't care which we pick, but they should be unique.
  // Multiple instantiations of the same template are not interesting.
  for (const auto &Loc : RHS.location()) {
    if (LHS.location_size() >= Limit) break;
    // Linear scan is fine because Limit is tiny.
    if (!llvm::is_contained(LHS.location(), Loc)) LHS.add_location(Loc);
  }
}

}  // namespace

SlotPartial partialFromEvidence(const Evidence &E) {
  SlotPartial P;
  ++(*P.mutable_kind_count())[E.kind()];
  if (E.has_location())
    (*P.mutable_kind_samples())[E.kind()].add_location(E.location());
  return P;
}

void mergePartials(SlotPartial &LHS, const SlotPartial &RHS) {
  for (auto [Kind, Count] : RHS.kind_count())
    (*LHS.mutable_kind_count())[Kind] += Count;
  for (const auto &[Kind, Samples] : RHS.kind_samples())
    mergeSampleLocations((*LHS.mutable_kind_samples())[Kind], Samples);
}

// Form a nullability conclusion from a set of evidence.
SlotInference finalize(const SlotPartial &P, bool EnableSoftRules) {
  SlotInference Inference;
  if (P.kind_count_size() == 0) return Inference;

  // Reconstitute samples, if we have them.
  for (const auto &[Kind, Samples] : P.kind_samples()) {
    for (const auto &Loc : Samples.location()) {
      auto *Sample = Inference.add_sample_evidence();
      Sample->set_location(Loc);
      Sample->set_kind(static_cast<Evidence::Kind>(Kind));
    }
  }
  llvm::stable_sort(*Inference.mutable_sample_evidence(),
                    [&](auto &L, auto &R) {
                      return std::forward_as_tuple(L.kind(), L.location()) <
                             std::forward_as_tuple(R.kind(), R.location());
                    });

  std::array<unsigned, Evidence::Kind_MAX + 1> KindCounts = {};
  for (auto [Kind, Count] : P.kind_count()) KindCounts[Kind] = Count;
  auto Result = infer(KindCounts, EnableSoftRules);
  Inference.set_nullability(Result.Nullability);
  if (Result.Conflict) Inference.set_conflict(true);
  if (Result.Trivial) Inference.set_trivial(true);
  return Inference;
}

namespace {
void update(std::optional<InferResult> &Result,
            Nullability ImpliedNullability) {
  if (!Result) {
    Result = {ImpliedNullability};
    return;
  }
  if (Result->Nullability != ImpliedNullability)
    // Leave the existing Nullability.
    Result->Conflict = true;
}
}  // namespace

InferResult infer(llvm::ArrayRef<unsigned> Counts, bool EnableSoftRules) {
  CHECK_EQ(Counts.size(), Evidence::Kind_MAX + 1);
  // Annotations take precedence over everything.
  // If some other evidence is incompatible with an annotation, that's not
  // an inference conflict, just an error to be caught by verification.
  if (Counts[Evidence::ANNOTATED_NONNULL] &&
      Counts[Evidence::ANNOTATED_NULLABLE]) {
    return {Nullability::UNKNOWN, /*Conflict=*/true};
  }
  if (Counts[Evidence::ANNOTATED_NONNULL])
    return {Nullability::NONNULL, /*Conflict=*/false, /*Trivial=*/true};
  if (Counts[Evidence::ANNOTATED_NULLABLE])
    return {Nullability::NULLABLE, /*Conflict=*/false, /*Trivial=*/true};

  bool AnyAssignmentFromNullable =
      Counts[Evidence::ASSIGNED_FROM_NULLABLE] ||
      (Counts[Evidence::LEFT_NULLABLE_BY_CONSTRUCTOR] &&
       !Counts[Evidence::LEFT_NOT_NULLABLE_BY_LATE_INITIALIZER]);

  // Mandatory inference rules, required by type-checking.
  // Ordered from most confident to least.
  std::optional<InferResult> Result;
  if (Counts[Evidence::UNCHECKED_DEREFERENCE])
    update(Result, Nullability::NONNULL);
  if (Counts[Evidence::NULLABLE_ARGUMENT])
    update(Result, Nullability::NULLABLE);
  if (Counts[Evidence::NULLABLE_REFERENCE_ARGUMENT])
    update(Result, Nullability::NULLABLE);
  if (Counts[Evidence::NONNULL_REFERENCE_ARGUMENT])
    update(Result, Nullability::NONNULL);
  if (AnyAssignmentFromNullable) update(Result, Nullability::NULLABLE);
  if (Counts[Evidence::NULLABLE_RETURN]) update(Result, Nullability::NULLABLE);
  if (Counts[Evidence::NULLABLE_REFERENCE_RETURN])
    update(Result, Nullability::NULLABLE);
  if (Counts[Evidence::NONNULL_REFERENCE_RETURN])
    update(Result, Nullability::NONNULL);
  if (Counts[Evidence::ASSIGNED_TO_NONNULL])
    update(Result, Nullability::NONNULL);
  if (Counts[Evidence::ASSIGNED_TO_MUTABLE_NULLABLE])
    update(Result, Nullability::NULLABLE);
  if (Counts[Evidence::ASSIGNED_TO_NONNULL_REFERENCE])
    update(Result, Nullability::NONNULL);
  if (Counts[Evidence::ABORT_IF_NULL]) update(Result, Nullability::NONNULL);
  if (Counts[Evidence::ARITHMETIC]) update(Result, Nullability::NONNULL);
  if (Counts[Evidence::ARRAY_SUBSCRIPT]) update(Result, Nullability::NONNULL);
  if (Result) return *Result;

  if (!EnableSoftRules) return {Nullability::UNKNOWN};

  // Optional "soft" inference heuristics.
  // These do not report conflicts.
  if (Counts[Evidence::WELL_KNOWN_NONNULL]) return {Nullability::NONNULL};
  if (Counts[Evidence::WELL_KNOWN_NULLABLE]) return {Nullability::NULLABLE};
  if (Counts[Evidence::GCC_NONNULL_ATTRIBUTE]) return {Nullability::NONNULL};
  if (!Counts[Evidence::NULLABLE_RETURN] && !Counts[Evidence::UNKNOWN_RETURN] &&
      Counts[Evidence::NONNULL_RETURN])
    return {Nullability::NONNULL};
  if (!Counts[Evidence::NULLABLE_ARGUMENT] &&
      !Counts[Evidence::UNKNOWN_ARGUMENT] && Counts[Evidence::NONNULL_ARGUMENT])
    return {Nullability::NONNULL};
  if (Counts[Evidence::NULLPTR_DEFAULT_MEMBER_INITIALIZER])
    return {Nullability::NULLABLE};
  if (!AnyAssignmentFromNullable && !Counts[Evidence::ASSIGNED_FROM_UNKNOWN] &&
      Counts[Evidence::ASSIGNED_FROM_NONNULL])
    return {Nullability::NONNULL};
  if (!Counts[Evidence::NULLABLE_REFERENCE_RETURN] &&
      !Counts[Evidence::UNKNOWN_REFERENCE_RETURN] &&
      !Counts[Evidence::NONNULL_REFERENCE_RETURN] &&
      Counts[Evidence::NONNULL_REFERENCE_RETURN_AS_CONST])
    return {Nullability::NONNULL};
  if (!Counts[Evidence::NULLABLE_REFERENCE_ARGUMENT] &&
      !Counts[Evidence::UNKNOWN_REFERENCE_ARGUMENT] &&
      !Counts[Evidence::NONNULL_REFERENCE_ARGUMENT] &&
      Counts[Evidence::NONNULL_REFERENCE_ARGUMENT_AS_CONST])
    return {Nullability::NONNULL};

  return {Nullability::UNKNOWN};
}

}  // namespace clang::tidy::nullability
