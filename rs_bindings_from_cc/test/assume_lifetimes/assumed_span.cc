// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/assume_lifetimes/assumed_span.h"

#include <string_view>

#include "absl/types/span.h"

absl::Span<int> IdentitySpan(absl::Span<int> x) { return x; }

absl::Span<std::string_view> IdentitySpanWithRef(
    absl::Span<std::string_view> x) {
  return x;
}
