// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_NULLABILITY_INFERENCE_USR_CACHE_H_
#define THIRD_PARTY_CRUBIT_NULLABILITY_INFERENCE_USR_CACHE_H_

#include <string>
#include <string_view>

#include "clang/include/clang/AST/DeclBase.h"
#include "llvm/include/llvm/ADT/DenseMap.h"

namespace clang::tidy::nullability {

using USRCache = llvm::DenseMap<const Decl *, std::string>;

std::string_view getOrGenerateUSR(USRCache &Cache, const Decl &);

}  // namespace clang::tidy::nullability

#endif  // THIRD_PARTY_CRUBIT_NULLABILITY_INFERENCE_USR_CACHE_H_
