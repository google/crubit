// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// consts_golden
// Features: custom_ffi_types, experimental, non_unpin_ctor, std_unique_ptr,
// std_vector, supported, wrapper

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_CONSTS_CONSTS_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_CONSTS_CONSTS_GOLDEN

#include "support/annotations_internal.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>

#include "support/ffi_11/ffi_11.h"

namespace consts {
static constexpr decltype(char(0)) kChar = 42;
static constexpr float kFloat32 = 0.125f;
static constexpr double kFloat64 = 0.0078125L;
static constexpr std::int32_t kIntNeg = INT32_C(-17);
static constexpr std::int32_t kIntPos = INT32_C(42);
static constexpr std::intptr_t kIsize = INT64_C(42);
static constexpr std::int64_t kLargeInt = INT64_C(9223372036854775807);
static constexpr float kRustF32Max = 3.40282347E+38f;
static constexpr float kRustF32Min = -3.40282347E+38f;
static constexpr double kRustF64Max = 1.7976931348623157E+308L;
static constexpr double kRustF64Min = -1.7976931348623157E+308L;
static constexpr bool kRustFalse = false;
static constexpr std::int16_t kRustInt16Max = 32767;
static constexpr std::int16_t kRustInt16Min = -32768;
static constexpr std::int32_t kRustInt32Max = INT32_C(2147483647);
static constexpr std::int32_t kRustInt32Min = INT32_C(-2147483648);
static constexpr std::int64_t kRustInt64Max = INT64_C(9223372036854775807);
static constexpr std::int64_t kRustInt64Min = INT64_MIN;
static constexpr std::int8_t kRustInt8Max = 127;
static constexpr std::int8_t kRustInt8Min = -128;
static constexpr std::intptr_t kRustIsizeMax = INT64_C(9223372036854775807);
static constexpr std::intptr_t kRustIsizeMin = INT64_MIN;
static constexpr bool kRustTrue = true;
static constexpr std::uint16_t kRustUint16Max = UINT16_C(65535);
static constexpr std::uint16_t kRustUint16Min = 0;
static constexpr std::uint32_t kRustUint32Max = UINT32_C(4294967295);
static constexpr std::uint32_t kRustUint32Min = 0;
static constexpr std::uint64_t kRustUint64Max = UINT64_C(18446744073709551615);
static constexpr std::uint64_t kRustUint64Min = 0;
static constexpr std::uint8_t kRustUint8Max = 255;
static constexpr std::uint8_t kRustUint8Min = 0;
static constexpr std::intptr_t kRustUsizeMax = INT64_C(9223372036854775807);
static constexpr std::intptr_t kRustUsizeMin = INT64_MIN;
static constexpr std::uintptr_t kSliceLength = 11;
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
  static constexpr std::int32_t kAssoc42 = INT32_C(42);

 private:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/consts/consts.rs;l=49
    std::uint8_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};
static constexpr std::uint32_t kUnsignedInt = UINT32_C(4294967295);
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
