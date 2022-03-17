// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// StrongInt<T> is a simple template class mechanism for defining "logical"
// integer-like class types that support almost all of the same functionality
// as native integer types, but which prevents assignment, construction, and
// other operations from other integer-like types.  In other words, you cannot
// assign from raw integer types or other StrongInt<> types, nor can you do
// most arithmetic or logical operations.  This provides a simple form of
// dimensionality in that you can add two instances of StrongInt<T>, producing
// a StrongInt<T>, but you can not add a StrongInt<T> and a raw T nor can you
// add a StrongInt<T> and a StrongInt<U>.  Details on supported operations are
// below.
//
// In addition to type strength, StrongInt provides a way to inject (optional)
// validation of the various operations.  This allows you to define StrongInt
// types that check for overflow conditions and react in standard or custom
// ways.
//
// A StrongInt<T> with a NullStrongIntValidator should compile away to a raw T
// in optimized mode.  What this means is that the generated assembly for:
//
//   int64 foo = 123;
//   int64 bar = 456;
//   int64 baz = foo + bar;
//   constexpr int64 fubar = 789;
//
// ...should be identical to the generated assembly for:
//
//    CRUBIT_DEFINE_STRONG_INT_TYPE(MyStrongInt, int64);
//    MyStrongInt foo(123);
//    MyStrongInt bar(456);
//    MyStrongInt baz = foo + bar;
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
//  StrongInt<TagType, NativeType, ValidatorType = NullStrongIntValidator>
//
//    Creates a new StrongInt instance directly.
//
//     TagType: The unique type which discriminates this StrongInt<T> from
//         other StrongInt<U> types.
//     NativeType: The primitive integral type this StrongInt will hold, as
//         defined by std::numeric_limits::is_integer (see <type_traits>).
//     ValidatorType: The type of validation used by this StrongInt type.  A
//         few pre-built validator types are provided here, but the caller can
//         define any custom validator they desire.
//
// Supported operations:
//     StrongInt<T> = StrongInt<T>
//     !StrongInt<T> => bool
//     ~StrongInt<T> => StrongInt<T>
//     -StrongInt<T> => StrongInt<T>
//     +StrongInt<T> => StrongInt<T>
//     ++StrongInt<T> => StrongInt<T>
//     StrongInt<T>++ => StrongInt<T>
//     --StrongInt<T> => StrongInt<T>
//     StrongInt<T>-- => StrongInt<T>
//     StrongInt<T> + StrongInt<T> => StrongInt<T>
//     StrongInt<T> - StrongInt<T> => StrongInt<T>
//     StrongInt<T> * (numeric type) => StrongInt<T>
//     StrongInt<T> / (numeric type) => StrongInt<T>
//     StrongInt<T> % (numeric type) => StrongInt<T>
//     StrongInt<T> << (numeric type) => StrongInt<T>
//     StrongInt<T> >> (numeric type) => StrongInt<T>
//     StrongInt<T> & StrongInt<T> => StrongInt<T>
//     StrongInt<T> | StrongInt<T> => StrongInt<T>
//     StrongInt<T> ^ StrongInt<T> => StrongInt<T>
//
//   For binary operations, the equivalent op-equal (eg += vs. +) operations are
//   also supported.  Other operator combinations should cause compile-time
//   errors.
//
//   This class also provides a .value() accessor method and defines a hash
//   functor that allows the IntType to be used as key to hashable containers.
//
// Validators:
//   NullStrongIntValidator: Do no validation.  This should be entirely
//       optimized away by the compiler.

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_UTIL_STRONG_INT_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_UTIL_STRONG_INT_H_

#include <cstdint>
#include <hash_map>
#include <iosfwd>
#include <iterator>
#include <limits>
#include <ostream>
#include <type_traits>
#include <utility>

#include "third_party/absl/base/macros.h"
#include "third_party/absl/meta/type_traits.h"
#include "third_party/absl/strings/string_view.h"

