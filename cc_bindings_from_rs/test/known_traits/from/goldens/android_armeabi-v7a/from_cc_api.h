// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// from_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_FROM_FROM_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_FROM_FROM_GOLDEN

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

namespace from {
struct CloneAllocType;
struct CloneCopyType;
struct LoopB;
struct NoCloneCopyDropType;
struct NoCloneDefaultType;
struct OpaqueRef;
// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: from_golden :: CloneAllocSource") alignas(
    4) [[clang::trivial_abi]] CloneAllocSource final {
 public:
  // CRUBIT_ANNOTATE: must_bind=
  static ::from::CloneAllocSource create(rs_std::StrRef s);

  // CRUBIT_ANNOTATE: must_bind=
  rs_std::StrRef get_value() const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  explicit operator ::from::CloneAllocType();

  ::rs::alloc::string::String value = {};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: from_golden :: CloneAllocType") alignas(4)
    [[clang::trivial_abi]] CloneAllocType final {
 public:
  // CRUBIT_ANNOTATE: must_bind=
  rs_std::StrRef get_value() const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  ::rs::alloc::string::String value = {};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: from_golden :: CloneCopySource") alignas(4)
    [[clang::trivial_abi]] CloneCopySource final {
 public:
  explicit operator ::from::CloneCopyType();

  ::std::int32_t __field0 = {};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: from_golden :: CloneCopyType") alignas(4)
    [[clang::trivial_abi]] CloneCopyType final {
 public:
  ::std::int32_t __field0 = {};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: from_golden :: CollidingConstructor") alignas(8) [[clang::trivial_abi]]
CollidingConstructor final {
 public:
  // `from_golden::CollidingConstructor` doesn't implement the `Default` trait
  CollidingConstructor() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~CollidingConstructor() = default;
  CollidingConstructor(CollidingConstructor&&) = default;
  CollidingConstructor& operator=(CollidingConstructor&&) = default;

  // `from_golden::CollidingConstructor` doesn't implement the `Clone` trait
  CollidingConstructor(const CollidingConstructor&) = delete;
  CollidingConstructor& operator=(const CollidingConstructor&) = delete;
  CollidingConstructor(::crubit::UnsafeRelocateTag,
                       CollidingConstructor&& value);

  // Error generating bindings for implementation
  // `<from_golden::CollidingConstructor as std::convert::From<u64>>` defined at
  // cc_bindings_from_rs/test/known_traits/from/from.rs;l=190:
  // From implementation for `u64` is not supported when `From<usize>` is
  // implemented as it may overlap.

  explicit CollidingConstructor(::std::uintptr_t value);

 private:
  union {
    ::std::uint64_t value;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: from_golden :: LoopA") alignas(4)
    [[clang::trivial_abi]] LoopA final {
 public:
  explicit operator ::from::LoopB();

  ::std::int32_t __field0 = {};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: from_golden :: LoopB") alignas(4)
    [[clang::trivial_abi]] LoopB final {
 public:
  explicit operator ::from::LoopA();

  ::std::int32_t __field0 = {};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: from_golden :: NoCloneCopyDropSource") alignas(4) [[clang::trivial_abi]]
NoCloneCopyDropSource final {
 public:
  explicit operator ::from::NoCloneCopyDropType();

  ::std::int32_t __field0 = {};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: from_golden :: NoCloneCopyDropType") alignas(4) [[clang::trivial_abi]]
NoCloneCopyDropType final {
 public:
  ::std::int32_t __field0 = {};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: from_golden :: NoCloneDefaultSource") alignas(4) [[clang::trivial_abi]]
NoCloneDefaultSource final {
 public:
  explicit operator ::from::NoCloneDefaultType();

  ::std::int32_t __field0 = {};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: from_golden :: NoCloneDefaultType") alignas(4) [[clang::trivial_abi]]
NoCloneDefaultType final {
 public:
  ::std::int32_t __field0 = {};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: from_golden :: NotFfiSafe") alignas(4)
    [[clang::trivial_abi]] NotFfiSafe final {
 public:
  // `from_golden::NotFfiSafe` doesn't implement the `Default` trait
  NotFfiSafe() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~NotFfiSafe() = default;
  NotFfiSafe(NotFfiSafe&&) = default;
  NotFfiSafe& operator=(NotFfiSafe&&) = default;

  // `from_golden::NotFfiSafe` doesn't implement the `Clone` trait
  NotFfiSafe(const NotFfiSafe&) = delete;
  NotFfiSafe& operator=(const NotFfiSafe&) = delete;
  NotFfiSafe(::crubit::UnsafeRelocateTag, NotFfiSafe&& value);

  // CRUBIT_ANNOTATE: must_bind=
  static ::from::NotFfiSafe create();

  explicit operator ::std::int32_t();

 private:
  // Field type has been replaced with a blob of bytes: Function pointers can't
  // have a thunk: Any calling convention other than `extern "C"` requires a
  // thunk
  ::std::array<unsigned char, 4> __field0;

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: from_golden :: Opaque") alignas(4)
    [[clang::trivial_abi]] Opaque final {
 public:
  explicit operator ::std::int32_t();

  explicit operator ::std::int64_t();

  explicit operator rs_std::StrRef();

  explicit operator ::std::int16_t();

  explicit operator ::from::OpaqueRef();

  ::std::int32_t __field0 = {};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(":: from_golden :: OpaqueRef") alignas(4)
    [[clang::trivial_abi]] OpaqueRef final {
 public:
  // `from_golden::OpaqueRef` doesn't implement the `Default` trait
  OpaqueRef() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~OpaqueRef() = default;
  OpaqueRef(OpaqueRef&&) = default;
  OpaqueRef& operator=(OpaqueRef&&) = default;

  // `from_golden::OpaqueRef` doesn't implement the `Clone` trait
  OpaqueRef(const OpaqueRef&) = delete;
  OpaqueRef& operator=(const OpaqueRef&) = delete;
  OpaqueRef(::crubit::UnsafeRelocateTag, OpaqueRef&& value);

  // CRUBIT_ANNOTATE: must_bind=
  static ::from::OpaqueRef create(rs_std::StrRef s);

  // CRUBIT_ANNOTATE: must_bind=
  rs_std::StrRef get_arg() const;

  explicit operator rs_std::StrRef();

  explicit OpaqueRef(::from::Opaque value);

 private:
  union {
    rs_std::StrRef __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

static_assert(
    sizeof(CloneAllocSource) == 12,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(CloneAllocSource) == 4,
    "Verify that ADT layout didn't change since this header got generated");

namespace __crubit_internal {
extern "C" void __crubit_thunk_create(rs_std::StrRef,
                                      ::from::CloneAllocSource* __ret_ptr);
}
inline ::from::CloneAllocSource CloneAllocSource::create(rs_std::StrRef s) {
  crubit::Slot<::from::CloneAllocSource> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(s, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" rs_std::StrRef __crubit_thunk_get_uvalue(
    ::from::CloneAllocSource const&);
}
inline rs_std::StrRef CloneAllocSource::get_value() const& $(__anon1)
    CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_get_uvalue(self);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aCloneAllocSource_ufrom_ugolden_x0000003a_x0000003aCloneAllocType(
    ::from::CloneAllocSource*, ::from::CloneAllocType* __ret_ptr);
}
inline CloneAllocSource::operator ::from::CloneAllocType() {
  auto&& self = *this;
  crubit::Slot self_slot((::std::move(self)));
  crubit::Slot<::from::CloneAllocType> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aCloneAllocSource_ufrom_ugolden_x0000003a_x0000003aCloneAllocType(
          self_slot.Get(), __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void CloneAllocSource::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(CloneAllocSource, value));
}
static_assert(
    sizeof(CloneAllocType) == 12,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(CloneAllocType) == 4,
    "Verify that ADT layout didn't change since this header got generated");

namespace __crubit_internal {
extern "C" rs_std::StrRef __crubit_thunk_get_uvalue(
    ::from::CloneAllocType const&);
}
inline rs_std::StrRef CloneAllocType::get_value() const& $(__anon1)
    CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_get_uvalue(self);
}
inline void CloneAllocType::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(CloneAllocType, value));
}
static_assert(
    sizeof(CloneCopySource) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(CloneCopySource) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<CloneCopySource>);
static_assert(
    ::std::is_trivially_move_constructible_v<::from::CloneCopySource>);
static_assert(::std::is_trivially_move_assignable_v<::from::CloneCopySource>);
static_assert(
    ::std::is_trivially_copy_constructible_v<::from::CloneCopySource>);
static_assert(::std::is_trivially_copy_assignable_v<::from::CloneCopySource>);
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aCloneCopySource_ufrom_ugolden_x0000003a_x0000003aCloneCopyType(
    ::from::CloneCopySource*, ::from::CloneCopyType* __ret_ptr);
}
inline CloneCopySource::operator ::from::CloneCopyType() {
  auto& self = const_cast<::std::remove_cvref_t<decltype(*this)>&>(*this);
  crubit::Slot<::from::CloneCopyType> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aCloneCopySource_ufrom_ugolden_x0000003a_x0000003aCloneCopyType(
          &self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void CloneCopySource::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(CloneCopySource, __field0));
}
static_assert(
    sizeof(CloneCopyType) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(CloneCopyType) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<CloneCopyType>);
static_assert(::std::is_trivially_move_constructible_v<::from::CloneCopyType>);
static_assert(::std::is_trivially_move_assignable_v<::from::CloneCopyType>);
static_assert(::std::is_trivially_copy_constructible_v<::from::CloneCopyType>);
static_assert(::std::is_trivially_copy_assignable_v<::from::CloneCopyType>);
inline void CloneCopyType::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(CloneCopyType, __field0));
}
static_assert(
    sizeof(CollidingConstructor) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(CollidingConstructor) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<CollidingConstructor>);
static_assert(
    ::std::is_trivially_move_constructible_v<::from::CollidingConstructor>);
static_assert(
    ::std::is_trivially_move_assignable_v<::from::CollidingConstructor>);
inline ::from::CollidingConstructor::CollidingConstructor(
    ::crubit::UnsafeRelocateTag, CollidingConstructor&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_From_ufrom_ufrom_ugolden_x0000003a_x0000003aCollidingConstructor_uusize(
    ::std::uintptr_t, ::from::CollidingConstructor* __ret_ptr);
}
inline CollidingConstructor::CollidingConstructor(::std::uintptr_t value) {
  __crubit_internal::
      __crubit_thunk_From_ufrom_ufrom_ugolden_x0000003a_x0000003aCollidingConstructor_uusize(
          value, this);
}
inline void CollidingConstructor::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(CollidingConstructor, value));
}
static_assert(
    sizeof(LoopA) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(LoopA) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<LoopA>);
static_assert(::std::is_trivially_move_constructible_v<::from::LoopA>);
static_assert(::std::is_trivially_move_assignable_v<::from::LoopA>);
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aLoopA_ufrom_ugolden_x0000003a_x0000003aLoopB(
    ::from::LoopA*, ::from::LoopB* __ret_ptr);
}
inline LoopA::operator ::from::LoopB() {
  auto&& self = *this;
  crubit::Slot<::from::LoopB> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aLoopA_ufrom_ugolden_x0000003a_x0000003aLoopB(
          &self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void LoopA::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(LoopA, __field0));
}
static_assert(
    sizeof(LoopB) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(LoopB) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<LoopB>);
static_assert(::std::is_trivially_move_constructible_v<::from::LoopB>);
static_assert(::std::is_trivially_move_assignable_v<::from::LoopB>);
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aLoopB_ufrom_ugolden_x0000003a_x0000003aLoopA(
    ::from::LoopB*, ::from::LoopA* __ret_ptr);
}
inline LoopB::operator ::from::LoopA() {
  auto&& self = *this;
  crubit::Slot<::from::LoopA> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aLoopB_ufrom_ugolden_x0000003a_x0000003aLoopA(
          &self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void LoopB::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(LoopB, __field0));
}
static_assert(
    sizeof(NoCloneCopyDropSource) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NoCloneCopyDropSource) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<NoCloneCopyDropSource>);
static_assert(
    ::std::is_trivially_move_constructible_v<::from::NoCloneCopyDropSource>);
static_assert(
    ::std::is_trivially_move_assignable_v<::from::NoCloneCopyDropSource>);
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aNoCloneCopyDropSource_ufrom_ugolden_x0000003a_x0000003aNoCloneCopyDropType(
    ::from::NoCloneCopyDropSource*, ::from::NoCloneCopyDropType* __ret_ptr);
}
inline NoCloneCopyDropSource::operator ::from::NoCloneCopyDropType() {
  auto&& self = *this;
  crubit::Slot<::from::NoCloneCopyDropType> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aNoCloneCopyDropSource_ufrom_ugolden_x0000003a_x0000003aNoCloneCopyDropType(
          &self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void NoCloneCopyDropSource::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NoCloneCopyDropSource, __field0));
}
static_assert(
    sizeof(NoCloneCopyDropType) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NoCloneCopyDropType) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<NoCloneCopyDropType>);
static_assert(
    ::std::is_trivially_move_constructible_v<::from::NoCloneCopyDropType>);
static_assert(
    ::std::is_trivially_move_assignable_v<::from::NoCloneCopyDropType>);
inline void NoCloneCopyDropType::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NoCloneCopyDropType, __field0));
}
static_assert(
    sizeof(NoCloneDefaultSource) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NoCloneDefaultSource) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<NoCloneDefaultSource>);
