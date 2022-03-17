// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Helper macros and methods to return and propagate errors with `absl::Status`.

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_UTIL_STATUS_MACROS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_UTIL_STATUS_MACROS_H_

#include <utility>

#include "third_party/absl/base/optimization.h"
#include "third_party/absl/status/status.h"
#include "third_party/absl/status/statusor.h"
#include "third_party/absl/types/source_location.h"

// Evaluates an expression that produces a `absl::Status`. If the status is not
// ok, returns it from the current function.
//
// For example:
//   absl::Status MultiStepFunction() {
//     CRUBIT_RETURN_IF_ERROR(Function(args...));
//     CRUBIT_RETURN_IF_ERROR(foo.Method(args...));
//     return absl::OkStatus();
//   }
#define CRUBIT_RETURN_IF_ERROR(expr)                 \
  CRUBIT_STATUS_MACROS_IMPL_ELSE_BLOCKER_            \
  if (::absl::Status status = (expr); status.ok()) { \
  } else /* NOLINT */                                \
    return status

// Executes an expression `rexpr` that returns an `absl::StatusOr<T>`. On OK,
// moves its value into the variable defined by `lhs`, otherwise returns
// from the current function.
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
#define CRUBIT_ASSIGN_OR_RETURN(lhs, rexpr)                               \
  CRUBIT_STATUS_MACROS_IMPL_ASSIGN_OR_RETURN_(                            \
      CRUBIT_STATUS_MACROS_IMPL_CONCAT_(_status_or_value, __LINE__), lhs, \
      rexpr,                                                              \
      return absl::Status(std::move(CRUBIT_STATUS_MACROS_IMPL_CONCAT_(    \
                                        _status_or_value, __LINE__))      \
                              .status(),                                  \
                          ABSL_LOC))

// =================================================================
// == Implementation details, do not rely on anything below here. ==
// =================================================================

// Some builds do not support C++14 fully yet, using C++11 constexpr technique.
constexpr bool HasPotentialConditionalOperator(const char* lhs, int index) {
  return (index == -1 ? false
                      : (lhs[index] == '?' ? true
                                           : HasPotentialConditionalOperator(
                                                 lhs, index - 1)));
}

#define CRUBIT_STATUS_MACROS_IMPL_ASSIGN_OR_RETURN_(statusor, lhs, rexpr, \
                                                    error_expression)     \
  auto statusor = (rexpr);                                                \
  if (ABSL_PREDICT_FALSE(!statusor.ok())) {                               \
    error_expression;                                                     \
  }                                                                       \
  {                                                                       \
    static_assert(                                                        \
        #lhs[0] != '(' || #lhs[sizeof(#lhs) - 2] != ')' ||                \
            !HasPotentialConditionalOperator(#lhs, sizeof(#lhs) - 2),     \
        "Identified potential conditional operator, consider not "        \
        "using CRUBIT_ASSIGN_OR_RETURN");                                 \
  }                                                                       \
  CRUBIT_STATUS_MACROS_IMPL_UNPARENTHESIZE_IF_PARENTHESIZED(lhs) =        \
      std::move(statusor).ValueOrDie()

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

// The GNU compiler emits a warning for code like:
//
//   if (foo)
//     if (bar) { } else baz;
//
// because it thinks you might want the else to bind to the first if.  This
// leads to problems with code like:
//
//   if (do_expr) CRUBIT_RETURN_IF_ERROR(expr) << "Some message";
//
// The "switch (0) case 0:" idiom is used to suppress this.
#define CRUBIT_STATUS_MACROS_IMPL_ELSE_BLOCKER_ \
  switch (0)                                    \
  case 0:                                       \
  default:  // NOLINT

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_UTIL_STATUS_MACROS_H_
