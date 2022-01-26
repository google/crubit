// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_LIFETIME_ANNOTATIONS_TYPE_LIFETIMES_H_
#define CRUBIT_LIFETIME_ANNOTATIONS_TYPE_LIFETIMES_H_

#include <functional>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "lifetime_annotations/lifetime.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Type.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/TypeOrdering.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/ADT/ArrayRef.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/ADT/DenseMapInfo.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/ADT/SmallVector.h"

namespace devtools_rust {

// Lifetimes for a `QualType`.
//
// These are ordered according to a post-order traversal of the type.
//
// TODO(mboehme): Replace occurrences of this type with ObjectLifetimes
// or otherwise clarify the relationship between the two types.
using TypeLifetimes = llvm::SmallVector<Lifetime>;
using TypeLifetimesRef = llvm::ArrayRef<Lifetime>;

// Returns a lifetime in some human-readable format.
using LifetimeFormatter = std::function<std::string(Lifetime)>;

// Variance of a reference-like type with respect to the type it refers to.
enum Variance {
  kCovariant,
  kInvariant,
};

// Returns a human-readable representation of `lifetimes`.
std::string DebugString(
    const TypeLifetimes& lifetimes,
    const LifetimeFormatter& formatter = [](Lifetime l) {
      return l.DebugString();
    });

TypeLifetimes CreateLifetimesForType(
    clang::QualType type, std::function<Lifetime()> lifetime_factory);

class ObjectLifetimes;

// Represents the lifetimes of a value; these may be 0 for non-reference-like
// types, 1 for pointers/references, and an arbitrary number for structs with
// template arguments/lifetime parameters.
// This is a more structured representation than TypeLifetimes that is easier
// to query.
class ValueLifetimes {
 public:
  static ValueLifetimes FromTypeLifetimes(TypeLifetimesRef& type_lifetimes,
                                          clang::QualType type);

  // Returns a ValueLifetimes for a lifetime-less type.
  // `type` must not be a pointer-like type or a record type.
  static ValueLifetimes ForLifetimeLessType(clang::QualType type);

  // Returns a ValueLifetimes for a pointer-like type that points to an object
  // having lifetimes `object_lifetimes`.
  static ValueLifetimes ForPointerLikeType(
      clang::QualType type, const ObjectLifetimes& object_lifetimes);

  // Returns a ValueLifetimes for a record type. If the record type has template
  // parameters, pass the corresponding template argument lifetimes in
  // `template_argument_lifetimes`; otherwise, pass an empty vector.
  static ValueLifetimes ForRecord(
      clang::QualType type,
      std::vector<std::optional<ValueLifetimes>> template_argument_lifetimes);

  ValueLifetimes(const ValueLifetimes& other) { *this = other; }

  ValueLifetimes& operator=(const ValueLifetimes& other);

  std::string DebugString() const;

  // Returns the type of the value.
  clang::QualType Type() const { return type_; }

  // Returns the ObjectLifetimes of the pointed-to object. Type() must be a
  // pointer or reference type.
  const ObjectLifetimes& GetPointeeLifetimes() const;

  // Returns the lifetimes of the i-th template argument.
  const std::optional<ValueLifetimes>& GetTemplateArgumentLifetimes(
      size_t i) const {
    assert(type_->isRecordType());
    return template_argument_lifetimes_.at(i);
  }

  size_t GetNumTemplateArguments() const {
    assert(type_->isRecordType());
    return template_argument_lifetimes_.size();
  }

 private:
  explicit ValueLifetimes(clang::QualType type) : type_(type) {}

  static void ReverseVisitTemplateArgs(
      llvm::ArrayRef<clang::TemplateArgument> template_args,
      TypeLifetimesRef& type_lifetimes, ValueLifetimes& out);

  // Note: only one of `pointee_lifetime` or `template_argument_lifetimes`
  // is non-empty.
  std::unique_ptr<ObjectLifetimes> pointee_lifetimes_;
  std::vector<std::optional<ValueLifetimes>> template_argument_lifetimes_;
  // TODO(veluca): add lifetime parameters here.
  clang::QualType type_;

