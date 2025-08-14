// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// tuple_structs_golden
// Features: supported, unsafe_types

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_STRUCTS_TUPLE_STRUCTS_TUPLE_STRUCTS_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_STRUCTS_TUPLE_STRUCTS_TUPLE_STRUCTS_GOLDEN

#include "support/annotations_internal.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>
#include <utility>

namespace tuple_structs {

// Generated from:
// cc_bindings_from_rs/test/structs/tuple_structs/tuple_structs.rs;l=6
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuple_structs_golden :: TupleStructOnePublicArg") alignas(4)
    [[clang::trivial_abi]] TupleStructOnePublicArg final {
 public:
  // `TupleStructOnePublicArg` doesn't implement the `Default` trait
  TupleStructOnePublicArg() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~TupleStructOnePublicArg() = default;
  TupleStructOnePublicArg(TupleStructOnePublicArg&&) = default;
  TupleStructOnePublicArg& operator=(TupleStructOnePublicArg&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  TupleStructOnePublicArg(const TupleStructOnePublicArg&) = default;
  TupleStructOnePublicArg& operator=(const TupleStructOnePublicArg&) = default;
  TupleStructOnePublicArg(::crubit::UnsafeRelocateTag,
                          TupleStructOnePublicArg&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/structs/tuple_structs/tuple_structs.rs;l=9
  static ::tuple_structs::TupleStructOnePublicArg create(std::int32_t arg);

  // Generated from:
  // cc_bindings_from_rs/test/structs/tuple_structs/tuple_structs.rs;l=12
  std::int32_t get_arg() const;

 public:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/tuple_structs/tuple_structs.rs;l=6
    std::int32_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/structs/tuple_structs/tuple_structs.rs;l=18
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuple_structs_golden :: TupleStructOnePrivateArg") alignas(4)
    [[clang::trivial_abi]] TupleStructOnePrivateArg final {
 public:
  // `TupleStructOnePrivateArg` doesn't implement the `Default` trait
  TupleStructOnePrivateArg() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~TupleStructOnePrivateArg() = default;
  TupleStructOnePrivateArg(TupleStructOnePrivateArg&&) = default;
  TupleStructOnePrivateArg& operator=(TupleStructOnePrivateArg&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  TupleStructOnePrivateArg(const TupleStructOnePrivateArg&) = default;
  TupleStructOnePrivateArg& operator=(const TupleStructOnePrivateArg&) =
      default;
  TupleStructOnePrivateArg(::crubit::UnsafeRelocateTag,
                           TupleStructOnePrivateArg&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/structs/tuple_structs/tuple_structs.rs;l=21
  static ::tuple_structs::TupleStructOnePrivateArg create(std::int32_t arg);

  // Generated from:
  // cc_bindings_from_rs/test/structs/tuple_structs/tuple_structs.rs;l=24
  std::int32_t get_arg() const;

 private:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/tuple_structs/tuple_structs.rs;l=18
    std::int32_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/structs/tuple_structs/tuple_structs.rs;l=30
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuple_structs_golden :: TupleStructTwoPublicArgs") alignas(4)
    [[clang::trivial_abi]] TupleStructTwoPublicArgs final {
 public:
  // `TupleStructTwoPublicArgs` doesn't implement the `Default` trait
  TupleStructTwoPublicArgs() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~TupleStructTwoPublicArgs() = default;
  TupleStructTwoPublicArgs(TupleStructTwoPublicArgs&&) = default;
  TupleStructTwoPublicArgs& operator=(TupleStructTwoPublicArgs&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  TupleStructTwoPublicArgs(const TupleStructTwoPublicArgs&) = default;
  TupleStructTwoPublicArgs& operator=(const TupleStructTwoPublicArgs&) =
      default;
  TupleStructTwoPublicArgs(::crubit::UnsafeRelocateTag,
                           TupleStructTwoPublicArgs&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/structs/tuple_structs/tuple_structs.rs;l=33
  static ::tuple_structs::TupleStructTwoPublicArgs create(
      std::int32_t first_arg, std::int32_t second_arg);

  // Generated from:
  // cc_bindings_from_rs/test/structs/tuple_structs/tuple_structs.rs;l=37
  std::int32_t get_first_arg() const;

  // Generated from:
  // cc_bindings_from_rs/test/structs/tuple_structs/tuple_structs.rs;l=41
  std::int32_t get_second_arg() const;

 public:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/tuple_structs/tuple_structs.rs;l=30
    std::int32_t __field0;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/tuple_structs/tuple_structs.rs;l=30
    std::int32_t __field1;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/structs/tuple_structs/tuple_structs.rs;l=47
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tuple_structs_golden :: TupleStructTwoPrivateArgs") alignas(4)
    [[clang::trivial_abi]] TupleStructTwoPrivateArgs final {
 public:
  // `TupleStructTwoPrivateArgs` doesn't implement the `Default` trait
  TupleStructTwoPrivateArgs() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~TupleStructTwoPrivateArgs() = default;
  TupleStructTwoPrivateArgs(TupleStructTwoPrivateArgs&&) = default;
  TupleStructTwoPrivateArgs& operator=(TupleStructTwoPrivateArgs&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  TupleStructTwoPrivateArgs(const TupleStructTwoPrivateArgs&) = default;
  TupleStructTwoPrivateArgs& operator=(const TupleStructTwoPrivateArgs&) =
      default;
  TupleStructTwoPrivateArgs(::crubit::UnsafeRelocateTag,
                            TupleStructTwoPrivateArgs&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/structs/tuple_structs/tuple_structs.rs;l=50
  static ::tuple_structs::TupleStructTwoPrivateArgs create(
      std::int32_t first_arg, std::int32_t second_arg);

  // Generated from:
  // cc_bindings_from_rs/test/structs/tuple_structs/tuple_structs.rs;l=54
  std::int32_t get_first_arg() const;

  // Generated from:
  // cc_bindings_from_rs/test/structs/tuple_structs/tuple_structs.rs;l=58
  std::int32_t get_second_arg() const;

 private:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/tuple_structs/tuple_structs.rs;l=47
    std::int32_t __field0;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/structs/tuple_structs/tuple_structs.rs;l=47
    std::int32_t __field1;
  };

 private:
  static void __crubit_field_offset_assertions();
};

static_assert(
    sizeof(TupleStructOnePublicArg) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(TupleStructOnePublicArg) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<TupleStructOnePublicArg>);
static_assert(std::is_trivially_move_constructible_v<TupleStructOnePublicArg>);
static_assert(std::is_trivially_move_assignable_v<TupleStructOnePublicArg>);
static_assert(std::is_trivially_copy_constructible_v<TupleStructOnePublicArg>);
static_assert(std::is_trivially_copy_assignable_v<TupleStructOnePublicArg>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_create(
    std::int32_t, ::tuple_structs::TupleStructOnePublicArg* __ret_ptr);
}
inline ::tuple_structs::TupleStructOnePublicArg TupleStructOnePublicArg::create(
    std::int32_t arg) {
  crubit::Slot<::tuple_structs::TupleStructOnePublicArg>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(arg, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_get_uarg(
    ::tuple_structs::TupleStructOnePublicArg*);
}
inline std::int32_t TupleStructOnePublicArg::get_arg() const {
  auto& self = const_cast<std::remove_cvref_t<decltype(*this)>&>(*this);
  return __crubit_internal::__crubit_thunk_get_uarg(&self);
}
inline void TupleStructOnePublicArg::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(TupleStructOnePublicArg, __field0));
}
static_assert(
    sizeof(TupleStructOnePrivateArg) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(TupleStructOnePrivateArg) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<TupleStructOnePrivateArg>);
static_assert(std::is_trivially_move_constructible_v<TupleStructOnePrivateArg>);
static_assert(std::is_trivially_move_assignable_v<TupleStructOnePrivateArg>);
static_assert(std::is_trivially_copy_constructible_v<TupleStructOnePrivateArg>);
static_assert(std::is_trivially_copy_assignable_v<TupleStructOnePrivateArg>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_create(
    std::int32_t, ::tuple_structs::TupleStructOnePrivateArg* __ret_ptr);
}
inline ::tuple_structs::TupleStructOnePrivateArg
TupleStructOnePrivateArg::create(std::int32_t arg) {
  crubit::Slot<::tuple_structs::TupleStructOnePrivateArg>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(arg, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_get_uarg(
    ::tuple_structs::TupleStructOnePrivateArg*);
}
inline std::int32_t TupleStructOnePrivateArg::get_arg() const {
  auto& self = const_cast<std::remove_cvref_t<decltype(*this)>&>(*this);
  return __crubit_internal::__crubit_thunk_get_uarg(&self);
}
inline void TupleStructOnePrivateArg::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(TupleStructOnePrivateArg, __field0));
}
static_assert(
    sizeof(TupleStructTwoPublicArgs) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(TupleStructTwoPublicArgs) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<TupleStructTwoPublicArgs>);
static_assert(std::is_trivially_move_constructible_v<TupleStructTwoPublicArgs>);
static_assert(std::is_trivially_move_assignable_v<TupleStructTwoPublicArgs>);
static_assert(std::is_trivially_copy_constructible_v<TupleStructTwoPublicArgs>);
static_assert(std::is_trivially_copy_assignable_v<TupleStructTwoPublicArgs>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_create(
    std::int32_t, std::int32_t,
    ::tuple_structs::TupleStructTwoPublicArgs* __ret_ptr);
}
inline ::tuple_structs::TupleStructTwoPublicArgs
TupleStructTwoPublicArgs::create(std::int32_t first_arg,
                                 std::int32_t second_arg) {
  crubit::Slot<::tuple_structs::TupleStructTwoPublicArgs>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(first_arg, second_arg,
                                           __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_get_ufirst_uarg(
    ::tuple_structs::TupleStructTwoPublicArgs*);
}
inline std::int32_t TupleStructTwoPublicArgs::get_first_arg() const {
  auto& self = const_cast<std::remove_cvref_t<decltype(*this)>&>(*this);
  return __crubit_internal::__crubit_thunk_get_ufirst_uarg(&self);
}

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_get_usecond_uarg(
    ::tuple_structs::TupleStructTwoPublicArgs*);
}
inline std::int32_t TupleStructTwoPublicArgs::get_second_arg() const {
  auto& self = const_cast<std::remove_cvref_t<decltype(*this)>&>(*this);
  return __crubit_internal::__crubit_thunk_get_usecond_uarg(&self);
}
inline void TupleStructTwoPublicArgs::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(TupleStructTwoPublicArgs, __field0));
  static_assert(4 == offsetof(TupleStructTwoPublicArgs, __field1));
}
static_assert(
    sizeof(TupleStructTwoPrivateArgs) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(TupleStructTwoPrivateArgs) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<TupleStructTwoPrivateArgs>);
static_assert(
    std::is_trivially_move_constructible_v<TupleStructTwoPrivateArgs>);
static_assert(std::is_trivially_move_assignable_v<TupleStructTwoPrivateArgs>);
static_assert(
    std::is_trivially_copy_constructible_v<TupleStructTwoPrivateArgs>);
static_assert(std::is_trivially_copy_assignable_v<TupleStructTwoPrivateArgs>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_create(
    std::int32_t, std::int32_t,
    ::tuple_structs::TupleStructTwoPrivateArgs* __ret_ptr);
}
inline ::tuple_structs::TupleStructTwoPrivateArgs
TupleStructTwoPrivateArgs::create(std::int32_t first_arg,
                                  std::int32_t second_arg) {
  crubit::Slot<::tuple_structs::TupleStructTwoPrivateArgs>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(first_arg, second_arg,
                                           __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_get_ufirst_uarg(
    ::tuple_structs::TupleStructTwoPrivateArgs*);
}
inline std::int32_t TupleStructTwoPrivateArgs::get_first_arg() const {
  auto& self = const_cast<std::remove_cvref_t<decltype(*this)>&>(*this);
  return __crubit_internal::__crubit_thunk_get_ufirst_uarg(&self);
}

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_get_usecond_uarg(
    ::tuple_structs::TupleStructTwoPrivateArgs*);
}
inline std::int32_t TupleStructTwoPrivateArgs::get_second_arg() const {
  auto& self = const_cast<std::remove_cvref_t<decltype(*this)>&>(*this);
  return __crubit_internal::__crubit_thunk_get_usecond_uarg(&self);
}
inline void TupleStructTwoPrivateArgs::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(TupleStructTwoPrivateArgs, __field0));
  static_assert(4 == offsetof(TupleStructTwoPrivateArgs, __field1));
}
}  // namespace tuple_structs
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_STRUCTS_TUPLE_STRUCTS_TUPLE_STRUCTS_GOLDEN
