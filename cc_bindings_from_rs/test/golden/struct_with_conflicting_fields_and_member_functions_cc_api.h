// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// struct_with_conflicting_fields_and_member_functions_rust_golden
// Features: assume_lifetimes, custom_ffi_types, experimental, non_unpin_ctor,
// std_unique_ptr, std_vector, supported, wrapper

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_GOLDEN_STRUCT_WITH_CONFLICTING_FIELDS_AND_MEMBER_FUNCTIONS_RUST_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_GOLDEN_STRUCT_WITH_CONFLICTING_FIELDS_AND_MEMBER_FUNCTIONS_RUST_GOLDEN

#include "support/annotations_internal.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>

namespace struct_with_conflicting_fields_and_member_functions_rust {

// Generated from:
// cc_bindings_from_rs/test/golden/struct_with_conflicting_fields_and_member_functions.rs;l=6
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: struct_with_conflicting_fields_and_member_functions_rust_golden :: "
    "X") alignas(4) [[clang::trivial_abi]] X final {
 public:
  // `struct_with_conflicting_fields_and_member_functions_rust_golden::X`
  // doesn't implement the `Default` trait
  X() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~X() = default;
  X(X&&) = default;
  X& operator=(X&&) = default;

  // `struct_with_conflicting_fields_and_member_functions_rust_golden::X`
  // doesn't implement the `Clone` trait
  X(const X&) = delete;
  X& operator=(const X&) = delete;
  X(::crubit::UnsafeRelocateTag, X&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/golden/struct_with_conflicting_fields_and_member_functions.rs;l=13
  std::int32_t a() const;

  // Generated from:
  // cc_bindings_from_rs/test/golden/struct_with_conflicting_fields_and_member_functions.rs;l=16
  std::int32_t b() const;

  union {
    // Generated from:
    // cc_bindings_from_rs/test/golden/struct_with_conflicting_fields_and_member_functions.rs;l=7
    std::int32_t a_;
  };

 private:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/golden/struct_with_conflicting_fields_and_member_functions.rs;l=8
    std::int32_t b_;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/golden/struct_with_conflicting_fields_and_member_functions.rs;l=9
    std::int32_t c;
  };

 private:
  static void __crubit_field_offset_assertions();
};

static_assert(
    sizeof(X) == 12,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(X) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<X>);
static_assert(std::is_trivially_move_constructible_v<X>);
static_assert(std::is_trivially_move_assignable_v<X>);
namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_a(
    ::struct_with_conflicting_fields_and_member_functions_rust::X const&);
}
inline std::int32_t X::a() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_a(self);
}

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_b(
    ::struct_with_conflicting_fields_and_member_functions_rust::X const&);
}
inline std::int32_t X::b() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_b(self);
}
inline void X::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(X, a_));
  static_assert(4 == offsetof(X, b_));
  static_assert(8 == offsetof(X, c));
}
}  // namespace struct_with_conflicting_fields_and_member_functions_rust
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_GOLDEN_STRUCT_WITH_CONFLICTING_FIELDS_AND_MEMBER_FUNCTIONS_RUST_GOLDEN
