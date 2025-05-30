// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_INFERENCE_COLLECT_EVIDENCE_H_
#define CRUBIT_NULLABILITY_INFERENCE_COLLECT_EVIDENCE_H_

#include <algorithm>
#include <memory>
#include <utility>
#include <vector>

#include "absl/base/nullability.h"
#include "absl/container/flat_hash_map.h"
#include "nullability/inference/inference.proto.h"
#include "nullability/inference/slot_fingerprint.h"
#include "nullability/inference/usr_cache.h"
#include "nullability/pointer_nullability_analysis.h"
#include "nullability/pragma.h"
#include "clang/include/clang/AST/ASTContext.h"
#include "clang/include/clang/AST/DeclBase.h"
#include "clang/include/clang/AST/DeclCXX.h"
#include "clang/include/clang/Analysis/FlowSensitive/Solver.h"
#include "clang/include/clang/Basic/SourceLocation.h"
#include "llvm/include/llvm/ADT/DenseMap.h"
#include "llvm/include/llvm/ADT/DenseSet.h"
#include "llvm/include/llvm/ADT/FunctionExtras.h"
#include "llvm/include/llvm/ADT/STLFunctionalExtras.h"
#include "llvm/include/llvm/Support/Error.h"
#include "llvm/include/llvm/Support/raw_ostream.h"

namespace clang::tidy::nullability {

/// Describes the direction of flow for a piece of evidence between a virtual
/// method and its overrides.
enum class VirtualMethodEvidenceFlowDirection {
  kFromBaseToDerived,
  kFromDerivedToBase,
  kBoth,
};

/// Returns the direction of flow for a piece of evidence between a virtual
/// method and its overrides.
///
/// The direction is determined by whether the evidence points towards Nonnull
/// or Nullable and is for a return slot or a parameter slot.
VirtualMethodEvidenceFlowDirection getFlowDirection(Evidence::Kind Kind,
                                                    bool ForReturnSlot);

// Map from virtual methods in a TU to the set of methods that override them in
// that TU.
using VirtualMethodOverridesMap =
    absl::flat_hash_map<const CXXMethodDecl *absl_nonnull,
                        llvm::DenseSet<const CXXMethodDecl *absl_nonnull>>;
/// Collect a map from virtual methods to a set of their overrides.
VirtualMethodOverridesMap getVirtualMethodOverrides(ASTContext &Ctx);

/// Callback used to report collected nullability evidence.
using EvidenceEmitter = void(const Decl &Target, Slot, Evidence::Kind,
                             SourceLocation);
/// Creates an EvidenceEmitter that serializes the evidence as Evidence protos.
/// This emitter caches USR generation, and should be reused for the whole AST.
/// All parameters must outlive the returned EvidenceEmitter.
llvm::unique_function<EvidenceEmitter> evidenceEmitter(
    llvm::unique_function<void(const Evidence &) const>, USRCache &USRCache,
    ASTContext &Ctx);

/// Creates an EvidenceEmitter as above, but allows re-use of a
/// VirtualMethodOverridesMap if one has already been computed.
llvm::unique_function<EvidenceEmitter> evidenceEmitter(
    llvm::unique_function<void(const Evidence &) const>, USRCache &USRCache,
    ASTContext &Ctx, const VirtualMethodOverridesMap &&OverridesMap);

class SortedFingerprintVector {
 public:
  SortedFingerprintVector() = default;
  // These are expected to often be very large containers, so disallow copying.
  SortedFingerprintVector(const SortedFingerprintVector &) = delete;
  SortedFingerprintVector &operator=(const SortedFingerprintVector &) = delete;
  explicit SortedFingerprintVector(std::vector<SlotFingerprint> &&V)
      : Vector(std::move(V)) {
    if (!std::is_sorted(Vector.begin(), Vector.end())) {
      // Performance is much improved if the incoming vector is already sorted,
      // but this is not a requirement.
      llvm::errs() << "Previous inferences are not sorted. Performance may be "
                      "degraded.\n";
      std::sort(Vector.begin(), Vector.end());
    }
    const auto &FirstDuplicate =
        std::adjacent_find(Vector.begin(), Vector.end());
    if (FirstDuplicate != Vector.end()) {
      // Duplicate fingerprints are not expected, and can cause incorrect
      // inference results, but only for symbols that have the same fingerprint.
      // Do not crash, to avoid invalidating all the other results, but do log
      // as much debugging information as possible in case this very unexpected
      // event occurs.
      llvm::errs() << "Found duplicate fingerprints in previous inferences.\n";
      llvm::DenseMap<SlotFingerprint, int> AppearanceCounts;
      // Because the vector is sorted, we can count the number of appearances
      // starting from FirstDuplicate and know that there will be no duplicates
      // of any of the fingerprints from Vector.begin() until FirstDuplicate.
      for (auto It = FirstDuplicate; It != Vector.end(); ++It) {
        AppearanceCounts[*It]++;
      }
      for (const auto &[Fingerprint, Count] : AppearanceCounts) {
        if (Count > 1) {
          llvm::errs() << "Fingerprint " << Fingerprint << " appears " << Count
                       << " times.\n";
        }
      }
      // Remove the duplicates before continuing.
      Vector.erase(std::unique(Vector.begin(), Vector.end()), Vector.end());
    }
  }

