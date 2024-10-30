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
// examples/rust/struct/example.rs;l=6
struct CRUBIT_INTERNAL_RUST_TYPE(":: example_crate :: Struct") alignas(4)
    [[clang::trivial_abi]] Struct final {
 public:
  // Default::default
  Struct();

  // No custom `Drop` impl and no custom \"drop glue\" required
  ~Struct() = default;
  Struct(Struct&&) = default;
  Struct& operator=(Struct&&) = default;

  // Clone::clone
  Struct(const Struct&);

  // Clone::clone_from
  Struct& operator=(const Struct&);

 public:
  union {
    // Generated from:
    // examples/rust/struct/example.rs;l=7
    std::int32_t a;
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
extern "C" void
__crubit_thunk__uRNvYNtCseF4PRLPR6bH_u13example_ucrate6StructNtNtCs6JfLQpFPRiJ_u4core7default7Default7defaultB4_u(
    ::example_crate::Struct* __ret_ptr);
}
inline Struct::Struct() {
  __crubit_internal::
      __crubit_thunk__uRNvYNtCseF4PRLPR6bH_u13example_ucrate6StructNtNtCs6JfLQpFPRiJ_u4core7default7Default7defaultB4_u(
          this);
}
static_assert(std::is_trivially_destructible_v<Struct>);
static_assert(std::is_trivially_move_constructible_v<Struct>);
static_assert(std::is_trivially_move_assignable_v<Struct>);
namespace __crubit_internal {
extern "C" void
__crubit_thunk__uRNvYNtCseF4PRLPR6bH_u13example_ucrate6StructNtNtCs6JfLQpFPRiJ_u4core5clone5Clone5cloneB4_u(
    ::example_crate::Struct const& [[clang::annotate_type("lifetime",
                                                          "__anon1")]],
    ::example_crate::Struct* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk__uRNvYNtCseF4PRLPR6bH_u13example_ucrate6StructNtNtCs6JfLQpFPRiJ_u4core5clone5Clone10clone_ufromB4_u(
    ::example_crate::Struct& [[clang::annotate_type("lifetime", "__anon1")]],
    ::example_crate::Struct const& [[clang::annotate_type("lifetime",
                                                          "__anon2")]]);
}
inline Struct::Struct(const Struct& other) {
  __crubit_internal::
      __crubit_thunk__uRNvYNtCseF4PRLPR6bH_u13example_ucrate6StructNtNtCs6JfLQpFPRiJ_u4core5clone5Clone5cloneB4_u(
          other, this);
}
inline Struct& Struct::operator=(const Struct& other) {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk__uRNvYNtCseF4PRLPR6bH_u13example_ucrate6StructNtNtCs6JfLQpFPRiJ_u4core5clone5Clone10clone_ufromB4_u(
            *this, other);
  }
  return *this;
}
inline void Struct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Struct, a));
}
}  // namespace example_crate
