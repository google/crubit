// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// cpp_name_golden
// Features: supported, unsafe_types

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ATTRIBUTE_CPP_NAME_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ATTRIBUTE_CPP_NAME_GOLDEN

#include "support/annotations_internal.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>
#include <utility>

namespace cpp_name {

// CRUBIT_ANNOTATE: cpp_name=Replaced
//
// Generated from:
// cc_bindings_from_rs/test/attribute/cpp_name.rs;l=6
struct CRUBIT_INTERNAL_RUST_TYPE(":: cpp_name_golden :: Original") alignas(4)
    [[clang::trivial_abi]] Replaced final {
 public:
  // `Original` doesn't implement the `Default` trait
  Replaced() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~Replaced() = default;
  Replaced(Replaced&&) = default;
  Replaced& operator=(Replaced&&) = default;

  // `Original` doesn't implement the `Clone` trait
  Replaced(const Replaced&) = delete;
  Replaced& operator=(const Replaced&) = delete;
  Replaced(::crubit::UnsafeRelocateTag, Replaced&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: cpp_name=create
  //
  // Generated from:
  // cc_bindings_from_rs/test/attribute/cpp_name.rs;l=12
  static ::cpp_name::Replaced create();

 public:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/attribute/cpp_name.rs;l=7
    std::int32_t x;
  };

 private:
  static void __crubit_field_offset_assertions();
};

static_assert(
    sizeof(Replaced) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Replaced) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<Replaced>);
static_assert(std::is_trivially_move_constructible_v<Replaced>);
static_assert(std::is_trivially_move_assignable_v<Replaced>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::cpp_name::Replaced* __ret_ptr);
}
inline ::cpp_name::Replaced Replaced::create() {
  crubit::Slot<::cpp_name::Replaced> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(__return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void Replaced::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Replaced, x));
}
}  // namespace cpp_name
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ATTRIBUTE_CPP_NAME_GOLDEN
