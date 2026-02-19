// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/consume_absl/uses_anyinvocable.h"

#include <utility>

#include "absl/functional/any_invocable.h"

namespace absl_functional_internal {

void CallVoidVoid(absl::AnyInvocable<void() &&> f) { std::move(f)(); }

absl::AnyInvocable<int(int) const> ReturnIntVoid() {
  return [](int x) -> int { return x + 1; };
}

}  // namespace absl_functional_internal
