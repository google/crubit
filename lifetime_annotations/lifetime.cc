// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_annotations/lifetime.h"

#include "third_party/absl/strings/str_cat.h"

namespace clang {
namespace tidy {
namespace lifetimes {

constexpr int INVALID_LIFETIME_ID_EMPTY = 0;
constexpr int INVALID_LIFETIME_ID_TOMBSTONE = 1;
constexpr int STATIC_LIFETIME_ID = -1;
constexpr int FIRST_VARIABLE_LIFETIME_ID = 2;
constexpr int FIRST_LOCAL_LIFETIME_ID = -2;

std::atomic<int> Lifetime::next_variable_id_{FIRST_VARIABLE_LIFETIME_ID};

std::atomic<int> Lifetime::next_local_id_{FIRST_LOCAL_LIFETIME_ID};

Lifetime::Lifetime() : id_(INVALID_LIFETIME_ID_EMPTY) {}

Lifetime Lifetime::CreateVariable() { return Lifetime(next_variable_id_++); }

Lifetime Lifetime::Static() { return Lifetime(STATIC_LIFETIME_ID); }

Lifetime Lifetime::CreateLocal() { return Lifetime(next_local_id_--); }

bool Lifetime::IsVariable() const {
  assert(IsValid());
  return id_ > 0;
}

bool Lifetime::IsConstant() const {
  assert(IsValid());
  return !IsVariable();
}

bool Lifetime::IsLocal() const {
  assert(IsValid());
  return id_ <= FIRST_LOCAL_LIFETIME_ID;
}

std::string Lifetime::DebugString() const {
  assert(IsValid());

  switch (id_) {
    case INVALID_LIFETIME_ID_EMPTY:
      return "INVALID_EMPTY";
    case INVALID_LIFETIME_ID_TOMBSTONE:
      return "INVALID_TOMBSTONE";
    case STATIC_LIFETIME_ID:
      return "'static";
    default:
      if (id_ <= FIRST_LOCAL_LIFETIME_ID) {
        return absl::StrCat("'local", -id_);
      } else {
        return absl::StrCat("'", id_);
      }
  }
}

Lifetime::Lifetime(int id) : id_(id) {}

Lifetime Lifetime::InvalidEmpty() {
  return Lifetime(INVALID_LIFETIME_ID_EMPTY);
}

Lifetime Lifetime::InvalidTombstone() {
  return Lifetime(INVALID_LIFETIME_ID_TOMBSTONE);
}

bool Lifetime::IsValid() const {
  return id_ != INVALID_LIFETIME_ID_EMPTY &&
         id_ != INVALID_LIFETIME_ID_TOMBSTONE;
}

std::ostream& operator<<(std::ostream& os, Lifetime lifetime) {
  return os << lifetime.DebugString();
}

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
