// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

template <typename T>
using Nullable [[clang::annotate("Nullable")]] = T;

template <typename T>
using Nonnull [[clang::annotate("Nonnull")]] = T;

template <typename T>
using NullabilityUnknown [[clang::annotate("Nullability_Unspecified")]] = T;
