// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_INFERENCE_COLLECT_EVIDENCE_H_
#define CRUBIT_NULLABILITY_INFERENCE_COLLECT_EVIDENCE_H_

#include <string>
#include <string_view>
#include <vector>

#include "nullability/inference/inference.proto.h"
#include "nullability/inference/slot_fingerprint.h"
#include "clang/AST/DeclBase.h"
#include "clang/Basic/SourceLocation.h"
#include "llvm/ADT/DenseMap.h"
#include "llvm/ADT/DenseSet.h"
#include "llvm/ADT/FunctionExtras.h"
#include "llvm/ADT/STLFunctionalExtras.h"
#include "llvm/Support/Error.h"

namespace clang::tidy::nullability {

using USRCache = llvm::DenseMap<const Decl *, std::string>;

std::string_view getOrGenerateUSR(USRCache &Cache, const Decl &);

// Callback used to report collected nullability evidence.
using EvidenceEmitter = void(const Decl &Target, Slot, Evidence::Kind,
                             SourceLocation);
// Creates an EvidenceEmitter that serializes the evidence as Evidence protos.
// This emitter caches USR generation, and should be reused for the whole AST.
llvm::unique_function<EvidenceEmitter> evidenceEmitter(
    llvm::unique_function<void(const Evidence &) const>, USRCache &USRCache);

// Analyze code (such as a function body) to infer nullability.
//
// Produces Evidence constraining the nullability slots of the symbols that
// the code interacts with, such as the function's own parameters.
// This is based on the function's behavior and our definition of null-safety.
//
// It is up to the caller to ensure the implementation is eligible for inference
// (function has a body, is not dependent, etc).
llvm::Error collectEvidenceFromImplementation(
    const Decl &, llvm::function_ref<EvidenceEmitter>, USRCache &USRCache,
    const llvm::DenseSet<SlotFingerprint> &PreviouslyInferredNullable = {},
    const llvm::DenseSet<SlotFingerprint> &PreviouslyInferredNonnull = {});

// Gathers evidence of a symbol's nullability from a declaration of it.
//
// These are trivial "inferences" of what's already written in the code. e.g:
//   void foo(Nullable<int*>);
// The first parameter of foo must be nullable.
//
// It is the caller's responsibility to ensure that the symbol is inferable.
void collectEvidenceFromTargetDeclaration(const clang::Decl &,
                                          llvm::function_ref<EvidenceEmitter>);

// Describes locations within an AST that provide evidence for use in inference.
struct EvidenceSites {
  // Declarations of inferable symbols.
  std::vector<const Decl *> Declarations;
  // Implementations (e.g. function body) that can be analyzed.
  // This will always be concrete code, not a template pattern.
  // These may be passed to collectEvidence().
  std::vector<const Decl *> Implementations;

  // Find the evidence sites within the provided AST.
  static EvidenceSites discover(ASTContext &);
};

// Returns the slot number for the I'th parameter (0-based).
inline Slot paramSlot(unsigned I) { return static_cast<Slot>(SLOT_PARAM + I); }

}  // namespace clang::tidy::nullability

#endif  // CRUBIT_NULLABILITY_INFERENCE_COLLECT_EVIDENCE_H_
