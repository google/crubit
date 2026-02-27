// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MANUAL_BRIDGE_VOCABULARY_TYPES_SPAN_LIB_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MANUAL_BRIDGE_VOCABULARY_TYPES_SPAN_LIB_H_

#include <cstddef>

#include "absl/types/span.h"
#include "support/annotations.h"

CRUBIT_MUST_BIND
absl::Span<int> TruncateSpanMut(absl::Span<int> span, size_t len);

CRUBIT_MUST_BIND
absl::Span<const int> TruncateSpan(absl::Span<const int> span, size_t len);

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_MANUAL_BRIDGE_VOCABULARY_TYPES_SPAN_LIB_H_
