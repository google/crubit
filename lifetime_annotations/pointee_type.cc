// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_annotations/pointee_type.h"

namespace devtools_rust {

// If `type` is a reference-like type, returns the type of its pointee.
// Otherwise, returns a null type.
clang::QualType PointeeType(clang::QualType type) {
  if (type.isNull()) {
    return clang::QualType();
  }

  // For the purposes of inference, we always use canonical types. Later, if we
  // need to produce a lifetime annotation for a type that is actually a typedef
  // for another type, we'll handle that there.
  type = type.getCanonicalType();

  if (auto ptr_type = type->getAs<clang::PointerType>()) {
    return ptr_type->getPointeeType();
  } else if (auto ref_type = type->getAs<clang::ReferenceType>()) {
    return ref_type->getPointeeType();
  }

  // TODO(mboehme): Handle these additional cases:
  // - For array types, recurse into the element type
  // - Types with type parameters
  // - Type arguments for class templates

  return clang::QualType();
}

}  // namespace devtools_rust
