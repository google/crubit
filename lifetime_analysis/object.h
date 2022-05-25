// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef DEVTOOLS_RUST_CC_INTEROP_LIFETIME_ANALYSIS_OBJECT_H_
#define DEVTOOLS_RUST_CC_INTEROP_LIFETIME_ANALYSIS_OBJECT_H_

#include <atomic>
#include <functional>
#include <string>

#include "lifetime_annotations/lifetime.h"
#include "clang/AST/Decl.h"
#include "clang/AST/Type.h"
#include "llvm/ADT/Hashing.h"

namespace clang {
namespace tidy {
namespace lifetimes {

// Any object that has a lifetime. Multiple objects might have the same
// lifetime, but two equal objects always have the same lifetime.
class Object {
 public:
  // Creates an invalid object.
  //
  // This is provided because containers need default constructors. It is not
  // legal to perform any operations on an invalid object except to copy or
  // delete it.
  //
  // Use one of the static member functions below to create a valid object.
  Object();

  Object(const Object&) = default;
  Object& operator=(const Object&) = default;

  // Creates a new object with the given lifetime and type.
  static Object Create(Lifetime lifetime, clang::QualType type);

  // Creates a new object representing a declared function.
  static Object CreateFromFunctionDecl(const clang::FunctionDecl& func);

  // Returns the lifetime of the object.
  Lifetime GetLifetime() const { return lifetime_; }

  clang::QualType Type() const { return type_; }

  // Returns a textual representation of the object for debug logging.
  std::string DebugString() const;

  // Returns the function that this object represents, if any.
  const clang::FunctionDecl* GetFunc() const { return func_; }

  bool operator==(Object other) const { return id_ == other.id_; }

  bool operator!=(Object other) const { return !(*this == other); }

 private:
  Object(int id, Lifetime lifetime, clang::QualType type);

  bool IsValid() const;

  static Object InvalidEmpty();
  static Object InvalidTombstone();

  friend class llvm::DenseMapInfo<Object>;
  friend class std::less<Object>;

  int id_;
  Lifetime lifetime_;
  clang::QualType type_;
  const clang::FunctionDecl* func_;
  static std::atomic<int> next_id_;
};

std::ostream& operator<<(std::ostream& os, Object object);

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

namespace llvm {

template <>
struct DenseMapInfo<clang::tidy::lifetimes::Object> {
  static clang::tidy::lifetimes::Object getEmptyKey() {
    return clang::tidy::lifetimes::Object::InvalidEmpty();
  }

  static clang::tidy::lifetimes::Object getTombstoneKey() {
    return clang::tidy::lifetimes::Object::InvalidTombstone();
  }

  static unsigned getHashValue(clang::tidy::lifetimes::Object object) {
    return llvm::hash_value(object.id_);
  }

  static bool isEqual(clang::tidy::lifetimes::Object lhs,
                      clang::tidy::lifetimes::Object rhs) {
    return lhs == rhs;
  }
};

}  // namespace llvm

namespace std {

template <>
struct less<clang::tidy::lifetimes::Object> {
  bool operator()(const clang::tidy::lifetimes::Object& p1,
                  const clang::tidy::lifetimes::Object& p2) const {
    return p1.id_ < p2.id_;
  }
};

}  // namespace std

#endif  // DEVTOOLS_RUST_CC_INTEROP_LIFETIME_ANALYSIS_OBJECT_H_
