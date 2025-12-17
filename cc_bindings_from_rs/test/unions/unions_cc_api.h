// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// unions_golden
// Features: non_unpin_ctor, std_unique_ptr, std_vector, supported

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_UNIONS_UNIONS_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_UNIONS_UNIONS_GOLDEN

#include "support/annotations_internal.h"
#include "support/internal/memswap.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>
#include <utility>

namespace unions {

namespace repr_c_packed {
union U;
}

namespace repr_c {
union U;
}

namespace repr_rust_packed {
union U;

// Generated from:
// cc_bindings_from_rs/test/unions/unions.rs;l=69
::unions::repr_rust_packed::U create();

}  // namespace repr_rust_packed

namespace repr_c_packed {

// Generated from:
// cc_bindings_from_rs/test/unions/unions.rs;l=56
::unions::repr_c_packed::U create();

// Generated from:
// cc_bindings_from_rs/test/unions/unions.rs;l=51
union CRUBIT_INTERNAL_RUST_TYPE(
    ":: unions_golden :: repr_c_packed :: U") alignas(1) [[clang::trivial_abi]]
__attribute__((packed)) U final {
 public:
  // `unions_golden::repr_c_packed::U` doesn't implement the `Default` trait
  U() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~U() = default;
  U(U&&) = default;
  U& operator=(U&&) = default;

  // `unions_golden::repr_c_packed::U` doesn't implement the `Clone` trait
  U(const U&) = delete;
  U& operator=(const U&) = delete;
  U(::crubit::UnsafeRelocateTag, U&& value) {
    memcpy(this, &value, sizeof(value));
  }
  // Generated from:
  // cc_bindings_from_rs/test/unions/unions.rs;l=52
  std::uint32_t x;
  // Generated from:
  // cc_bindings_from_rs/test/unions/unions.rs;l=53
  std::uint32_t y;

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace repr_c_packed

namespace repr_c_clone {

// Generated from:
// cc_bindings_from_rs/test/unions/unions.rs;l=77
union CRUBIT_INTERNAL_RUST_TYPE(
    ":: unions_golden :: repr_c_clone :: U") alignas(4) [[clang::trivial_abi]]
U final {
 public:
  // `unions_golden::repr_c_clone::U` doesn't implement the `Default` trait
  U() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~U() = default;
  U(U&&) = default;
  U& operator=(U&&) = default;

  // Clone::clone
  U(const U&);

  // Clone::clone_from
  U& operator=(const U&);

  U(::crubit::UnsafeRelocateTag, U&& value) {
    memcpy(this, &value, sizeof(value));
  }
  // Generated from:
  // cc_bindings_from_rs/test/unions/unions.rs;l=78
  std::uint32_t x;

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace repr_c_clone

namespace repr_rust_clone {

// Generated from:
// cc_bindings_from_rs/test/unions/unions.rs;l=94
union CRUBIT_INTERNAL_RUST_TYPE(
    ":: unions_golden :: repr_rust_clone :: U") alignas(4)
    [[clang::trivial_abi]] U final {
 public:
  // `unions_golden::repr_rust_clone::U` doesn't implement the `Default` trait
  U() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~U() = default;
  U(U&&) = default;
  U& operator=(U&&) = default;

  // Clone::clone
  U(const U&);

  // Clone::clone_from
  U& operator=(const U&);

  U(::crubit::UnsafeRelocateTag, U&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/unions/unions.rs;l=109
  void set_x(std::uint32_t x);

  // Generated from:
  // cc_bindings_from_rs/test/unions/unions.rs;l=112
  std::uint32_t get_x() const;

  // Generated from:
  // cc_bindings_from_rs/test/unions/unions.rs;l=95
  struct {
    std::uint32_t value;
  } x;

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace repr_rust_clone

namespace repr_rust_drop {

// Generated from:
// cc_bindings_from_rs/test/unions/unions.rs;l=141
union CRUBIT_INTERNAL_RUST_TYPE(
    ":: unions_golden :: repr_rust_drop :: U") alignas(8) [[clang::trivial_abi]]
U final {
 public:
  // Default::default
  U();

  // Drop::drop
  ~U();

  U(U&&);
  U& operator=(U&&);

  // `unions_golden::repr_rust_drop::U` doesn't implement the `Clone` trait
  U(const U&) = delete;
  U& operator=(const U&) = delete;
  U(::crubit::UnsafeRelocateTag, U&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/unions/unions.rs;l=158
  void set_x(std::int32_t* x);

  // Generated from:
  // cc_bindings_from_rs/test/unions/unions.rs;l=161
  std::int32_t* get_x() const;

  // Generated from:
  // cc_bindings_from_rs/test/unions/unions.rs;l=142
  struct {
    std::int32_t* value;
  } x;

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace repr_rust_drop

namespace repr_rust {

// Generated from:
// cc_bindings_from_rs/test/unions/unions.rs;l=23
union CRUBIT_INTERNAL_RUST_TYPE(":: unions_golden :: repr_rust :: U") alignas(4)
    [[clang::trivial_abi]] U final {
 public:
  // `unions_golden::repr_rust::U` doesn't implement the `Default` trait
  U() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~U() = default;
  U(U&&) = default;
  U& operator=(U&&) = default;

  // `unions_golden::repr_rust::U` doesn't implement the `Clone` trait
  U(const U&) = delete;
  U& operator=(const U&) = delete;
  U(::crubit::UnsafeRelocateTag, U&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/unions/unions.rs;l=33
  void set_x(std::uint32_t x);

  // Generated from:
  // cc_bindings_from_rs/test/unions/unions.rs;l=36
  std::uint32_t get_x() const;

  // Generated from:
  // cc_bindings_from_rs/test/unions/unions.rs;l=39
  void set_y(std::uint32_t y);

  // Generated from:
  // cc_bindings_from_rs/test/unions/unions.rs;l=42
  std::uint32_t get_y() const;

  // Generated from:
  // cc_bindings_from_rs/test/unions/unions.rs;l=24
  struct {
    std::uint32_t value;
  } x;
  // Generated from:
  // cc_bindings_from_rs/test/unions/unions.rs;l=25
  struct {
    std::uint32_t value;
  } y;

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace repr_rust

namespace repr_c {

// Generated from:
// cc_bindings_from_rs/test/unions/unions.rs;l=16
::unions::repr_c::U create();

}  // namespace repr_c

namespace repr_c_clone {

// Generated from:
// cc_bindings_from_rs/test/unions/unions.rs;l=87
::unions::repr_c_clone::U create();

}  // namespace repr_c_clone

namespace repr_c {

// Generated from:
// cc_bindings_from_rs/test/unions/unions.rs;l=11
union CRUBIT_INTERNAL_RUST_TYPE(":: unions_golden :: repr_c :: U") alignas(4)
    [[clang::trivial_abi]] U final {
 public:
  // `unions_golden::repr_c::U` doesn't implement the `Default` trait
  U() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~U() = default;
  U(U&&) = default;
  U& operator=(U&&) = default;

  // `unions_golden::repr_c::U` doesn't implement the `Clone` trait
  U(const U&) = delete;
  U& operator=(const U&) = delete;
  U(::crubit::UnsafeRelocateTag, U&& value) {
    memcpy(this, &value, sizeof(value));
  }
  // Generated from:
  // cc_bindings_from_rs/test/unions/unions.rs;l=12
  std::uint32_t x;
  // Generated from:
  // cc_bindings_from_rs/test/unions/unions.rs;l=13
  std::uint32_t y;

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace repr_c

namespace repr_rust_clone {

// Generated from:
// cc_bindings_from_rs/test/unions/unions.rs;l=104
::unions::repr_rust_clone::U create();

}  // namespace repr_rust_clone

namespace repr_c_drop {

// Generated from:
// cc_bindings_from_rs/test/unions/unions.rs;l=122
union CRUBIT_INTERNAL_RUST_TYPE(":: unions_golden :: repr_c_drop :: U") alignas(
    8) [[clang::trivial_abi]] U final {
 public:
  // Default::default
  U();

  // Drop::drop
  ~U();

  U(U&&);
  U& operator=(U&&);

  // `unions_golden::repr_c_drop::U` doesn't implement the `Clone` trait
  U(const U&) = delete;
  U& operator=(const U&) = delete;
  U(::crubit::UnsafeRelocateTag, U&& value) {
    memcpy(this, &value, sizeof(value));
  }
  // Generated from:
  // cc_bindings_from_rs/test/unions/unions.rs;l=123
  std::int32_t* x;

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace repr_c_drop

namespace repr_rust {

// Generated from:
// cc_bindings_from_rs/test/unions/unions.rs;l=28
::unions::repr_rust::U create();

}  // namespace repr_rust

namespace repr_rust_packed {

// Generated from:
// cc_bindings_from_rs/test/unions/unions.rs;l=64
union CRUBIT_INTERNAL_RUST_TYPE(
    ":: unions_golden :: repr_rust_packed :: U") alignas(1)
    [[clang::trivial_abi]] __attribute__((packed)) U final {
 public:
  // `unions_golden::repr_rust_packed::U` doesn't implement the `Default` trait
  U() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~U() = default;
  U(U&&) = default;
  U& operator=(U&&) = default;

  // `unions_golden::repr_rust_packed::U` doesn't implement the `Clone` trait
  U(const U&) = delete;
  U& operator=(const U&) = delete;
  U(::crubit::UnsafeRelocateTag, U&& value) {
    memcpy(this, &value, sizeof(value));
  }
  // Generated from:
  // cc_bindings_from_rs/test/unions/unions.rs;l=65
  struct {
    std::uint32_t value;
  } x;
  // Generated from:
  // cc_bindings_from_rs/test/unions/unions.rs;l=66
  struct {
    std::uint32_t value;
  } y;

 private:
  static void __crubit_field_offset_assertions();
};

namespace __crubit_internal {
extern "C" void __crubit_thunk_create(::unions::repr_rust_packed::U* __ret_ptr);
}
inline ::unions::repr_rust_packed::U create() {
  crubit::Slot<::unions::repr_rust_packed::U> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(__return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

}  // namespace repr_rust_packed

namespace repr_c_packed {

namespace __crubit_internal {
extern "C" void __crubit_thunk_create(::unions::repr_c_packed::U* __ret_ptr);
}
inline ::unions::repr_c_packed::U create() {
  crubit::Slot<::unions::repr_c_packed::U> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(__return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

static_assert(
    sizeof(U) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(U) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<U>);
static_assert(std::is_trivially_move_constructible_v<U>);
static_assert(std::is_trivially_move_assignable_v<U>);
inline void U::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(U, x));
  static_assert(0 == offsetof(U, y));
}
}  // namespace repr_c_packed

namespace repr_c_clone {

static_assert(
    sizeof(U) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(U) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<U>);
static_assert(std::is_trivially_move_constructible_v<U>);
static_assert(std::is_trivially_move_assignable_v<U>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone(::unions::repr_c_clone::U const&,
                                     ::unions::repr_c_clone::U* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone_ufrom(::unions::repr_c_clone::U&,
                                           ::unions::repr_c_clone::U const&);
}
inline U::U(const U& other) {
  __crubit_internal::__crubit_thunk_clone(other, this);
}
inline U& U::operator=(const U& other) {
  if (this != &other) {
    __crubit_internal::__crubit_thunk_clone_ufrom(*this, other);
  }
  return *this;
}
inline void U::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(U, x));
}
}  // namespace repr_c_clone

namespace repr_rust_clone {

static_assert(
    sizeof(U) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(U) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<U>);
static_assert(std::is_trivially_move_constructible_v<U>);
static_assert(std::is_trivially_move_assignable_v<U>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone(::unions::repr_rust_clone::U const&,
                                     ::unions::repr_rust_clone::U* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone_ufrom(::unions::repr_rust_clone::U&,
                                           ::unions::repr_rust_clone::U const&);
}
inline U::U(const U& other) {
  __crubit_internal::__crubit_thunk_clone(other, this);
}
inline U& U::operator=(const U& other) {
  if (this != &other) {
    __crubit_internal::__crubit_thunk_clone_ufrom(*this, other);
  }
  return *this;
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_set_ux(::unions::repr_rust_clone::U&,
                                      std::uint32_t);
}
inline void U::set_x(std::uint32_t x) {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_set_ux(self, x);
}

namespace __crubit_internal {
extern "C" std::uint32_t __crubit_thunk_get_ux(
    ::unions::repr_rust_clone::U const&);
}
inline std::uint32_t U::get_x() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_get_ux(self);
}
inline void U::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(U, x));
}
}  // namespace repr_rust_clone

namespace repr_rust_drop {

static_assert(
    sizeof(U) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(U) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(::unions::repr_rust_drop::U* __ret_ptr);
}
inline U::U() { __crubit_internal::__crubit_thunk_default(this); }
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(::unions::repr_rust_drop::U&);
}
inline U::~U() { __crubit_internal::__crubit_thunk_drop(*this); }
inline U::U(U&& other) : U() { *this = std::move(other); }
inline U& U::operator=(U&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_set_ux(::unions::repr_rust_drop::U&,
                                      std::int32_t*);
}
inline void U::set_x(std::int32_t* x) {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_set_ux(self, x);
}

namespace __crubit_internal {
extern "C" std::int32_t* __crubit_thunk_get_ux(
    ::unions::repr_rust_drop::U const&);
}
inline std::int32_t* U::get_x() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_get_ux(self);
}
inline void U::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(U, x));
}
}  // namespace repr_rust_drop

namespace repr_rust {

static_assert(
    sizeof(U) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(U) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<U>);
static_assert(std::is_trivially_move_constructible_v<U>);
static_assert(std::is_trivially_move_assignable_v<U>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_set_ux(::unions::repr_rust::U&, std::uint32_t);
}
inline void U::set_x(std::uint32_t x) {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_set_ux(self, x);
}

namespace __crubit_internal {
extern "C" std::uint32_t __crubit_thunk_get_ux(::unions::repr_rust::U const&);
}
inline std::uint32_t U::get_x() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_get_ux(self);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_set_uy(::unions::repr_rust::U&, std::uint32_t);
}
inline void U::set_y(std::uint32_t y) {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_set_uy(self, y);
}

namespace __crubit_internal {
extern "C" std::uint32_t __crubit_thunk_get_uy(::unions::repr_rust::U const&);
}
inline std::uint32_t U::get_y() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_get_uy(self);
}
inline void U::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(U, x));
  static_assert(0 == offsetof(U, y));
}
}  // namespace repr_rust

namespace repr_c {

namespace __crubit_internal {
extern "C" void __crubit_thunk_create(::unions::repr_c::U* __ret_ptr);
}
inline ::unions::repr_c::U create() {
  crubit::Slot<::unions::repr_c::U> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(__return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

}  // namespace repr_c

namespace repr_c_clone {

namespace __crubit_internal {
extern "C" void __crubit_thunk_create(::unions::repr_c_clone::U* __ret_ptr);
}
inline ::unions::repr_c_clone::U create() {
  crubit::Slot<::unions::repr_c_clone::U> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(__return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

}  // namespace repr_c_clone

namespace repr_c {

static_assert(
    sizeof(U) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(U) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<U>);
static_assert(std::is_trivially_move_constructible_v<U>);
static_assert(std::is_trivially_move_assignable_v<U>);
inline void U::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(U, x));
  static_assert(0 == offsetof(U, y));
}
}  // namespace repr_c

namespace repr_rust_clone {

namespace __crubit_internal {
extern "C" void __crubit_thunk_create(::unions::repr_rust_clone::U* __ret_ptr);
}
inline ::unions::repr_rust_clone::U create() {
  crubit::Slot<::unions::repr_rust_clone::U> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(__return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

}  // namespace repr_rust_clone

namespace repr_c_drop {

static_assert(
    sizeof(U) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(U) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(::unions::repr_c_drop::U* __ret_ptr);
}
inline U::U() { __crubit_internal::__crubit_thunk_default(this); }
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(::unions::repr_c_drop::U&);
}
inline U::~U() { __crubit_internal::__crubit_thunk_drop(*this); }
inline U::U(U&& other) : U() { *this = std::move(other); }
inline U& U::operator=(U&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
inline void U::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(U, x));
}
}  // namespace repr_c_drop

namespace repr_rust {

namespace __crubit_internal {
extern "C" void __crubit_thunk_create(::unions::repr_rust::U* __ret_ptr);
}
inline ::unions::repr_rust::U create() {
  crubit::Slot<::unions::repr_rust::U> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(__return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

}  // namespace repr_rust

namespace repr_rust_packed {

static_assert(
    sizeof(U) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(U) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<U>);
static_assert(std::is_trivially_move_constructible_v<U>);
static_assert(std::is_trivially_move_assignable_v<U>);
inline void U::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(U, x));
  static_assert(0 == offsetof(U, y));
}
}  // namespace repr_rust_packed

}  // namespace unions
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_UNIONS_UNIONS_GOLDEN
