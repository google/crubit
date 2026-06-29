// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// example_crate_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_EXAMPLES_RUST_RUST_UNION_EXAMPLE_CRATE_GOLDEN
#define THIRD_PARTY_CRUBIT_EXAMPLES_RUST_RUST_UNION_EXAMPLE_CRATE_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <cstdint>
#include <cstring>
#include <type_traits>

namespace example_crate {

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

  // `example_crate_golden::ReprRustUnion` doesn't implement the `Clone` trait
  ReprRustUnion(const ReprRustUnion&) = delete;
  ReprRustUnion& operator=(const ReprRustUnion&) = delete;
  ReprRustUnion(::crubit::UnsafeRelocateTag, ReprRustUnion&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  void set_a(::std::int32_t a);

  void set_b(double b);

 private:
  struct {
    ::std::int32_t value;
  } a;
  struct {
    double value;
  } b;

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
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_default(
    ::example_crate::ReprRustUnion* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::example_crate::ReprRustUnion::ReprRustUnion() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(::std::is_trivially_destructible_v<ReprRustUnion>);
static_assert(
    ::std::is_trivially_move_constructible_v<::example_crate::ReprRustUnion>);
static_assert(
    ::std::is_trivially_move_assignable_v<::example_crate::ReprRustUnion>);
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_set_ua(::example_crate::ReprRustUnion&,
                                      ::std::int32_t);
/// \endcond
}  // namespace __crubit_internal
inline void ReprRustUnion::set_a(::std::int32_t a) {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_set_ua(self, a);
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_set_ub(::example_crate::ReprRustUnion&, double);
/// \endcond
}  // namespace __crubit_internal
inline void ReprRustUnion::set_b(double b) {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_set_ub(self, b);
}
inline void ReprRustUnion::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(ReprRustUnion, a));
  static_assert(0 == offsetof(ReprRustUnion, b));
}
}  // namespace example_crate

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_EXAMPLES_RUST_RUST_UNION_EXAMPLE_CRATE_GOLDEN
