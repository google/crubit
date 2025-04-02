// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// example_crate_golden
// Features: supported

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_EXAMPLES_RUST_UNION_EXAMPLE_CRATE_GOLDEN
#define THIRD_PARTY_CRUBIT_EXAMPLES_RUST_UNION_EXAMPLE_CRATE_GOLDEN

#include "support/internal/attribute_macros.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>

namespace example_crate {

// Generated from: examples/rust/union/example.rs;l=6
union CRUBIT_INTERNAL_RUST_TYPE(
    ":: example_crate_golden :: ReprCUnion") alignas(8) [[clang::trivial_abi]]
ReprCUnion final {
 public:
  // Default::default
  ReprCUnion();

  // No custom `Drop` impl and no custom \"drop glue\" required
  ~ReprCUnion() = default;
  ReprCUnion(ReprCUnion&&) = default;
  ReprCUnion& operator=(ReprCUnion&&) = default;

  // `ReprCUnion` doesn't implement the `Clone` trait
  ReprCUnion(const ReprCUnion&) = delete;
  ReprCUnion& operator=(const ReprCUnion&) = delete;
  ReprCUnion(::crubit::UnsafeRelocateTag, ReprCUnion&& value) {
    memcpy(this, &value, sizeof(value));
  }

 public:
  // Generated from:
  // examples/rust/union/example.rs;l=7
  std::int32_t a;

 public:
  // Generated from:
  // examples/rust/union/example.rs;l=8
  double b;

 private:
  static void __crubit_field_offset_assertions();
};

static_assert(
    sizeof(ReprCUnion) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(ReprCUnion) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(::example_crate::ReprCUnion* __ret_ptr);
}
inline ReprCUnion::ReprCUnion() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(std::is_trivially_destructible_v<ReprCUnion>);
static_assert(std::is_trivially_move_constructible_v<ReprCUnion>);
static_assert(std::is_trivially_move_assignable_v<ReprCUnion>);
inline void ReprCUnion::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(ReprCUnion, a));
  static_assert(0 == offsetof(ReprCUnion, b));
}
}  // namespace example_crate
#endif  // THIRD_PARTY_CRUBIT_EXAMPLES_RUST_UNION_EXAMPLE_CRATE_GOLDEN
