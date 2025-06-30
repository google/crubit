// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_LIFETIME_ANNOTATIONS_POINTEE_TYPE_H_
#define CRUBIT_LIFETIME_ANNOTATIONS_POINTEE_TYPE_H_

#include "clang/AST/Type.h"
#include "clang/AST/TypeLoc.h"

namespace clang {
namespace tidy {
namespace lifetimes {

// If `type` is a pointer or reference type, returns the type of its pointee.
// Otherwise, returns a null type.
// Unlike `type->getPointeeType()`, this returns a null type if `type`, though
// it has a pointee type, is not a type for which we infer lifetimes, such as
// a pointer-to-member type. In other words, this function can be used to
// succinctly answer the question "does `type` have pointee type and do we infer
// lifetimes for it".
clang::QualType PointeeType(clang::QualType type);

// Analogous to `PointeeType` but operates on a `TypeLoc`.
clang::TypeLoc PointeeTypeLoc(clang::TypeLoc type_loc);

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

#endif  // CRUBIT_LIFETIME_ANNOTATIONS_POINTEE_TYPE_H_
