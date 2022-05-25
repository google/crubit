// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_analysis/pointer_compatibility.h"

#include "lifetime_annotations/pointee_type.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/DeclCXX.h"

namespace clang {
namespace tidy {
namespace lifetimes {

// Returns whether `type` is an unsigned integer type or an enum with an
// underlying unsigned integer type.
// Note that, unlike this function, Type::isUnsignedIntegerType() considers
// `bool` to be an unsigned integer type.
static bool IsUnsignedIntegerOrEnumType(clang::QualType type) {
  type = type.getCanonicalType();
  return type->isUnsignedIntegerType() && !type->isBooleanType();
}

bool PointeesCompatible(clang::QualType pointee_type,
                        clang::QualType object_type,
                        clang::ASTContext& ast_context) {
  assert(!pointee_type.isNull());
  assert(!object_type.isNull());

  pointee_type = pointee_type.getCanonicalType();
  object_type = object_type.getCanonicalType();

  // `void *`, `char *`, `unsigned char *` and `std::byte *` are allowed to
  // point at anything.
  if (pointee_type->isVoidType() || pointee_type->isCharType() ||
      pointee_type->isStdByteType()) {
    return true;
  }

  // Anything is allowed to point at `void`. IOW, a function is allowed to cast
  // a void pointer back to any other type of pointer.
  if (object_type->isVoidType()) {
    return true;
  }

  // Records.
  if (pointee_type->isRecordType()) {
    const clang::CXXRecordDecl* pointee_record_decl =
        pointee_type->getAsCXXRecordDecl();
    const clang::CXXRecordDecl* object_record_decl =
        object_type->getAsCXXRecordDecl();
    // We leave the case where the records are the same to the hasSimilarType()
    // case below.
    if (pointee_record_decl && object_record_decl &&
        (object_record_decl->isDerivedFrom(pointee_record_decl) ||
         pointee_record_decl->isDerivedFrom(object_record_decl))) {
      return true;
    }
  }

  // A signed integer pointer may point to the unsigned variant of the integer
  // type and vice versa -- so arbitrarily canonicalize integer types to the
  // signed version.
  if (IsUnsignedIntegerOrEnumType(pointee_type)) {
    pointee_type = ast_context.getCorrespondingSignedType(pointee_type);
  }
  if (IsUnsignedIntegerOrEnumType(object_type)) {
    object_type = ast_context.getCorrespondingSignedType(object_type);
  }

  return ast_context.hasSimilarType(pointee_type, object_type);
}

bool MayPointTo(clang::QualType pointer_type, clang::QualType object_type,
                clang::ASTContext& ast_context) {
  assert(!pointer_type.isNull());
  assert(!object_type.isNull());

  pointer_type = pointer_type.getCanonicalType();
  object_type = object_type.getCanonicalType();

  clang::QualType pointee_type = PointeeType(pointer_type);

  if (pointee_type.isNull()) {
    llvm::report_fatal_error("pointee_type is null");
  }

  return PointeesCompatible(pointee_type, object_type, ast_context);
}

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
