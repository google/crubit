// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_EXAMPLES_TYPES_CALLBACKS_CPP_API_H_
#define THIRD_PARTY_CRUBIT_EXAMPLES_TYPES_CALLBACKS_CPP_API_H_

#include <cstdint>

#include "absl/functional/any_invocable.h"

// Takes a const callback and invokes it with an integer.
int32_t ApplyMultiplier(absl::AnyInvocable<int32_t(int32_t) const> callback,
                        int32_t value);

// Takes a single-use callback and invokes it.
void InvokeOnce(absl::AnyInvocable<void() &&> callback);

// Returns a callback that adds 1 to its argument.
absl::AnyInvocable<int32_t(int32_t) const> MakeAdder();

// Type alias to an AnyInvocable receives bindings in Rust.
using CallbackAlias = absl::AnyInvocable<int32_t(int32_t) const>;
int32_t ApplyAlias(CallbackAlias callback, int32_t value);

#endif  // THIRD_PARTY_CRUBIT_EXAMPLES_TYPES_CALLBACKS_CPP_API_H_
