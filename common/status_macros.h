// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Helper macros and methods to return and propagate errors with `absl::Status`
// and `llvm::Error`.

#ifndef CRUBIT_COMMON_STATUS_MACROS_H_
#define CRUBIT_COMMON_STATUS_MACROS_H_

#include <utility>

#include "absl/base/optimization.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "common/status_absl_compat.h"
#include "llvm/Support/Compiler.h"
#include "llvm/Support/Error.h"

namespace crubit {
namespace status_macros_internal {

inline bool IsOk(const absl::Status& status) { return status.ok(); }
inline ErrorReturn TakeError(absl::Status& status) {
  return ErrorReturn(FromAbslStatus(status));
}

template <typename T>
inline bool IsOk(const absl::StatusOr<T>& status_or) {
  return status_or.ok();
}
template <typename T>
inline ErrorReturn TakeError(absl::StatusOr<T>& status_or) {
  return ErrorReturn(FromAbslStatus(status_or.status()));
}
template <typename T>
inline auto TakeValue(absl::StatusOr<T>& status_or)
    -> decltype(std::move(status_or).value()) {
  return std::move(status_or).value();
}

inline bool IsOk(llvm::Error& error) { return !error; }
inline ErrorReturn TakeError(llvm::Error& error) {
  return ErrorReturn(std::move(error));
}

template <typename T>
inline bool IsOk(llvm::Expected<T>& expected) {
  return (bool)expected;
}
template <typename T>
inline ErrorReturn TakeError(llvm::Expected<T>& expected) {
  return ErrorReturn(expected.takeError());
}
template <typename T>
inline auto TakeValue(llvm::Expected<T>& expected)
    -> decltype(std::move(*expected)) {
  return std::move(*expected);
}

}  // namespace status_macros_internal

// Some builds do not support C++14 fully yet, using C++11 constexpr technique.
constexpr bool HasPotentialConditionalOperator(const char* lhs, int index) {
  return (index == -1 ? false
                      : (lhs[index] == '?' ? true
                                           : HasPotentialConditionalOperator(
                                                 lhs, index - 1)));
}
}  // namespace crubit

// Evaluates an expression that produces an `absl::Status` or `llvm::Error`. If
// it is not ok, returns it from the current function. The enclosing function
// may return any of `absl::Status`, `absl::StatusOr<T>`, `llvm::Error`, or
// `llvm::Expected<T>`; the error is converted as needed.
//
// For example:
//   absl::Status MultiStepFunction() {
//     CRUBIT_RETURN_IF_ERROR(Function(args...));
//     CRUBIT_RETURN_IF_ERROR(foo.Method(args...));
//     return absl::OkStatus();
//   }
#define CRUBIT_RETURN_IF_ERROR(expr) \
  CRUBIT_RETURN_IF_ERROR_IMPL_(      \
      CRUBIT_STATUS_MACROS_IMPL_CONCAT_(_status_macro_val, __COUNTER__), expr)

// =================================================================
// == Implementation details, do not rely on anything below here. ==
// =================================================================
#define CRUBIT_RETURN_IF_ERROR_IMPL_(val, expr)                        \
  do {                                                                 \
    auto val = (expr);                                                 \
    if (LLVM_UNLIKELY(!::crubit::status_macros_internal::IsOk(val))) { \
      return ::crubit::status_macros_internal::TakeError(val);         \
    }                                                                  \
  } while (false)

// Executes an expression `rexpr` that returns an `absl::StatusOr<T>` or an
// `llvm::Expected<T>`. On OK, moves its value into the variable defined by
// `lhs`, otherwise returns from the current function (see
// `CRUBIT_RETURN_IF_ERROR` for the supported return types).
//
// Interface:
//
//   CRUBIT_ASSIGN_OR_RETURN(lhs, rexpr)
//
// WARNING: if lhs is parenthesized, the parentheses are removed. See examples
// for more details.
//
// WARNING: expands into multiple statements; it cannot be used in a single
// statement (e.g. as the body of an if statement without {})!
//
// Example: Declaring and initializing a new variable (ValueType can be anything
//          that can be initialized with assignment, including references):
//   CRUBIT_ASSIGN_OR_RETURN(ValueType value, MaybeGetValue(arg));
//
// Example: Assigning to an existing variable:
//   ValueType value;
//   CRUBIT_ASSIGN_OR_RETURN(value, MaybeGetValue(arg));
//
// Example: Assigning to an expression with side effects:
//   MyProto data;
//   CRUBIT_ASSIGN_OR_RETURN(*data.mutable_str(), MaybeGetValue(arg));
//   // No field "str" is added on error.
//
// Example: Initializing a `std::unique_ptr`.
//   CRUBIT_ASSIGN_OR_RETURN(std::unique_ptr<T> ptr, MaybeGetPtr(arg));
//
// Example: Initializing a map. Because of C++ preprocessor limitations,
// the type used in CRUBIT_ASSIGN_OR_RETURN cannot contain commas, so wrap the
// lhs in parentheses:
//   CRUBIT_ASSIGN_OR_RETURN((absl::flat_hash_map<Foo, Bar> my_map), GetMap());
// Or use `auto` if the type is obvious enough:
//   CRUBIT_ASSIGN_OR_RETURN(auto my_map, GetMap());
//
// Example: Assigning to structured bindings (<internal link>/169). The same situation
// with comma as in map, so wrap the statement in parentheses.
//   CRUBIT_ASSIGN_OR_RETURN((auto [first, second]), GetPair());
#define CRUBIT_ASSIGN_OR_RETURN(lhs, rexpr)                                   \
  CRUBIT_ASSIGN_OR_RETURN_IMPL_(                                              \
      CRUBIT_STATUS_MACROS_IMPL_CONCAT_(_status_macro_val, __COUNTER__), lhs, \
      rexpr)

