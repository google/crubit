// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// IWYU pragma: private, include "support/annotations.h"

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_ANNOTATE_INTERNAL_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_ANNOTATE_INTERNAL_H_

// Keep this header small: it is included by widely used C++ headers (via
// `support/bitwise_relocatability_annotations.h`).

// Style waiver granted in crubit.rs-style-waiver-attribute-annotate
#if defined(__clang__) && __has_cpp_attribute(clang::annotate) && \
    __has_cpp_attribute(clang::annotate_type)
#define CRUBIT_INTERNAL_ANNOTATE(...) [[clang::annotate(__VA_ARGS__)]]
#define CRUBIT_INTERNAL_ANNOTATE_TYPE(...) [[clang::annotate_type(__VA_ARGS__)]]
#else
#define CRUBIT_INTERNAL_ANNOTATE(...)
#define CRUBIT_INTERNAL_ANNOTATE_TYPE(...)
#endif

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_ANNOTATE_INTERNAL_H_
