// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// This header is included in all code passed to checkDiagnostics.

#ifndef CRUBIT_NULLABILITY_TEST_CHECK_DIAGNOSTICS_PREAMBLE_H_
#define CRUBIT_NULLABILITY_TEST_CHECK_DIAGNOSTICS_PREAMBLE_H_

#include "nullability_annotations.h"  // IWYU pragma: export

enum NullabilityKind {
  NK_nonnull,
  NK_nullable,
  NK_unspecified,
};

template <NullabilityKind... NK, typename T>
void __assert_nullability(const T &);

template <typename T>
T value();

#endif  // CRUBIT_NULLABILITY_TEST_CHECK_DIAGNOSTICS_PREAMBLE_H_
