// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef DEVTOOLS_RUST_CC_INTEROP_LIFETIME_ANALYSIS_POINTER_COMPATIBILITY_H_
#define DEVTOOLS_RUST_CC_INTEROP_LIFETIME_ANALYSIS_POINTER_COMPATIBILITY_H_

#include "clang/AST/Type.h"

namespace clang {
namespace tidy {
namespace lifetimes {

// Returns whether a pointer with the given `pointee_type` may point to an
// object of type `object_type`.
// In the case where `object_type` is a class type, we also return true if
// `pointee_type` may point to a type derived from `object_type`. This accounts
// for the fact that `Object::Type()` may be a base class of the dynamic type
// of the object instead of being identical to the dynamic type.
// As described in TransferLifetimesForCall(), this is similar to but more
// permissive than C++'s strict aliasing rules.
bool PointeesCompatible(clang::QualType pointee_type,
                        clang::QualType object_type,
                        clang::ASTContext& ast_context);

// Returns whether a pointer of the given type may point to an object of type
// `object_type`.
bool MayPointTo(clang::QualType pointer_type, clang::QualType object_type,
                clang::ASTContext& ast_context);

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

#endif  // DEVTOOLS_RUST_CC_INTEROP_LIFETIME_ANALYSIS_POINTER_COMPATIBILITY_H_
