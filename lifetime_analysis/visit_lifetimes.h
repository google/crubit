// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef DEVTOOLS_RUST_CC_INTEROP_LIFETIME_ANALYSIS_VISIT_LIFETIMES_H_
#define DEVTOOLS_RUST_CC_INTEROP_LIFETIME_ANALYSIS_VISIT_LIFETIMES_H_

#include "lifetime_analysis/object_set.h"
#include "lifetime_annotations/type_lifetimes.h"
#include "clang/AST/Decl.h"
#include "clang/AST/Type.h"
#include "llvm/ADT/DenseSet.h"

namespace clang {
namespace tidy {
namespace lifetimes {

// A visitor interface used with VisitLifetimes.
//
// An implementation of this interface does two things:
// - Defines how a type with lifetimes should be traversed
// - Collects information from the traversal
//
// An implementation needs to define two functions:
// - Traverse() maps an object of reference-like type to the corresponding
//   points-to set. This function typically also collects information from
//   the traversal.
// - GetFieldObjects() maps an object of struct type to the objects for
//   its fields.
class LifetimeVisitor {
 public:
  // Returns the object representing the given `field` of the struct represented
  // by `objects`. As all the objects in `objects` represent a single class
  // hierarchy, down to the class that defines the field, they must all have the
  // same field object.
  virtual const Object* GetFieldObject(const ObjectSet& objects,
                                       const clang::FieldDecl* field) = 0;
  // Returns the object representing the given `base` of the struct represented
  // by `objects`. As all the objects in `objects` represent a single class
  // hierarchy, down to the class that defines the base class, they must all
  // have the same base object.
  virtual const Object* GetBaseClassObject(const ObjectSet& objects,
                                           clang::QualType base) = 0;
  // Returns the ObjectSet pointed to by the objects in the input
  // ObjectSet, which are assumed to have lifetimes
  // `lifetimes`. Returning an empty set will stop the visit.
  virtual ObjectSet Traverse(const ObjectLifetimes& lifetimes,
                             const ObjectSet& objects, int pointee_depth) = 0;
  virtual ~LifetimeVisitor() {}
};

// Visits the objects and fields of `type` using the given `visitor`;
// `object_lifetimes` corresponds to the lifetimes of an object of type `type`.
// `points_to_set` should contain a set of objects that are assumed to be of
// type `type`.
void VisitLifetimes(const ObjectSet& points_to_set, clang::QualType type,
                    const ObjectLifetimes& object_lifetimes,
                    LifetimeVisitor& visitor);

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

#endif  // DEVTOOLS_RUST_CC_INTEROP_LIFETIME_ANALYSIS_VISIT_LIFETIMES_H_
