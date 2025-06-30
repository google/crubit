// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_LIFETIME_ANNOTATIONS_TEST_RUN_ON_CODE_H_
#define CRUBIT_LIFETIME_ANNOTATIONS_TEST_RUN_ON_CODE_H_

#include <functional>
#include <string>

#include "lifetime_annotations/lifetime_annotations.h"
#include "clang/AST/ASTContext.h"
#include "clang/Tooling/Tooling.h"
#include "llvm/ADT/ArrayRef.h"
#include "llvm/ADT/StringRef.h"

namespace clang {
namespace tidy {
namespace lifetimes {

bool runOnCodeWithLifetimeHandlers(
    llvm::StringRef code,
    const std::function<void(clang::ASTContext&,
                             const LifetimeAnnotationContext&)>& operation,
    llvm::ArrayRef<std::string> args,
    const clang::tooling::FileContentMappings& file_contents =
        clang::tooling::FileContentMappings());

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

#endif  // CRUBIT_LIFETIME_ANNOTATIONS_TEST_RUN_ON_CODE_H_
