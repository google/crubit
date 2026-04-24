// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#ifndef THIRD_PARTY_CRUBIT_SUPPORT_INTERNAL_CHECK_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_INTERNAL_CHECK_H_

#include "absl/log/absl_check.h"

// <internal link>/216 and b/501547066 explain why we need to `#include` and use
// `ABSL_CHECK` instead of `CHECK`.
#define CRUBIT_CHECK(condition) ABSL_CHECK(condition)

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_INTERNAL_CHECK_H_
