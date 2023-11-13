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

#include <optional>
#include <string>
#include <tuple>
#include <vector>

#include "absl/log/check.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Expr.h"
#include "clang/AST/Type.h"
#include "clang/Analysis/FlowSensitive/Arena.h"
#include "clang/Analysis/FlowSensitive/Formula.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/Specifiers.h"
#include "llvm/ADT/STLFunctionalExtras.h"

namespace clang::tidy::nullability {

/// Is this exactly a pointer type that we track outer nullability for?
/// This unwraps sugar, i.e. it looks at the canonical type.
///
/// (For now, only regular `PointerType`s, in future we should consider
/// supporting pointer-to-member, ObjC pointers, `unique_ptr`, etc).
bool isSupportedPointerType(QualType);

/// Is this exactly a raw (non-smart) pointer type that we track outer
/// nullability for?
/// This unwraps sugar, i.e. it looks at the canonical type.
bool isSupportedRawPointerType(QualType);

/// Is this exactly a smart pointer type that we track outer nullability for?
/// This unwraps sugar, i.e. it looks at the canonical type.
bool isSupportedSmartPointerType(QualType);

/// Describes the nullability contract of a pointer "slot" within a type.
///
/// This may be concrete: nullable/non-null/unknown nullability.
/// Or may be symbolic:   this nullability is being inferred, and the presence
///                       of a "nullable" annotation is bound to a SAT variable
class PointerTypeNullability {
  // If concrete: NK is set, others are default.
  // If symbolic: NK=Unspecified, Symbolic=true, Nonnull/Nullable are set.
  NullabilityKind NK = NullabilityKind::Unspecified;
  bool Symbolic = false;
  dataflow::Atom Nonnull{0};
  dataflow::Atom Nullable{0};

 public:
  PointerTypeNullability(NullabilityKind NK = NullabilityKind::Unspecified)
      : NK(NK) {}
  // Creates a symbolic nullability variable.
  // A owns the underlying SAT variables nonnullAtom() and nullableAtom().
  static PointerTypeNullability createSymbolic(dataflow::Arena &A);

  // Returns the concrete nullability, or Unspecified if symbolic.
  NullabilityKind concrete() const { return NK; }

  // Returns symbolic nullability atoms.
  // Requires: isSymbolic().
  dataflow::Atom nonnullAtom() const {
    CHECK(isSymbolic());
    return Nonnull;
  }

  dataflow::Atom nullableAtom() const {
    CHECK(isSymbolic());
    return Nullable;
  }

  bool isSymbolic() const { return Symbolic; }

  // Returns the condition under which this slot is non-null.
  const dataflow::Formula &isNonnull(dataflow::Arena &A) const {
    return Symbolic ? A.makeAtomRef(Nonnull)
                    : A.makeLiteral(NK == NullabilityKind::NonNull);
  }

  // Returns the condition under which this slot is nullable.
  const dataflow::Formula &isNullable(dataflow::Arena &A) const {
    return Symbolic ? A.makeAtomRef(Nullable)
                    : A.makeLiteral(NK == NullabilityKind::Nullable);
  }

  friend bool operator==(const PointerTypeNullability &L,
                         const PointerTypeNullability &R) {
    return std::tie(L.NK, L.Nonnull, L.Nullable) ==
           std::tie(R.NK, R.Nonnull, R.Nullable);
  }

  friend llvm::raw_ostream &operator<<(llvm::raw_ostream &,
                                       const PointerTypeNullability &);
};

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
using TypeNullability = std::vector<PointerTypeNullability>;

/// Returns the `NullabilityKind` corresponding to the nullability annotation on
/// `Type` if present. Otherwise, returns `NullabilityKind::Unspecified`.
NullabilityKind getNullabilityKind(QualType Type, ASTContext &Ctx);

/// Returns a human-readable debug representation of a nullability vector.
std::string nullabilityToString(const TypeNullability &Nullability);

/// A function that may provide enhanced nullability information for a
/// substituted template parameter (which has no sugar of its own).
using GetTypeParamNullability =
    std::optional<TypeNullability>(const SubstTemplateTypeParmType *ST);
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
std::string printWithNullability(QualType, const TypeNullability &,
                                 ASTContext &);
/// Returns an equivalent type annotated with the provided nullability.
/// Any existing sugar (including nullability) is discarded.
/// Symbolic nullability is not annotated.
/// rebuildWithNullability(int *, {Nullable}) ==> int * _Nullable.
QualType rebuildWithNullability(QualType, const TypeNullability &,
                                ASTContext &);

/// Computes the number of pointer slots within a type.
/// Each of these could conceptually be nullable, so this is the length of
/// the nullability vector computed by getNullabilityAnnotationsFromType().
unsigned countPointersInType(QualType T);
unsigned countPointersInType(const Expr *E);
unsigned countPointersInType(TemplateArgument TA);
unsigned countPointersInType(const DeclContext *DC);

/// Returns the type of an expression for the purposes of nullability.
/// This handles wrinkles in the type system like BoundMember.
QualType exprType(const Expr *E);

TypeNullability unspecifiedNullability(const Expr *E);

}  // namespace clang::tidy::nullability

#endif
