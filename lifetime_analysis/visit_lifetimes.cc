// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_analysis/visit_lifetimes.h"

#include <string>
#include <utility>

#include "lifetime_analysis/object.h"
#include "lifetime_analysis/object_set.h"
#include "lifetime_annotations/pointee_type.h"
#include "lifetime_annotations/type_lifetimes.h"
#include "clang/AST/Attr.h"
#include "clang/AST/Attrs.inc"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/Type.h"
#include "llvm/ADT/SmallVector.h"
#include "llvm/ADT/StringRef.h"

namespace clang {
namespace tidy {
namespace lifetimes {
namespace {

llvm::SmallVector<std::string> GetFieldLifetimeArguments(
    const clang::FieldDecl* field) {
  // TODO(mboehme): Report errors as Clang diagnostics, not through
  // llvm::report_fatal_error().

  const clang::AnnotateAttr* member_lifetimes_attr = nullptr;
  for (auto annotate : field->specific_attrs<clang::AnnotateAttr>()) {
    if (annotate->getAnnotation() == "member_lifetimes") {
      if (member_lifetimes_attr) {
        llvm::report_fatal_error("repeated lifetime annotation");
      }
      member_lifetimes_attr = annotate;
    }
  }
  if (!member_lifetimes_attr) {
    return {};
  }

  llvm::SmallVector<std::string> ret;
  for (const auto& arg : member_lifetimes_attr->args()) {
    llvm::StringRef lifetime;
    if (llvm::Error err = EvaluateAsStringLiteral(arg, field->getASTContext())
                              .moveInto(lifetime)) {
      llvm::report_fatal_error(llvm::StringRef(toString(std::move(err))));
    }
    ret.push_back(lifetime.str());
  }

  return ret;
}

template <typename Callback>
void ForEachField(ObjectSet objects, clang::QualType record_type,
                  const ObjectLifetimes& object_lifetimes,
                  LifetimeVisitor& visitor, const Callback& callback) {
  for (clang::FieldDecl* f :
       record_type->getAs<clang::RecordType>()->getDecl()->fields()) {
    ObjectLifetimes field_lifetimes = object_lifetimes.GetFieldOrBaseLifetimes(
        f->getType(), GetFieldLifetimeArguments(f));
    callback(objects, field_lifetimes, f);
  }
  if (auto* cxxrecord = clang::dyn_cast<clang::CXXRecordDecl>(
          record_type->getAs<clang::RecordType>()->getDecl())) {
    for (const clang::CXXBaseSpecifier& base : cxxrecord->bases()) {
      auto base_object_lifetimes = object_lifetimes.GetFieldOrBaseLifetimes(
          base.getType(), GetLifetimeParameters(base.getType()));
      auto base_object = visitor.GetBaseClassObject(objects, base.getType());
      ObjectSet next_objects = objects;
      next_objects.Add(base_object);
      ForEachField(next_objects, base.getType(), base_object_lifetimes, visitor,
                   callback);
    }
  }
}

void VisitLifetimesImpl(const ObjectSet& points_to_set,
                        const ObjectLifetimes& object_lifetimes,
                        llvm::DenseSet<const Object*>& visited_objects,
                        LifetimeVisitor& visitor, int pointee_depth);

// Traverse fields while walking up base classes. This can be a bit wasteful
// for cases like diamond inheritance (which is hopefully not common).
void TraverseObjectFieldsWithBases(
    const ObjectSet& object_set, clang::QualType record_type,
    const ObjectLifetimes& object_lifetimes,
    llvm::DenseSet<const Object*>& visited_object, LifetimeVisitor& visitor,
    int pointee_depth) {
  assert(record_type->isRecordType());
  if (record_type->isIncompleteType()) {
    return;
  }
  // Our analysis relies on objects reachable in the same way to be visited in
  // the same call, thus we need to "merge" together the `Object`s that come
  // from the same field but different `object`s in the object_set.
  llvm::SmallVector<std::pair<ObjectSet, ObjectLifetimes>> fields_to_visit;
  for (const Object* object : object_set) {
    // This code relies on the vist order of ForEachField being independent
    // of `object`.
    size_t next_field = 0;
    ForEachField(
        {object}, record_type, object_lifetimes, visitor,
        [&](const ObjectSet& bases, const ObjectLifetimes& field_lifetimes,
            const clang::FieldDecl* f) {
          size_t field = next_field++;
          if (field == fields_to_visit.size()) {
            fields_to_visit.emplace_back(ObjectSet(),
                                         std::move(field_lifetimes));
          }
          const Object* field_object = visitor.GetFieldObject(bases, f);
          fields_to_visit[field].first.Add(field_object);
        });
  }
  for (auto [objects, lifetimes] : std::move(fields_to_visit)) {
    VisitLifetimesImpl(objects, lifetimes, visited_object, visitor,
                       pointee_depth);
  }
}

void VisitLifetimesImpl(const ObjectSet& points_to_set,
                        const ObjectLifetimes& object_lifetimes,
                        llvm::DenseSet<const Object*>& visited_objects,
                        LifetimeVisitor& visitor, int pointee_depth) {
  size_t num_visited_before = visited_objects.size();
  for (const Object* object : points_to_set) {
    visited_objects.insert(object);
  }
  if (num_visited_before == visited_objects.size()) {
    // No new object -> nothing to do. This avoids infinite loops.
    return;
  }
  object_lifetimes.GetValueLifetimes().Type()->dump();

  if (const clang::QualType type = object_lifetimes.GetValueLifetimes().Type();
      type->isRecordType()) {
    TraverseObjectFieldsWithBases(points_to_set, type, object_lifetimes,
                                  visited_objects, visitor, pointee_depth);
  }

  // TODO(veluca): here we call Traverse even when there is no child type.
  // This is likely an indication that it is better to split up Traverse into
  // multiple methods.
  clang::QualType child_type =
      PointeeType(object_lifetimes.GetValueLifetimes().Type());

  ObjectSet child_object =
      visitor.Traverse(object_lifetimes, points_to_set, pointee_depth);

  if (!child_object.empty() && !child_type.isNull()) {
    VisitLifetimesImpl(
        child_object,
        object_lifetimes.GetValueLifetimes().GetPointeeLifetimes(),
        visited_objects, visitor, pointee_depth + 1);
  }
}

}  // namespace

void VisitLifetimes(const ObjectSet& points_to_set, clang::QualType type,
                    const ObjectLifetimes& object_lifetimes,
                    LifetimeVisitor& visitor) {
  llvm::DenseSet<const Object*> visited_objects;
  VisitLifetimesImpl(points_to_set, object_lifetimes, visited_objects, visitor,
                     /*pointee_depth=*/0);
}

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
