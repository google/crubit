// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/assume_lifetimes/assumed_optional_ref.h"

#include <optional>

int ValueOrDefault(absl::optional_ref<const int> x, int default_val) {
  return x.has_value() ? *x : default_val;
}

void IncrementIfPresent(absl::optional_ref<int> x) {
  if (x.has_value()) {
    ++(*x);
  }
}

absl::optional_ref<const int> IdentityConstOptionalRef(
    absl::optional_ref<const int> x) {
  return x;
}

absl::optional_ref<int> IdentityMutOptionalRef(absl::optional_ref<int> x) {
  return x;
}

absl::optional_ref<const int> FilterPositive(absl::optional_ref<const int> x) {
  if (x.has_value() && *x > 0) {
    return x;
  }
  return std::nullopt;
}
