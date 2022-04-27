// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_LIFETIME_ANNOTATIONS_LIFETIME_H_
#define CRUBIT_LIFETIME_ANNOTATIONS_LIFETIME_H_

#include <atomic>
#include <cassert>
#include <functional>
#include <ostream>
#include <string>
#include <utility>

#include "llvm/ADT/Hashing.h"

namespace clang {
namespace tidy {
namespace lifetimes {

// A lifetime variable or constant lifetime.
class Lifetime {
 public:
  // Creates an invalid lifetime.
  //
  // This is provided because containers need default constructors. It is not
  // legal to perform any operations on an invalid lifetime except to copy or
  // delete it.
  //
  // Use one of the static member functions below to create a valid lifetime.
  Lifetime();

  Lifetime(const Lifetime&) = default;
  Lifetime& operator=(const Lifetime&) = default;

  // Creates a new lifetime variable.
  static Lifetime CreateVariable();

  // Returns the 'static lifetime constant.
  static Lifetime Static();

  // Creates a new local lifetime constant.
  static Lifetime CreateLocal();

  // Returns whether this lifetime is a lifetime variable.
  bool IsVariable() const;

  // Returns whether this lifetime is a constant lifetime.
  bool IsConstant() const;

  // Returns whether this lifetime is a local lifetime.
  bool IsLocal() const;

  // Returns a unique numeric ID for the lifetime.
  int Id() const { return id_; }

  // Returns a textual representation of the lifetime for debug logging.
  std::string DebugString() const;

  bool operator==(Lifetime other) const {
    assert(IsValid());
    assert(other.IsValid());
    return id_ == other.id_;
  }

  bool operator!=(Lifetime other) const { return !(*this == other); }

 private:
  explicit Lifetime(int id);

  static Lifetime InvalidEmpty();
  static Lifetime InvalidTombstone();

  bool IsValid() const;

  friend class llvm::DenseMapInfo<Lifetime, void>;
  friend class std::less<Lifetime>;

  int id_;
  static std::atomic<int> next_variable_id_;
  static std::atomic<int> next_local_id_;
};

std::ostream& operator<<(std::ostream& os, Lifetime lifetime);

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

namespace llvm {

template <>
struct DenseMapInfo<clang::tidy::lifetimes::Lifetime, void> {
  static clang::tidy::lifetimes::Lifetime getEmptyKey() {
    return clang::tidy::lifetimes::Lifetime::InvalidEmpty();
  }

  static clang::tidy::lifetimes::Lifetime getTombstoneKey() {
    return clang::tidy::lifetimes::Lifetime::InvalidTombstone();
  }

  static unsigned getHashValue(clang::tidy::lifetimes::Lifetime lifetime) {
    return llvm::hash_value(lifetime.id_);
  }

  static bool isEqual(clang::tidy::lifetimes::Lifetime lhs,
                      clang::tidy::lifetimes::Lifetime rhs) {
    return lhs.id_ == rhs.id_;
  }
};

}  // namespace llvm

namespace std {

template <>
struct less<clang::tidy::lifetimes::Lifetime> {
  bool operator()(const clang::tidy::lifetimes::Lifetime& l1,
                  const clang::tidy::lifetimes::Lifetime& l2) const {
    return l1.id_ < l2.id_;
  }
};

}  // namespace std

#endif  // CRUBIT_LIFETIME_ANNOTATIONS_LIFETIME_H_