#define CRUBIT_ASSIGN_OR_RETURN_IMPL_(val, lhs, rexpr)                       \
  auto val = (rexpr);                                                        \
  if (LLVM_UNLIKELY(!::crubit::status_macros_internal::IsOk(val))) {         \
    return ::crubit::status_macros_internal::TakeError(val);                 \
  }                                                                          \
  {                                                                          \
    static_assert(#lhs[0] != '(' || #lhs[sizeof(#lhs) - 2] != ')' ||         \
                      !::crubit::HasPotentialConditionalOperator(            \
                          #lhs, sizeof(#lhs) - 2),                           \
                  "Identified potential conditional operator, consider not " \
                  "using CRUBIT_ASSIGN_OR_RETURN");                          \
  }                                                                          \
  CRUBIT_STATUS_MACROS_IMPL_UNPARENTHESIZE_IF_PARENTHESIZED(lhs) =           \
      ::crubit::status_macros_internal::TakeValue(val)

// Internal helpers for macro expansion.
#define CRUBIT_STATUS_MACROS_IMPL_EAT(...)
#define CRUBIT_STATUS_MACROS_IMPL_REM(...) __VA_ARGS__
#define CRUBIT_STATUS_MACROS_IMPL_EMPTY()

// Internal helpers for emptyness arguments check.
#define CRUBIT_STATUS_MACROS_IMPL_IS_EMPTY_INNER(...) \
  CRUBIT_STATUS_MACROS_IMPL_IS_EMPTY_INNER_HELPER((__VA_ARGS__, 0, 1))
// MSVC expands variadic macros incorrectly, so we need this extra indirection
// to work around that (b/110959038).
#define CRUBIT_STATUS_MACROS_IMPL_IS_EMPTY_INNER_HELPER(args) \
  CRUBIT_STATUS_MACROS_IMPL_IS_EMPTY_INNER_I args
#define CRUBIT_STATUS_MACROS_IMPL_IS_EMPTY_INNER_I(e0, e1, is_empty, ...) \
  is_empty

#define CRUBIT_STATUS_MACROS_IMPL_IS_EMPTY(...) \
  CRUBIT_STATUS_MACROS_IMPL_IS_EMPTY_I(__VA_ARGS__)
#define CRUBIT_STATUS_MACROS_IMPL_IS_EMPTY_I(...) \
  CRUBIT_STATUS_MACROS_IMPL_IS_EMPTY_INNER(_, ##__VA_ARGS__)

// Internal helpers for if statement.
#define CRUBIT_STATUS_MACROS_IMPL_IF_1(_Then, _Else) _Then
#define CRUBIT_STATUS_MACROS_IMPL_IF_0(_Then, _Else) _Else
#define CRUBIT_STATUS_MACROS_IMPL_IF(_Cond, _Then, _Else)                 \
  CRUBIT_STATUS_MACROS_IMPL_CONCAT_(CRUBIT_STATUS_MACROS_IMPL_IF_, _Cond) \
  (_Then, _Else)

// Expands to 1 if the input is parenthesized. Otherwise expands to 0.
#define CRUBIT_STATUS_MACROS_IMPL_IS_PARENTHESIZED(...) \
  CRUBIT_STATUS_MACROS_IMPL_IS_EMPTY(CRUBIT_STATUS_MACROS_IMPL_EAT __VA_ARGS__)

// If the input is parenthesized, removes the parentheses. Otherwise expands to
// the input unchanged.
#define CRUBIT_STATUS_MACROS_IMPL_UNPARENTHESIZE_IF_PARENTHESIZED(...)  \
  CRUBIT_STATUS_MACROS_IMPL_IF(                                         \
      CRUBIT_STATUS_MACROS_IMPL_IS_PARENTHESIZED(__VA_ARGS__),          \
      CRUBIT_STATUS_MACROS_IMPL_REM, CRUBIT_STATUS_MACROS_IMPL_EMPTY()) \
  __VA_ARGS__

// Internal helper for concatenating macro values.
#define CRUBIT_STATUS_MACROS_IMPL_CONCAT_INNER_(x, y) x##y
#define CRUBIT_STATUS_MACROS_IMPL_CONCAT_(x, y) \
  CRUBIT_STATUS_MACROS_IMPL_CONCAT_INNER_(x, y)

#endif  // CRUBIT_COMMON_STATUS_MACROS_H_
