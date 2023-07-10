// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_annotations/pointee_type.h"

#include "clang/AST/Type.h"
#include "clang/AST/TypeLoc.h"

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
    auto ret = reference_type_loc.getPointeeLoc();
    if (auto tmplpar = ret.getAs<clang::SubstTemplateTypeParmTypeLoc>()) {
      // When we have a T&& substituted with T = int&, the TypeLoc does not
      // take reference collapsing into account, and would thus return a typeloc
      // of a int& type as the pointee of an int&.
      // TODO(veluca): figure out how to get at the typeloc of the underlying
      // type, if it exists.
      if (tmplpar.getType()->getAs<clang::ReferenceType>()) {
        return clang::TypeLoc();
      }
    }
    return ret;
  }

  return clang::TypeLoc();
}

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
