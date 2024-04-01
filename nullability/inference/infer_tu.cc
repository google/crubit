// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/infer_tu.h"

#include <utility>
#include <vector>

#include "nullability/inference/collect_evidence.h"
#include "nullability/inference/inference.proto.h"
#include "nullability/inference/merge.h"
#include "nullability/inference/slot_fingerprint.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/DeclBase.h"
#include "clang/Basic/SourceManager.h"
#include "llvm/ADT/ArrayRef.h"
#include "llvm/ADT/DenseSet.h"
#include "llvm/ADT/STLExtras.h"
#include "llvm/ADT/STLFunctionalExtras.h"
#include "llvm/Support/Error.h"
#include "llvm/Support/raw_ostream.h"

namespace clang::tidy::nullability {
namespace {

class InferenceManager {
 public:
  InferenceManager(ASTContext& Ctx, unsigned Iterations,
                   llvm::function_ref<bool(const Decl&)> Filter)
      : Ctx(Ctx), Iterations(Iterations), Filter(Filter) {}

  std::vector<Inference> inferenceRound(
      EvidenceSites Sites, USRCache USRCache,
      PreviousInferences InferencesFromLastRound) const {
    std::vector<Inference> AllInference;
    std::vector<Evidence> AllEvidence;

    // Collect all evidence.
    auto Emitter =
        evidenceEmitter([&](auto& E) { AllEvidence.push_back(E); }, USRCache);
    for (const auto* Decl : Sites.Declarations) {
      if (Filter && !Filter(*Decl)) continue;
      collectEvidenceFromTargetDeclaration(*Decl, Emitter);
    }
    for (const auto* Impl : Sites.Implementations) {
      if (Filter && !Filter(*Impl)) continue;
      if (auto Err = collectEvidenceFromImplementation(
              *Impl, Emitter, USRCache, InferencesFromLastRound)) {
        llvm::errs() << "Error in evidence collection: "
                     << toString(std::move(Err)) << "\n";
      }
    }
    // Group by symbol.
    llvm::sort(AllEvidence, [&](const Evidence& L, const Evidence& R) {
      return L.symbol().usr() < R.symbol().usr();
    });
    // For each symbol, combine evidence into an inference.
    llvm::ArrayRef<Evidence> RemainingEvidence = AllEvidence;

    while (!RemainingEvidence.empty()) {
      auto Batch = RemainingEvidence.take_while([&](const Evidence& E) {
        return E.symbol().usr() == RemainingEvidence.front().symbol().usr();
      });
      RemainingEvidence = RemainingEvidence.drop_front(Batch.size());
      AllInference.push_back(mergeEvidence(Batch));
    }
    return AllInference;
  }

  std::vector<Inference> iterativelyInfer() const {
    if (!Ctx.getLangOpts().CPlusPlus) {
      llvm::errs() << "Skipping non-C++ input file: "
                   << Ctx.getSourceManager()
                          .getFileEntryRefForID(
                              Ctx.getSourceManager().getMainFileID())
                          ->getName()
                   << "\n";
      return std::vector<Inference>();
    }
    auto Sites = EvidenceSites::discover(Ctx);
    USRCache USRCache;

    std::vector<Inference> AllInference = inferenceRound(Sites, USRCache, {});

    for (unsigned Iteration = 1; Iteration < Iterations; ++Iteration) {
      llvm::DenseSet<SlotFingerprint> NullableFromLastRound;
      llvm::DenseSet<SlotFingerprint> NonnullFromLastRound;

      for (const auto& Inference : AllInference) {
        for (const auto& slot_inference : Inference.slot_inference()) {
          if (slot_inference.trivial() || slot_inference.conflict()) continue;
          switch (slot_inference.nullability()) {
            case Nullability::NULLABLE:
              NullableFromLastRound.insert(
                  fingerprint(Inference.symbol().usr(), slot_inference.slot()));
              break;
            case Nullability::NONNULL:
              NonnullFromLastRound.insert(
                  fingerprint(Inference.symbol().usr(), slot_inference.slot()));
              break;
            default:
              break;
          }
        }
      }

      AllInference = inferenceRound(
          Sites, USRCache, {NullableFromLastRound, NonnullFromLastRound});
    }
    return AllInference;
  }

 private:
  ASTContext& Ctx;
  unsigned Iterations;
  llvm::function_ref<bool(const Decl&)> Filter;
};
}  // namespace

std::vector<Inference> inferTU(ASTContext& Ctx, unsigned Iterations,
                               llvm::function_ref<bool(const Decl&)> Filter) {
  return InferenceManager(Ctx, Iterations, Filter).iterativelyInfer();
}

}  // namespace clang::tidy::nullability
