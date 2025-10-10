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

#include "absl/base/nullability.h"
#include "absl/log/check.h"
#include "nullability/pragma.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Decl.h"
#include "clang/AST/Expr.h"
#include "clang/AST/NestedNameSpecifier.h"
#include "clang/AST/Type.h"
#include "clang/AST/TypeLoc.h"
#include "clang/Analysis/FlowSensitive/Arena.h"
#include "clang/Analysis/FlowSensitive/Formula.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Basic/Specifiers.h"
#include "llvm/ADT/STLFunctionalExtras.h"

namespace clang::tidy::nullability {

/// Is this exactly a pointer type that we track outer nullability for?
/// This unwraps sugar, i.e. it looks at the canonical type.
///
/// (For now, only regular `PointerType`s and smart pointers, in future we
/// should consider supporting pointer-to-member, ObjC pointers, etc).
bool isSupportedPointerType(QualType);

/// Is this exactly a raw (non-smart) pointer type that we track outer
/// nullability for?
/// This unwraps sugar, i.e. it looks at the canonical type.
bool isSupportedRawPointerType(QualType);

/// Is this exactly a smart pointer type that we track outer nullability for?
/// This unwraps sugar, i.e. it looks at the canonical type.
bool isSupportedSmartPointerType(QualType);

/// The Unknown annotation should only be applied directly to pointer types.
/// Typedefs are not valid, except for certain "transparent aliases" of smart
/// pointer templates (conceptually, those that just give them a new name).
/// When applied to other types, Unknown is ignored.
bool isUnknownValidOn(QualType);

/// Returns the raw pointer type underlying a smart pointer type.
/// If this isn't a supported smart pointer type, returns a null type.
/// If the smart pointer type is not instantiated, falls back to determining
/// the raw pointer type from the first template argument, rather than from the
/// `pointer` or `element_type` type aliases.
/// `BaseAccess` is the most restrictive base class access specifier to accept
/// when checking whether the type is derived from a smart pointer type. We
/// need to make a distinction here as follows:
/// - A type derived from a smart pointer type is only itself considerd to be a
///   supported smart pointer type if the inheritance is public.
/// - However, if the inheritance is protected or private, we still need to
///   model the underlying pointer field because the implementation may perform
///   a copy or move from a supported smart pointer type.
QualType underlyingRawPointerType(QualType,
                                  AccessSpecifier BaseAccess = AS_public);

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

  // Creates a symbolic nullability variable with exactly the atoms specified.
  static PointerTypeNullability createSymbolic(dataflow::Atom Nonnull,
                                               dataflow::Atom Nullable);

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

/// Returns a human-readable debug representation of a nullability vector.
std::string nullabilityToString(const TypeNullability &Nullability);

/// A function that may provide enhanced nullability information for a
/// substituted template parameter (which has no sugar of its own).
using GetTypeParamNullability =
    std::optional<TypeNullability>(const SubstTemplateTypeParmType *ST);

/// Describes how we should interpret unannotated pointer types (like `int*`).
/// Typically these are treated as Unknown, and this behavior can be overridden
/// by per-file pragmas.
struct TypeNullabilityDefaults {
  // TODO(sammccall): remove this legacy constructor that ignores pragmas
  TypeNullabilityDefaults() : Ctx(nullptr), FileNullability(nullptr) {}
  TypeNullabilityDefaults(ASTContext &Ctx, const NullabilityPragmas &Pragmas)
      : Ctx(&Ctx), FileNullability(&Pragmas) {}

  // Get the effective default nullability for a particular file.
  NullabilityKind get(FileID) const;

