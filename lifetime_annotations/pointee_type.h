// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_LIFETIME_ANNOTATIONS_POINTEE_TYPE_H_
#define CRUBIT_LIFETIME_ANNOTATIONS_POINTEE_TYPE_H_

#include "third_party/llvm/llvm-project/clang/include/clang/AST/Type.h"

namespace devtools_rust {

// If `type` is a pointer or reference type, returns the type of its pointee.
// Otherwise, returns a null type.
// Unlike `type->getPointeeType()`, this returns a null type if `type`, though
// it has a pointee type, is not a type for which we infer lifetimes, such as
// a pointer-to-member type. In other words, this function can be used to
// succinctly answer the question "does `type` have pointee type and do we infer
// lifetimes for it".
clang::QualType PointeeType(clang::QualType type);

}  // namespace devtools_rust

#endif  // CRUBIT_LIFETIME_ANNOTATIONS_POINTEE_TYPE_H_
