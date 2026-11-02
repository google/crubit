// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Helper macros and methods to return and propagate errors with `llvm::Error`.

#ifndef CRUBIT_COMMON_STATUS_MACROS_H_
#define CRUBIT_COMMON_STATUS_MACROS_H_

#include <utility>

#include "llvm/Support/Compiler.h"
#include "llvm/Support/Error.h"

namespace crubit {
namespace status_macros_internal {

inline bool IsOk(llvm::Error& error) { return !error; }
inline llvm::Error TakeError(llvm::Error& error) { return std::move(error); }

template <typename T>
inline bool IsOk(llvm::Expected<T>& expected) {
  return (bool)expected;
}
template <typename T>
inline llvm::Error TakeError(llvm::Expected<T>& expected) {
  return expected.takeError();
}
template <typename T>
inline auto TakeValue(llvm::Expected<T>& expected)
    -> decltype(std::move(*expected)) {
  return std::move(*expected);
}

}  // namespace status_macros_internal

}  // namespace crubit

// Evaluates an expression that produces an `llvm::Error`. If
// it is not ok, returns it from the current function. The enclosing function
// may return `llvm::Error` or `llvm::Expected<T>`.
//
// For example:
//   llvm::Error MultiStepFunction() {
//     CRUBIT_RETURN_IF_ERROR(Function(args...));
//     CRUBIT_RETURN_IF_ERROR(foo.Method(args...));
//     return llvm::Error::success();
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

// Executes an expression `rexpr` that returns an `llvm::Expected<T>`. On OK,
// moves its value into the variable defined by `lhs`, otherwise returns from
// the current function (see `CRUBIT_RETURN_IF_ERROR` for the supported return
// types).
//
// Interface:
//
//   CRUBIT_ASSIGN_OR_RETURN(lhs, rexpr)
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
// Example: Initializing a map.
//   CRUBIT_ASSIGN_OR_RETURN(auto my_map, GetMap());
//
// Example: Assigning to structured bindings (<internal link>/169).
//   CRUBIT_ASSIGN_OR_RETURN(auto [first, second], GetPair());
#define CRUBIT_ASSIGN_OR_RETURN(lhs, rexpr)                                   \
  CRUBIT_ASSIGN_OR_RETURN_IMPL_(                                              \
      CRUBIT_STATUS_MACROS_IMPL_CONCAT_(_status_macro_val, __COUNTER__), lhs, \
      rexpr)

#define CRUBIT_ASSIGN_OR_RETURN_IMPL_(val, lhs, rexpr)               \
  auto val = (rexpr);                                                \
  if (LLVM_UNLIKELY(!::crubit::status_macros_internal::IsOk(val))) { \
    return ::crubit::status_macros_internal::TakeError(val);         \
  }                                                                  \
  lhs = ::crubit::status_macros_internal::TakeValue(val)

// Internal helper for concatenating macro values.
#define CRUBIT_STATUS_MACROS_IMPL_CONCAT_INNER_(x, y) x##y
#define CRUBIT_STATUS_MACROS_IMPL_CONCAT_(x, y) \
  CRUBIT_STATUS_MACROS_IMPL_CONCAT_INNER_(x, y)

#endif  // CRUBIT_COMMON_STATUS_MACROS_H_
