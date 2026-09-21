// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BRIDGING_NESTED_LAYOUT_COMPAT_STRING_LIB_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BRIDGING_NESTED_LAYOUT_COMPAT_STRING_LIB_H_

#include <optional>
#include <string>
#include <utility>

#include "absl/status/status.h"
#include "absl/status/statusor.h"

// =====================================================================
// Bound: containers which hold their payload layout-compatibly.
// =====================================================================

// A bare `std::string` is not bridged by value under `layout_compat_string`.
inline std::string MakeLayoutCompatString() { return "hello"; }

// `absl::StatusOr<T>` is layout-compatible rather than bridged under
// `CRUBIT_NEW_STATUS`, so it can hold a layout-compatible `std::string`.
inline absl::StatusOr<std::string> MakeLayoutCompatStringOrError(bool is_ok) {
  if (is_ok) return std::string("hello");
  return absl::InvalidArgumentError("requested an error");
}

// A user-defined Rust-movable type is layout-compatible, and can therefore be
// memcpy'd through a bridge buffer inside a `std::optional`.
struct RustMovable final {
  int x;
  bool y;
};

inline std::optional<RustMovable> MaybeMakeRustMovable(bool engaged) {
  if (engaged) return RustMovable{.x = 7, .y = true};
  return std::nullopt;
}

// =====================================================================
// Not bound: bridged containers holding a payload which cannot round-trip
// through the bridge buffer. Each of these gets an error comment instead of
// bindings; the point of this test is that the rest of the target still binds.
// =====================================================================

// A user-defined type which is not Rust-movable cannot be memcpy'd through a
// bridge buffer. This is pre-existing behavior, covered here for contrast with
// the `std::string` cases below.
struct NotRustMovable final {
  NotRustMovable() = default;
  NotRustMovable(const NotRustMovable&) {}
  ~NotRustMovable() {}

  int x;
};

inline std::optional<NotRustMovable> MaybeMakeNotRustMovable(bool engaged) {
  if (engaged) return NotRustMovable{};
  return std::nullopt;
}

// A layout-compatible `std::string` is likewise not Rust-movable, so it cannot
// be bridged by value either.
inline std::optional<std::string> MaybeMakeLayoutCompatString(bool engaged) {
  if (engaged) return std::string("hello");
  return std::nullopt;
}

inline std::pair<std::string, int> MakeLayoutCompatStringPair() {
  return {std::string("hello"), 7};
}

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BRIDGING_NESTED_LAYOUT_COMPAT_STRING_LIB_H_