static_assert(
    ::std::is_trivially_move_constructible_v<::from::NoCloneDefaultSource>);
static_assert(
    ::std::is_trivially_move_assignable_v<::from::NoCloneDefaultSource>);
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aNoCloneDefaultSource_ufrom_ugolden_x0000003a_x0000003aNoCloneDefaultType(
    ::from::NoCloneDefaultSource*, ::from::NoCloneDefaultType* __ret_ptr);
}
inline NoCloneDefaultSource::operator ::from::NoCloneDefaultType() {
  auto&& self = *this;
  crubit::Slot<::from::NoCloneDefaultType> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aNoCloneDefaultSource_ufrom_ugolden_x0000003a_x0000003aNoCloneDefaultType(
          &self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void NoCloneDefaultSource::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NoCloneDefaultSource, __field0));
}
static_assert(
    sizeof(NoCloneDefaultType) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NoCloneDefaultType) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<NoCloneDefaultType>);
static_assert(
    ::std::is_trivially_move_constructible_v<::from::NoCloneDefaultType>);
static_assert(
    ::std::is_trivially_move_assignable_v<::from::NoCloneDefaultType>);
inline void NoCloneDefaultType::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NoCloneDefaultType, __field0));
}
static_assert(
    sizeof(NotFfiSafe) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NotFfiSafe) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<NotFfiSafe>);
