// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef DEVTOOLS_RUST_CC_INTEROP_LIFETIME_ANALYSIS_ANALYZE_H_
#define DEVTOOLS_RUST_CC_INTEROP_LIFETIME_ANALYSIS_ANALYZE_H_

#include <functional>
#include <string>
#include <variant>

#include "lifetime_analysis/lifetime_analysis.h"
#include "lifetime_annotations/function_lifetimes.h"
#include "lifetime_annotations/lifetime_annotations.h"
#include "clang/AST/Decl.h"
#include "llvm/ADT/DenseMap.h"
#include "llvm/ADT/SmallVector.h"

namespace clang {
namespace tidy {
namespace lifetimes {

// Lifetime analysis debug info for a single function.
struct FunctionDebugInfo {
  // Human-readable representation of the function's AST.
  std::string ast;

  // Human-readable representation of the function's ObjectRepository.
  std::string object_repository;

  // A graph of the exit-block's points-to map in .dot file format.
  std::string points_to_map_dot;

  // A graph of the CFG in .dot file format.
  std::string cfg_dot;
};

// Returns if the two FunctionLifetimes have the same structures, without
// requiring them to have the same exact Lifetimes. They have the same
// structure if unique vs reoccuring Lifetimes in `a` and `b` are found
// in the same positions.
bool IsIsomorphic(const FunctionLifetimes& a, const FunctionLifetimes& b);

// A map from an analyzed function to the corresponding debug info.
using FunctionDebugInfoMap =
    llvm::DenseMap<const clang::FunctionDecl*, FunctionDebugInfo>;

// Runs a static analysis on `func` and returns the result.
FunctionLifetimesOrError AnalyzeFunction(
    const clang::FunctionDecl* func,
    const LifetimeAnnotationContext& lifetime_context,
    FunctionDebugInfo* debug_info = nullptr);

// Runs a static analysis on all function definitions in `tu`.
// The map that is returned references functions by their canonical declaration.
llvm::DenseMap<const clang::FunctionDecl*, FunctionLifetimesOrError>
AnalyzeTranslationUnit(const clang::TranslationUnitDecl* tu,
                       const LifetimeAnnotationContext& lifetime_context,
                       DiagnosticReporter diag_reporter = {},
                       FunctionDebugInfoMap* debug_info = nullptr);

// Callback that is used to report function analysis results.
// Do not retain the `FunctionDecl*`, the `FunctionLifetimes`, or other objects
// reachable from them for later use; they refer to entities from an
// `ASTContext` that may be destroyed as soon as the callback returns. In
// particular, note that this also applies to `clang::Type`s contained in the
// `FunctionLifetimes`.
using FunctionAnalysisResultCallback =
    std::function<void(const clang::FunctionDecl* func,
                       const FunctionLifetimesOrError& lifetimes_or_error)>;

// Runs a static analysis on all function definitions in `tu`.
// Analyzes and reports results for uninstantiated templates by instantiating
// them with placeholder types, reporting results via `result_callback`.
void AnalyzeTranslationUnitWithTemplatePlaceholder(
    const clang::TranslationUnitDecl* tu,
    const LifetimeAnnotationContext& lifetime_context,
    const FunctionAnalysisResultCallback& result_callback,
    DiagnosticReporter diag_reporter = {},
    FunctionDebugInfoMap* debug_info = nullptr);

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

#endif  // DEVTOOLS_RUST_CC_INTEROP_LIFETIME_ANALYSIS_ANALYZE_H_
