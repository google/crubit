// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_LIFETIME_ANNOTATIONS_POINTEE_TYPE_H_
#define CRUBIT_LIFETIME_ANNOTATIONS_POINTEE_TYPE_H_

#include "third_party/llvm/llvm-project/clang/include/clang/AST/Type.h"

namespace devtools_rust {

// If `type` is a reference-like type, returns the type of its pointee.
// Otherwise, returns a null type.
clang::QualType PointeeType(clang::QualType type);

}  // namespace devtools_rust

#endif  // CRUBIT_LIFETIME_ANNOTATIONS_POINTEE_TYPE_H_