  // The AST context is needed to resolve the associated file in some cases.
  // TODO(sammccall): this should always be provided, clean up callers.
  ASTContext *absl_nullable Ctx;
  // The nullability of pointer types in this translation unit, where no
  // nullability annotations or pragmas apply.
  NullabilityKind DefaultNullability = NullabilityKind::Unspecified;
  // Files where per-file pragmas have changed the default nullability.
  // TODO(sammccall)): this should always be provided, clean up callers.
  const NullabilityPragmas *absl_nullable FileNullability;
};

/// Traverse over a type to get its nullability. For example, if T is the type
/// Struct3Arg<int * _Nonnull, int, pair<int * _Nullable, int *>> * _Nonnull,
/// the resulting nullability annotations will be {_Nonnull, _Nonnull,
/// _Nullable, _Unknown}. Note that non-pointer elements (e.g., the second
/// argument of Struct3Arg) do not get a nullability annotation.

/// Extract nullability of a clang type written somewhere in the code.
///
/// The file where it is written affects the interpretation of unannotated
/// pointer types.
/// Where possible, prefer the foolproof TypeLoc or Decl overloads.
TypeNullability getTypeNullability(
    QualType, FileID, const TypeNullabilityDefaults &,
    llvm::function_ref<GetTypeParamNullability> SubstituteTypeParam = nullptr);

TypeNullability getTypeNullability(
    TypeLoc, const TypeNullabilityDefaults &,
    llvm::function_ref<GetTypeParamNullability> SubstituteTypeParam = nullptr);

TypeNullability getTypeNullability(
    const ValueDecl &, const TypeNullabilityDefaults &,
    llvm::function_ref<GetTypeParamNullability> SubstituteTypeParam = nullptr);

TypeNullability getTypeNullability(
    const TypeDecl &, const TypeNullabilityDefaults &,
    llvm::function_ref<GetTypeParamNullability> SubstituteTypeParam = nullptr);

/// Returns the `FileID` of the file that governs the nullability of `D`.
FileID getGoverningFile(const Decl *absl_nullable D);

/// Legacy getTypeNullability variant; treats unannotated pointers as Unknown.
/// Per-file pragmas are ignored.
/// TODO(sammccall): clean up all callers and remove this.
inline TypeNullability getNullabilityAnnotationsFromType(
    QualType T,
    llvm::function_ref<GetTypeParamNullability> SubstituteTypeParam = nullptr) {
  TypeNullabilityDefaults LegacyDefaults;
  return getTypeNullability(T, FileID(), LegacyDefaults, SubstituteTypeParam);
}

/// Prints QualType's underlying canonical type, annotated with nullability.
/// See rebuildWithNullability().
std::string printWithNullability(QualType, const TypeNullability &,
                                 ASTContext &);
/// Returns an equivalent type annotated with the provided nullability.
/// Any existing sugar (including nullability) is discarded.
/// Symbolic nullability is not annotated.
/// Smart pointers are not annotated.
/// rebuildWithNullability(int *, {Nullable}) ==> int * _Nullable.
QualType rebuildWithNullability(QualType, const TypeNullability &,
                                ASTContext &);

/// Computes the number of pointer slots within a type.
/// Each of these could conceptually be nullable, so this is the length of
/// the nullability vector computed by getTypeNullability().
unsigned countPointersInType(QualType T);
unsigned countPointersInType(const Expr *absl_nonnull E);
unsigned countPointersInType(const TemplateArgument &TA);
unsigned countPointersInType(const DeclContext *absl_nonnull DC);

/// Returns the type of an expression for the purposes of nullability.
/// This handles wrinkles in the type system like BoundMember.
QualType exprType(const Expr *absl_nonnull E);

TypeNullability unspecifiedNullability(const Expr *absl_nonnull E);

// Type and optionally location and nullability information for a single pointer
// type seen within a potentially more complex type. `Slot` indicates the
// position of this type within the nullability vector (see TypeNullability
// documentation) of the type within which `Type` was seen.
struct TypeNullabilityLoc {
  unsigned Slot = 0;
  const Type *Type = nullptr;
  std::optional<TypeLoc> Loc;
  // If either explicitly annotated (with any supported syntax) or subject to a
  // file-level pragma, this is the existing nullability annotation governing
  // the TypeLoc. Otherwise, this is empty and
  // TypeNullabilityDefaults.DefaultNullability should be used if the
  // nullability is needed.
  std::optional<NullabilityKind> ExistingAnnotation;
};

// Assembles TypeNullabilityLocs for each pointer type in the canonical type for
// `Loc`, corresponding directly with the TypeNullability that would be
// assembled for `Loc`'s type, except that the ExistingAnnotations in the
// results do not fall back to Defaults.DefaultNullability and instead report an
// empty ExistingAnnotation in those cases.
std::vector<TypeNullabilityLoc> getTypeNullabilityLocs(
    TypeLoc Loc, const TypeNullabilityDefaults &Defaults);

}  // namespace clang::tidy::nullability

#endif