namespace rs_bindings_from_cc {

// Define the validators which can be plugged-in to make StrongInt resilient to
// things like overflows. This is a do-nothing implementation of the
// compile-time interface.
//
// NOTE: For all validation functions that operate on an existing StrongInt<T>,
// the type argument 'T' *must* be StrongInt<T>::ValueType (the int type being
// strengthened).
//
struct NullStrongIntValidator {
  // Note that this templated default implementation has an arbitrary bool
  // return value for the sole purpose of conforming to c++11 constexpr.
  //
  // Custom validator implementations can choose to return void or use a similar
  // return value constexpr construct if constexpr initialization is desirable.
  //
  // The StrongInt class does not care about or use the returned value. Any
  // returned value is solely there to allow the constexpr declaration; custom
  // validators can only fail / abort when detecting an invalid value.
  //
  // For example, other than the constexpr behavior, the below 2 custom
  // validator implementations are logically equivalent:
  //
  //   template<typename T, typename U>
  //   static void ValidateInit(U arg) {
  //     if (arg < 0) LOG(FATAL) << "arg < 0";
  //   }
  //
  //   template<typename T, typename U>
  //   static constexpr bool ValidateInit(U arg) {
  //     return (arg < 0) ? (LOG(FATAL) << "arg < 0", false) : false;
  //   }
  //
  // A constexpr implementation has the added advantage that the validation can
  // take place (fail) at compile time.

  // Verify initialization of StrongInt<T> from arg, type U.
  template<typename T, typename U>
  static constexpr bool ValidateInit(U arg) { return true; }
  // Verify -value.
  template <typename T>
  static constexpr bool ValidateNegate(T /*value*/) {
    return true;
  }
  // Verify ~value;
  template <typename T>
  static constexpr bool ValidateBitNot(T /*value*/) {
    return true;
  }
  // Verify lhs + rhs.
  template <typename T>
  static constexpr bool ValidateAdd(T /*lhs*/, T /*rhs*/) {
    return true;
  }
  // Verify lhs - rhs.
  template <typename T>
  static constexpr bool ValidateSubtract(T /*lhs*/, T /*rhs*/) {
    return true;
  }
  // Verify lhs * rhs.
  template <typename T, typename U>
  static constexpr bool ValidateMultiply(T /*lhs*/, U /*rhs*/) {
    return true;
  }
  // Verify lhs / rhs.
  template <typename T, typename U>
  static constexpr bool ValidateDivide(T /*lhs*/, U /*rhs*/) {
    return true;
  }
  // Verify lhs % rhs.
  template <typename T, typename U>
  static constexpr bool ValidateModulo(T /*lhs*/, U /*rhs*/) {
    return true;
  }
  // Verify lhs << rhs.
  template <typename T>
  static constexpr bool ValidateLeftShift(T /*lhs*/, int64_t /*rhs*/) {
    return true;
  }
  // Verify lhs >> rhs.
  template <typename T>
  static constexpr bool ValidateRightShift(T /*lhs*/, int64_t /*rhs*/) {
    return true;
  }
  // Verify lhs & rhs.
  template <typename T>
  static constexpr bool ValidateBitAnd(T /*lhs*/, T /*rhs*/) {
    return true;
  }
  // Verify lhs | rhs.
  template <typename T>
  static constexpr bool ValidateBitOr(T /*lhs*/, T /*rhs*/) {
    return true;
  }
  // Verify lhs ^ rhs.
  template <typename T>
  static constexpr bool ValidateBitXor(T /*lhs*/, T /*rhs*/) {
    return true;
  }
};

// Holds an integer value (of type NativeType) and behaves as a NativeType by
// exposing assignment, unary, comparison, and arithmetic operators.
//
// This class is NOT thread-safe.
template<typename TagType, typename NativeType,
         typename ValidatorType = NullStrongIntValidator>
class StrongInt {
 public:
  typedef NativeType ValueType;

  struct ABSL_DEPRECATED("Use absl::Hash instead") Hasher {
    size_t operator()(const StrongInt &x) const {
      return static_cast<size_t>(x.value());
    }
  };

  static constexpr absl::string_view TypeName() { return TagType::TypeName(); }

  // Default value initialization.
  constexpr StrongInt()
      : value_((ValidatorType::template ValidateInit<ValueType>(NativeType()),
                NativeType())) {}

