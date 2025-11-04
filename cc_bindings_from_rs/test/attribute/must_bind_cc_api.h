// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// must_bind_golden
// Features: infer_operator_lifetimes, non_unpin_ctor, std_unique_ptr,
// std_vector, supported

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ATTRIBUTE_MUST_BIND_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ATTRIBUTE_MUST_BIND_GOLDEN

#include "support/annotations_internal.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>
#include <utility>

namespace must_bind {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/attribute/must_bind.rs;l=7
struct CRUBIT_INTERNAL_RUST_TYPE(":: must_bind_golden :: Original") alignas(4)
    [[clang::trivial_abi]] Original final {
 public:
  // `must_bind_golden::Original` doesn't implement the `Default` trait
  Original() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~Original() = default;
  Original(Original&&) = default;
  Original& operator=(Original&&) = default;

  // `must_bind_golden::Original` doesn't implement the `Clone` trait
  Original(const Original&) = delete;
  Original& operator=(const Original&) = delete;
  Original(::crubit::UnsafeRelocateTag, Original&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  //
  // Generated from:
  // cc_bindings_from_rs/test/attribute/must_bind.rs;l=14
  static ::must_bind::Original new_();

  union {
    // Generated from:
    // cc_bindings_from_rs/test/attribute/must_bind.rs;l=8
    std::int32_t x;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/attribute/must_bind.rs;l=20
void bar();

static_assert(
    sizeof(Original) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Original) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<Original>);
static_assert(std::is_trivially_move_constructible_v<Original>);
static_assert(std::is_trivially_move_assignable_v<Original>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::must_bind::Original* __ret_ptr);
}
inline ::must_bind::Original Original::new_() {
  crubit::Slot<::must_bind::Original> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(__return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void Original::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Original, x));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_bar();
}
inline void bar() { return __crubit_internal::__crubit_thunk_bar(); }

}  // namespace must_bind
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ATTRIBUTE_MUST_BIND_GOLDEN
