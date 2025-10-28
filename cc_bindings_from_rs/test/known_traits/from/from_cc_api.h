// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// from_golden
// Features: non_unpin_ctor, std_unique_ptr, std_vector, supported

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_FROM_FROM_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_FROM_FROM_GOLDEN

#include "support/annotations_internal.h"
#include "support/internal/slot.h"
#include "support/rs_std/str_ref.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>
#include <utility>

namespace from {
struct OpaqueRef;
// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/known_traits/from/from.rs;l=11
struct CRUBIT_INTERNAL_RUST_TYPE(":: from_golden :: Opaque") alignas(4)
    [[clang::trivial_abi]] Opaque final {
 public:
  // `Opaque` doesn't implement the `Default` trait
  Opaque() = delete;

  // Synthesized tuple constructor
  explicit Opaque(std::int32_t __field0) : __field0(std::move(__field0)) {}

  // No custom `Drop` impl and no custom "drop glue" required
  ~Opaque() = default;
  Opaque(Opaque&&) = default;
  Opaque& operator=(Opaque&&) = default;

  // `Opaque` doesn't implement the `Clone` trait
  Opaque(const Opaque&) = delete;
  Opaque& operator=(const Opaque&) = delete;
  Opaque(::crubit::UnsafeRelocateTag, Opaque&& value) {
    memcpy(this, &value, sizeof(value));
  }
  // Generated from:
  // cc_bindings_from_rs/test/known_traits/from/from.rs;l=13
  explicit operator std::int32_t();

  // Generated from:
  // cc_bindings_from_rs/test/known_traits/from/from.rs;l=19
  explicit operator std::int64_t();

  // Generated from:
  // cc_bindings_from_rs/test/known_traits/from/from.rs;l=25
  explicit operator rs_std::StrRef();

  // Generated from:
  // cc_bindings_from_rs/test/known_traits/from/from.rs;l=32
  explicit operator std::int16_t();

  // Generated from:
  // cc_bindings_from_rs/test/known_traits/from/from.rs;l=38
  explicit operator ::from::OpaqueRef();

  union {
    // Generated from:
    // cc_bindings_from_rs/test/known_traits/from/from.rs;l=11
    std::int32_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/known_traits/from/from.rs;l=45
struct CRUBIT_INTERNAL_RUST_TYPE(":: from_golden :: OpaqueRef") alignas(8)
    [[clang::trivial_abi]] OpaqueRef final {
 public:
  // `OpaqueRef<'_>` doesn't implement the `Default` trait
  OpaqueRef() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~OpaqueRef() = default;
  OpaqueRef(OpaqueRef&&) = default;
  OpaqueRef& operator=(OpaqueRef&&) = default;

  // `OpaqueRef<'_>` doesn't implement the `Clone` trait
  OpaqueRef(const OpaqueRef&) = delete;
  OpaqueRef& operator=(const OpaqueRef&) = delete;
  OpaqueRef(::crubit::UnsafeRelocateTag, OpaqueRef&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // CRUBIT_ANNOTATE: must_bind=
  //
  // Generated from:
  // cc_bindings_from_rs/test/known_traits/from/from.rs;l=49
  static ::from::OpaqueRef create(rs_std::StrRef s);

  // CRUBIT_ANNOTATE: must_bind=
  //
  // Generated from:
  // cc_bindings_from_rs/test/known_traits/from/from.rs;l=54
  rs_std::StrRef get_arg() const;

  // Generated from:
  // cc_bindings_from_rs/test/known_traits/from/from.rs;l=59
  explicit operator rs_std::StrRef();

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
// cc_bindings_from_rs/test/known_traits/from/from.rs;l=67
struct CRUBIT_INTERNAL_RUST_TYPE(":: from_golden :: NotFfiSafe") alignas(8)
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

  // CRUBIT_ANNOTATE: must_bind=
  //
  // Generated from:
  // cc_bindings_from_rs/test/known_traits/from/from.rs;l=74
  static ::from::NotFfiSafe create();

  // Generated from:
  // cc_bindings_from_rs/test/known_traits/from/from.rs;l=78
  explicit operator std::int32_t();

 private:
  // Field type has been replaced with a blob of bytes: Function pointers can't
  // have a thunk: Any calling convention other than `extern "C"` requires a
  // thunk
  unsigned char __field0[8];

 private:
  static void __crubit_field_offset_assertions();
};

static_assert(
    sizeof(Opaque) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Opaque) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<Opaque>);
static_assert(std::is_trivially_move_constructible_v<Opaque>);
static_assert(std::is_trivially_move_assignable_v<Opaque>);
namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_into_ui32(::from::Opaque*);
}
inline Opaque::operator std::int32_t() {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_into_ui32(&self);
}
namespace __crubit_internal {
extern "C" std::int64_t __crubit_thunk_into_ui64(::from::Opaque*);
}
inline Opaque::operator std::int64_t() {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_into_ui64(&self);
}
namespace __crubit_internal {
extern "C" rs_std::StrRef
__crubit_thunk_into_u_x00000026_x00000027static_x00000020str(::from::Opaque*);
}
inline Opaque::operator rs_std::StrRef() {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_into_u_x00000026_x00000027static_x00000020str(&self);
}
namespace __crubit_internal {
extern "C" std::int16_t __crubit_thunk_into_ui16(::from::Opaque*);
}
inline Opaque::operator std::int16_t() {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_into_ui16(&self);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_into_uOpaqueRef_x0000003c_x00000027static_x0000003e(
    ::from::Opaque*, ::from::OpaqueRef* __ret_ptr);
}
inline Opaque::operator ::from::OpaqueRef() {
  auto&& self = *this;
  crubit::Slot<::from::OpaqueRef> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_into_uOpaqueRef_x0000003c_x00000027static_x0000003e(
          &self, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void Opaque::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Opaque, __field0));
}
static_assert(
    sizeof(OpaqueRef) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(OpaqueRef) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<OpaqueRef>);
static_assert(std::is_trivially_move_constructible_v<OpaqueRef>);
static_assert(std::is_trivially_move_assignable_v<OpaqueRef>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_create(rs_std::StrRef,
                                      ::from::OpaqueRef* __ret_ptr);
}
inline ::from::OpaqueRef OpaqueRef::create(rs_std::StrRef s) {
  crubit::Slot<::from::OpaqueRef> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(s, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" rs_std::StrRef __crubit_thunk_get_uarg(::from::OpaqueRef const&);
}
inline rs_std::StrRef OpaqueRef::get_arg() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_get_uarg(self);
}
namespace __crubit_internal {
extern "C" rs_std::StrRef
__crubit_thunk_into_u_x00000026_x00000027a_x00000020str(::from::OpaqueRef*);
}
inline OpaqueRef::operator rs_std::StrRef() {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_into_u_x00000026_x00000027a_x00000020str(&self);
}
inline void OpaqueRef::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(OpaqueRef, __field0));
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
namespace __crubit_internal {
extern "C" void __crubit_thunk_create(::from::NotFfiSafe* __ret_ptr);
}
inline ::from::NotFfiSafe NotFfiSafe::create() {
  crubit::Slot<::from::NotFfiSafe> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(__return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_into_ui32(::from::NotFfiSafe*);
}
inline NotFfiSafe::operator std::int32_t() {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_into_ui32(&self);
}
inline void NotFfiSafe::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NotFfiSafe, __field0));
}
}  // namespace from
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_FROM_FROM_GOLDEN
