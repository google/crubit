// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/manual_bridge_vocabulary_types/span_lib.h"

#include <cstddef>

#include "absl/types/span.h"

absl::Span<int> TruncateSpanMut(absl::Span<int> span, size_t len) {
  return span.subspan(0, len);
}

absl::Span<const int> TruncateSpan(absl::Span<const int> span, size_t len) {
  return span.subspan(0, len);
}

NonTrivial MakeNonTrivial(absl::Span<const int> span) {
  return NonTrivial{span.size()};
}

absl::Span<NonTrivial> TruncateSpanNonTrivialMut(absl::Span<NonTrivial> span,
                                                 size_t len) {
  return span.subspan(0, len);
}

absl::Span<const NonTrivial> TruncateSpanNonTrivial(
    absl::Span<const NonTrivial> span, size_t len) {
  return span.subspan(0, len);
}
