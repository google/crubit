// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_ANY_INVOCABLE_REPRODUCER_ANY_INVOCABLE_USAGE_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_ANY_INVOCABLE_REPRODUCER_ANY_INVOCABLE_USAGE_H_

#include "absl/functional/any_invocable.h"

inline void call_it(absl::AnyInvocable<void()> f) { f(); }

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_ANY_INVOCABLE_REPRODUCER_ANY_INVOCABLE_USAGE_H_
