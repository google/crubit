// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// example_crate_golden
// Features: infer_operator_lifetimes, supported, unsafe_types

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_EXAMPLES_RUST_RUST_UNION_EXAMPLE_CRATE_GOLDEN
#define THIRD_PARTY_CRUBIT_EXAMPLES_RUST_RUST_UNION_EXAMPLE_CRATE_GOLDEN

#include "support/internal/attribute_macros.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>

namespace example_crate {

// Generated from:
// examples/rust/rust_union/example.rs;l=5
union CRUBIT_INTERNAL_RUST_TYPE(
    ":: example_crate_golden :: ReprRustUnion") alignas(8)
    [[clang::trivial_abi]] ReprRustUnion final {
 public:
  // Default::default
  ReprRustUnion();

  // No custom `Drop` impl and no custom "drop glue" required
  ~ReprRustUnion() = default;
  ReprRustUnion(ReprRustUnion&&) = default;
  ReprRustUnion& operator=(ReprRustUnion&&) = default;

  // `ReprRustUnion` doesn't implement the `Clone` trait
  ReprRustUnion(const ReprRustUnion&) = delete;
  ReprRustUnion& operator=(const ReprRustUnion&) = delete;
  ReprRustUnion(::crubit::UnsafeRelocateTag, ReprRustUnion&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // examples/rust/rust_union/example.rs;l=21
  void set_a(std::int32_t a) [[clang::annotate_type("lifetime", "__anon1")]];

  // Generated from:
  // examples/rust/rust_union/example.rs;l=25
  void set_b(double b) [[clang::annotate_type("lifetime", "__anon1")]];

 private:
  // Field type has been replaced with a blob of bytes: support for non-repr(C)
  // unions requires //features:experimental
  unsigned char __opaque_blob_of_bytes[8];

 private:
  static void __crubit_field_offset_assertions();
};

static_assert(
    sizeof(ReprRustUnion) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(ReprRustUnion) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(
    ::example_crate::ReprRustUnion* __ret_ptr);
}
inline ReprRustUnion::ReprRustUnion() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(std::is_trivially_destructible_v<ReprRustUnion>);
static_assert(std::is_trivially_move_constructible_v<ReprRustUnion>);
static_assert(std::is_trivially_move_assignable_v<ReprRustUnion>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_set_ua(
    ::example_crate::ReprRustUnion& [[clang::annotate_type("lifetime",
                                                           "__anon1")]],
    std::int32_t);
}
inline void ReprRustUnion::set_a(std::int32_t a)
    [[clang::annotate_type("lifetime", "__anon1")]] {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_set_ua(self, a);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_set_ub(
    ::example_crate::ReprRustUnion& [[clang::annotate_type("lifetime",
                                                           "__anon1")]],
    double);
}
inline void ReprRustUnion::set_b(double b)
    [[clang::annotate_type("lifetime", "__anon1")]] {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_set_ub(self, b);
}
inline void ReprRustUnion::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(ReprRustUnion, __opaque_blob_of_bytes));
}
}  // namespace example_crate
#endif  // THIRD_PARTY_CRUBIT_EXAMPLES_RUST_RUST_UNION_EXAMPLE_CRATE_GOLDEN
