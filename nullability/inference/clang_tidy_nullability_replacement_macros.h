// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// This file contains macro definitions that enable inference tooling to detect
// calls to these macros and collect appropriate evidence based on knowledge of
// the macros' behavior.

// Each macro to be replaced should be defined as a call to our copy, with all
// arguments passed to the appropriate argument-capture function based on the
// behavior of the macro and its number of parameters, followed on a new line by
// an empty definition of the copy, of the format
// `#define __clang_tidy_nullability_<macro name>(<params>)` with the same
// parameters as the original macro.

// The inference tools will then give __clang_tidy_nullability_<macro name> the
// definition of the original macro, ensuring code compiles properly.

// This file's name is intentionally additionally namespaced because it will be
// included in affected translation units without the containing directories.

// IWYU pragma: private, include macros from their original definition sites
#ifndef CRUBIT_NULLABILITY_INFERENCE_CLANG_TIDY_NULLABILITY_REPLACEMENT_MACROS_H_
#define CRUBIT_NULLABILITY_INFERENCE_CLANG_TIDY_NULLABILITY_REPLACEMENT_MACROS_H_

#ifdef __cplusplus

// Forwarding function to allow detection of assert-like macro arguments.
// Used for single-argument macros that abort if the argument is false.
template <typename T>
constexpr T&& clang_tidy_nullability_internal_abortIfFalse(T&& Arg) {
  return static_cast<T&&>(Arg);
}

// Forwarding function to allow detection of both arguments of assert-like
// not-equal comparisons.
template <typename First, typename Second>
constexpr First&& clang_tidy_nullability_internal_abortIfEqual(First&& FirstArg,
                                                               Second&&) {
  return static_cast<First&&>(FirstArg);
}

#define CHECK(x)                  \
  __clang_tidy_nullability_CHECK( \
      ::clang_tidy_nullability_internal_abortIfFalse(x))
#define __clang_tidy_nullability_CHECK(x)

#define QCHECK(x)                  \
  __clang_tidy_nullability_QCHECK( \
      ::clang_tidy_nullability_internal_abortIfFalse(x))
#define __clang_tidy_nullability_QCHECK(x)

#define DCHECK(x)                  \
  __clang_tidy_nullability_DCHECK( \
      ::clang_tidy_nullability_internal_abortIfFalse(x))
#define __clang_tidy_nullability_DCHECK(x)

#define PCHECK(x)                  \
  __clang_tidy_nullability_PCHECK( \
      ::clang_tidy_nullability_internal_abortIfFalse(x))
#define __clang_tidy_nullability_PCHECK(x)

#define ABSL_DIE_IF_NULL(x)                  \
  __clang_tidy_nullability_ABSL_DIE_IF_NULL( \
      ::clang_tidy_nullability_internal_abortIfFalse(x))
#define __clang_tidy_nullability_ABSL_DIE_IF_NULL(x)

#define CHECK_NE(x, y)               \
  __clang_tidy_nullability_CHECK_NE( \
      ::clang_tidy_nullability_internal_abortIfEqual(x, y), y)
#define __clang_tidy_nullability_CHECK_NE(x, y)

#define QCHECK_NE(x, y)               \
  __clang_tidy_nullability_QCHECK_NE( \
      ::clang_tidy_nullability_internal_abortIfEqual(x, y), y)
#define __clang_tidy_nullability_QCHECK_NE(x, y)

#define DCHECK_NE(x, y)               \
  __clang_tidy_nullability_DCHECK_NE( \
      ::clang_tidy_nullability_internal_abortIfEqual(x, y), y)
#define __clang_tidy_nullability_DCHECK_NE(x, y)

// Could infer Nullable from CHECK_EQ comparisons with nullptr, but not as
// likely to provide additional coverage as CHECK_NE leading to Nonnull.

#endif  // __cplusplus
#endif  // CRUBIT_NULLABILITY_INFERENCE_CLANG_TIDY_NULLABILITY_REPLACEMENT_MACROS_H_
