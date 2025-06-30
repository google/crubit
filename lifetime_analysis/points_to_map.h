// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef DEVTOOLS_RUST_CC_INTEROP_LIFETIME_ANALYSIS_POINTS_TO_MAP_H_
#define DEVTOOLS_RUST_CC_INTEROP_LIFETIME_ANALYSIS_POINTS_TO_MAP_H_

#include <string>
#include <vector>

#include "lifetime_analysis/object.h"
#include "lifetime_analysis/object_set.h"
#include "lifetime_annotations/lifetime.h"
#include "clang/AST/Expr.h"
#include "llvm/ADT/DenseMap.h"

namespace clang {
namespace tidy {
namespace lifetimes {

// Maintains the points-to sets needed for the analysis of a function.
// A `PointsToMap` stores points-to sets for
// - Objects of reference-like type
// - Expressions that are prvalues of pointer type or glvalues (glvalues are,
//   in spirit, references to the object they refer to.)
// - The function's return value, if it is of reference-like type
//
// Note that the relationship between an expression's type and the type of the
// objects associated with it depends on whether the expression is a glvalue or
// prvalue:
// - glvalue expressions are associated with the object that is identified by
//   the glvalue. This means that the object has the same type as the glvalue
//   expression.
// - prvalue expressions of pointer type as are associated with the object that
//   the pointer points to. This means that if the prvalue expression has type
//   `T *`, the object has type `T`.
// The PointsToMap class does not enforce these type relationships because we
// intend to allow type punning (at least within the implementations of
// functions).
class PointsToMap {
 public:
  PointsToMap() = default;

  PointsToMap(const PointsToMap&) = default;
  PointsToMap(PointsToMap&&) = default;
  PointsToMap& operator=(const PointsToMap&) = default;
  PointsToMap& operator=(PointsToMap&&) = default;

  bool operator==(const PointsToMap& other) const;
  bool operator!=(const PointsToMap& other) const { return !(*this == other); }

  // Returns a human-readable representation of this object.
  std::string DebugString() const;

  const llvm::DenseMap<const Object*, ObjectSet>& PointerPointsTos() const {
    return pointer_points_tos_;
  }

  // Returns a `PointsToMap` containing the union of mappings from this map and
  // `other`.
  // If both this map and `other` associate a points-to set with the same
  // entity, the returned map associates that entity with the union of the
  // corresponding points-to sets.
  PointsToMap Union(const PointsToMap& other) const;

  // Returns the points-to set associated with `pointer`, or an empty set if
  // `pointer` is not associated with a points-to set.
  ObjectSet GetPointerPointsToSet(const Object* pointer) const;

  // Associates `pointer` with the given points-to set.
  void SetPointerPointsToSet(const Object* pointer, ObjectSet points_to);

  // Associates all `pointers` with the given points-to set.
  void SetPointerPointsToSet(const ObjectSet& pointers,
                             const ObjectSet& points_to);

  // Extends a single `pointer`'s points-to set with the given points-to set.
  void ExtendPointerPointsToSet(const Object* pointer,
                                const ObjectSet& points_to);

  // Returns the union of the points-to sets associated with the given pointers,
  // or an empty set if none of the pointers is associated with a points-to set.
  ObjectSet GetPointerPointsToSet(const ObjectSet& pointers) const;

  // Returns the object set associated with `expr`.
  // `expr` must previously have been associated with an object set through
  // a call to SetExprObjectSet(), and the function asserts that this is the
  // case. We intentionally don't return an empty object set in this case
  // because we want to notice if we're not propagating object sets through
  // expressions.
  ObjectSet GetExprObjectSet(const clang::Expr* expr) const;

  // Associates `expr` with the given object set.
  void SetExprObjectSet(const clang::Expr* expr, ObjectSet objects);

  // Returns if `expr` has an object set.
  bool ExprHasObjectSet(const clang::Expr* expr) const;

  // Returns all the pointers (not objects) with the given `lifetime`.
  std::vector<const Object*> GetAllPointersWithLifetime(
      Lifetime lifetime) const;

 private:
  llvm::DenseMap<const Object*, ObjectSet> pointer_points_tos_;
  llvm::DenseMap<const clang::Expr*, ObjectSet> expr_objects_;
};

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

#endif  // DEVTOOLS_RUST_CC_INTEROP_LIFETIME_ANALYSIS_POINTS_TO_MAP_H_
