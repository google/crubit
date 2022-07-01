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
  Object(Object&&) = delete;
  Object& operator=(Object&&) = delete;

  // Creates an object with the given lifetime and type.
  // This constructor should only be used in tests. Outside of tests, use
  // one of the ObjectRepository::CreateObject...() functions.
  Object(Lifetime lifetime, clang::QualType type);

  // Creates an object representing a declared function.
  // This constructor should only be used in tests. Outside of tests, use
  // one of the ObjectRepository::CreateObject...() functions.
  Object(const clang::FunctionDecl& func);

  Object(const Object&) = default;
  Object& operator=(const Object&) = default;

  // Returns the lifetime of the object.
  Lifetime GetLifetime() const { return lifetime_; }

  clang::QualType Type() const { return type_; }

  // Returns a textual representation of the object for debug logging.
  std::string DebugString() const;

  // Returns the function that this object represents, if any.
  const clang::FunctionDecl* GetFunc() const { return func_; }

 private:
  Lifetime lifetime_;
  clang::QualType type_;
  const clang::FunctionDecl* func_;
};

std::ostream& operator<<(std::ostream& os, Object object);

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang

#endif  // DEVTOOLS_RUST_CC_INTEROP_LIFETIME_ANALYSIS_OBJECT_H_
