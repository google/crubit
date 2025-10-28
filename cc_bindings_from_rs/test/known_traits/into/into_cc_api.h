// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// into_golden
// Features: non_unpin_ctor, std_unique_ptr, std_vector, supported

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_INTO_INTO_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_INTO_INTO_GOLDEN

#include "support/annotations_internal.h"
#include "support/internal/slot.h"
#include "support/rs_std/str_ref.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>
#include <utility>

namespace into {

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/known_traits/into/into.rs;l=14
struct CRUBIT_INTERNAL_RUST_TYPE(":: into_golden :: Convert") alignas(4)
    [[clang::trivial_abi]] Convert final {
 public:
  // `Convert` doesn't implement the `Default` trait
  Convert() = delete;

  // Synthesized tuple constructor
  explicit Convert(std::int32_t __field0) : __field0(std::move(__field0)) {}

  // No custom `Drop` impl and no custom "drop glue" required
  ~Convert() = default;
  Convert(Convert&&) = default;
  Convert& operator=(Convert&&) = default;

  // `Convert` doesn't implement the `Clone` trait
  Convert(const Convert&) = delete;
  Convert& operator=(const Convert&) = delete;
  Convert(::crubit::UnsafeRelocateTag, Convert&& value) {
    memcpy(this, &value, sizeof(value));
  }
  // Generated from:
  // cc_bindings_from_rs/test/known_traits/into/into.rs;l=16
  explicit operator std::int32_t();

  // Generated from:
  // cc_bindings_from_rs/test/known_traits/into/into.rs;l=23
  explicit operator std::int64_t();

  // Generated from:
  // cc_bindings_from_rs/test/known_traits/into/into.rs;l=30
  explicit operator rs_std::StrRef();

  // Generated from:
  // cc_bindings_from_rs/test/known_traits/into/into.rs;l=38
  explicit operator std::int16_t();

  union {
    // Generated from:
    // cc_bindings_from_rs/test/known_traits/into/into.rs;l=14
    std::int32_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/known_traits/into/into.rs;l=46
struct CRUBIT_INTERNAL_RUST_TYPE(":: into_golden :: ConvertRef") alignas(8)
    [[clang::trivial_abi]] ConvertRef final {
 public:
  // `ConvertRef<'_>` doesn't implement the `Default` trait
  ConvertRef() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~ConvertRef() = default;
  ConvertRef(ConvertRef&&) = default;
  ConvertRef& operator=(ConvertRef&&) = default;

  // `ConvertRef<'_>` doesn't implement the `Clone` trait
  ConvertRef(const ConvertRef&) = delete;
  ConvertRef& operator=(const ConvertRef&) = delete;
  ConvertRef(::crubit::UnsafeRelocateTag, ConvertRef&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  //
  // Generated from:
  // cc_bindings_from_rs/test/known_traits/into/into.rs;l=50
  static ::into::ConvertRef create(rs_std::StrRef s);

  // CRUBIT_ANNOTATE: must_bind=
  //
  // Generated from:
  // cc_bindings_from_rs/test/known_traits/into/into.rs;l=55
  ::into::Convert transmigrate() &&;

  // Generated from:
  // cc_bindings_from_rs/test/known_traits/into/into.rs;l=60
  explicit operator rs_std::StrRef();

  // Generated from:
  // cc_bindings_from_rs/test/known_traits/into/into.rs;l=67
  explicit operator ::into::Convert();

 private:
  // Field type has been replaced with a blob of bytes: Can't format `&str`,
  // because references are only supported in function parameter types, return
  // types, and consts (b/286256327)
  unsigned char __field0[16];

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/known_traits/into/into.rs;l=76
struct CRUBIT_INTERNAL_RUST_TYPE(":: into_golden :: NotFfiSafe") alignas(8)
    [[clang::trivial_abi]] NotFfiSafe final {
 public:
  // `NotFfiSafe` doesn't implement the `Default` trait
  NotFfiSafe() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~NotFfiSafe() = default;
  NotFfiSafe(NotFfiSafe&&) = default;
  NotFfiSafe& operator=(NotFfiSafe&&) = default;

  // `NotFfiSafe` doesn't implement the `Clone` trait
  NotFfiSafe(const NotFfiSafe&) = delete;
  NotFfiSafe& operator=(const NotFfiSafe&) = delete;
  NotFfiSafe(::crubit::UnsafeRelocateTag, NotFfiSafe&& value) {
    memcpy(this, &value, sizeof(value));
  }

 private:
  // Field type has been replaced with a blob of bytes: Function pointers can't
  // have a thunk: Any calling convention other than `extern "C"` requires a
  // thunk
  unsigned char __field0[8];

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/known_traits/into/into.rs;l=85
struct CRUBIT_INTERNAL_RUST_TYPE(":: into_golden :: ConvertModule") alignas(4)
    [[clang::trivial_abi]] ConvertModule final {
 public:
  // `ConvertModule` doesn't implement the `Default` trait
  ConvertModule() = delete;

  // Synthesized tuple constructor
  explicit ConvertModule(std::int32_t __field0)
      : __field0(std::move(__field0)) {}

  // No custom `Drop` impl and no custom "drop glue" required
  ~ConvertModule() = default;
  ConvertModule(ConvertModule&&) = default;
  ConvertModule& operator=(ConvertModule&&) = default;

  // `ConvertModule` doesn't implement the `Clone` trait
  ConvertModule(const ConvertModule&) = delete;
  ConvertModule& operator=(const ConvertModule&) = delete;
  ConvertModule(::crubit::UnsafeRelocateTag, ConvertModule&& value) {
    memcpy(this, &value, sizeof(value));
  }
  // Generated from:
  // cc_bindings_from_rs/test/known_traits/into/into.rs;l=91
  explicit operator std::int32_t();

  // Generated from:
  // cc_bindings_from_rs/test/known_traits/into/into.rs;l=103
  explicit operator std::int64_t();

  union {
    // Generated from:
    // cc_bindings_from_rs/test/known_traits/into/into.rs;l=85
    std::int32_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

static_assert(
    sizeof(Convert) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Convert) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<Convert>);
static_assert(std::is_trivially_move_constructible_v<Convert>);
static_assert(std::is_trivially_move_assignable_v<Convert>);
namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_into_ui32(::into::Convert*);
}
inline Convert::operator std::int32_t() {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_into_ui32(&self);
}
namespace __crubit_internal {
extern "C" std::int64_t __crubit_thunk_into_ui64(::into::Convert*);
}
inline Convert::operator std::int64_t() {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_into_ui64(&self);
}
namespace __crubit_internal {
extern "C" rs_std::StrRef
__crubit_thunk_into_u_x00000026_x00000027static_x00000020str(::into::Convert*);
}
inline Convert::operator rs_std::StrRef() {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_into_u_x00000026_x00000027static_x00000020str(&self);
}
namespace __crubit_internal {
extern "C" std::int16_t __crubit_thunk_into_ui16(::into::Convert*);
}
inline Convert::operator std::int16_t() {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_into_ui16(&self);
}
inline void Convert::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Convert, __field0));
}
static_assert(
    sizeof(ConvertRef) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(ConvertRef) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<ConvertRef>);
static_assert(std::is_trivially_move_constructible_v<ConvertRef>);
static_assert(std::is_trivially_move_assignable_v<ConvertRef>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_create(rs_std::StrRef,
                                      ::into::ConvertRef* __ret_ptr);
}
inline ::into::ConvertRef ConvertRef::create(rs_std::StrRef s) {
  crubit::Slot<::into::ConvertRef> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(s, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_transmigrate(::into::ConvertRef*,
                                            ::into::Convert* __ret_ptr);
}
inline ::into::Convert ConvertRef::transmigrate() && {
  auto&& self = *this;
  crubit::Slot<::into::Convert> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_transmigrate(&self, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
namespace __crubit_internal {
extern "C" rs_std::StrRef
__crubit_thunk_into_u_x00000026_x00000027a_x00000020str(::into::ConvertRef*);
}
inline ConvertRef::operator rs_std::StrRef() {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_into_u_x00000026_x00000027a_x00000020str(&self);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_into_uConvert(::into::ConvertRef*,
                                             ::into::Convert* __ret_ptr);
}
inline ConvertRef::operator ::into::Convert() {
  auto&& self = *this;
  crubit::Slot<::into::Convert> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_into_uConvert(&self,
                                                  __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void ConvertRef::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(ConvertRef, __field0));
}
static_assert(
    sizeof(NotFfiSafe) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NotFfiSafe) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<NotFfiSafe>);
static_assert(std::is_trivially_move_constructible_v<NotFfiSafe>);
static_assert(std::is_trivially_move_assignable_v<NotFfiSafe>);
inline void NotFfiSafe::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NotFfiSafe, __field0));
}
static_assert(
    sizeof(ConvertModule) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(ConvertModule) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<ConvertModule>);
static_assert(std::is_trivially_move_constructible_v<ConvertModule>);
static_assert(std::is_trivially_move_assignable_v<ConvertModule>);
namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_into_ui32(::into::ConvertModule*);
}
inline ConvertModule::operator std::int32_t() {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_into_ui32(&self);
}
namespace __crubit_internal {
extern "C" std::int64_t __crubit_thunk_into_ui64(::into::ConvertModule*);
}
inline ConvertModule::operator std::int64_t() {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_into_ui64(&self);
}
inline void ConvertModule::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(ConvertModule, __field0));
}
}  // namespace into
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_INTO_INTO_GOLDEN
