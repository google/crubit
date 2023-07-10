// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// StrongInt<T> is a simple template class mechanism for defining "logical"
// integer-like class types that support almost all of the same functionality
// as native integer types, but which prevents assignment, construction, and
// other operations from other integer-like types.  In other words, you cannot
// assign from raw integer types or other StrongInt<> types, nor can you do
// most arithmetic or logical operations.
//
// A StrongInt<T> should compile away to a raw T in optimized mode. What this
// means is that the generated assembly for:
//
//    int64 foo = 123;
//    int64 bar = 456;
//    bool is_equal = (foo == bar);
//    constexpr int64 fubar = 789;
//
// ...should be identical to the generated assembly for:
//
//    CRUBIT_DEFINE_STRONG_INT_TYPE(MyStrongInt, int64);
//    MyStrongInt foo(123);
//    MyStrongInt bar(456);
//    bool is_equal = (foo == bar);
//    constexpr MyStrongInt fubar(789);
//
// Since the methods are all inline and non-virtual and the class has just
// one data member, the compiler can erase the StrongInt class entirely in its
// code-generation phase.  This also means that you can pass StrongInt<T>
// around by value just as you would a raw T.
//
// It is important to note that StrongInt does NOT generate compile time
// warnings or errors for overflows on implicit constant conversions.
// For example, the below demonstrates a case where the 2 are not equivalent
// at compile time and can lead to subtle initialization bugs:
//
//    CRUBIT_DEFINE_STRONG_INT_TYPE(MyStrongInt8, int8);
//    int8 foo = 1024;        // Compile error: const conversion to ...
//    MyStrongInt8 foo(1024); // Compiles ok: foo has undefined / 0 value.
//
// Usage:
//   CRUBIT_DEFINE_STRONG_INT_TYPE(Name, NativeType);
//
//     Defines a new StrongInt type named 'Name' in the current namespace with
//     no validation of operations.
//
//     Name: The desired name for the new StrongInt typedef.  Must be unique
//         within the current namespace.
//     NativeType: The primitive integral type this StrongInt will hold, as
//         defined by std::numeric_limits::is_integer (see <type_traits>).
//
// Supported operations:
//     StrongInt<T> = StrongInt<T>
//     StrongInt<T> == StrongInt<T>
//     StrongInt<T> < StrongInt<T>
//
//   This class also provides a .value() accessor method and defines a hash
//   functor that allows the IntType to be used as key to hashable containers.

#ifndef CRUBIT_COMMON_STRONG_INT_H_
#define CRUBIT_COMMON_STRONG_INT_H_

#include <cstdint>
#include <iosfwd>
#include <limits>
#include <ostream>
#include <type_traits>
#include <utility>

#include "absl/hash/hash.h"

namespace crubit {

// Holds an integer value (of type NativeType) and behaves as a NativeType by
// exposing assignment, unary, comparison, and arithmetic operators.
//
// This class is NOT thread-safe.
template <typename TagType, typename NativeType>
class StrongInt {
 public:
  typedef NativeType ValueType;

  // Default value initialization.
  constexpr StrongInt() : value_(NativeType()) {}

  // Explicit initialization from a numeric primitive.
  template <
      class T,
      class = std::enable_if_t<std::is_same_v<
          decltype(static_cast<ValueType>(std::declval<T>())), ValueType>>>
  explicit constexpr StrongInt(T init_value)
      : value_(static_cast<ValueType>(init_value)) {}

  // Use the default copy constructor, assignment, and destructor.

  // Accesses the raw value.
  constexpr ValueType value() const { return value_; }

  // Metadata functions.
  static constexpr StrongInt Max() {
    return StrongInt(std::numeric_limits<ValueType>::max());
  }
  static constexpr StrongInt Min() {
    return StrongInt(std::numeric_limits<ValueType>::min());
  }

  template <typename H>
  friend H AbslHashValue(H h, const StrongInt &i) {
    return H::combine(std::move(h), i.value_);
  }

 private:
  // The integer value of type ValueType.
  ValueType value_;

  static_assert(std::numeric_limits<ValueType>::is_integer,
                "invalid integer type for strong int");
};

// Provide the << operator, primarily for logging purposes.
template <typename TagType, typename ValueType>
std::ostream &operator<<(std::ostream &os, StrongInt<TagType, ValueType> arg) {
  return os << arg.value();
}

// Provide the << operator, primarily for logging purposes. Specialized for int8
// so that an integer and not a character is printed.
template <typename TagType>
std::ostream &operator<<(std::ostream &os, StrongInt<TagType, int8_t> arg) {
  return os << static_cast<int>(arg.value());
}

// Provide the << operator, primarily for logging purposes. Specialized for
// uint8 so that an integer and not a character is printed.
template <typename TagType>
std::ostream &operator<<(std::ostream &os, StrongInt<TagType, uint8_t> arg) {
  return os << static_cast<unsigned int>(arg.value());
}

// Define comparison operators.  We allow all comparison operators.
#define CRUBIT_STRONG_INT_COMPARISON_OP(op)                       \
  template <typename TagType, typename ValueType>                 \
  constexpr bool operator op(StrongInt<TagType, ValueType> lhs,   \
                             StrongInt<TagType, ValueType> rhs) { \
    return lhs.value() op rhs.value();                            \
  }
CRUBIT_STRONG_INT_COMPARISON_OP(==);  // NOLINT(whitespace/operators)
CRUBIT_STRONG_INT_COMPARISON_OP(!=);  // NOLINT(whitespace/operators)
CRUBIT_STRONG_INT_COMPARISON_OP(<);   // NOLINT(whitespace/operators)
CRUBIT_STRONG_INT_COMPARISON_OP(<=);  // NOLINT(whitespace/operators)
CRUBIT_STRONG_INT_COMPARISON_OP(>);   // NOLINT(whitespace/operators)
CRUBIT_STRONG_INT_COMPARISON_OP(>=);  // NOLINT(whitespace/operators)
#undef CRUBIT_STRONG_INT_COMPARISON_OP

}  // namespace crubit

// Defines the StrongInt using value_type and typedefs it to type_name.
// The struct int_type_name ## _tag_ trickery is needed to ensure that a new
// type is created per type_name.
#define CRUBIT_DEFINE_STRONG_INT_TYPE(type_name, value_type)                 \
  typedef ::crubit::StrongInt<class type_name##_strong_int_tag_, value_type> \
      type_name;

#endif  // CRUBIT_COMMON_STRONG_INT_H_
