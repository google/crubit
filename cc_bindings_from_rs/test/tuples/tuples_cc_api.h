// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// tuples_golden
// Features: custom_ffi_types, experimental, non_unpin_ctor, std_unique_ptr,
// std_vector, supported, wrapper

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TUPLES_TUPLES_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TUPLES_TUPLES_GOLDEN

#include "support/annotations_internal.h"
#include "support/internal/memswap.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <cstdint>
#include <tuple>
#include <type_traits>
#include <utility>

namespace tuples {

// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=11
void return_unit_is_not_tuple();

// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=12
std::tuple<std::int32_t> return_c_abi_compatible_five_in_tuple();

// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=15
void param_c_abi_compatible_five_in_tuple(std::tuple<std::int32_t> five);

// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=19
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuples_golden :: AdtHoldingFiveAndSix") alignas(4)
    [[clang::trivial_abi]] AdtHoldingFiveAndSix final {
 public:
  // `tuples_golden::AdtHoldingFiveAndSix` doesn't implement the `Default` trait
  AdtHoldingFiveAndSix() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~AdtHoldingFiveAndSix() = default;
  AdtHoldingFiveAndSix(AdtHoldingFiveAndSix&&) = default;
  AdtHoldingFiveAndSix& operator=(AdtHoldingFiveAndSix&&) = default;

  // `tuples_golden::AdtHoldingFiveAndSix` doesn't implement the `Clone` trait
  AdtHoldingFiveAndSix(const AdtHoldingFiveAndSix&) = delete;
  AdtHoldingFiveAndSix& operator=(const AdtHoldingFiveAndSix&) = delete;
  AdtHoldingFiveAndSix(::crubit::UnsafeRelocateTag,
                       AdtHoldingFiveAndSix&& value) {
    memcpy(this, &value, sizeof(value));
  }

 private:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/tuples/tuples.rs;l=20
    std::int32_t five;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/tuples/tuples.rs;l=21
    std::int32_t six;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=23
std::tuple<::tuples::AdtHoldingFiveAndSix> return_adt_in_tuple();

// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=26
void param_adt_in_tuple(std::tuple<::tuples::AdtHoldingFiveAndSix> adt);

// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=35
struct CRUBIT_INTERNAL_RUST_TYPE(":: tuples_golden :: NontrivialDrop") alignas(
    1) [[clang::trivial_abi]] NontrivialDrop final {
 public:
  // Default::default
  NontrivialDrop();

  // Drop::drop
  ~NontrivialDrop();

  NontrivialDrop(NontrivialDrop&&);
  NontrivialDrop& operator=(NontrivialDrop&&);

  // `tuples_golden::NontrivialDrop` doesn't implement the `Clone` trait
  NontrivialDrop(const NontrivialDrop&) = delete;
  NontrivialDrop& operator=(const NontrivialDrop&) = delete;
  NontrivialDrop(::crubit::UnsafeRelocateTag, NontrivialDrop&& value) {
    memcpy(this, &value, sizeof(value));
  }

 private:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/tuples/tuples.rs;l=35
    std::uint8_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=46
std::tuple<::tuples::NontrivialDrop> return_new_nontrivial_drop_in_tuple();

// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=49
void param_nontrivial_drop_in_tuple(
    std::tuple<::tuples::NontrivialDrop> nontrivial_drop);

// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=52
void assert_nontrivial_drop_count(std::uint8_t drop_count);

//  The same as NontrivialDrop, but without a C++ move operation. This can be
//  returned by value,
//
//  even inside a tuple!
//
// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=58
struct CRUBIT_INTERNAL_RUST_TYPE(":: tuples_golden :: NonCppMovable") alignas(1)
    [[clang::trivial_abi]] NonCppMovable final {
 public:
  // `tuples_golden::NonCppMovable` doesn't implement the `Default` trait
  NonCppMovable() = delete;

  // Drop::drop
  ~NonCppMovable();

  // C++ moves are deleted because there's no non-destructive implementation
  // available.
  NonCppMovable(NonCppMovable&&) = delete;
  NonCppMovable& operator=(NonCppMovable&&) = delete;
  // `tuples_golden::NonCppMovable` doesn't implement the `Clone` trait
  NonCppMovable(const NonCppMovable&) = delete;
  NonCppMovable& operator=(const NonCppMovable&) = delete;
  NonCppMovable(::crubit::UnsafeRelocateTag, NonCppMovable&& value) {
    memcpy(this, &value, sizeof(value));
  }
  union {
    // Generated from:
    // cc_bindings_from_rs/test/tuples/tuples.rs;l=59
    std::uint8_t value;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Error generating bindings for `return_new_non_cpp_movable_in_tuple` defined
// at cc_bindings_from_rs/test/tuples/tuples.rs;l=66:
// Can't return a type by value inside a compound data type without a move
// constructor

// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=73
void param_nested_tuples(
    std::tuple<std::tuple<std::int32_t, std::int32_t>, std::int32_t> v);

// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=76
std::tuple<std::tuple<std::int32_t, std::int32_t>, std::int32_t>
return_nested_tuples();

// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=80
void param_triply_nested_tuple(
    std::tuple<std::tuple<std::tuple<std::int32_t>>> v);

// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=83
std::tuple<std::tuple<std::tuple<std::int32_t>>> return_triply_nested_tuple();

// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=87
void param_ffi_alias_in_tuple(std::tuple<char> five);

// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=90
std::tuple<char> return_ffi_alias_in_tuple();

// Generated from:
// cc_bindings_from_rs/test/tuples/tuples.rs;l=94
struct CRUBIT_INTERNAL_RUST_TYPE(":: tuples_golden :: TupleStruct") alignas(4)
    [[clang::trivial_abi]] TupleStruct final {
 public:
  // `tuples_golden::TupleStruct` doesn't implement the `Default` trait
  TupleStruct() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~TupleStruct() = default;
  TupleStruct(TupleStruct&&) = default;
  TupleStruct& operator=(TupleStruct&&) = default;

  // `tuples_golden::TupleStruct` doesn't implement the `Clone` trait
  TupleStruct(const TupleStruct&) = delete;
  TupleStruct& operator=(const TupleStruct&) = delete;
  TupleStruct(::crubit::UnsafeRelocateTag, TupleStruct&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Error generating bindings for `TupleStruct::tuple_not_by_value` defined at
  // cc_bindings_from_rs/test/tuples/tuples.rs;l=101:
  // Error formatting function return type `*const ()`: Failed to format the
  // pointee of the pointer type `*const ()`: Tuple types cannot be used inside
  // of compound data types, because std::tuple is not layout-compatible with a
  // Rust tuple.

 private:
  // Field type has been replaced with a blob of bytes: Tuple types cannot be
  // used inside of compound data types, because std::tuple is not
  // layout-compatible with a Rust tuple.
  unsigned char tuple_field[4];
  // Skipped bindings for field `empty_tuple_field`: ZST fields are not
  // supported (b/258259459)
 private:
  static void __crubit_field_offset_assertions();
};

// Error generating bindings for `TUPLE_CONSTANT` defined at
// cc_bindings_from_rs/test/tuples/tuples.rs;l=106:
// Unsupported constant type: (i32,)

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_uunit_uis_unot_utuple();
}
inline void return_unit_is_not_tuple() {
  return __crubit_internal::__crubit_thunk_return_uunit_uis_unot_utuple();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_uc_uabi_ucompatible_ufive_uin_utuple(
    void** __ret_ptr);
}
inline std::tuple<std::int32_t> return_c_abi_compatible_five_in_tuple() {
  std::int32_t __return_value_0_ret_val_holder;
  std::int32_t* __return_value_0_storage = &__return_value_0_ret_val_holder;
  void* __return_value_storage[] = {__return_value_0_storage};
  __crubit_internal::__crubit_thunk_return_uc_uabi_ucompatible_ufive_uin_utuple(
      __return_value_storage);
  return std::make_tuple(*__return_value_0_storage);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_param_uc_uabi_ucompatible_ufive_uin_utuple(
    void**);
}
inline void param_c_abi_compatible_five_in_tuple(
    std::tuple<std::int32_t> five) {
  auto&& five_0 = std::get<0>(five);
  auto&& five_cabi_0 = five_0;
  void* five_cabi[] = {&five_cabi_0};
  return __crubit_internal::
      __crubit_thunk_param_uc_uabi_ucompatible_ufive_uin_utuple(five_cabi);
}

static_assert(
    sizeof(AdtHoldingFiveAndSix) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(AdtHoldingFiveAndSix) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<AdtHoldingFiveAndSix>);
static_assert(std::is_trivially_move_constructible_v<AdtHoldingFiveAndSix>);
static_assert(std::is_trivially_move_assignable_v<AdtHoldingFiveAndSix>);
inline void AdtHoldingFiveAndSix::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(AdtHoldingFiveAndSix, five));
  static_assert(4 == offsetof(AdtHoldingFiveAndSix, six));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_return_uadt_uin_utuple(void** __ret_ptr);
}
inline std::tuple<::tuples::AdtHoldingFiveAndSix> return_adt_in_tuple() {
  crubit::Slot<::tuples::AdtHoldingFiveAndSix> __return_value_0_ret_val_holder;
  auto* __return_value_0_storage = __return_value_0_ret_val_holder.Get();
  void* __return_value_storage[] = {__return_value_0_storage};
  __crubit_internal::__crubit_thunk_return_uadt_uin_utuple(
      __return_value_storage);
  return std::make_tuple(
      std::move(__return_value_0_ret_val_holder).AssumeInitAndTakeValue());
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_param_uadt_uin_utuple(void**);
}
inline void param_adt_in_tuple(std::tuple<::tuples::AdtHoldingFiveAndSix> adt) {
  auto&& adt_0 = std::get<0>(adt);
  auto&& adt_cabi_0 = &adt_0;
  void* adt_cabi[] = {&adt_cabi_0};
  return __crubit_internal::__crubit_thunk_param_uadt_uin_utuple(adt_cabi);
}

static_assert(
    sizeof(NontrivialDrop) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NontrivialDrop) == 1,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(::tuples::NontrivialDrop* __ret_ptr);
}
inline NontrivialDrop::NontrivialDrop() {
  __crubit_internal::__crubit_thunk_default(this);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(::tuples::NontrivialDrop&);
}
inline NontrivialDrop::~NontrivialDrop() {
  __crubit_internal::__crubit_thunk_drop(*this);
}
inline NontrivialDrop::NontrivialDrop(NontrivialDrop&& other)
    : NontrivialDrop() {
  *this = std::move(other);
}
inline NontrivialDrop& NontrivialDrop::operator=(NontrivialDrop&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
inline void NontrivialDrop::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NontrivialDrop, __field0));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_return_unew_unontrivial_udrop_uin_utuple(
    void** __ret_ptr);
}
inline std::tuple<::tuples::NontrivialDrop>
return_new_nontrivial_drop_in_tuple() {
  crubit::Slot<::tuples::NontrivialDrop> __return_value_0_ret_val_holder;
  auto* __return_value_0_storage = __return_value_0_ret_val_holder.Get();
  void* __return_value_storage[] = {__return_value_0_storage};
  __crubit_internal::__crubit_thunk_return_unew_unontrivial_udrop_uin_utuple(
      __return_value_storage);
  return std::make_tuple(
      std::move(__return_value_0_ret_val_holder).AssumeInitAndTakeValue());
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_param_unontrivial_udrop_uin_utuple(void**);
}
inline void param_nontrivial_drop_in_tuple(
    std::tuple<::tuples::NontrivialDrop> nontrivial_drop) {
  auto&& nontrivial_drop_0 = std::get<0>(nontrivial_drop);
  crubit::Slot nontrivial_drop_0_slot((std::move(nontrivial_drop_0)));
  auto&& nontrivial_drop_cabi_0 = nontrivial_drop_0_slot.Get();
  void* nontrivial_drop_cabi[] = {&nontrivial_drop_cabi_0};
  return __crubit_internal::__crubit_thunk_param_unontrivial_udrop_uin_utuple(
      nontrivial_drop_cabi);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_assert_unontrivial_udrop_ucount(std::uint8_t);
}
inline void assert_nontrivial_drop_count(std::uint8_t drop_count) {
  return __crubit_internal::__crubit_thunk_assert_unontrivial_udrop_ucount(
      drop_count);
}

