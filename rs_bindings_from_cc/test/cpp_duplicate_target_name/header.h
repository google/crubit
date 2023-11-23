// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CPP_DUPLICATE_TARGET_NAME_HEADER_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CPP_DUPLICATE_TARGET_NAME_HEADER_H_

#include "rs_bindings_from_cc/test/cpp_duplicate_target_name/subdir1/subdir1_A.h"

// For repro, we need `SubdirA` so that Crubit generates `A::Subdir1A`, which
// induces a build failure for ambiguous crate `A`.
void inline Func(Subdir1A) {}

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CPP_DUPLICATE_TARGET_NAME_HEADER_H_