  // Explicit initialization from another StrongInt type that has an
  // implementation of:
  //
  //    ToType StrongIntConvert(FromType source, ToType*);
  //
  // This uses Argument Dependent Lookup (ADL) to find which function to
  // call.
  //
  // Example: Assume you have two StrongInt types.
  //
  //      CRUBIT_DEFINE_STRONG_INT_TYPE(Bytes, int64);
  //      CRUBIT_DEFINE_STRONG_INT_TYPE(Megabytes, int64);
  //
  //  If you want to be able to (explicitly) construct an instance of Bytes from
  //  an instance of Megabytes, simply define a converter function in the same
  //  namespace as either Bytes or Megabytes (or both):
  //
  //      Megabytes StrongIntConvert(Bytes arg, Megabytes* /* unused */) {
  //        return Megabytes((arg >> 20).value());
  //      };
  //
  //  The second argument is needed to differentiate conversions, and it always
  //  passed as NULL.
  template <typename ArgTagType, typename ArgNativeType,
            typename ArgValidatorType>
  explicit constexpr StrongInt(
      StrongInt<ArgTagType, ArgNativeType, ArgValidatorType> arg)
      // We have to pass both the "from" type and the "to" type as args for the
      // conversions to be differentiated.  The converter can not be a template
      // because explicit template call syntax defeats ADL.
      : value_(
            StrongIntConvert(arg, static_cast<StrongInt *>(nullptr)).value()) {}

  // Explicit initialization from a numeric primitive.
  template <
      class T,
      class = std::enable_if_t<std::is_same_v<
          decltype(static_cast<ValueType>(std::declval<T>())), ValueType>>>
  explicit constexpr StrongInt(T init_value)
      : value_((ValidatorType::template ValidateInit<ValueType>(init_value),
                static_cast<ValueType>(init_value))) {}

  // Use the default copy constructor, assignment, and destructor.

  // Accesses the raw value.
  constexpr ValueType value() const { return value_; }

  // Accesses the raw value, with cast.
  // Primarily for compatibility with int-type.h
  template <typename ValType>
  constexpr ValType value() const { return static_cast<ValType>(value_); }

  // Explicitly cast the raw value only if the underlying value is convertible
  // to T.
  template <typename T,
            typename = absl::enable_if_t<absl::conjunction<
                std::bool_constant<std::numeric_limits<T>::is_integer>,
                std::is_convertible<ValueType, T>>::value>>
  constexpr explicit operator T() const {
    return value_;
  }

  // Metadata functions.
  static constexpr StrongInt Max() {
    return StrongInt(std::numeric_limits<ValueType>::max());
  }
  static constexpr StrongInt Min() {
    return StrongInt(std::numeric_limits<ValueType>::min());
  }

  // Unary operators.
  bool operator!() const {
    return value_ == 0;
  }
  const StrongInt operator+() const {
    return StrongInt(value_);
  }
  const StrongInt operator-() const {
    ValidatorType::template ValidateNegate<ValueType>(value_);
    return StrongInt(-value_);
  }
  const StrongInt operator~() const {
    ValidatorType::template ValidateBitNot<ValueType>(value_);
    return StrongInt(ValueType(~value_));
  }

  // Increment and decrement operators.
  StrongInt &operator++() {  // ++x
    ValidatorType::template ValidateAdd<ValueType>(value_, ValueType(1));
    ++value_;
    return *this;
  }
  const StrongInt operator++(int postfix_flag) {  // x++
    ValidatorType::template ValidateAdd<ValueType>(value_, ValueType(1));
    StrongInt temp(*this);
    ++value_;
    return temp;
  }
  StrongInt &operator--() {  // --x
    ValidatorType::template ValidateSubtract<ValueType>(value_, ValueType(1));
    --value_;
    return *this;
  }
  const StrongInt operator--(int postfix_flag) {  // x--
    ValidatorType::template ValidateSubtract<ValueType>(value_, ValueType(1));
    StrongInt temp(*this);
    --value_;
    return temp;
  }

