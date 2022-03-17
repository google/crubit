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

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_UTIL_STRING_TYPE_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_UTIL_STRING_TYPE_H_

#include <memory>
#include <ostream>  // NOLINT
#include <string>

#include "third_party/absl/container/flat_hash_set.h"
#include "third_party/absl/flags/marshalling.h"
#include "third_party/absl/meta/type_traits.h"
#include "third_party/absl/strings/string_view.h"

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
#define CRUBIT_DEFINE_STRING_TYPE_AS(string_type_name,                \
                                     StringTypeRepresentation)        \
  struct string_type_name##_tag_ {                                    \
    static absl::string_view TypeName() { return #string_type_name; } \
    using Representation = StringTypeRepresentation;                  \
  };                                                                  \
  using string_type_name =                                            \
      ::rs_bindings_from_cc::StringType<string_type_name##_tag_>;

#define CRUBIT_DEFINE_STRING_TYPE(string_type_name) \
  CRUBIT_DEFINE_STRING_TYPE_AS(string_type_name, std::string);

namespace rs_bindings_from_cc {

// StringType provides these operations:
//   * relational operators (==, !=, <, <=, >, >=)
//   * compare (future <=> operator)
//   * AbslHashValue
//   * streaming with operator<<
//   * value(), which should return a string-like object (const string&,
//     absl::string_view, ShortString<N>, etc.)
//
// It is parameterized here by a "TagAndRepresentation" struct.  It's a unique
// struct defined in each CRUBIT_DEFINE_STRING_TYPE_AS macro invocation, so
// every StringType is its own unique, strong type.  In addition, it provides a
// static TypeName() method that returns the name of the type, and it provides a
// "Representation" type alias which is held as a member by each StringType
// instance.
//
// When a StringType instance method is called, it will first try to dispatch
// the call directly to the Representation instance.  If the Representation
// class doesn't have this method, StringType will call value() on its
// representation instance, and then call the method on the returned value.
// We do this dispatch at compile-time using <internal link>.
template <typename TagAndRepresentation>
class StringType {
 public:
  // Shorthand.
  using Rep = typename TagAndRepresentation::Representation;

 private:
  // Using SFINAE, you can "rank" implementations using this mechanism.  Given
  // two acceptable implementations that substitute correctly, the compiler will
  // choose the one that requires fewer type conversions, so a method given a
  // Rank0 object will prefer an implementation that accepts a Rank0 type to an
  // implementation that accepts a Rank1 type, since a Rank1 argument requires a
  // derived->base conversion.
  struct Rank1 {};
  struct Rank0 : Rank1 {};

  // This would normally be declared at the bottom of the class, but we need to
  // move it up here to satisfy the compiler.
  //
  // Since the return type of value(), below, is either the same as the return
  // type of t_.value(), if such a method exists, or t_ itself, if t_.value()
  // doesn't exists, we need to declare our t_ up here so the compiler knows
  // what it is and whether it has a value() method.
  Rep t_;

  // These must be defined before value() is defined below, since value()'s
  // return type is determined by what these methods return.
  template <typename T>
  static auto DispatchValue(const T& t, Rank0) -> decltype(t.value()) {
    return t.value();
  }
  static const Rep& DispatchValue(const Rep& t, Rank1) { return t; }

 public:
  StringType() = default;
  template <typename T, typename = absl::enable_if_t<
                            std::is_constructible<Rep, T&&>::value>>
  explicit StringType(T&& value) : t_(std::forward<T>(value)) {}

  // Returns the name of this StringType, as used in code.
  static absl::string_view TypeName() {
    return TagAndRepresentation::TypeName();
  }

  ABSL_DEPRECATED("Use TypeName()")
  static absl::string_view TypeId() { return TypeName(); }

  // Returns the result of the representation's value() method, or the
  // representation itself if the representation has no value() method.
  auto value() const -> decltype(DispatchValue(t_, Rank0{})) {
    return DispatchValue(t_, Rank0{});
  }

  bool empty() const { return DispatchEmpty(t_, Rank0{}); }

  // If you want to optimize your relational methods, you need only implement
  // these three: compare, operator==, and operator<.
  int compare(const StringType& other) const {
    return DispatchCompare(*this, other, Rank0{});
  }
  friend bool operator==(const StringType& left, const StringType& right) {
    return DispatchEquals(left, right, Rank0{});
  }
  friend bool operator<(const StringType& left, const StringType& right) {
    return DispatchLessThan(left, right, Rank0{});
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
    return s.DispatchHash(std::move(h), Rank0{});
  }

  friend std::ostream& operator<<(std::ostream& os, const StringType& s) {
    return DispatchOstream(os, s.t_, Rank0{});
  }

 private:
  template <typename T>
  static auto DispatchEmpty(const T& t, Rank0) -> decltype(t.empty()) {
    return t.empty();
  }
  static bool DispatchEmpty(const Rep& t, Rank1) {
    return DispatchValue(t, Rank0{}).empty();
  }

  // This overload exists specifically for absl::Cord, which spells 'compare' as
  // 'Compare'.
  template <typename T>
  static auto DispatchCompare(const T& left, const T& right, Rank0)
      -> decltype(left.value().Compare(right.value())) {
    return left.value().Compare(right.value());
  }
  template <typename T>
  static auto DispatchCompare(const T& left, const T& right, Rank0)
      -> decltype(left.t_.compare(right.t_)) {
    return left.t_.compare(right.t_);
  }
  template <typename T>
  static auto DispatchCompare(const T& left, const T& right, Rank1)
      -> decltype(left.value().compare(right.value())) {
    return left.value().compare(right.value());
  }

  template <typename T>
  static auto DispatchEquals(const T& left, const T& right, Rank0)
      -> decltype(left.t_ == right.t_) {
    return left.t_ == right.t_;
  }
  static bool DispatchEquals(const StringType& left, const StringType& right,
                             Rank1) {
    return left.value() == right.value();
  }

  template <typename T>
  static auto DispatchLessThan(const T& left, const T& right, Rank0)
      -> decltype(left.t_ < right.t_) {
    return left.t_ < right.t_;
  }
  static bool DispatchLessThan(const StringType& left, const StringType& right,
                               Rank1) {
    return left.value() < right.value();
  }

  template <
      typename H, typename T = Rep,
      typename Hashable = decltype(absl::Hash<T>()(std::declval<const T&>()))>
  H DispatchHash(H h, Rank0) const {
    return H::combine(std::move(h), t_);
  }
  template <typename H>
  H DispatchHash(H h, Rank1) const {
    return H::combine(std::move(h), value());
  }

  template <typename T>
  static auto DispatchOstream(std::ostream& os, const T& t, Rank0)
      -> decltype(os << t) {
    return os << t;
  }
  static std::ostream& DispatchOstream(std::ostream& os, const Rep& t, Rank1) {
    return os << DispatchValue(t, Rank0{});
  }
};

// Allows typed strings to be used as ABSL_FLAG values.
//
// This is equivalent in behavior to just using a raw std::string.
template <typename TagAndRepresentation>
bool AbslParseFlag(absl::string_view text,
                   StringType<TagAndRepresentation>* out, std::string* error) {
  *out = StringType<TagAndRepresentation>(text);
  return true;
}

template <typename TagAndRepresentation>
std::string AbslUnparseFlag(const StringType<TagAndRepresentation>& val) {
  return absl::UnparseFlag(std::string(val.value()));
}

}  // namespace rs_bindings_from_cc

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_UTIL_STRING_TYPE_H_
