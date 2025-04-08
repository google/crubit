// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_analysis/object.h"

#include <cassert>
#include <optional>
#include <ostream>
#include <string>
#include <utility>

#include "absl/strings/str_cat.h"
#include "absl/strings/str_format.h"
#include "lifetime_annotations/function_lifetimes.h"
#include "lifetime_annotations/lifetime.h"
#include "clang/include/clang/AST/Type.h"

namespace clang {
namespace tidy {
namespace lifetimes {

Object::Object(Lifetime lifetime, clang::QualType type,
               std::optional<FunctionLifetimes> func_lifetimes)
    : lifetime_(lifetime),
      type_(type),
      func_lifetimes_(std::move(func_lifetimes)) {
  assert(!type.isNull());
}

std::string Object::DebugString() const {
  std::string result = absl::StrFormat("p%p %s", this, lifetime_.DebugString());
  if (func_lifetimes_.has_value()) {
    absl::StrAppend(&result, " (fn: ", func_lifetimes_->DebugString(), ")");
  }
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
