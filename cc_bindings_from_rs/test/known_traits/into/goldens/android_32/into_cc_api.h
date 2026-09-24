// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// into_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_INTO_INTO_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_INTO_INTO_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/slot.h"
#include "support/lifetime_annotations.h"
#include "support/rs_std/str_ref.h"

#include <array>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <type_traits>
#include <utility>

#include "support/rs_std/rs_alloc.h"

namespace into {
struct LoopB;
// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: into_golden :: CloneAllocTarget") alignas(
    4) [[clang::trivial_abi]] CloneAllocTarget final {
 public:
  // CRUBIT_ANNOTATE: must_bind=
  rs_std::StrRef get_value() const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  ::rs::alloc::string::String value{};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: into_golden :: CloneAllocType") alignas(4)
    [[clang::trivial_abi]] CloneAllocType final {
 public:
  // CRUBIT_ANNOTATE: must_bind=
  static ::into::CloneAllocType create(rs_std::StrRef s);

  // CRUBIT_ANNOTATE: must_bind=
  rs_std::StrRef get_value() const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  explicit operator ::into::CloneAllocTarget();

  ::rs::alloc::string::String value{};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: into_golden :: CloneCopyTarget") alignas(4)
    [[clang::trivial_abi]] CloneCopyTarget final {
 public:
  ::std::int32_t __field0{};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: into_golden :: CloneCopyType") alignas(4)
    [[clang::trivial_abi]] CloneCopyType final {
 public:
  explicit operator ::into::CloneCopyTarget() const;

  ::std::int32_t __field0{};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: into_golden :: CollidingOperators") alignas(4) [[clang::trivial_abi]]
CollidingOperators final {
 public:
  // Error generating bindings for implementation
  // `<into_golden::CollidingOperators as std::convert::Into<u64>>` defined at
  // cc_bindings_from_rs/test/known_traits/into/into.rs;l=215:
  // Conversion to `u64` is not supported when conversion to `usize` is
  // implemented as they may overlap in C++.

  explicit operator ::std::uintptr_t();

  ::std::uint64_t __field0{};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: into_golden :: Convert") alignas(4)
    [[clang::trivial_abi]] Convert final {
 public:
  explicit operator ::std::int32_t();

  explicit operator ::std::int64_t();

  explicit operator rs_std::StrRef();

  explicit operator ::std::int16_t();

  ::std::int32_t __field0{};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: into_golden :: ConvertModule") alignas(4)
    [[clang::trivial_abi]] ConvertModule final {
 public:
  explicit operator ::std::int32_t();

  explicit operator ::std::int64_t();

  ::std::int32_t __field0{};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: into_golden :: ConvertRef") alignas(4)
    [[clang::trivial_abi]] ConvertRef final {
 public:
  // `into_golden::ConvertRef` doesn't implement the `Default` trait
  ConvertRef() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~ConvertRef() = default;
  ConvertRef(ConvertRef&&) = default;
  ConvertRef& operator=(ConvertRef&&) = default;

  // `into_golden::ConvertRef` doesn't implement the `Clone` trait
  ConvertRef(const ConvertRef&) = delete;
  ConvertRef& operator=(const ConvertRef&) = delete;
  ConvertRef(::crubit::UnsafeRelocateTag, ConvertRef&& value);

  // CRUBIT_ANNOTATE: must_bind=
  static ::into::ConvertRef create(rs_std::StrRef s);

  // CRUBIT_ANNOTATE: must_bind=
  ::into::Convert transmigrate() &&;

  explicit operator rs_std::StrRef();

  explicit operator ::into::Convert();

 private:
  union {
    rs_std::StrRef __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: into_golden :: LoopA") alignas(4)
    [[clang::trivial_abi]] LoopA final {
 public:
  explicit operator ::into::LoopB();

  ::std::int32_t __field0{};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: into_golden :: LoopB") alignas(4)
    [[clang::trivial_abi]] LoopB final {
 public:
  explicit operator ::into::LoopA();

  ::std::int32_t __field0{};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: into_golden :: NoCloneCopyDropTarget") alignas(4) [[clang::trivial_abi]]
NoCloneCopyDropTarget final {
 public:
  ::std::int32_t __field0{};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: into_golden :: NoCloneCopyDropType") alignas(4) [[clang::trivial_abi]]
NoCloneCopyDropType final {
 public:
  explicit operator ::into::NoCloneCopyDropTarget();

  ::std::int32_t __field0{};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: into_golden :: NoCloneDefaultTarget") alignas(4) [[clang::trivial_abi]]
NoCloneDefaultTarget final {
 public:
  ::std::int32_t __field0{};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: into_golden :: NoCloneDefaultType") alignas(4) [[clang::trivial_abi]]
NoCloneDefaultType final {
 public:
  explicit operator ::into::NoCloneDefaultTarget();

  ::std::int32_t __field0{};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: into_golden :: NotFfiSafe") alignas(4)
    [[clang::trivial_abi]] NotFfiSafe final {
 public:
  // `into_golden::NotFfiSafe` doesn't implement the `Default` trait
  NotFfiSafe() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~NotFfiSafe() = default;
  NotFfiSafe(NotFfiSafe&&) = default;
  NotFfiSafe& operator=(NotFfiSafe&&) = default;

  // `into_golden::NotFfiSafe` doesn't implement the `Clone` trait
  NotFfiSafe(const NotFfiSafe&) = delete;
  NotFfiSafe& operator=(const NotFfiSafe&) = delete;
  NotFfiSafe(::crubit::UnsafeRelocateTag, NotFfiSafe&& value);

 private:
  // Field type has been replaced with a blob of bytes: Function pointers can't
  // have a thunk: Any calling convention other than `extern "C"` requires a
  // thunk
  ::std::array<unsigned char, 4> __field0;

 private:
  static void __crubit_field_offset_assertions();
};

static_assert(
    sizeof(::into::CloneAllocTarget) == 12,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::into::CloneAllocTarget) == 4,
    "Verify that ADT layout didn't change since this header got generated");

namespace __crubit_internal {
extern "C" void __crubit_thunk_get_uvalue(::into::CloneAllocTarget const&,
                                          rs_std::StrRef* __ret_ptr);
}
inline rs_std::StrRef(CloneAllocTarget::get_value)() const& $(__anon1)
    CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  crubit::Slot<rs_std::StrRef> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_get_uvalue(self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void ::into::CloneAllocTarget::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::into::CloneAllocTarget;
  static_assert(0 == offsetof(__crubit_assert_type, value));
}
static_assert(
    sizeof(::into::CloneAllocType) == 12,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::into::CloneAllocType) == 4,
    "Verify that ADT layout didn't change since this header got generated");

namespace __crubit_internal {
extern "C" void __crubit_thunk_create(rs_std::StrRef*,
                                      ::into::CloneAllocType* __ret_ptr);
}
inline ::into::CloneAllocType(CloneAllocType::create)(rs_std::StrRef s) {
  crubit::Slot<::into::CloneAllocType> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(&s, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_get_uvalue(::into::CloneAllocType const&,
                                          rs_std::StrRef* __ret_ptr);
}
inline rs_std::StrRef(CloneAllocType::get_value)() const& $(__anon1)
    CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  crubit::Slot<rs_std::StrRef> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_get_uvalue(self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aCloneAllocType_uinto_ugolden_x0000003a_x0000003aCloneAllocTarget(
    ::into::CloneAllocType*, ::into::CloneAllocTarget* __ret_ptr);
}
inline ::into::CloneAllocType::operator ::into::CloneAllocTarget() {
  auto&& self = *this;
  crubit::Slot self_slot((::std::move(self)));
  crubit::Slot<::into::CloneAllocTarget> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aCloneAllocType_uinto_ugolden_x0000003a_x0000003aCloneAllocTarget(
          self_slot.Get(), __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void ::into::CloneAllocType::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::into::CloneAllocType;
  static_assert(0 == offsetof(__crubit_assert_type, value));
}
static_assert(
    sizeof(::into::CloneCopyTarget) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::into::CloneCopyTarget) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<CloneCopyTarget>);
static_assert(
    ::std::is_trivially_move_constructible_v<::into::CloneCopyTarget>);
static_assert(::std::is_trivially_move_assignable_v<::into::CloneCopyTarget>);
static_assert(
    ::std::is_trivially_copy_constructible_v<::into::CloneCopyTarget>);
static_assert(::std::is_trivially_copy_assignable_v<::into::CloneCopyTarget>);
inline void ::into::CloneCopyTarget::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::into::CloneCopyTarget;
  static_assert(0 == offsetof(__crubit_assert_type, __field0));
}
static_assert(
    sizeof(::into::CloneCopyType) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::into::CloneCopyType) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<CloneCopyType>);
static_assert(::std::is_trivially_move_constructible_v<::into::CloneCopyType>);
static_assert(::std::is_trivially_move_assignable_v<::into::CloneCopyType>);
static_assert(::std::is_trivially_copy_constructible_v<::into::CloneCopyType>);
static_assert(::std::is_trivially_copy_assignable_v<::into::CloneCopyType>);
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aCloneCopyType_uinto_ugolden_x0000003a_x0000003aCloneCopyTarget(
    ::into::CloneCopyType*, ::into::CloneCopyTarget* __ret_ptr);
}
inline ::into::CloneCopyType::operator ::into::CloneCopyTarget() const {
  auto& self = const_cast<::std::remove_cvref_t<decltype(*this)>&>(*this);
  crubit::Slot<::into::CloneCopyTarget> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aCloneCopyType_uinto_ugolden_x0000003a_x0000003aCloneCopyTarget(
          &self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void ::into::CloneCopyType::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::into::CloneCopyType;
  static_assert(0 == offsetof(__crubit_assert_type, __field0));
}
static_assert(
    sizeof(::into::CollidingOperators) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::into::CollidingOperators) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<CollidingOperators>);
static_assert(
    ::std::is_trivially_move_constructible_v<::into::CollidingOperators>);
static_assert(
    ::std::is_trivially_move_assignable_v<::into::CollidingOperators>);
namespace __crubit_internal {
extern "C" ::std::uintptr_t
__crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aCollidingOperators_uusize(
    ::into::CollidingOperators*);
}
inline ::into::CollidingOperators::operator ::std::uintptr_t() {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aCollidingOperators_uusize(
          &self);
}
inline void ::into::CollidingOperators::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::into::CollidingOperators;
  static_assert(0 == offsetof(__crubit_assert_type, __field0));
}
static_assert(
    sizeof(::into::Convert) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::into::Convert) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<Convert>);
static_assert(::std::is_trivially_move_constructible_v<::into::Convert>);
static_assert(::std::is_trivially_move_assignable_v<::into::Convert>);
namespace __crubit_internal {
extern "C" ::std::int32_t
__crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aConvert_ui32(
    ::into::Convert*);
}
inline ::into::Convert::operator ::std::int32_t() {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aConvert_ui32(
          &self);
}
namespace __crubit_internal {
extern "C" ::std::int64_t
__crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aConvert_ui64(
    ::into::Convert*);
}
inline ::into::Convert::operator ::std::int64_t() {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aConvert_ui64(
          &self);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aConvert_u_x00000026_x00000027static_x00000020str(
    ::into::Convert*, rs_std::StrRef* __ret_ptr);
}
inline ::into::Convert::operator rs_std::StrRef() {
  auto&& self = *this;
  crubit::Slot<rs_std::StrRef> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aConvert_u_x00000026_x00000027static_x00000020str(
          &self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
namespace __crubit_internal {
extern "C" ::std::int16_t
__crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aConvert_ui16(
    ::into::Convert*);
}
inline ::into::Convert::operator ::std::int16_t() {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aConvert_ui16(
          &self);
}
inline void ::into::Convert::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::into::Convert;
  static_assert(0 == offsetof(__crubit_assert_type, __field0));
}
static_assert(
    sizeof(::into::ConvertModule) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::into::ConvertModule) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<ConvertModule>);
static_assert(::std::is_trivially_move_constructible_v<::into::ConvertModule>);
static_assert(::std::is_trivially_move_assignable_v<::into::ConvertModule>);
namespace __crubit_internal {
extern "C" ::std::int32_t
__crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aConvertModule_ui32(
    ::into::ConvertModule*);
}
inline ::into::ConvertModule::operator ::std::int32_t() {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aConvertModule_ui32(
          &self);
}
namespace __crubit_internal {
extern "C" ::std::int64_t
__crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aConvertModule_ui64(
    ::into::ConvertModule*);
}
inline ::into::ConvertModule::operator ::std::int64_t() {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aConvertModule_ui64(
          &self);
}
inline void ::into::ConvertModule::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::into::ConvertModule;
  static_assert(0 == offsetof(__crubit_assert_type, __field0));
}
static_assert(
    sizeof(::into::ConvertRef) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::into::ConvertRef) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<::into::ConvertRef>);
static_assert(::std::is_trivially_move_constructible_v<::into::ConvertRef>);
static_assert(::std::is_trivially_move_assignable_v<::into::ConvertRef>);
inline ::into::ConvertRef::ConvertRef(::crubit::UnsafeRelocateTag,
                                      ConvertRef&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_create_u_x00000027a(
    rs_std::StrRef*, ::into::ConvertRef* __ret_ptr);
}
inline ::into::ConvertRef(ConvertRef::create)(rs_std::StrRef s) {
  crubit::Slot<::into::ConvertRef> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create_u_x00000027a(&s,
                                                        __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_transmigrate_u_x00000027a(
    ::into::ConvertRef*, ::into::Convert* __ret_ptr);
}
inline ::into::Convert(ConvertRef::transmigrate)() && {
  auto&& self = *this;
  crubit::Slot<::into::Convert> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_transmigrate_u_x00000027a(
      &self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aConvertRef_x0000003c_x00000027_u_x0000003e_u_x00000026_x00000027a_x00000020str(
    ::into::ConvertRef*, rs_std::StrRef* __ret_ptr);
}
inline ::into::ConvertRef::operator rs_std::StrRef() {
  auto&& self = *this;
  crubit::Slot<rs_std::StrRef> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aConvertRef_x0000003c_x00000027_u_x0000003e_u_x00000026_x00000027a_x00000020str(
          &self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aConvertRef_x0000003c_x00000027_u_x0000003e_uinto_ugolden_x0000003a_x0000003aConvert(
    ::into::ConvertRef*, ::into::Convert* __ret_ptr);
}
inline ::into::ConvertRef::operator ::into::Convert() {
  auto&& self = *this;
  crubit::Slot<::into::Convert> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aConvertRef_x0000003c_x00000027_u_x0000003e_uinto_ugolden_x0000003a_x0000003aConvert(
          &self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void ::into::ConvertRef::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::into::ConvertRef;
  static_assert(0 == offsetof(__crubit_assert_type, __field0));
}
static_assert(
    sizeof(::into::LoopA) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::into::LoopA) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<LoopA>);
static_assert(::std::is_trivially_move_constructible_v<::into::LoopA>);
static_assert(::std::is_trivially_move_assignable_v<::into::LoopA>);
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aLoopA_uinto_ugolden_x0000003a_x0000003aLoopB(
    ::into::LoopA*, ::into::LoopB* __ret_ptr);
}
inline ::into::LoopA::operator ::into::LoopB() {
  auto&& self = *this;
  crubit::Slot<::into::LoopB> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aLoopA_uinto_ugolden_x0000003a_x0000003aLoopB(
          &self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void ::into::LoopA::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::into::LoopA;
  static_assert(0 == offsetof(__crubit_assert_type, __field0));
}
static_assert(
    sizeof(::into::LoopB) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::into::LoopB) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<LoopB>);
static_assert(::std::is_trivially_move_constructible_v<::into::LoopB>);
static_assert(::std::is_trivially_move_assignable_v<::into::LoopB>);
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aLoopB_uinto_ugolden_x0000003a_x0000003aLoopA(
    ::into::LoopB*, ::into::LoopA* __ret_ptr);
}
inline ::into::LoopB::operator ::into::LoopA() {
  auto&& self = *this;
  crubit::Slot<::into::LoopA> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aLoopB_uinto_ugolden_x0000003a_x0000003aLoopA(
          &self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void ::into::LoopB::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::into::LoopB;
  static_assert(0 == offsetof(__crubit_assert_type, __field0));
}
static_assert(
    sizeof(::into::NoCloneCopyDropTarget) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::into::NoCloneCopyDropTarget) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<NoCloneCopyDropTarget>);
static_assert(
    ::std::is_trivially_move_constructible_v<::into::NoCloneCopyDropTarget>);
static_assert(
    ::std::is_trivially_move_assignable_v<::into::NoCloneCopyDropTarget>);
inline void ::into::NoCloneCopyDropTarget::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::into::NoCloneCopyDropTarget;
  static_assert(0 == offsetof(__crubit_assert_type, __field0));
}
static_assert(
    sizeof(::into::NoCloneCopyDropType) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::into::NoCloneCopyDropType) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<NoCloneCopyDropType>);
static_assert(
    ::std::is_trivially_move_constructible_v<::into::NoCloneCopyDropType>);
static_assert(
    ::std::is_trivially_move_assignable_v<::into::NoCloneCopyDropType>);
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aNoCloneCopyDropType_uinto_ugolden_x0000003a_x0000003aNoCloneCopyDropTarget(
    ::into::NoCloneCopyDropType*, ::into::NoCloneCopyDropTarget* __ret_ptr);
}
inline ::into::NoCloneCopyDropType::operator ::into::NoCloneCopyDropTarget() {
  auto&& self = *this;
  crubit::Slot<::into::NoCloneCopyDropTarget> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aNoCloneCopyDropType_uinto_ugolden_x0000003a_x0000003aNoCloneCopyDropTarget(
          &self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void ::into::NoCloneCopyDropType::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::into::NoCloneCopyDropType;
  static_assert(0 == offsetof(__crubit_assert_type, __field0));
}
static_assert(
    sizeof(::into::NoCloneDefaultTarget) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::into::NoCloneDefaultTarget) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<NoCloneDefaultTarget>);
static_assert(
    ::std::is_trivially_move_constructible_v<::into::NoCloneDefaultTarget>);
static_assert(
    ::std::is_trivially_move_assignable_v<::into::NoCloneDefaultTarget>);
inline void ::into::NoCloneDefaultTarget::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::into::NoCloneDefaultTarget;
  static_assert(0 == offsetof(__crubit_assert_type, __field0));
}
static_assert(
    sizeof(::into::NoCloneDefaultType) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::into::NoCloneDefaultType) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<NoCloneDefaultType>);
static_assert(
    ::std::is_trivially_move_constructible_v<::into::NoCloneDefaultType>);
static_assert(
    ::std::is_trivially_move_assignable_v<::into::NoCloneDefaultType>);
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aNoCloneDefaultType_uinto_ugolden_x0000003a_x0000003aNoCloneDefaultTarget(
    ::into::NoCloneDefaultType*, ::into::NoCloneDefaultTarget* __ret_ptr);
}
inline ::into::NoCloneDefaultType::operator ::into::NoCloneDefaultTarget() {
  auto&& self = *this;
  crubit::Slot<::into::NoCloneDefaultTarget> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_Into_uinto_uinto_ugolden_x0000003a_x0000003aNoCloneDefaultType_uinto_ugolden_x0000003a_x0000003aNoCloneDefaultTarget(
          &self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void ::into::NoCloneDefaultType::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::into::NoCloneDefaultType;
  static_assert(0 == offsetof(__crubit_assert_type, __field0));
}
static_assert(
    sizeof(::into::NotFfiSafe) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::into::NotFfiSafe) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<::into::NotFfiSafe>);
static_assert(::std::is_trivially_move_constructible_v<::into::NotFfiSafe>);
static_assert(::std::is_trivially_move_assignable_v<::into::NotFfiSafe>);
inline ::into::NotFfiSafe::NotFfiSafe(::crubit::UnsafeRelocateTag,
                                      NotFfiSafe&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline void ::into::NotFfiSafe::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::into::NotFfiSafe;
  static_assert(0 == offsetof(__crubit_assert_type, __field0));
}
}  // namespace into

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_INTO_INTO_GOLDEN
