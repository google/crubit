// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_INFERENCE_ANALYZE_TARGET_FOR_TEST_H_
#define CRUBIT_NULLABILITY_INFERENCE_ANALYZE_TARGET_FOR_TEST_H_

#include "clang/AST/Decl.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "llvm/ADT/STLFunctionalExtras.h"
#include "llvm/ADT/StringRef.h"

namespace clang::tidy::nullability {

void analyzeTargetForTest(
    llvm::StringRef Source,
    llvm::function_ref<
        void(const clang::FunctionDecl&,
             const clang::ast_matchers::MatchFinder::MatchResult&)>
        AnalysisCallback);

}  // namespace clang::tidy::nullability

#endif  // CRUBIT_NULLABILITY_INFERENCE_ANALYZE_TARGET_FOR_TEST_H_
