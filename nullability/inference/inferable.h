// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_INFERENCE_INFERRABLE_H_
#define CRUBIT_NULLABILITY_INFERENCE_INFERRABLE_H_

#include "clang/AST/DeclBase.h"
#include "clang/AST/Type.h"
#include "llvm/ADT/SmallVector.h"

namespace clang::tidy::nullability {

// Enables or disables support for inferring nullability for pointers in
// template arguments for select templates. (Disabled by default.)

// This should only be called once before the first analysis is started.
void setSelectTemplatesOfPointersInferable(bool Enabled);

// Returns whether support for inferring nullability for pointers in
// template arguments for select templates has been turned on.
bool selectTemplatesOfPointersInferable();

// Are there inferable slots in this type?
bool hasInferable(QualType T);

/// Should we attempt to deduce nullability for this symbol?
bool isInferenceTarget(const Decl &);

/// The number of nullability slots in this symbol's type which can be inferred.
///
/// This may not be all the slots in the type: e.g. `int** X` has outer and
/// inner nullability; we may support only inferring outer.
int countInferableSlots(const clang::Decl &);

/// The indices of nullability slots in this symbol's type which can be
/// inferred.
///
/// This may not be all the slots in the type: e.g. `int** X` has outer and
/// inner nullability; we may support only inferring outer.
llvm::SmallVector<int> getInferableSlotIndices(const clang::Decl&);

}  // namespace clang::tidy::nullability

#endif  // THIRD_PARTY_CRUBIT_NULLABILITY_INFERENCE_INFERRABLE_H_
