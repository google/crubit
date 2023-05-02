// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// This file defines an extension of C++'s type system to cover nullability:
// Each pointer "slot" within a compound type is marked with a nullability kind.
//
// e.g. vector<int *> *
//         Nullable^  ^Nonnull
// This type describes non-null pointers to vectors of possibly-null pointers.
//
// This model interacts with clang's nullability attributes: the type
// above can be written `vector<int * _Nullable> _Nonnull`.
// The two are not quite the same thing:
//   - we may infer nullability or use defaults where no attributes are written
//   - we generally pass nullability around as a separate data structure rather
//     than materializing the sugared types
//   - we do not use _Nullable_result
//
// This is separate from our model of pointer values as part of the Value graph
// (see pointer_nullability.h). The analysis makes use of both: generally type
// nullability is useful with compound types like templates and functions where
// the concrete pointer values are not visible to analysis.

#ifndef CRUBIT_NULLABILITY_TYPE_NULLABILITY_H_
#define CRUBIT_NULLABILITY_TYPE_NULLABILITY_H_

#include <string>
#include <utility>

#include "clang/AST/ASTContext.h"
#include "clang/AST/Expr.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/Specifiers.h"

namespace clang::tidy::nullability {

/// Externalized nullability of a clang::Type.
///
/// Each pointer type nested inside is mapped to a nullability.
/// This describes the nullability for pointer values used with the type.
///
/// For example, in: pair<int*, double*>
/// We can map:      int* => Unspecified, double* => Nonnull
/// And given such a pair p, p.second is considered Nonnull.
///
/// We could represent this as the Type pair<int *, double *_Nonnull>.
/// However clang frequently drops type sugar such as _Nonnull, and Types are
/// inconvenient to manipulate. We pass nullability explicitly instead.
///
/// The concrete representation is currently the nullability of each nested
/// PointerType encountered in a preorder traversal of the canonical type.
using TypeNullability = std::vector<NullabilityKind>;

/// Returns the `NullabilityKind` corresponding to the nullability annotation on
/// `Type` if present. Otherwise, returns `NullabilityKind::Unspecified`.
NullabilityKind getNullabilityKind(QualType Type, ASTContext& Ctx);

/// Returns a human-readable debug representation of a nullability vector.
std::string nullabilityToString(const TypeNullability& Nullability);

/// A function that may provide enhanced nullability information for a
/// substituted template parameter (which has no sugar of its own).
using GetTypeParamNullability =
    std::optional<TypeNullability>(const SubstTemplateTypeParmType* ST);
/// Traverse over a type to get its nullability. For example, if T is the type
/// Struct3Arg<int * _Nonnull, int, pair<int * _Nullable, int *>> * _Nonnull,
/// the resulting nullability annotations will be {_Nonnull, _Nonnull,
/// _Nullable, _Unknown}. Note that non-pointer elements (e.g., the second
/// argument of Struct3Arg) do not get a nullability annotation.
TypeNullability getNullabilityAnnotationsFromType(
    QualType T,
    llvm::function_ref<GetTypeParamNullability> SubstituteTypeParam = nullptr);

/// Prints QualType's underlying canonical type, annotated with nullability.
/// See rebuildWithNullability().
std::string printWithNullability(QualType, const TypeNullability&, ASTContext&);
/// Returns an equivalent type annotated with the provided nullability.
/// Any existing sugar (including nullability) is discarded.
/// rebuildWithNullability(int *, {Nullable}) ==> int * _Nullable.
QualType rebuildWithNullability(QualType, const TypeNullability&, ASTContext&);

/// Computes the number of pointer slots within a type.
/// Each of these could conceptually be nullable, so this is the length of
/// the nullability vector computed by getNullabilityAnnotationsFromType().
unsigned countPointersInType(QualType T);
unsigned countPointersInType(const Expr* E);
unsigned countPointersInType(TemplateArgument TA);
unsigned countPointersInType(const DeclContext* DC);

/// Returns the type of an expression for the purposes of nullability.
/// This handles wrinkles in the type system like BoundMember.
QualType exprType(const Expr* E);

TypeNullability unspecifiedNullability(const Expr* E);

}  // namespace clang::tidy::nullability

#endif
