// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// non_trivially_destructible_rust_golden
// Features: assume_lifetimes, custom_ffi_types, experimental, non_unpin_ctor,
// std_unique_ptr, std_vector, supported, wrapper

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_GOLDEN_NON_TRIVIALLY_DESTRUCTIBLE_RUST_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_GOLDEN_NON_TRIVIALLY_DESTRUCTIBLE_RUST_GOLDEN

#include "support/annotations_internal.h"
#include "support/internal/memswap.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <cstdint>
#include <utility>

namespace non_trivially_destructible_rust {

// Generated from:
// cc_bindings_from_rs/test/golden/non_trivially_destructible.rs;l=6
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: non_trivially_destructible_rust_golden :: "
    "NonTriviallyDestructable") alignas(4) [[clang::trivial_abi]]
NonTriviallyDestructable final {
 public:
  // Default::default
  NonTriviallyDestructable();

  // Drop::drop
  ~NonTriviallyDestructable();

  NonTriviallyDestructable(NonTriviallyDestructable&&);
  NonTriviallyDestructable& operator=(NonTriviallyDestructable&&);

  // Clone::clone
  NonTriviallyDestructable(const NonTriviallyDestructable&);

  // Clone::clone_from
  NonTriviallyDestructable& operator=(const NonTriviallyDestructable&);

  NonTriviallyDestructable(::crubit::UnsafeRelocateTag,
                           NonTriviallyDestructable&& value) {
    memcpy(this, &value, sizeof(value));
  }
  union {
    // Generated from:
    // cc_bindings_from_rs/test/golden/non_trivially_destructible.rs;l=7
    std::int32_t field;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/golden/non_trivially_destructible.rs;l=18
::non_trivially_destructible_rust::NonTriviallyDestructable return_by_value();

// Generated from:
// cc_bindings_from_rs/test/golden/non_trivially_destructible.rs;l=16
void take_by_value(
    ::non_trivially_destructible_rust::NonTriviallyDestructable _x);

static_assert(
    sizeof(NonTriviallyDestructable) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NonTriviallyDestructable) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(
    ::non_trivially_destructible_rust::NonTriviallyDestructable* __ret_ptr);
}
inline NonTriviallyDestructable::NonTriviallyDestructable() {
  __crubit_internal::__crubit_thunk_default(this);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(
    ::non_trivially_destructible_rust::NonTriviallyDestructable&);
}
inline NonTriviallyDestructable::~NonTriviallyDestructable() {
  __crubit_internal::__crubit_thunk_drop(*this);
}
inline NonTriviallyDestructable::NonTriviallyDestructable(
    NonTriviallyDestructable&& other)
    : NonTriviallyDestructable() {
  *this = std::move(other);
}
inline NonTriviallyDestructable& NonTriviallyDestructable::operator=(
    NonTriviallyDestructable&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone(
    ::non_trivially_destructible_rust::NonTriviallyDestructable const&,
    ::non_trivially_destructible_rust::NonTriviallyDestructable* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone_ufrom(
    ::non_trivially_destructible_rust::NonTriviallyDestructable&,
    ::non_trivially_destructible_rust::NonTriviallyDestructable const&);
}
inline NonTriviallyDestructable::NonTriviallyDestructable(
    const NonTriviallyDestructable& other) {
  __crubit_internal::__crubit_thunk_clone(other, this);
}
inline NonTriviallyDestructable& NonTriviallyDestructable::operator=(
    const NonTriviallyDestructable& other) {
  if (this != &other) {
    __crubit_internal::__crubit_thunk_clone_ufrom(*this, other);
  }
  return *this;
}
inline void NonTriviallyDestructable::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NonTriviallyDestructable, field));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_return_uby_uvalue(
    ::non_trivially_destructible_rust::NonTriviallyDestructable* __ret_ptr);
}
inline ::non_trivially_destructible_rust::NonTriviallyDestructable
return_by_value() {
  crubit::Slot<::non_trivially_destructible_rust::NonTriviallyDestructable>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_return_uby_uvalue(__return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_take_uby_uvalue(
    ::non_trivially_destructible_rust::NonTriviallyDestructable*);
}
inline void take_by_value(
    ::non_trivially_destructible_rust::NonTriviallyDestructable _x) {
  crubit::Slot _x_slot((std::move(_x)));
  return __crubit_internal::__crubit_thunk_take_uby_uvalue(_x_slot.Get());
}

}  // namespace non_trivially_destructible_rust
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_GOLDEN_NON_TRIVIALLY_DESTRUCTIBLE_RUST_GOLDEN
