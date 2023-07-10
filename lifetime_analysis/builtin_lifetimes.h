// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef DEVTOOLS_RUST_CC_INTEROP_LIFETIME_ANALYSIS_BUILTIN_LIFETIMES_H_
#define DEVTOOLS_RUST_CC_INTEROP_LIFETIME_ANALYSIS_BUILTIN_LIFETIMES_H_

#include "lifetime_annotations/function_lifetimes.h"
#include "clang/AST/Decl.h"

namespace clang {
namespace tidy {
namespace lifetimes {

FunctionLifetimesOrError GetBuiltinLifetimes(const clang::FunctionDecl* decl);

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

#endif  // DEVTOOLS_RUST_CC_INTEROP_LIFETIME_ANALYSIS_BUILTIN_LIFETIMES_H_