  bool contains(SlotFingerprint Fingerprint) const {
    return std::binary_search(Vector.begin(), Vector.end(), Fingerprint);
  }

 private:
  std::vector<SlotFingerprint> Vector;
};

struct PreviousInferences {
  const std::shared_ptr<const SortedFingerprintVector> absl_nonnull Nullable =
      std::make_shared<const SortedFingerprintVector>();
  const std::shared_ptr<const SortedFingerprintVector> absl_nonnull Nonnull =
      std::make_shared<const SortedFingerprintVector>();
};

/// Creates a solver with default parameters that is suitable for passing to
/// `collectEvidenceFromDefinition()`.
std::unique_ptr<dataflow::Solver> makeDefaultSolverForInference();

/// Analyze code (such as a function body or variable initializer) to infer
/// nullability.
///
/// Produces Evidence constraining the nullability slots of the symbols that
/// the code interacts with, such as the function's own parameters.
/// This is based on the code's behavior and our definition of null-safety.
///
/// It is up to the caller to ensure the definition is eligible for inference
/// (function has a body, is not dependent, etc).
llvm::Error collectEvidenceFromDefinition(
    const Decl &, llvm::function_ref<EvidenceEmitter>, USRCache &USRCache,
    const NullabilityPragmas &Pragmas,
    const PreviousInferences &PreviousInferences = {},
    const SolverFactory &MakeSolver = makeDefaultSolverForInference);

/// Gathers evidence of a symbol's nullability from a declaration of it.
///
/// These are trivial "inferences" of what's already written in the code. e.g:
///   void foo(Nullable<int*>);
/// The first parameter of foo must be nullable.
///
/// It is the caller's responsibility to ensure that the symbol is inferable.
void collectEvidenceFromTargetDeclaration(const clang::Decl &,
                                          llvm::function_ref<EvidenceEmitter>,
                                          const NullabilityPragmas &Pragmas);

/// Describes locations within an AST that provide evidence for use in
/// inference.
struct EvidenceSites {
  /// Declarations of inferable symbols.
  llvm::DenseSet<const Decl * absl_nonnull> Declarations;
  /// Definitions (e.g. function body, variable initializer) that can be
  /// analyzed.
  /// This will always be concrete code, not a template pattern. These may be
  /// passed to collectEvidenceFromDefinition().
  llvm::DenseSet<const Decl * absl_nonnull> Definitions;

  /// Find the evidence sites within the provided AST. If
  /// RestrictToMainFileOrHeader is true, only looks for evidence sites in the
  /// main file or its associated header. Implicit declarations are never
  /// considered to be in the main file or header.
  static EvidenceSites discover(ASTContext &,
                                bool RestrictToMainFileOrHeader = false);

  using ForEach = llvm::function_ref<void(const Decl &)>;
  /// For each evidence site with the provided AST, calls the provided
  /// callback(s). A single Decl may be passed to both callbacks if it is also a
  /// useful definition. If RestrictToMainFileOrHeader is true, only looks for
  /// evidence sites in the main file or its associated header. Implicit
  /// declarations are never considered to be in the main file or header.
  static void forDefinitionsAndForDeclarations(
      ForEach ForDefinitions, ForEach ForDeclarations, ASTContext &Ctx,
      bool RestrictToMainFileOrHeader = false);
};

/// Returns the slot number for the I'th parameter (0-based).
inline Slot paramSlot(unsigned I) { return static_cast<Slot>(SLOT_PARAM + I); }

}  // namespace clang::tidy::nullability

#endif  // CRUBIT_NULLABILITY_INFERENCE_COLLECT_EVIDENCE_H_
