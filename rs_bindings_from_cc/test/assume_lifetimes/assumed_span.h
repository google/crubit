// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ASSUME_LIFETIMES_ASSUMED_C9_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ASSUME_LIFETIMES_ASSUMED_C9_H_

#include <cstdint>
#include <string_view>

#include "absl/types/span.h"

absl::Span<int> IdentitySpan(absl::Span<int> x);

absl::Span<std::string_view> IdentitySpanWithRef(
    absl::Span<std::string_view> x);

class TwoArgSpanCtor {
 public:
  constexpr TwoArgSpanCtor(uint64_t mask, absl::Span<const uint32_t> numbers)
      : mask_(mask), numbers_(numbers) {}

 private:
  uint64_t mask_;
  absl::Span<const uint32_t> numbers_;
};

class OneArgSpanCtor {
 public:
  explicit constexpr OneArgSpanCtor(absl::Span<const uint32_t> table)
      : table_(table) {}

 private:
  absl::Span<const uint32_t> table_;
};

class UnnamedSpanParamCtor {
 public:
  explicit constexpr UnnamedSpanParamCtor(absl::Span<const uint32_t>) {}
};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ASSUME_LIFETIMES_ASSUMED_C9_H_
