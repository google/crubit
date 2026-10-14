// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "examples/types/callbacks/cpp_api.h"

#include <cstdint>
#include <utility>

#include "absl/functional/any_invocable.h"

int32_t ApplyMultiplier(absl::AnyInvocable<int32_t(int32_t) const> callback,
                        int32_t value) {
  return callback(value);
}

void InvokeOnce(absl::AnyInvocable<void() &&> callback) {
  std::move(callback)();
}

absl::AnyInvocable<int32_t(int32_t) const> MakeAdder() {
  return [](int32_t x) { return x + 1; };
}

int32_t ApplyAlias(CallbackAlias callback, int32_t value) {
  return callback(value);
}
