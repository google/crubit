// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef DEVTOOLS_RUST_CC_INTEROP_LIFETIME_ANALYSIS_TEMPLATE_PLACEHOLDER_SUPPORT_H_
#define DEVTOOLS_RUST_CC_INTEROP_LIFETIME_ANALYSIS_TEMPLATE_PLACEHOLDER_SUPPORT_H_

#include <functional>
#include <string>

#include "clang/AST/Decl.h"
#include "llvm/ADT/DenseMap.h"
#include "llvm/Support/Error.h"

namespace clang {
namespace tidy {
namespace lifetimes {

struct GeneratedCode {
  std::string filename;
  std::string code;
};

// Generates a source code that includes the original code for `tu`
// and also has explicit template instantiation code with placeholder
// classes for the templates in `templates`.
// For example, if the main file for the `tu` has the filename
// "original-file.cc" and looks like the following:
//
//    template <typename T>
//    T* target(T* t) {
//      return t;
//    }
//
//  This will generate and return the code like the following
//  (actual generated placeholder classnames will be more cryptic than `T0`):
//
//    #include "original-file.cc"
//    struct T0 {};
//    template T0* target<T0>(T0* t);
//
llvm::Expected<GeneratedCode> GenerateTemplateInstantiationCode(
    const clang::TranslationUnitDecl* tu,
    const llvm::DenseMap<clang::FunctionTemplateDecl*,
                         const clang::FunctionDecl*>& templates);

// Runs the given `operation` on the `code` with `filename`. The `code` is
// turned into a memory-backed file on a memory filesystem overlaid on top
// of the original filesystem that's being used by `original_context`.
void RunToolOnCodeWithOverlay(
    clang::ASTContext& original_context, const std::string& filename,
    const std::string& code,
    const std::function<void(clang::ASTContext&)> operation);

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

#endif  // DEVTOOLS_RUST_CC_INTEROP_LIFETIME_ANALYSIS_TEMPLATE_PLACEHOLDER_SUPPORT_H_
