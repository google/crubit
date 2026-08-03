// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_SPAN_REPRODUCER_SPAN_USAGE_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_SPAN_REPRODUCER_SPAN_USAGE_H_

#include <cstdint>

#include "absl/types/span.h"

inline uint64_t take_span(absl::Span<const int> s) { return s.size(); }

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_SPAN_REPRODUCER_SPAN_USAGE_H_
