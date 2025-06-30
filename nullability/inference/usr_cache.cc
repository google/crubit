// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/usr_cache.h"

#include <string_view>

#include "clang/AST/DeclBase.h"
#include "clang/Index/USRGeneration.h"
#include "llvm/ADT/SmallString.h"

namespace clang::tidy::nullability {
std::string_view getOrGenerateUSR(USRCache &Cache, const Decl &Decl) {
  auto [It, Inserted] = Cache.try_emplace(&Decl);
  if (Inserted) {
    llvm::SmallString<128> USR;
    if (!index::generateUSRForDecl(&Decl, USR)) It->second = USR.str();
  }
  return It->second;
}

}  // namespace clang::tidy::nullability