  // Action-Assignment operators.
  StrongInt &operator+=(StrongInt arg) {
    ValidatorType::template ValidateAdd<ValueType>(value_, arg.value());
    value_ += arg.value();
    return *this;
  }
  StrongInt &operator-=(StrongInt arg) {
    ValidatorType::template ValidateSubtract<ValueType>(value_, arg.value());
    value_ -= arg.value();
    return *this;
  }
  template<typename ArgType>
  StrongInt &operator*=(ArgType arg) {
    ValidatorType::template ValidateMultiply<ValueType, ArgType>(value_, arg);
    value_ *= arg;
    return *this;
  }
  template<typename ArgType>
  StrongInt &operator/=(ArgType arg) {
    ValidatorType::template ValidateDivide<ValueType, ArgType>(value_, arg);
    value_ /= arg;
    return *this;
  }
  template<typename ArgType>
  StrongInt &operator%=(ArgType arg) {
    ValidatorType::template ValidateModulo<ValueType, ArgType>(value_, arg);
    value_ %= arg;
    return *this;
  }
  StrongInt &operator<<=(int64_t arg) {  // NOLINT(whitespace/operators)
    ValidatorType::template ValidateLeftShift<ValueType>(value_, arg);
    value_ <<= arg;
    return *this;
  }
  StrongInt &operator>>=(int64_t arg) {  // NOLINT(whitespace/operators)
    ValidatorType::template ValidateRightShift<ValueType>(value_, arg);
    value_ >>= arg;
    return *this;
  }
  StrongInt &operator&=(StrongInt arg) {
    ValidatorType::template ValidateBitAnd<ValueType>(value_, arg.value());
    value_ &= arg.value();
    return *this;
  }
  StrongInt &operator|=(StrongInt arg) {
    ValidatorType::template ValidateBitOr<ValueType>(value_, arg.value());
    value_ |= arg.value();
    return *this;
  }
  StrongInt &operator^=(StrongInt arg) {
    ValidatorType::template ValidateBitXor<ValueType>(value_, arg.value());
    value_ ^= arg.value();
    return *this;
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
template<typename TagType, typename ValueType, typename ValidatorType>
std::ostream &operator<<(std::ostream &os,
                         StrongInt<TagType, ValueType, ValidatorType> arg) {
  return os << arg.value();
}

// Provide the << operator, primarily for logging purposes. Specialized for int8
// so that an integer and not a character is printed.
template <typename TagType, typename ValidatorType>
std::ostream &operator<<(std::ostream &os,
                         StrongInt<TagType, int8_t, ValidatorType> arg) {
  return os << static_cast<int>(arg.value());
}

// Provide the << operator, primarily for logging purposes. Specialized for
// uint8 so that an integer and not a character is printed.
template <typename TagType, typename ValidatorType>
std::ostream &operator<<(std::ostream &os,
                         StrongInt<TagType, uint8_t, ValidatorType> arg) {
  return os << static_cast<unsigned int>(arg.value());
}

// Define operators that take two StrongInt arguments.
#define CRUBIT_STRONG_INT_VS_STRONG_INT_BINARY_OP(op, validator)          \
  template <typename TagType, typename ValueType, typename ValidatorType> \
  constexpr StrongInt<TagType, ValueType, ValidatorType> operator op(     \
      StrongInt<TagType, ValueType, ValidatorType> lhs,                   \
      StrongInt<TagType, ValueType, ValidatorType> rhs) {                 \
    return ValidatorType::template validator<ValueType>(lhs.value(),      \
                                                        rhs.value()),     \
           StrongInt<TagType, ValueType, ValidatorType>(                  \
               static_cast<ValueType>(lhs.value() op rhs.value()));       \
  }
CRUBIT_STRONG_INT_VS_STRONG_INT_BINARY_OP(+, ValidateAdd);
CRUBIT_STRONG_INT_VS_STRONG_INT_BINARY_OP(-, ValidateSubtract);
CRUBIT_STRONG_INT_VS_STRONG_INT_BINARY_OP(&, ValidateBitAnd);
CRUBIT_STRONG_INT_VS_STRONG_INT_BINARY_OP(|, ValidateBitOr);
CRUBIT_STRONG_INT_VS_STRONG_INT_BINARY_OP(^, ValidateBitXor);
#undef CRUBIT_STRONG_INT_VS_STRONG_INT_BINARY_OP

// Define operators that take one StrongInt and one native integer argument.
// These operators are defined in terms of their op-equal member function
// cousins, mostly.
#define CRUBIT_STRONG_INT_VS_NUMERIC_BINARY_OP(op, validator)              \
  template <typename TagType, typename ValueType, typename ValidatorType,  \
            typename NumType>                                              \
  constexpr StrongInt<TagType, ValueType, ValidatorType> operator op(      \
      StrongInt<TagType, ValueType, ValidatorType> lhs, NumType rhs) {     \
    return ValidatorType::template validator<ValueType>(lhs.value(), rhs), \
           StrongInt<TagType, ValueType, ValidatorType>(                   \
               static_cast<ValueType>(lhs.value() op rhs));                \
  }
// This is used for commutative operators between one StrongInt and one native
// integer argument.  That is a long way of saying "multiplication".
#define CRUBIT_NUMERIC_VS_STRONG_INT_BINARY_OP(op, validator)              \
  template <typename TagType, typename ValueType, typename ValidatorType,  \
            typename NumType>                                              \
  constexpr StrongInt<TagType, ValueType, ValidatorType> operator op(      \
      NumType lhs, StrongInt<TagType, ValueType, ValidatorType> rhs) {     \
    return ValidatorType::template validator<ValueType>(rhs.value(), lhs), \
           StrongInt<TagType, ValueType, ValidatorType>(                   \
               static_cast<ValueType>(rhs.value() op lhs));                \
  }
CRUBIT_STRONG_INT_VS_NUMERIC_BINARY_OP(*, ValidateMultiply);
CRUBIT_NUMERIC_VS_STRONG_INT_BINARY_OP(*, ValidateMultiply);
CRUBIT_STRONG_INT_VS_NUMERIC_BINARY_OP(/, ValidateDivide);
CRUBIT_STRONG_INT_VS_NUMERIC_BINARY_OP(%, ValidateModulo);
CRUBIT_STRONG_INT_VS_NUMERIC_BINARY_OP(<<, ValidateLeftShift);
CRUBIT_STRONG_INT_VS_NUMERIC_BINARY_OP(>>, ValidateRightShift);
#undef CRUBIT_STRONG_INT_VS_NUMERIC_BINARY_OP
#undef CRUBIT_NUMERIC_VS_STRONG_INT_BINARY_OP

// Define comparison operators.  We allow all comparison operators.
#define CRUBIT_STRONG_INT_COMPARISON_OP(op)                               \
  template <typename TagType, typename ValueType, typename ValidatorType> \
  constexpr bool operator op(                                             \
      StrongInt<TagType, ValueType, ValidatorType> lhs,                   \
      StrongInt<TagType, ValueType, ValidatorType> rhs) {                 \
    return lhs.value() op rhs.value();                                    \
  }
CRUBIT_STRONG_INT_COMPARISON_OP(==);  // NOLINT(whitespace/operators)
CRUBIT_STRONG_INT_COMPARISON_OP(!=);  // NOLINT(whitespace/operators)
CRUBIT_STRONG_INT_COMPARISON_OP(<);   // NOLINT(whitespace/operators)
CRUBIT_STRONG_INT_COMPARISON_OP(<=);  // NOLINT(whitespace/operators)
CRUBIT_STRONG_INT_COMPARISON_OP(>);   // NOLINT(whitespace/operators)
CRUBIT_STRONG_INT_COMPARISON_OP(>=);  // NOLINT(whitespace/operators)
#undef CRUBIT_STRONG_INT_COMPARISON_OP

// Support for-range loops. Enables easier looping over ranges of StrongInts,
// especially looping over sub-ranges of StrongVectors.
template <typename IntType>
class StrongIntRange {
 public:
  // Iterator over the indices.
  class StrongIntRangeIterator {
   public:
    using value_type = IntType;
    using difference_type = IntType;
    using reference = const IntType &;
    using pointer = const IntType *;
    using iterator_category = std::input_iterator_tag;

    explicit StrongIntRangeIterator(IntType initial) : current_(initial) {}
    bool operator!=(const StrongIntRangeIterator &other) const {
      return current_ != other.current_;
    }
    bool operator==(const StrongIntRangeIterator &other) const {
      return current_ == other.current_;
    }
    value_type operator*() const { return current_; }
    pointer operator->() const { return &current_; }
    StrongIntRangeIterator &operator++() {
      ++current_;
      return *this;
    }
    StrongIntRangeIterator operator++(int) {
      StrongIntRangeIterator old_iter = *this;
      ++current_;
      return old_iter;
    }

   private:
    IntType current_;
  };

  // Loops from IntType(0) up to (but not including) end.
  explicit StrongIntRange(IntType end) : begin_(IntType(0)), end_(end) {}
  // Loops from begin up to (but not including) end.
  StrongIntRange(IntType begin, IntType end) : begin_(begin), end_(end) {}
  StrongIntRangeIterator begin() const { return begin_; }
  StrongIntRangeIterator end() const { return end_; }

 private:
  const StrongIntRangeIterator begin_;
  const StrongIntRangeIterator end_;
};

template <typename IntType>
StrongIntRange<IntType> MakeStrongIntRange(IntType end) {
  return StrongIntRange<IntType>(end);
}

template <typename IntType>
StrongIntRange<IntType> MakeStrongIntRange(IntType begin, IntType end) {
  return StrongIntRange<IntType>(begin, end);
}

// Type trait for detecting if a type T is a StrongInt type.
template <typename T>
struct IsStrongInt : public std::false_type {};

template <typename... Ts>
struct IsStrongInt<StrongInt<Ts...>> : public std::true_type {};

}  // namespace rs_bindings_from_cc

// Defines the StrongInt using value_type and typedefs it to type_name, with no
// validation of under/overflow situations.
// The struct int_type_name ## _tag_ trickery is needed to ensure that a new
// type is created per type_name.
#define CRUBIT_DEFINE_STRONG_INT_TYPE(type_name, value_type)             \
  struct type_name##_strong_int_tag_ {                                   \
    static constexpr absl::string_view TypeName() { return #type_name; } \
  };                                                                     \
  typedef ::rs_bindings_from_cc::StrongInt<                              \
      type_name##_strong_int_tag_, value_type,                           \
      ::rs_bindings_from_cc::NullStrongIntValidator>                     \
      type_name;

// Allow StrongInt to be used as a key to hashable containers.
HASH_NAMESPACE_DECLARATION_START
template <typename Tag, typename Value, typename Validator>
struct hash<rs_bindings_from_cc::StrongInt<Tag, Value, Validator>>
    : ::rs_bindings_from_cc::StrongInt<Tag, Value, Validator>::Hasher {};
HASH_NAMESPACE_DECLARATION_END

// Numeric_limits override for strong int.
namespace std {

template <typename TagType, typename NativeType, typename ValidatorType>
struct numeric_limits<
    rs_bindings_from_cc::StrongInt<TagType, NativeType, ValidatorType>> {
 private:
  using StrongIntT =
      rs_bindings_from_cc::StrongInt<TagType, NativeType, ValidatorType>;

 public:
  static constexpr bool is_specialized = true;
  static constexpr bool is_signed = numeric_limits<NativeType>::is_signed;
  static constexpr bool is_integer = numeric_limits<NativeType>::is_integer;
  static constexpr bool is_exact = numeric_limits<NativeType>::is_exact;
  static constexpr bool has_infinity = numeric_limits<NativeType>::has_infinity;
  static constexpr bool has_quiet_NaN =
      numeric_limits<NativeType>::has_quiet_NaN;
  static constexpr bool has_signaling_NaN =
      numeric_limits<NativeType>::has_signaling_NaN;
  static constexpr float_denorm_style has_denorm =
      numeric_limits<NativeType>::has_denorm;
  static constexpr bool has_denorm_loss =
      numeric_limits<NativeType>::has_denorm_loss;
  static constexpr float_round_style round_style =
      numeric_limits<NativeType>::round_style;
  static constexpr bool is_iec559 = numeric_limits<NativeType>::is_iec559;
  static constexpr bool is_bounded = numeric_limits<NativeType>::is_bounded;
  static constexpr bool is_modulo = numeric_limits<NativeType>::is_modulo;
  static constexpr int digits = numeric_limits<NativeType>::digits;
  static constexpr int digits10 = numeric_limits<NativeType>::digits10;
  static constexpr int max_digits10 = numeric_limits<NativeType>::max_digits10;
  static constexpr int radix = numeric_limits<NativeType>::radix;
  static constexpr int min_exponent = numeric_limits<NativeType>::min_exponent;
  static constexpr int min_exponent10 =
      numeric_limits<NativeType>::min_exponent10;
  static constexpr int max_exponent = numeric_limits<NativeType>::max_exponent;
  static constexpr int max_exponent10 =
      numeric_limits<NativeType>::max_exponent10;
  static constexpr bool traps = numeric_limits<NativeType>::traps;
  static constexpr bool tinyness_before =
      numeric_limits<NativeType>::tinyness_before;

  static constexpr StrongIntT(min)() { return StrongIntT(StrongIntT::Min()); }
  static constexpr StrongIntT lowest() { return StrongIntT(StrongIntT::Min()); }
  static constexpr StrongIntT(max)() { return StrongIntT(StrongIntT::Max()); }
  static constexpr StrongIntT epsilon() { return StrongIntT(); }
  static constexpr StrongIntT round_error() { return StrongIntT(); }
  static constexpr StrongIntT infinity() { return StrongIntT(); }
  static constexpr StrongIntT quiet_NaN() { return StrongIntT(); }
  static constexpr StrongIntT signaling_NaN() { return StrongIntT(); }
  static constexpr StrongIntT denorm_min() { return StrongIntT(); }
};

}  // namespace std

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_UTIL_STRONG_INT_H_
