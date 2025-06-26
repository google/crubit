// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "support/status_bridge.h"

#include "support/bridge.h"

#include <cstddef>
#include <cstdint>
#include <string>
#include <utility>

#include "absl/status/status.h"
#include "absl/strings/string_view.h"

namespace crubit {

constexpr uintptr_t kRustOkStatusRep = 0;

static_assert(
    sizeof(absl::Status) == sizeof(uintptr_t) &&
        alignof(absl::Status) == alignof(uintptr_t),
    "Crubit invariant broken, please reach out to us at <internal link>");

void StatusAbi::Encode(absl::Status value, Encoder& encoder) {
  if (value.ok()) {
    // No reference counting, okay to just drop.
    encoder.EncodeTransmute<uintptr_t>(kRustOkStatusRep);
    return;
  }

  // Ownership of the Status is transferred into the buffer.
  alignas(absl::Status) char rep[sizeof(absl::Status)];
  new (rep) absl::Status(std::move(value));
  encoder.EncodeTransmute<uintptr_t>(*reinterpret_cast<uintptr_t*>(rep));
}

absl::Status StatusAbi::Decode(Decoder& decoder) {
  uintptr_t rep = decoder.DecodeTransmute<uintptr_t>();
  if (rep == kRustOkStatusRep) {
    return absl::OkStatus();
  }

  return absl::Status(reinterpret_cast<absl::Status&&>(rep));
}

// These functions do not have prototypes because they are extern "C" functions
// that are linked to Rust code, and never called from C++ code.
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wmissing-prototypes"

// Increments the reference count of the `Status` with the provided `rep`.
//
// The caller must ensure that:
//   * `rep` is a valid `rep_` from a `Status`,
//   * if `rep` is the allocated variant, the underlying `StatusRep*` has not
//   been deleted.
//
// The caller may assume that the reference count has been incremented.
extern "C" void absl_status_internal_ref(uintptr_t rep) {
  alignas(absl::Status) char erase[sizeof(absl::Status)];
  new (erase) absl::Status(reinterpret_cast<const absl::Status&>(rep));
}

// Takes semantic overship over the `rep` by creating a `Status` from it, and
// then immediately destroys it to decrement the reference count.
//
// The caller must ensure that:
//   * `rep` is a valid `rep_` from a `Status`,
//   * if `rep` is the allocated variant, the underlying `StatusRep*` has not
//   been deleted prior to calling this function.
extern "C" void absl_status_internal_unref(uintptr_t rep) {
  (void)absl::Status(reinterpret_cast<absl::Status&&>(rep));
}

// A C-compatible representation of a `string_view` used to pass string views by
// value across the C ABI.
struct c_string_view {
  size_t size;
  const char* data;
};

// Creates a rep for a new `Status` with the provided code and message.
//
// The caller must ensure that `message` constitutes valid `absl::string_view`.
//
// The caller may assume that the returned rep is a valid `rep_` from a C++
// `Status` meaning it is never 0, and that if it is the allocated variant, the
// reference count is set to 1 which accounts for the ownership that the caller
// is expected to take.
extern "C" uintptr_t absl_status_internal_new(int code, c_string_view message) {
  alignas(absl::Status) char rep[sizeof(absl::Status)];
  new (rep) absl::Status(static_cast<absl::StatusCode>(code),
                         absl::string_view(message.data, message.size));
  return *reinterpret_cast<uintptr_t*>(rep);
}

// Returns the raw code of the `Status`.
//
// The caller must ensure that:
//   * `rep` is a valid `rep_` from a `Status`,
//   * if `rep` is the allocated variant, the underlying `StatusRep*` has not
//   been deleted.
//
// The caller may not assume that the returned value is a valid `StatusCode`
// value.
extern "C" int absl_status_internal_raw_code(uintptr_t rep) {
  return reinterpret_cast<const absl::Status&>(rep).raw_code();
}

// Returns the message of the `Status`.
//
// The caller must ensure that:
//   * `rep` is a valid `rep_` from a `Status`,
//   * if `rep` is the allocated variant, the underlying `StatusRep*` has not
//   been deleted,
//
// The caller may assume that the returned `string_view` is valid until `rep`
// is converted back to a `Status` and the `Status` is destroyed.
extern "C" c_string_view absl_status_internal_message(uintptr_t rep) {
  auto message = reinterpret_cast<const absl::Status&>(rep).message();
  return {message.size(), message.data()};
}

// Returns true if two Status values are equal, false otherwise.
//
// The caller must ensure that for `rep` in (lhs, rhs):
//   * `rep` is a valid `rep_` from a `Status`,
//   * if `rep` is the allocated variant, the underlying `StatusRep*` has not
//   been deleted,
extern "C" bool absl_status_internal_operator_equals(uintptr_t lhs,
                                                     uintptr_t rhs) {
  return reinterpret_cast<const absl::Status&>(lhs) ==
         reinterpret_cast<const absl::Status&>(rhs);
}

// Writes the stringified representation to a Rust `fmt::Formatter`.
//
// The caller must ensure that:
//   * `rep` is a valid `rep_` from a `Status`,
//   * if `rep` is the allocated variant, the underlying `StatusRep*` has not
//   been deleted,
//   * `formatter` can be safely casted to a `&mut fmt::Formatter`,
//   * `cb` is a function that takes the underlying type of `formatter` and
//     returns true if the string was successfully written.
//
// The caller may assume that if `cb` is called, then the void* is the
// `formatter` passed into this function, and that the c_string_view is valid
// for the duration of `cb`.
extern "C" bool absl_status_internal_to_string(uintptr_t rep, void* formatter,
                                               bool (*cb)(void*,
                                                          c_string_view)) {
  std::string s = reinterpret_cast<absl::Status&>(rep).ToString();
  if (s.empty()) {
    return true;
  }

  // Need to use a function pointer so that :status can compile without linking
  // against :additional_status_src.
  return cb(formatter, {s.size(), s.data()});
}

#pragma clang diagnostic pop

}  // namespace crubit
