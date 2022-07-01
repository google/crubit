// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_analysis/object.h"

#include <string>

#include "absl/strings/str_cat.h"
#include "absl/strings/str_format.h"
#include "lifetime_annotations/lifetime.h"
#include "clang/AST/Decl.h"

namespace clang {
namespace tidy {
namespace lifetimes {

Object::Object(Lifetime lifetime, clang::QualType type)
    : lifetime_(lifetime), type_(type), func_(nullptr) {
  assert(!type.isNull());
}

Object::Object(const clang::FunctionDecl& func)
    : Object(Lifetime::Static(), func.getType()) {
  func_ = &func;
}

std::string Object::DebugString() const {
  std::string result = absl::StrFormat("p%p %s", this, lifetime_.DebugString());
  if (!type_.isNull()) {
    absl::StrAppend(&result, " (", type_.getAsString(), ")");
  }
  return result;
}

std::ostream& operator<<(std::ostream& os, Object object) {
  return os << object.DebugString();
}

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
