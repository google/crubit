// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CONSUME_ABSL_ABSL_FUNCTIONAL_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CONSUME_ABSL_ABSL_FUNCTIONAL_H_

#include "absl/functional/any_invocable.h"

namespace absl_functional_internal {

// Calls the invocable and returns void.
void CallVoidVoid(absl::AnyInvocable<void() &&> f);

// Returns an invocable that returns 42.
absl::AnyInvocable<int(int) const> ReturnIntVoid();

}  // namespace absl_functional_internal

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_CONSUME_ABSL_ABSL_FUNCTIONAL_H_