  friend class llvm::DenseMapInfo<devtools_rust::ValueLifetimes>;
};

// Represents all the lifetimes of an object.
// This is a more structured representation than TypeLifetimes that is easier
// to query.
class ObjectLifetimes {
 public:
  // Constructs the ObjectLifetimes corresponding to the given `type_lifetimes`
  // when interpreted as the lifetimes of a glvalue of the given
  // `type`. Removes the consumed lifetimes from `type_lifetimes` (which
  // simulates undoing a post-order visit of the lifetime_ tree).
  static ObjectLifetimes FromTypeLifetimes(TypeLifetimesRef& type_lifetimes,
                                           clang::QualType type);

  // Returns the lifetime of the object itself.
  Lifetime GetLifetime() const { return lifetime_; }

  // Returns the lifetime of the contained value.
  const ValueLifetimes& GetValueLifetimes() const { return value_lifetimes_; }

  std::string DebugString() const;

  // Returns the ObjectLifetimes for an object of a given type, whose lifetimes
  // are scoped within or derived from the object that this lifetimes
  // represents, i.e. it is a field or a base class of the object.
  // `type` must be a record type (class, struct or union).
  ObjectLifetimes GetRecordObjectLifetimes(clang::QualType type) const;

 private:
  ObjectLifetimes(Lifetime lifetime, ValueLifetimes value_lifetimes)
      : lifetime_(lifetime), value_lifetimes_(value_lifetimes) {}

  friend class llvm::DenseMapInfo<devtools_rust::ObjectLifetimes>;
  Lifetime lifetime_;
  ValueLifetimes value_lifetimes_;
};

// TODO(lukasza): Try deduplicating GetTemplateArgs(QualType) vs
// GetTemplateArgs(TypeLoc) in
// google3/devtools/cymbal/clang_tidy/runtime/lifetimes.cc
const llvm::ArrayRef<clang::TemplateArgument> GetTemplateArgs(
    clang::QualType type);

}  // namespace devtools_rust

namespace llvm {

template <>
struct DenseMapInfo<devtools_rust::ValueLifetimes> {
  static devtools_rust::ValueLifetimes getEmptyKey() {
    return devtools_rust::ValueLifetimes(
        DenseMapInfo<clang::QualType>().getEmptyKey());
  }

  static devtools_rust::ValueLifetimes getTombstoneKey() {
    return devtools_rust::ValueLifetimes(
        DenseMapInfo<clang::QualType>().getTombstoneKey());
  }

  static bool isEqual(const devtools_rust::ValueLifetimes& lhs,
                      const devtools_rust::ValueLifetimes& rhs);

  static unsigned getHashValue(
      const devtools_rust::ValueLifetimes& value_lifetimes);
};

template <>
struct DenseMapInfo<devtools_rust::ObjectLifetimes> {
  static devtools_rust::ObjectLifetimes getEmptyKey() {
    return devtools_rust::ObjectLifetimes(
        DenseMapInfo<devtools_rust::Lifetime>().getEmptyKey(),
        DenseMapInfo<devtools_rust::ValueLifetimes>().getEmptyKey());
  }

  static devtools_rust::ObjectLifetimes getTombstoneKey() {
    return devtools_rust::ObjectLifetimes(
        DenseMapInfo<devtools_rust::Lifetime>().getTombstoneKey(),
        DenseMapInfo<devtools_rust::ValueLifetimes>().getTombstoneKey());
  }

  static bool isEqual(const devtools_rust::ObjectLifetimes& lhs,
                      const devtools_rust::ObjectLifetimes& rhs) {
    return DenseMapInfo<devtools_rust::Lifetime>::isEqual(lhs.lifetime_,
                                                          rhs.lifetime_) &&
           DenseMapInfo<devtools_rust::ValueLifetimes>::isEqual(
               lhs.value_lifetimes_, rhs.value_lifetimes_);
  }

  static unsigned getHashValue(
      const devtools_rust::ObjectLifetimes& object_lifetimes) {
    unsigned hash = DenseMapInfo<devtools_rust::Lifetime>::getHashValue(
        object_lifetimes.lifetime_);
    return hash_combine(
        hash, DenseMapInfo<devtools_rust::ValueLifetimes>::getHashValue(
                  object_lifetimes.value_lifetimes_));
  }
};

}  // namespace llvm

#endif  // CRUBIT_LIFETIME_ANNOTATIONS_TYPE_LIFETIMES_H_