static_assert(::std::is_trivially_move_constructible_v<::from::NotFfiSafe>);
static_assert(::std::is_trivially_move_assignable_v<::from::NotFfiSafe>);
inline ::from::NotFfiSafe::NotFfiSafe(::crubit::UnsafeRelocateTag,
                                      NotFfiSafe&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_create(::from::NotFfiSafe* __ret_ptr);
}
inline ::from::NotFfiSafe NotFfiSafe::create() {
  crubit::Slot<::from::NotFfiSafe> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
namespace __crubit_internal {
extern "C" ::std::int32_t
__crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aNotFfiSafe_ui32(
    ::from::NotFfiSafe*);
}
inline NotFfiSafe::operator ::std::int32_t() {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aNotFfiSafe_ui32(
          &self);
}
inline void NotFfiSafe::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NotFfiSafe, __field0));
}
static_assert(
    sizeof(Opaque) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Opaque) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<Opaque>);
static_assert(::std::is_trivially_move_constructible_v<::from::Opaque>);
static_assert(::std::is_trivially_move_assignable_v<::from::Opaque>);
namespace __crubit_internal {
extern "C" ::std::int32_t
__crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aOpaque_ui32(
    ::from::Opaque*);
}
inline Opaque::operator ::std::int32_t() {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aOpaque_ui32(
          &self);
}
namespace __crubit_internal {
extern "C" ::std::int64_t
__crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aOpaque_ui64(
    ::from::Opaque*);
}
inline Opaque::operator ::std::int64_t() {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aOpaque_ui64(
          &self);
}
namespace __crubit_internal {
extern "C" rs_std::StrRef
__crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aOpaque_u_x00000026_x00000027static_x00000020str(
    ::from::Opaque*);
}
inline Opaque::operator rs_std::StrRef() {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aOpaque_u_x00000026_x00000027static_x00000020str(
          &self);
}
namespace __crubit_internal {
extern "C" ::std::int16_t
__crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aOpaque_ui16(
    ::from::Opaque*);
}
inline Opaque::operator ::std::int16_t() {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aOpaque_ui16(
          &self);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aOpaque_ufrom_ugolden_x0000003a_x0000003aOpaqueRef_x0000003c_x00000027static_x0000003e(
    ::from::Opaque*, ::from::OpaqueRef* __ret_ptr);
}
inline Opaque::operator ::from::OpaqueRef() {
  auto&& self = *this;
  crubit::Slot<::from::OpaqueRef> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aOpaque_ufrom_ugolden_x0000003a_x0000003aOpaqueRef_x0000003c_x00000027static_x0000003e(
          &self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void Opaque::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Opaque, __field0));
}
static_assert(
    sizeof(OpaqueRef) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(OpaqueRef) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<OpaqueRef>);
