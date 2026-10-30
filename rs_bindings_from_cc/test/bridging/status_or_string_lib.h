// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BRIDGING_STATUS_OR_STRING_LIB_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BRIDGING_STATUS_OR_STRING_LIB_H_

#include <optional>
#include <string>

#include "absl/status/status.h"
#include "absl/status/statusor.h"

// Note: This target does NOT enable `layout_compat_string`.
// `absl::StatusOr<std::string>`, `absl::StatusOr<std::optional<int>>`, etc.
// should still automatically use layout-compatible representations because
// `StatusOr` is a layout-compatible generic.
inline absl::StatusOr<std::string> MakeStringOrError(bool is_ok) {
  if (is_ok) return std::string("hello");
  return absl::InvalidArgumentError("requested an error");
}

inline absl::StatusOr<std::optional<int>> MakeOptionalOrError(bool is_ok) {
  if (is_ok) return std::optional<int>(42);
  return absl::InvalidArgumentError("requested an error");
}

inline absl::StatusOr<std::optional<std::string>> MakeOptionalStringOrError(
    bool is_ok) {
  if (is_ok) return std::optional<std::string>("world");
  return absl::InvalidArgumentError("requested an error");
}

struct NonTrivialToken {
  int value = 0;
  NonTrivialToken() = default;
  explicit NonTrivialToken(int v) : value(v) {}
  NonTrivialToken(NonTrivialToken&&) = default;
  ~NonTrivialToken() {}
};

struct TokenHelper {
  absl::StatusOr<std::string> GetStringWithToken(NonTrivialToken token) const {
    if (token.value >= 0) return std::string("token_ok");
    return absl::InvalidArgumentError("negative token");
  }
};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BRIDGING_STATUS_OR_STRING_LIB_H_
