// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// example_crate_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_EXAMPLES_RUST_STRUCT_EXAMPLE_CRATE_GOLDEN
#define THIRD_PARTY_CRUBIT_EXAMPLES_RUST_STRUCT_EXAMPLE_CRATE_GOLDEN

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

struct CRUBIT_INTERNAL_RUST_TYPE(":: example_crate_golden :: Struct") alignas(4)
    [[clang::trivial_abi]] Struct final {
 public:
  // Default::default
  Struct();

  // No custom `Drop` impl and no custom "drop glue" required
  ~Struct() = default;
  Struct(Struct&&) = default;
  Struct& operator=(Struct&&) = default;

  // Clone::clone
  Struct(const Struct&);

  // Clone::clone_from
  ::example_crate::Struct& operator=(const Struct&);

  Struct(::crubit::UnsafeRelocateTag, Struct&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  union {
    ::std::int32_t a;
  };

 private:
  static void __crubit_field_offset_assertions();
};

static_assert(
    sizeof(Struct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Struct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_default(::example_crate::Struct* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::example_crate::Struct::Struct() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(::std::is_trivially_destructible_v<Struct>);
static_assert(
    ::std::is_trivially_move_constructible_v<::example_crate::Struct>);
static_assert(::std::is_trivially_move_assignable_v<::example_crate::Struct>);
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_clone(::example_crate::Struct const&,
                                     ::example_crate::Struct* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_clone_ufrom(::example_crate::Struct&,
                                           ::example_crate::Struct const&);
/// \endcond
}  // namespace __crubit_internal
inline ::example_crate::Struct::Struct(const Struct& other) {
  __crubit_internal::__crubit_thunk_clone(other, this);
}
inline ::example_crate::Struct& ::example_crate::Struct::operator=(
    const Struct& other) {
  if (this != &other) {
    __crubit_internal::__crubit_thunk_clone_ufrom(*this, other);
  }
  return *this;
}
inline void Struct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Struct, a));
}
}  // namespace example_crate

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_EXAMPLES_RUST_STRUCT_EXAMPLE_CRATE_GOLDEN