static_assert(
    sizeof(NonCppMovable) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NonCppMovable) == 1,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(::tuples::NonCppMovable&);
}
inline NonCppMovable::~NonCppMovable() {
  __crubit_internal::__crubit_thunk_drop(*this);
}
inline void NonCppMovable::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NonCppMovable, value));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_param_unested_utuples(void**);
}
inline void param_nested_tuples(
    std::tuple<std::tuple<std::int32_t, std::int32_t>, std::int32_t> v) {
  auto&& v_0 = std::get<0>(v);
  auto&& v_0_0 = std::get<0>(v_0);
  auto&& v_0_cabi_0 = v_0_0;
  auto&& v_0_1 = std::get<1>(v_0);
  auto&& v_0_cabi_1 = v_0_1;
  void* v_0_cabi[] = {&v_0_cabi_0, &v_0_cabi_1};
  auto* v_cabi_0 = &v_0_cabi;
  auto&& v_1 = std::get<1>(v);
  auto&& v_cabi_1 = v_1;
  void* v_cabi[] = {&v_cabi_0, &v_cabi_1};
  return __crubit_internal::__crubit_thunk_param_unested_utuples(v_cabi);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_unested_utuples(void** __ret_ptr);
}
inline std::tuple<std::tuple<std::int32_t, std::int32_t>, std::int32_t>
return_nested_tuples() {
  std::int32_t __return_value_0_0_ret_val_holder;
  std::int32_t* __return_value_0_0_storage = &__return_value_0_0_ret_val_holder;
  std::int32_t __return_value_0_1_ret_val_holder;
  std::int32_t* __return_value_0_1_storage = &__return_value_0_1_ret_val_holder;
  void* __return_value_0_storage[] = {__return_value_0_0_storage,
                                      __return_value_0_1_storage};
  std::int32_t __return_value_1_ret_val_holder;
  std::int32_t* __return_value_1_storage = &__return_value_1_ret_val_holder;
  void* __return_value_storage[] = {__return_value_0_storage,
                                    __return_value_1_storage};
  __crubit_internal::__crubit_thunk_return_unested_utuples(
      __return_value_storage);
  return std::make_tuple(
      std::make_tuple(*__return_value_0_0_storage, *__return_value_0_1_storage),
      *__return_value_1_storage);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_param_utriply_unested_utuple(void**);
}
inline void param_triply_nested_tuple(
    std::tuple<std::tuple<std::tuple<std::int32_t>>> v) {
  auto&& v_0 = std::get<0>(v);
  auto&& v_0_0 = std::get<0>(v_0);
  auto&& v_0_0_0 = std::get<0>(v_0_0);
  auto&& v_0_0_cabi_0 = v_0_0_0;
  void* v_0_0_cabi[] = {&v_0_0_cabi_0};
  auto* v_0_cabi_0 = &v_0_0_cabi;
  void* v_0_cabi[] = {&v_0_cabi_0};
  auto* v_cabi_0 = &v_0_cabi;
  void* v_cabi[] = {&v_cabi_0};
  return __crubit_internal::__crubit_thunk_param_utriply_unested_utuple(v_cabi);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_utriply_unested_utuple(void** __ret_ptr);
}
inline std::tuple<std::tuple<std::tuple<std::int32_t>>>
return_triply_nested_tuple() {
  std::int32_t __return_value_0_0_0_ret_val_holder;
  std::int32_t* __return_value_0_0_0_storage =
      &__return_value_0_0_0_ret_val_holder;
  void* __return_value_0_0_storage[] = {__return_value_0_0_0_storage};
  void* __return_value_0_storage[] = {__return_value_0_0_storage};
  void* __return_value_storage[] = {__return_value_0_storage};
  __crubit_internal::__crubit_thunk_return_utriply_unested_utuple(
      __return_value_storage);
  return std::make_tuple(
      std::make_tuple(std::make_tuple(*__return_value_0_0_0_storage)));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_param_uffi_ualias_uin_utuple(void**);
}
inline void param_ffi_alias_in_tuple(std::tuple<char> five) {
  auto&& five_0 = std::get<0>(five);
  auto&& five_cabi_0 = five_0;
  void* five_cabi[] = {&five_cabi_0};
  return __crubit_internal::__crubit_thunk_param_uffi_ualias_uin_utuple(
      five_cabi);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_uffi_ualias_uin_utuple(void** __ret_ptr);
}
inline std::tuple<char> return_ffi_alias_in_tuple() {
  char __return_value_0_ret_val_holder;
  char* __return_value_0_storage = &__return_value_0_ret_val_holder;
  void* __return_value_storage[] = {__return_value_0_storage};
  __crubit_internal::__crubit_thunk_return_uffi_ualias_uin_utuple(
      __return_value_storage);
  return std::make_tuple(*__return_value_0_storage);
}

static_assert(
    sizeof(TupleStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(TupleStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<TupleStruct>);
static_assert(std::is_trivially_move_constructible_v<TupleStruct>);
static_assert(std::is_trivially_move_assignable_v<TupleStruct>);
inline void TupleStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(TupleStruct, tuple_field));
}
}  // namespace tuples
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TUPLES_TUPLES_GOLDEN
