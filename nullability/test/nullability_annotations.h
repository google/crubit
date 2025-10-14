// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

template <typename T>
using Nullable [[clang::annotate("Nullable")]] = T _Nullable;

template <typename T>
using Nonnull [[clang::annotate("Nonnull")]] = T _Nonnull;

template <typename T>
using NullabilityUnknown [[clang::annotate("Nullability_Unspecified")]] =
    T _Null_unspecified;

#define absl_nullable _Nullable
#define absl_nonnull _Nonnull
#define absl_nullability_unknown _Null_unspecified
#define absl_nullability_conflict _Null_unspecified
