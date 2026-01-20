// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/infer_tu.h"

#include <memory>
#include <optional>
#include <utility>
#include <vector>

#include "nullability/inference/collect_evidence.h"
#include "nullability/inference/inference.proto.h"
#include "nullability/inference/merge.h"
#include "nullability/inference/slot_fingerprint.h"
#include "nullability/inference/usr_cache.h"
#include "nullability/pragma.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/DeclBase.h"
#include "clang/Basic/SourceManager.h"
#include "llvm/ADT/ArrayRef.h"
#include "llvm/ADT/STLExtras.h"
#include "llvm/ADT/STLFunctionalExtras.h"
#include "llvm/ADT/StringMap.h"
#include "llvm/Support/Error.h"
#include "llvm/Support/raw_ostream.h"

namespace clang::tidy::nullability {
namespace {

class InferenceManager {
 public:
  InferenceManager(ASTContext& Ctx, unsigned Iterations,
                   llvm::function_ref<bool(const Decl&)> Filter,
                   const NullabilityPragmas& Pragmas)
      : Ctx(Ctx), Iterations(Iterations), Filter(Filter), Pragmas(Pragmas) {}

  InferenceResults groupAndMergeEvidence(
      std::vector<Evidence> AllEvidence) const {
    // Group by symbol and then slot number.
    llvm::sort(AllEvidence, [&](const Evidence& L, const Evidence& R) {
      if (L.symbol().usr() != R.symbol().usr())
        return L.symbol().usr() < R.symbol().usr();
      return L.slot() < R.slot();
    });
    // For each symbol, for each slot, combine evidence into an inference.
    llvm::ArrayRef<Evidence> RemainingEvidence = AllEvidence;

    InferenceResults AllInference;
    while (!RemainingEvidence.empty()) {
      auto Batch = RemainingEvidence.take_while([&](const Evidence& E) {
        return E.symbol().usr() == RemainingEvidence.front().symbol().usr() &&
               E.slot() == RemainingEvidence.front().slot();
      });
      RemainingEvidence = RemainingEvidence.drop_front(Batch.size());
      AllInference[Batch.front().symbol().usr()][Slot(Batch.front().slot())] =
          mergeEvidence(Batch);
    }
    return AllInference;
  }

  struct FunctionSummariesAndEvidence {
    // Theoretically, could deduplicate Summaries, but there shouldn't generally
    // be duplicates in a single TU.
    std::vector<CFGSummary> Summaries;
    std::vector<Evidence> DeclarationsEvidence;
    llvm::StringMap<MethodSummary> BaseToOverrides;
  };

  FunctionSummariesAndEvidence summarizeFromEvidenceSites(
      const EvidenceSites& Sites, USRCache& USRCache) const {
    FunctionSummariesAndEvidence Result;

    auto VMI = std::make_shared<VirtualMethodIndex>(
        getVirtualMethodIndex(Ctx, USRCache));
    Result.BaseToOverrides = VMI->Overrides;

    // Collect evidence from decls and summaries of definitions.
    auto DeclEmitter = evidenceEmitterWithPropagation(
        [&](Evidence E) { Result.DeclarationsEvidence.push_back(E); }, VMI);
    for (const auto* Decl : Sites.Declarations) {
      if (Filter && !Filter(*Decl)) continue;
      collectEvidenceFromTargetDeclaration(*Decl, DeclEmitter, USRCache,
                                           Pragmas);
    }
    for (const auto* Impl : Sites.Definitions) {
      if (Filter && !Filter(*Impl)) continue;

      if (llvm::Expected<std::optional<CFGSummary>> Summary =
              summarizeDefinition(*Impl, USRCache, Pragmas, *VMI)) {
        if (Summary->has_value()) {
          Result.Summaries.push_back(**std::move(Summary));
        }
      } else {
        llvm::errs() << "Error summarizing definition: " << Summary.takeError()
                     << "\n";
      }
    }
    return Result;
  }

  InferenceResults inferenceRound(
      const FunctionSummariesAndEvidence& SummariesAndEvidence,
      const PreviousInferences& InferencesFromLastRound) const {
    std::vector<Evidence> AllEvidence =
        SummariesAndEvidence.DeclarationsEvidence;

    // Collect evidence from summaries.
    for (const CFGSummary& Summary : SummariesAndEvidence.Summaries) {
      auto VMI = std::make_shared<VirtualMethodIndex>(
          loadVirtualMethodsIndex(Summary.virtual_method_index()));
      VMI->Overrides = SummariesAndEvidence.BaseToOverrides;
      auto Emitter = evidenceEmitterWithPropagation(
          [&](Evidence E) { AllEvidence.push_back(E); }, VMI);
      if (llvm::Error Err = collectEvidenceFromSummary(
              Summary, Emitter, InferencesFromLastRound)) {
        llvm::errs() << "Error collecting evidence from summary "
                     << llvm::toString(std::move(Err)) << "\n";
      }
    }

    return groupAndMergeEvidence(std::move(AllEvidence));
  }

  InferenceResults iterativelyInfer() const {
    if (!Ctx.getLangOpts().CPlusPlus) {
      llvm::errs() << "Skipping non-C++ input file: "
                   << Ctx.getSourceManager()
                          .getFileEntryRefForID(
                              Ctx.getSourceManager().getMainFileID())
                          ->getName()
                   << "\n";
      return InferenceResults();
    }
    auto Sites = EvidenceSites::discover(Ctx);
    USRCache USRCache;

    InferenceResults AllInference;
    FunctionSummariesAndEvidence SummariesAndEvidence =
        summarizeFromEvidenceSites(Sites, USRCache);

    for (unsigned Iteration = 0; Iteration < Iterations; ++Iteration) {
      std::vector<SlotFingerprint> NullableFromLastRound;
      std::vector<SlotFingerprint> NonnullFromLastRound;

      for (const auto& [USR, Inferences] : AllInference) {
        for (const auto& [Slot, SlotInference] : Inferences) {
          if (SlotInference.trivial() || SlotInference.conflict()) continue;
          switch (SlotInference.nullability()) {
            case Nullability::NULLABLE:
              NullableFromLastRound.push_back(fingerprint(USR, Slot));
              break;
            case Nullability::NONNULL:
              NonnullFromLastRound.push_back(fingerprint(USR, Slot));
              break;
            default:
              break;
          }
        }
      }

      AllInference =
          inferenceRound(SummariesAndEvidence,
                         {.Nullable = std::make_shared<SortedFingerprintVector>(
                              std::move(NullableFromLastRound)),
                          .Nonnull = std::make_shared<SortedFingerprintVector>(
                              std::move(NonnullFromLastRound))});
    }
    return AllInference;
  }

 private:
  ASTContext& Ctx;
  unsigned Iterations;
  llvm::function_ref<bool(const Decl&)> Filter;
  const NullabilityPragmas& Pragmas;
};
}  // namespace

InferenceResults inferTU(ASTContext& Ctx, const NullabilityPragmas& Pragmas,
                         unsigned Iterations,
                         llvm::function_ref<bool(const Decl&)> Filter) {
  return InferenceManager(Ctx, Iterations, Filter, Pragmas).iterativelyInfer();
}

}  // namespace clang::tidy::nullability
