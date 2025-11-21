// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// consts_golden
// Features: custom_ffi_types, experimental, infer_operator_lifetimes,
// non_unpin_ctor, std_unique_ptr, std_vector, supported, wrapper

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_CONSTS_CONSTS_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_CONSTS_CONSTS_GOLDEN

#include "support/annotations_internal.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>

namespace consts {
static constexpr bool RUST_TRUE = true;
static constexpr bool RUST_FALSE = false;
static constexpr std::int8_t RUST_INT8_MIN = -128;
static constexpr std::int8_t RUST_INT8_MAX = 127;
static constexpr std::int16_t RUST_INT16_MIN = -32768;
static constexpr std::int16_t RUST_INT16_MAX = 32767;
static constexpr std::int32_t RUST_INT32_MIN = INT32_C(-2147483648);
static constexpr std::int32_t RUST_INT32_MAX = INT32_C(2147483647);
static constexpr std::int64_t RUST_INT64_MIN = INT64_MIN;
static constexpr std::int64_t RUST_INT64_MAX = INT64_C(9223372036854775807);
static constexpr std::uint8_t RUST_UINT8_MIN = 0;
static constexpr std::uint8_t RUST_UINT8_MAX = 255;
static constexpr std::uint16_t RUST_UINT16_MIN = 0;
static constexpr std::uint16_t RUST_UINT16_MAX = UINT16_C(65535);
static constexpr std::uint32_t RUST_UINT32_MIN = 0;
static constexpr std::uint32_t RUST_UINT32_MAX = UINT32_C(4294967295);
static constexpr std::uint64_t RUST_UINT64_MIN = 0;
static constexpr std::uint64_t RUST_UINT64_MAX = UINT64_C(18446744073709551615);
static constexpr std::intptr_t RUST_ISIZE_MIN = INT64_MIN;
static constexpr std::intptr_t RUST_ISIZE_MAX = INT64_C(9223372036854775807);
static constexpr std::intptr_t RUST_USIZE_MIN = INT64_MIN;
static constexpr std::intptr_t RUST_USIZE_MAX = INT64_C(9223372036854775807);
static constexpr float RUST_F32_MIN = -3.40282347E+38f;
static constexpr float RUST_F32_MAX = 3.40282347E+38f;
static constexpr double RUST_F64_MIN = -1.7976931348623157E+308L;
static constexpr double RUST_F64_MAX = 1.7976931348623157E+308L;
static constexpr std::int32_t INT_POS = INT32_C(42);
static constexpr std::int32_t INT_NEG = INT32_C(-17);
static constexpr float FLOAT_32 = 0.125f;
static constexpr double FLOAT_64 = 0.0078125L;
static constexpr std::int64_t LARGE_INT = INT64_C(9223372036854775807);
static constexpr std::uint32_t UNSIGNED_INT = UINT32_C(4294967295);
static constexpr std::uintptr_t SLICE_LENGTH = 11;
static constexpr std::intptr_t ISIZE = INT64_C(42);
static constexpr char CHAR = 42;
// Generated from:
// cc_bindings_from_rs/test/consts/consts.rs;l=49
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: consts_golden :: TyWithAssocConsts") alignas(1) [[clang::trivial_abi]]
TyWithAssocConsts final {
 public:
  // `consts_golden::TyWithAssocConsts` doesn't implement the `Default` trait
  TyWithAssocConsts() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~TyWithAssocConsts() = default;
  TyWithAssocConsts(TyWithAssocConsts&&) = default;
  TyWithAssocConsts& operator=(TyWithAssocConsts&&) = default;

  // `consts_golden::TyWithAssocConsts` doesn't implement the `Clone` trait
  TyWithAssocConsts(const TyWithAssocConsts&) = delete;
  TyWithAssocConsts& operator=(const TyWithAssocConsts&) = delete;
  TyWithAssocConsts(::crubit::UnsafeRelocateTag, TyWithAssocConsts&& value) {
    memcpy(this, &value, sizeof(value));
  }
  static constexpr std::int32_t ASSOC_42 = INT32_C(42);

 private:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/consts/consts.rs;l=49
    std::uint8_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

static_assert(
    sizeof(TyWithAssocConsts) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(TyWithAssocConsts) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<TyWithAssocConsts>);
static_assert(std::is_trivially_move_constructible_v<TyWithAssocConsts>);
static_assert(std::is_trivially_move_assignable_v<TyWithAssocConsts>);
inline void TyWithAssocConsts::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(TyWithAssocConsts, __field0));
}
}  // namespace consts
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_CONSTS_CONSTS_GOLDEN
