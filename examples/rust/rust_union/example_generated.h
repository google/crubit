// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// example_crate
// Features: <none>

// clang-format off
#pragma once

#include "support/internal/attribute_macros.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>

namespace example_crate {

// Generated from:
// examples/rust/rust_union/example.rs;l=5
union CRUBIT_INTERNAL_RUST_TYPE(":: example_crate :: ReprRustUnion") alignas(8)
    [[clang::trivial_abi]] ReprRustUnion final {
 public:
  // Default::default
  ReprRustUnion();

  // No custom `Drop` impl and no custom \"drop glue\" required
  ~ReprRustUnion() = default;
  ReprRustUnion(ReprRustUnion&&) = default;
  ReprRustUnion& operator=(ReprRustUnion&&) = default;

  // `ReprRustUnion` doesn't implement the `Clone` trait
  ReprRustUnion(const ReprRustUnion&) = delete;
  ReprRustUnion& operator=(const ReprRustUnion&) = delete;

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
extern "C" void
__crubit_thunk__uRNvYNtCsh2mUQSogkZl_u13example_ucrate13ReprRustUnionNtNtCs8sGNUgcxoFi_u4core7default7Default7defaultB4_u(
    ::example_crate::ReprRustUnion* __ret_ptr);
}
inline ReprRustUnion::ReprRustUnion() {
  __crubit_internal::
      __crubit_thunk__uRNvYNtCsh2mUQSogkZl_u13example_ucrate13ReprRustUnionNtNtCs8sGNUgcxoFi_u4core7default7Default7defaultB4_u(
          this);
}
static_assert(std::is_trivially_destructible_v<ReprRustUnion>);
static_assert(std::is_trivially_move_constructible_v<ReprRustUnion>);
static_assert(std::is_trivially_move_assignable_v<ReprRustUnion>);
namespace __crubit_internal {
extern "C" void
__crubit_thunk__uRNvMs_uCsh2mUQSogkZl_u13example_ucrateNtB4_u13ReprRustUnion5set_ua(
    ::example_crate::ReprRustUnion& [[clang::annotate_type("lifetime",
                                                           "__anon1")]],
    std::int32_t);
}
inline void ReprRustUnion::set_a(std::int32_t a)
    [[clang::annotate_type("lifetime", "__anon1")]] {
  return __crubit_internal::
      __crubit_thunk__uRNvMs_uCsh2mUQSogkZl_u13example_ucrateNtB4_u13ReprRustUnion5set_ua(
          *this, a);
}

namespace __crubit_internal {
extern "C" void
__crubit_thunk__uRNvMs_uCsh2mUQSogkZl_u13example_ucrateNtB4_u13ReprRustUnion5set_ub(
    ::example_crate::ReprRustUnion& [[clang::annotate_type("lifetime",
                                                           "__anon1")]],
    double);
}
inline void ReprRustUnion::set_b(double b)
    [[clang::annotate_type("lifetime", "__anon1")]] {
  return __crubit_internal::
      __crubit_thunk__uRNvMs_uCsh2mUQSogkZl_u13example_ucrateNtB4_u13ReprRustUnion5set_ub(
          *this, b);
}
inline void ReprRustUnion::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(ReprRustUnion, __opaque_blob_of_bytes));
}
}  // namespace example_crate
