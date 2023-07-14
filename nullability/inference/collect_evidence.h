// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_INFERENCE_COLLECT_EVIDENCE_H_
#define CRUBIT_NULLABILITY_INFERENCE_COLLECT_EVIDENCE_H_

#include <string>
#include <vector>

#include "nullability/inference/inference.proto.h"
#include "clang/AST/DeclBase.h"
#include "llvm/ADT/FunctionExtras.h"
#include "llvm/ADT/STLFunctionalExtras.h"
#include "llvm/Support/Error.h"

namespace clang::tidy::nullability {

// Callback used to report collected nullability evidence.
using EvidenceEmitter = void(const Decl &Target, Slot, NullabilityConstraint);
// Creates an EvidenceEmitter that serializes the evidence as Evidence protos.
// This emitter caches USR generation, and should be reused for the whole AST.
llvm::unique_function<EvidenceEmitter> evidenceEmitter(
    llvm::unique_function<void(const Evidence &) const>);

// Analyze code (such as a function body) to infer nullability.
//
// Produces Evidence constraining the nullability slots of the symbols that
// the code interacts with, such as the function's own parameters.
// This is based on the function's behavior and our definition of null-safety.
//
// It is up to the caller to ensure the implementation is eligible for inference
// (function has a body, is not dependent, etc).
llvm::Error collectEvidenceFromImplementation(
    const Decl &, llvm::function_ref<EvidenceEmitter>);

// Gathers evidence of a symbol's nullability from a declaration of it.
//
// These are trivial "inferences" of what's already written in the code. e.g:
//   void foo(Nullable<int*>);
// The first parameter of foo must be nullable.
//
// It is the caller's responsibility to ensure that the symbol is inferrable.
void collectEvidenceFromTargetDeclaration(const clang::Decl &,
                                          llvm::function_ref<EvidenceEmitter>);

}  // namespace clang::tidy::nullability

#endif  // CRUBIT_NULLABILITY_INFERENCE_COLLECT_EVIDENCE_H_
