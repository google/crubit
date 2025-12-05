// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// example_crate_golden
// Features: custom_ffi_types, non_unpin_ctor, std_unique_ptr, std_vector,
// supported

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_EXAMPLES_RUST_STRUCT_EXAMPLE_CRATE_GOLDEN
#define THIRD_PARTY_CRUBIT_EXAMPLES_RUST_STRUCT_EXAMPLE_CRATE_GOLDEN

#include "support/annotations_internal.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>

namespace example_crate {

// Generated from:
// examples/rust/struct/example.rs;l=6
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
  Struct& operator=(const Struct&);

  Struct(::crubit::UnsafeRelocateTag, Struct&& value) {
    memcpy(this, &value, sizeof(value));
  }
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
extern "C" void __crubit_thunk_default(::example_crate::Struct* __ret_ptr);
}
inline Struct::Struct() { __crubit_internal::__crubit_thunk_default(this); }
static_assert(std::is_trivially_destructible_v<Struct>);
static_assert(std::is_trivially_move_constructible_v<Struct>);
static_assert(std::is_trivially_move_assignable_v<Struct>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone(::example_crate::Struct const&,
                                     ::example_crate::Struct* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone_ufrom(::example_crate::Struct&,
                                           ::example_crate::Struct const&);
}
inline Struct::Struct(const Struct& other) {
  __crubit_internal::__crubit_thunk_clone(other, this);
}
inline Struct& Struct::operator=(const Struct& other) {
  if (this != &other) {
    __crubit_internal::__crubit_thunk_clone_ufrom(*this, other);
  }
  return *this;
}
inline void Struct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Struct, a));
}
}  // namespace example_crate
#endif  // THIRD_PARTY_CRUBIT_EXAMPLES_RUST_STRUCT_EXAMPLE_CRATE_GOLDEN
