// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_analysis/object.h"

#include <string>

#include "absl/strings/str_cat.h"
#include "lifetime_annotations/lifetime.h"
#include "clang/AST/Decl.h"

namespace clang {
namespace tidy {
namespace lifetimes {

constexpr int INVALID_OBJECT_ID_EMPTY = 0;
constexpr int INVALID_OBJECT_ID_TOMBSTONE = 1;
constexpr int FIRST_OBJECT_ID = 2;

std::atomic<int> Object::next_id_{FIRST_OBJECT_ID};

Object::Object() : id_(INVALID_OBJECT_ID_EMPTY) {}

Object Object::Create(Lifetime lifetime, clang::QualType type) {
  assert(!type.isNull());
  return Object(next_id_++, lifetime, type);
}

Object Object::CreateFromFunctionDecl(const clang::FunctionDecl& func) {
  Object ret = Create(Lifetime::Static(), func.getType());
  ret.func_ = &func;
  return ret;
}

std::string Object::DebugString() const {
  assert(IsValid());

  switch (id_) {
    case INVALID_OBJECT_ID_EMPTY:
      return "INVALID_EMPTY";
    case INVALID_OBJECT_ID_TOMBSTONE:
      return "INVALID_TOMBSTONE";
    default: {
      std::string result = absl::StrCat("p", id_, " ", lifetime_.DebugString());
      if (!type_.isNull()) {
        absl::StrAppend(&result, " (", type_.getAsString(), ")");
      }
      return result;
    }
  }
}

Object::Object(int id, Lifetime lifetime, clang::QualType type)
    : id_(id), lifetime_(lifetime), type_(type), func_(nullptr) {}

Object Object::InvalidEmpty() {
  return Object(INVALID_OBJECT_ID_EMPTY, Lifetime(), clang::QualType());
}

Object Object::InvalidTombstone() {
  return Object(INVALID_OBJECT_ID_TOMBSTONE, Lifetime(), clang::QualType());
}

bool Object::IsValid() const {
  return id_ != INVALID_OBJECT_ID_EMPTY && id_ != INVALID_OBJECT_ID_TOMBSTONE;
}

std::ostream& operator<<(std::ostream& os, Object object) {
  return os << object.DebugString();
}

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
