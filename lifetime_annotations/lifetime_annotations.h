// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_LIFETIME_ANNOTATIONS_LIFETIME_ANNOTATIONS_H_
#define CRUBIT_LIFETIME_ANNOTATIONS_LIFETIME_ANNOTATIONS_H_

#include <memory>
#include <string>

#include "lifetime_annotations/function_lifetimes.h"
#include "lifetime_annotations/lifetime_symbol_table.h"
#include "clang/include/clang/AST/Decl.h"
#include "clang/include/clang/Basic/SourceLocation.h"
#include "clang/include/clang/Frontend/CompilerInstance.h"
#include "llvm/include/llvm/ADT/DenseSet.h"
#include "llvm/include/llvm/Support/Error.h"

namespace clang {
namespace tidy {
namespace lifetimes {

// Context that is required to obtain lifetime annotations for a function.
struct LifetimeAnnotationContext {
  // Files in which the `lifetime_elision` pragma was specified.
  llvm::DenseSet<clang::FileID> lifetime_elision_files;
};

// Returns the lifetimes annotated on `func`.
// If the file containing the function definition specifies the
// `lifetime_elision` pragma, lifetime elision rules are used to determine
// any unannotated lifetimes.
// Returns an error if the function contains unannotated lifetimes that could
// not be determined through lifetime elision, either because the
// `lifetime_elision`pragma was not specified or because the lifetime elision
// rules were not applicable.
// The names of annotated function lifetimes as well as autogenerated names for
// elided lifetimes are added to `symbol_table`.
//
// Returns structured error information as a `LifetimeError`.
llvm::Expected<FunctionLifetimes> GetLifetimeAnnotations(
    const clang::FunctionDecl* func, const LifetimeAnnotationContext& context,
    LifetimeSymbolTable* symbol_table = nullptr);

// Parses "a: b, a -> b"-style lifetime annotations from `lifetimes_str` for the
// function declaration `func`. Lifetimes are inserted into the given
// `symbol_table`, or used from there if already known.
//
// Returns structured error information as a `LifetimeError`.
llvm::Expected<FunctionLifetimes> ParseLifetimeAnnotations(
    const clang::FunctionDecl* func, const std::string& lifetimes_str,
    LifetimeSymbolTable* symbol_table = nullptr);

// Adds handlers to `preprocessor` to populate `context`.
// To be able to use GetLifetimeAnnotations(), call this function to add the
// necessary handlers before compiling any code.
void AddLifetimeAnnotationHandlers(
    clang::Preprocessor& preprocessor,
    std::shared_ptr<LifetimeAnnotationContext> context);

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

#endif  // CRUBIT_LIFETIME_ANNOTATIONS_LIFETIME_ANNOTATIONS_H_
