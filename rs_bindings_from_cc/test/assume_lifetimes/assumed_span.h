// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ASSUME_LIFETIMES_ASSUMED_C9_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ASSUME_LIFETIMES_ASSUMED_C9_H_

#include <string_view>

#include "absl/types/span.h"

absl::Span<int> IdentitySpan(absl::Span<int> x);

absl::Span<std::string_view> IdentitySpanWithRef(
    absl::Span<std::string_view> x);

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ASSUME_LIFETIMES_ASSUMED_C9_H_
