// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_INFERENCE_COLLECT_EVIDENCE_H_
#define CRUBIT_NULLABILITY_INFERENCE_COLLECT_EVIDENCE_H_

#include <vector>

#include "nullability/inference/inference.proto.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "llvm/Support/Error.h"

namespace clang::tidy::nullability {

// Returns a collection of Evidence constraining the nullability slots of Func's
// type based on the function's behavior and our definition of null-safety.
llvm::Expected<std::vector<Evidence>> collectEvidence(
    const clang::FunctionDecl &Func, clang::ASTContext &Context);

}  // namespace clang::tidy::nullability

#endif  // CRUBIT_NULLABILITY_INFERENCE_COLLECT_EVIDENCE_H_
