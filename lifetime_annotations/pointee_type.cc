// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_annotations/pointee_type.h"

namespace clang {
namespace tidy {
namespace lifetimes {

clang::QualType PointeeType(clang::QualType type) {
  if (auto ptr_type = type->getAs<clang::PointerType>()) {
    return ptr_type->getPointeeType();
  } else if (auto ref_type = type->getAs<clang::ReferenceType>()) {
    return ref_type->getPointeeType();
  }

  return clang::QualType();
}

clang::TypeLoc PointeeTypeLoc(clang::TypeLoc type_loc) {
  type_loc = type_loc.getUnqualifiedLoc();

  if (auto pointer_type_loc = type_loc.getAs<clang::PointerTypeLoc>()) {
    return pointer_type_loc.getPointeeLoc();
  } else if (auto reference_type_loc =
                 type_loc.getAs<clang::ReferenceTypeLoc>()) {
    return reference_type_loc.getPointeeLoc();
  }

  return clang::TypeLoc();
}

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