static_assert(::std::is_trivially_move_constructible_v<::from::OpaqueRef>);
static_assert(::std::is_trivially_move_assignable_v<::from::OpaqueRef>);
inline ::from::OpaqueRef::OpaqueRef(::crubit::UnsafeRelocateTag,
                                    OpaqueRef&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_create(rs_std::StrRef,
                                      ::from::OpaqueRef* __ret_ptr);
}
inline ::from::OpaqueRef OpaqueRef::create(rs_std::StrRef s) {
  crubit::Slot<::from::OpaqueRef> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(s, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
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
__crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aOpaqueRef_x0000003c_x00000027_u_x0000003e_u_x00000026_x00000027a_x00000020str(
    ::from::OpaqueRef*);
}
inline OpaqueRef::operator rs_std::StrRef() {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_Into_uinto_ufrom_ugolden_x0000003a_x0000003aOpaqueRef_x0000003c_x00000027_u_x0000003e_u_x00000026_x00000027a_x00000020str(
          &self);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_From_ufrom_ufrom_ugolden_x0000003a_x0000003aOpaqueRef_x0000003c_x00000027_u_x0000003e_ufrom_ugolden_x0000003a_x0000003aOpaque(
    ::from::Opaque*, ::from::OpaqueRef* __ret_ptr);
}
inline OpaqueRef::OpaqueRef(::from::Opaque value) {
  __crubit_internal::
      __crubit_thunk_From_ufrom_ufrom_ugolden_x0000003a_x0000003aOpaqueRef_x0000003c_x00000027_u_x0000003e_ufrom_ugolden_x0000003a_x0000003aOpaque(
          &value, this);
}
inline void OpaqueRef::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(OpaqueRef, __field0));
}
}  // namespace from

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_FROM_FROM_GOLDEN
