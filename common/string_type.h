// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Typed strings.

// Example usage:
//
//  CRUBIT_DEFINE_STRING_TYPE(Foo);
//  CRUBIT_DEFINE_STRING_TYPE(Bar);
//  Foo foo("foo_value");
//  Bar bar("bar_value");
//
// The following two statements will not compile.
//
//  foo = bar;
//
//  if (foo == bar) { } else { }; ...
//
// The strongly-typed types are hashable with the Abseil hashing framework,
// so they work out of the box with any modern hashing system. For non-Abseil
// uses, the explicit functors absl::Hash<Foo>, absl::Hash<Bar> may be used:
//
//  std::unordered_set<Foo, absl::Hash<Foo>>
//  __gnu_cxx::hash_map<Bar, int, absl::Hash<Bar>>
//
// (But absl::flat_hash_set<Foo> is much better!)

#ifndef CRUBIT_COMMON_STRING_TYPE_H_
#define CRUBIT_COMMON_STRING_TYPE_H_

#include <memory>
#include <ostream>  // NOLINT
#include <string>
#include <utility>

#include "absl/container/flat_hash_set.h"
#include "absl/flags/marshalling.h"
#include "absl/meta/type_traits.h"
#include "absl/strings/string_view.h"

// Defines the StringType using StringTypeRepresentation and provides a type
// alias to string_type_name.  The struct string_type_name ## _tag_ trickery is
// needed to ensure that a new type is created per string_type_name.
//
// StringTypeRepresentation classes, as a rule, should either *be* a string-like
// object, or should provide a "value()" method that returns a string-like
// object.  If they can provide more optimal implementations of relational
// operators, they should define operator== and operator<; all other relational
// operators are defined in terms of those.  If they can provide more optimal
// implementations of AbslHashValue or operator<<, they should provide those as
// well.
#define CRUBIT_DEFINE_STRING_TYPE(string_type_name) \
  using string_type_name = ::crubit::StringType<class string_type_name##_tag_>;

namespace crubit {

// StringType provides these operations:
//   * relational operators (==, !=, <, <=, >, >=)
//   * compare (future <=> operator)
//   * AbslHashValue
//   * streaming with operator<<
//   * value(), which should return a string-like object (const string&,
//     absl::string_view, ShortString<N>, etc.)
template <typename Tag>
class StringType {
 public:
  StringType() = default;
  explicit StringType(std::string value) : s_(std::move(value)) {}

  const std::string& value() const { return s_; }

  bool empty() const { return value().empty(); }

  // If you want to optimize your relational methods, you need only implement
  // these three: compare, operator==, and operator<.
  int compare(const StringType& other) const {
    return value().compare(other.value());
  }
  friend bool operator==(const StringType& left, const StringType& right) {
    return left.value() == right.value();
  }
  friend bool operator<(const StringType& left, const StringType& right) {
    return left.value() < right.value();
  }

  // These methods are defined in terms of the above.
  friend bool operator!=(const StringType& left, const StringType& right) {
    return !(left == right);
  }
  friend bool operator>(const StringType& left, const StringType& right) {
    return right < left;
  }
  friend bool operator<=(const StringType& left, const StringType& right) {
    return !(left > right);
  }
  friend bool operator>=(const StringType& left, const StringType& right) {
    return !(left < right);
  }

  template <typename H>
  friend H AbslHashValue(H h, const StringType& s) {
    H::combine(std::move(h), s.value());
  }

  friend std::ostream& operator<<(std::ostream& os, const StringType& s) {
    return os << s.value();
  }

 private:
  std::string s_;
};

// Allows typed strings to be used as ABSL_FLAG values.
//
// This is equivalent in behavior to just using a raw std::string.
template <typename Tag>
bool AbslParseFlag(absl::string_view text, StringType<Tag>* out,
                   std::string* error) {
  *out = StringType<Tag>(text);
  return true;
}

template <typename Tag>
std::string AbslUnparseFlag(const StringType<Tag>& val) {
  return absl::UnparseFlag(std::string(val.value()));
}

}  // namespace crubit

#endif  // CRUBIT_COMMON_STRING_TYPE_H_
