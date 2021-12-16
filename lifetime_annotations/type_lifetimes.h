// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_LIFETIME_ANNOTATIONS_TYPE_LIFETIMES_H_
#define CRUBIT_LIFETIME_ANNOTATIONS_TYPE_LIFETIMES_H_

#include <functional>
#include <memory>
#include <string>

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
  ValueLifetimes() = default;

  ValueLifetimes(const ValueLifetimes& other) { *this = other; }

  ValueLifetimes& operator=(const ValueLifetimes& other);

  std::string DebugString() const;

 private:
  static ValueLifetimes FromTypeLifetimes(TypeLifetimesRef& type_lifetimes,
                                          clang::QualType type);

  const std::optional<ValueLifetimes>& GetTemplateArgumentLifetimes(
      const clang::SubstTemplateTypeParmType* targ) const {
    size_t template_arg_idx = targ->getReplacedParameter()->getIndex();
    assert(template_arg_idx < template_argument_lifetimes_.size());
    return template_argument_lifetimes_[template_arg_idx];
  }

  // Note: only one of `pointee_lifetime` or `template_argument_lifetimes`
  // is non-empty.
  std::unique_ptr<ObjectLifetimes> pointee_lifetimes_;
  std::vector<std::optional<ValueLifetimes>> template_argument_lifetimes_;
  // TODO(veluca): add lifetime parameters here.

  friend class ObjectLifetimes;
  friend class llvm::DenseMapInfo<devtools_rust::ObjectLifetimes>;
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

  std::string DebugString() const;

  // Returns the ObjectLifetimes for an object of a given type, whose lifetimes
  // are scoped within or derived from the object that this lifetimes
  // represents, i.e. it is a field or a base class of the object.
  // `type` must be a record type (class, struct or union).
  ObjectLifetimes GetRecordObjectLifetimes(clang::QualType type) const;

  // Returns the ObjectLifetimes of the pointed-to object. Type() must be a
  // pointer or reference type.
  const ObjectLifetimes& GetPointeeLifetimes() const;

  // Returns the lifetime of the object itself.
  Lifetime GetLifetime() const { return lifetime_; }

  // Returns the type of the object.
  clang::QualType Type() const { return type_; }

 private:
  ObjectLifetimes() = default;

  friend class llvm::DenseMapInfo<devtools_rust::ObjectLifetimes>;
  friend class ValueLifetimes;
  Lifetime lifetime_;
  ValueLifetimes value_lifetimes_;
  clang::QualType type_;
};

const llvm::ArrayRef<clang::TemplateArgument> GetTemplateArgs(
    clang::QualType type);

}  // namespace devtools_rust

namespace llvm {

template <>
struct DenseMapInfo<devtools_rust::ObjectLifetimes> {
  static devtools_rust::ObjectLifetimes getEmptyKey() {
    devtools_rust::ObjectLifetimes ret;
    ret.lifetime_ = DenseMapInfo<devtools_rust::Lifetime>().getEmptyKey();
    return ret;
  }

  static devtools_rust::ObjectLifetimes getTombstoneKey() {
    devtools_rust::ObjectLifetimes ret;
    ret.lifetime_ = DenseMapInfo<devtools_rust::Lifetime>().getTombstoneKey();
    return ret;
  }

  static bool isEqual(const devtools_rust::ObjectLifetimes& lhs,
                      const devtools_rust::ObjectLifetimes& rhs) {
    return DenseMapInfo<devtools_rust::Lifetime>::isEqual(lhs.lifetime_,
                                                          rhs.lifetime_) &&
           isEqual(lhs.value_lifetimes_, rhs.value_lifetimes_) &&
           lhs.type_ == rhs.type_;
  }

  static unsigned getHashValue(
      const devtools_rust::ObjectLifetimes& object_lifetimes) {
    unsigned hash = DenseMapInfo<devtools_rust::Lifetime>::getHashValue(
        object_lifetimes.lifetime_);
    hash = hash_combine(hash, getHashValue(object_lifetimes.value_lifetimes_));
    return hash_combine(hash, DenseMapInfo<clang::QualType>::getHashValue(
                                  object_lifetimes.type_));
  }

 private:
  static bool isEqual(const devtools_rust::ValueLifetimes& lhs,
                      const devtools_rust::ValueLifetimes& rhs);

  static unsigned getHashValue(
      const devtools_rust::ValueLifetimes& lifetime_node);
};

}  // namespace llvm

#endif  // CRUBIT_LIFETIME_ANNOTATIONS_TYPE_LIFETIMES_H_
