// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_INFERENCE_INFER_NULLABILITY_CONSTRAINTS_H_
#define CRUBIT_NULLABILITY_INFERENCE_INFER_NULLABILITY_CONSTRAINTS_H_

#include "nullability/inference/inference.proto.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "llvm/ADT/DenseMap.h"
#include "llvm/Support/Error.h"

namespace clang::tidy::nullability {

// Collects constraints on nullability annotations that could be added to the
// types of Func's parameters based on the function's behavior and our
// definition of null-safety.
llvm::Expected<llvm::DenseMap<const clang::Decl *, NullabilityConstraint>>
inferNullabilityConstraints(const clang::FunctionDecl &Func,
                            clang::ASTContext &Context);

// Returns whether `Type` is annotated as non-nullable at the outermost pointer
// layer.
//
// e.g. returns true for int* _Nonnull,
// but returns false for int * _Nonnull *.
//
// Requires that `Type` is a pointer type or reference to a pointer type,
// possibly nested, but not e.g. a container of pointers.
bool isNonNullAnnotated(clang::QualType Type);

}  // namespace clang::tidy::nullability

#endif  // CRUBIT_NULLABILITY_INFERENCE_INFER_NULLABILITY_CONSTRAINTS_H_
