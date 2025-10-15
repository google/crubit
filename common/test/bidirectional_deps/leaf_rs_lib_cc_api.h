// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// leaf_rs_lib_golden
// Features: do_not_hardcode_status_bridge, std_unique_ptr, std_vector,
// supported

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_COMMON_TEST_BIDIRECTIONAL_DEPS_LEAF_RS_LIB_GOLDEN
#define THIRD_PARTY_CRUBIT_COMMON_TEST_BIDIRECTIONAL_DEPS_LEAF_RS_LIB_GOLDEN

#include "support/annotations_internal.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>
#include <utility>

namespace leaf_rs_lib {

// Generated from:
// common/test/bidirectional_deps/leaf_rs_lib.rs;l=7
struct CRUBIT_INTERNAL_RUST_TYPE(":: leaf_rs_lib_golden :: LeafRsType") alignas(
    1) [[clang::trivial_abi]] LeafRsType final {
 public:
  // Default::default
  LeafRsType();

  // No custom `Drop` impl and no custom "drop glue" required
  ~LeafRsType() = default;
  LeafRsType(LeafRsType&&) = default;
  LeafRsType& operator=(LeafRsType&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  LeafRsType(const LeafRsType&) = default;
  LeafRsType& operator=(const LeafRsType&) = default;
  LeafRsType(::crubit::UnsafeRelocateTag, LeafRsType&& value) {
    memcpy(this, &value, sizeof(value));
  }

 private:
  union {
    // Generated from:
    // common/test/bidirectional_deps/leaf_rs_lib.rs;l=8
    std::uint8_t field;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// common/test/bidirectional_deps/leaf_rs_lib.rs;l=11
::leaf_rs_lib::LeafRsType wrap(std::uint8_t x);

// Generated from:
// common/test/bidirectional_deps/leaf_rs_lib.rs;l=15
std::uint8_t unwrap(::leaf_rs_lib::LeafRsType x);

// Generated from:
// common/test/bidirectional_deps/leaf_rs_lib.rs;l=21
struct CRUBIT_INTERNAL_RUST_TYPE(":: leaf_rs_lib_golden :: LeafRsEnum") alignas(
    1) [[clang::trivial_abi]] LeafRsEnum final {
 public:
  // `LeafRsEnum` doesn't implement the `Default` trait
  LeafRsEnum() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~LeafRsEnum() = default;
  LeafRsEnum(LeafRsEnum&&) = default;
  LeafRsEnum& operator=(LeafRsEnum&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  LeafRsEnum(const LeafRsEnum&) = default;
  LeafRsEnum& operator=(const LeafRsEnum&) = default;
  LeafRsEnum(::crubit::UnsafeRelocateTag, LeafRsEnum&& value) {
    memcpy(this, &value, sizeof(value));
  }

 private:
  // Field type has been replaced with a blob of bytes: No support for bindings
  // of individual non-repr(C) `enum`s
  unsigned char __opaque_blob_of_bytes[1];

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// common/test/bidirectional_deps/leaf_rs_lib.rs;l=28
::leaf_rs_lib::LeafRsEnum wrap_enum(std::uint8_t x);

// Generated from:
// common/test/bidirectional_deps/leaf_rs_lib.rs;l=37
std::uint8_t unwrap_enum(::leaf_rs_lib::LeafRsEnum x);
using LeafRsTypeAlias CRUBIT_INTERNAL_RUST_TYPE(
    ":: leaf_rs_lib_golden :: LeafRsType") = ::leaf_rs_lib::LeafRsType;
static_assert(
    sizeof(LeafRsType) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(LeafRsType) == 1,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(::leaf_rs_lib::LeafRsType* __ret_ptr);
}
inline LeafRsType::LeafRsType() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(std::is_trivially_destructible_v<LeafRsType>);
static_assert(std::is_trivially_move_constructible_v<LeafRsType>);
static_assert(std::is_trivially_move_assignable_v<LeafRsType>);
static_assert(std::is_trivially_copy_constructible_v<LeafRsType>);
static_assert(std::is_trivially_copy_assignable_v<LeafRsType>);
inline void LeafRsType::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(LeafRsType, field));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_wrap(std::uint8_t,
                                    ::leaf_rs_lib::LeafRsType* __ret_ptr);
}
inline ::leaf_rs_lib::LeafRsType wrap(std::uint8_t x) {
  crubit::Slot<::leaf_rs_lib::LeafRsType> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_wrap(x, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" std::uint8_t __crubit_thunk_unwrap(::leaf_rs_lib::LeafRsType*);
}
inline std::uint8_t unwrap(::leaf_rs_lib::LeafRsType x) {
  return __crubit_internal::__crubit_thunk_unwrap(&x);
}

static_assert(
    sizeof(LeafRsEnum) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(LeafRsEnum) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<LeafRsEnum>);
static_assert(std::is_trivially_move_constructible_v<LeafRsEnum>);
static_assert(std::is_trivially_move_assignable_v<LeafRsEnum>);
static_assert(std::is_trivially_copy_constructible_v<LeafRsEnum>);
static_assert(std::is_trivially_copy_assignable_v<LeafRsEnum>);
inline void LeafRsEnum::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(LeafRsEnum, __opaque_blob_of_bytes));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_wrap_uenum(std::uint8_t,
                                          ::leaf_rs_lib::LeafRsEnum* __ret_ptr);
}
inline ::leaf_rs_lib::LeafRsEnum wrap_enum(std::uint8_t x) {
  crubit::Slot<::leaf_rs_lib::LeafRsEnum> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_wrap_uenum(x, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" std::uint8_t __crubit_thunk_unwrap_uenum(::leaf_rs_lib::LeafRsEnum*);
}
inline std::uint8_t unwrap_enum(::leaf_rs_lib::LeafRsEnum x) {
  return __crubit_internal::__crubit_thunk_unwrap_uenum(&x);
}

}  // namespace leaf_rs_lib
#endif  // THIRD_PARTY_CRUBIT_COMMON_TEST_BIDIRECTIONAL_DEPS_LEAF_RS_LIB_GOLDEN
