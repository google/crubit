// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// vec_golden

// clang-format off
#ifndef CC_BINDINGS_FROM_RS_TEST_VEC_VEC_GOLDEN
#define CC_BINDINGS_FROM_RS_TEST_VEC_VEC_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#pragma clang diagnostic ignored "-Wshadow"
#include "support/annotations_internal.h"
#include "support/internal/check.h"
#include "support/internal/memswap.h"
#include "support/internal/slot.h"
#include "support/lifetime_annotations.h"
#include "support/rs/vec.h"

#include <array>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <memory>
#include <new>
#include <type_traits>
#include <utility>

#pragma clang assume_nonnull begin

namespace vec {

struct CRUBIT_INTERNAL_RUST_TYPE(":: vec_golden :: MoveOnly") alignas(4)
    [[clang::trivial_abi]] MoveOnly final {
 public:
  // Default::default
  MoveOnly();

  // Drop::drop
  ~MoveOnly();

  MoveOnly(MoveOnly&&) noexcept;
  ::vec::MoveOnly& operator=(MoveOnly&&) noexcept;

  // `vec_golden::MoveOnly` doesn't implement the `Clone` trait
  MoveOnly(const MoveOnly&) = delete;
  MoveOnly& operator=(const MoveOnly&) = delete;
  MoveOnly(::crubit::UnsafeRelocateTag, MoveOnly&& value);

  static ::vec::MoveOnly new_();

  ::std::uintptr_t strong_count() const;

 private:
  // Field type has been replaced with a blob of bytes: Field is private
  ::std::array<unsigned char, 4> counter;

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(":: vec_golden :: NonDefault") alignas(4)
    [[clang::trivial_abi]] NonDefault final {
 public:
  // `vec_golden::NonDefault` doesn't implement the `Default` trait
  NonDefault() = delete;

  // Drop::drop
  ~NonDefault();

  // Clone::clone
  NonDefault(const NonDefault&) noexcept;

  // Clone::clone_from
  ::vec::NonDefault& operator=(const NonDefault&) noexcept;

  NonDefault(::crubit::UnsafeRelocateTag, NonDefault&& value);

  static ::vec::NonDefault new_(::std::int32_t value);

  ::std::int32_t value() const;

 private:
  // Field type has been replaced with a blob of bytes: Field is private
  ::std::array<unsigned char, 4> value_;

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(":: vec_golden :: RustVecOwner") alignas(4)
    [[clang::trivial_abi]] RustVecOwner final {
 public:
  // Default::default
  RustVecOwner();

  // Drop::drop
  ~RustVecOwner();

  RustVecOwner(RustVecOwner&&) noexcept;
  ::vec::RustVecOwner& operator=(RustVecOwner&&) noexcept;

  // `vec_golden::RustVecOwner` doesn't implement the `Clone` trait
  RustVecOwner(const RustVecOwner&) = delete;
  RustVecOwner& operator=(const RustVecOwner&) = delete;
  RustVecOwner(::crubit::UnsafeRelocateTag, RustVecOwner&& value);

  static ::vec::RustVecOwner new_();

  ::rs::Vec<::std::int32_t>& $(__anon1) vec_mut() &
      $(__anon1) CRUBIT_LIFETIME_BOUND;

  ::std::uintptr_t len() const;

  bool is_empty() const;

  ::std::int32_t element(::std::uintptr_t index) const;

 private:
  // Field type has been replaced with a blob of bytes: Field is private
  ::std::array<unsigned char, 12> v;

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(":: vec_golden :: Tracked") alignas(4)
    [[clang::trivial_abi]] Tracked final {
 public:
  // Default::default
  Tracked();

  // Drop::drop
  ~Tracked();

  Tracked(Tracked&&) noexcept;
  ::vec::Tracked& operator=(Tracked&&) noexcept;

  // Clone::clone
  Tracked(const Tracked&) noexcept;

  // Clone::clone_from
  ::vec::Tracked& operator=(const Tracked&) noexcept;

  Tracked(::crubit::UnsafeRelocateTag, Tracked&& value);

  static ::vec::Tracked new_();

  ::std::uintptr_t strong_count() const;

 private:
  // Field type has been replaced with a blob of bytes: Field is private
  ::std::array<unsigned char, 4> counter;

 private:
  static void __crubit_field_offset_assertions();
};

void drop_vec(::rs::Vec<::std::int32_t> _v);

::rs::Vec<::std::int32_t> return_grown_vec();

::rs::Vec<::std::uint8_t> return_u8_vec();

::rs::Vec<::std::int32_t> return_vec();

void rust_add_elements(::rs::Vec<::std::int32_t>& v);

::std::uintptr_t take_move_only_vec(::rs::Vec<::vec::MoveOnly> v);

::std::uintptr_t take_non_default_vec(::rs::Vec<::vec::NonDefault> v);

::std::uintptr_t take_tracked_vec(::rs::Vec<::vec::Tracked> v);

::std::int32_t take_vec(::rs::Vec<::std::int32_t> v);

}

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    ":: alloc :: vec :: Vec < i32 >")::rs::Vec<::std::int32_t>
    : public ::rs::VecBase<::std::int32_t> {
 public:
  using ::rs::VecBase<::std::int32_t>::VecBase;

  // Default::default
  Vec() noexcept;

  // Clone::clone
  Vec(const Vec&) noexcept;

  // Clone::clone_from
  ::rs::Vec<::std::int32_t>& operator=(const Vec&) noexcept;

  Vec(Vec&&) noexcept;
  ::rs::Vec<::std::int32_t>& operator=(Vec&&) noexcept;
  Vec(::crubit::UnsafeRelocateTag, Vec&& value);

  ~Vec() noexcept;

 private:
  friend class ::rs::VecBase<::std::int32_t>;
  static constexpr ::std::size_t kPtrOffset = 4;
  static constexpr ::std::size_t kCapOffset = 0;
  static constexpr ::std::size_t kLenOffset = 8;
  unsigned char storage_[12];
};
#endif

namespace vec {

struct CRUBIT_INTERNAL_RUST_TYPE(":: vec_golden :: StructWithVec")
    [[clang::trivial_abi]] StructWithVec final {
 public:
  static ::vec::StructWithVec new_(::std::int32_t val);

  ::rs::Vec<::std::int32_t> v{};

 private:
  static void __crubit_field_offset_assertions();
};

}

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    ":: alloc :: vec :: Vec < u8 >")::rs::Vec<::std::uint8_t>
    : public ::rs::VecBase<::std::uint8_t> {
 public:
  using ::rs::VecBase<::std::uint8_t>::VecBase;

  // Default::default
  Vec() noexcept;

  // Clone::clone
  Vec(const Vec&) noexcept;

  // Clone::clone_from
  ::rs::Vec<::std::uint8_t>& operator=(const Vec&) noexcept;

  Vec(Vec&&) noexcept;
  ::rs::Vec<::std::uint8_t>& operator=(Vec&&) noexcept;
  Vec(::crubit::UnsafeRelocateTag, Vec&& value);

  ~Vec() noexcept;

 private:
  friend class ::rs::VecBase<::std::uint8_t>;
  static constexpr ::std::size_t kPtrOffset = 4;
  static constexpr ::std::size_t kCapOffset = 0;
  static constexpr ::std::size_t kLenOffset = 8;
  unsigned char storage_[12];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020vec_x00000020_x0000003a_x0000003a_x00000020MoveOnly_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020vec_x00000020_x0000003a_x0000003a_x00000020MoveOnly_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    ":: alloc :: vec :: Vec < :: vec_golden :: MoveOnly >")::rs::
    Vec<::vec::MoveOnly> : public ::rs::VecBase<::vec::MoveOnly> {
 public:
  using ::rs::VecBase<::vec::MoveOnly>::VecBase;

  // Default::default
  Vec() noexcept;

  // `alloc::vec::Vec` doesn't implement the `Clone` trait
  Vec(const Vec&) = delete;
  Vec& operator=(const Vec&) = delete;
  Vec(Vec&&) noexcept;
  ::rs::Vec<::vec::MoveOnly>& operator=(Vec&&) noexcept;
  Vec(::crubit::UnsafeRelocateTag, Vec&& value);

  ~Vec() noexcept;

 private:
  friend class ::rs::VecBase<::vec::MoveOnly>;
  static constexpr ::std::size_t kPtrOffset = 4;
  static constexpr ::std::size_t kCapOffset = 0;
  static constexpr ::std::size_t kLenOffset = 8;
  unsigned char storage_[12];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020vec_x00000020_x0000003a_x0000003a_x00000020NonDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020vec_x00000020_x0000003a_x0000003a_x00000020NonDefault_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    ":: alloc :: vec :: Vec < :: vec_golden :: NonDefault >")::rs::
    Vec<::vec::NonDefault> : public ::rs::VecBase<::vec::NonDefault> {
 public:
  using ::rs::VecBase<::vec::NonDefault>::VecBase;

  // Default::default
  Vec() noexcept;

  // Clone::clone
  Vec(const Vec&) noexcept;

  // Clone::clone_from
  ::rs::Vec<::vec::NonDefault>& operator=(const Vec&) noexcept;

  Vec(Vec&&) noexcept;
  ::rs::Vec<::vec::NonDefault>& operator=(Vec&&) noexcept;
  Vec(::crubit::UnsafeRelocateTag, Vec&& value);

  ~Vec() noexcept;

 private:
  friend class ::rs::VecBase<::vec::NonDefault>;
  static constexpr ::std::size_t kPtrOffset = 4;
  static constexpr ::std::size_t kCapOffset = 0;
  static constexpr ::std::size_t kLenOffset = 8;
  unsigned char storage_[12];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020vec_x00000020_x0000003a_x0000003a_x00000020Tracked_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020vec_x00000020_x0000003a_x0000003a_x00000020Tracked_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    ":: alloc :: vec :: Vec < :: vec_golden :: Tracked >")::rs::
    Vec<::vec::Tracked> : public ::rs::VecBase<::vec::Tracked> {
 public:
  using ::rs::VecBase<::vec::Tracked>::VecBase;

  // Default::default
  Vec() noexcept;

  // Clone::clone
  Vec(const Vec&) noexcept;

  // Clone::clone_from
  ::rs::Vec<::vec::Tracked>& operator=(const Vec&) noexcept;

  Vec(Vec&&) noexcept;
  ::rs::Vec<::vec::Tracked>& operator=(Vec&&) noexcept;
  Vec(::crubit::UnsafeRelocateTag, Vec&& value);

  ~Vec() noexcept;

 private:
  friend class ::rs::VecBase<::vec::Tracked>;
  static constexpr ::std::size_t kPtrOffset = 4;
  static constexpr ::std::size_t kCapOffset = 0;
  static constexpr ::std::size_t kLenOffset = 8;
  unsigned char storage_[12];
};
#endif

namespace vec {

static_assert(
    sizeof(::vec::MoveOnly) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::vec::MoveOnly) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_uvec_ugolden_x0000003a_x0000003aMoveOnly(
    ::vec::MoveOnly* __ret_ptr);
}
inline ::vec::MoveOnly::MoveOnly() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_uvec_ugolden_x0000003a_x0000003aMoveOnly(
          this);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Drop_udrop_uvec_ugolden_x0000003a_x0000003aMoveOnly(
    ::vec::MoveOnly&);
}
inline ::vec::MoveOnly::~MoveOnly() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_uvec_ugolden_x0000003a_x0000003aMoveOnly(*this);
}
inline ::vec::MoveOnly::MoveOnly(MoveOnly&& other) noexcept : MoveOnly() {
  *this = ::std::move(other);
}
inline ::vec::MoveOnly& ::vec::MoveOnly::operator=(MoveOnly&& other) noexcept {
  crubit::MemSwap(*this, other);
  return *this;
}
inline ::vec::MoveOnly::MoveOnly(::crubit::UnsafeRelocateTag,
                                 MoveOnly&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::vec::MoveOnly* __ret_ptr);
}
inline ::vec::MoveOnly(MoveOnly::new_)() {
  crubit::Slot<::vec::MoveOnly> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::uintptr_t __crubit_thunk_strong_ucount(
    ::vec::MoveOnly const&);
}
inline ::std::uintptr_t(MoveOnly::strong_count)() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_strong_ucount(self);
}
inline void ::vec::MoveOnly::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::vec::MoveOnly;
  static_assert(0 == offsetof(__crubit_assert_type, counter));
}
static_assert(
    sizeof(::vec::NonDefault) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::vec::NonDefault) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Drop_udrop_uvec_ugolden_x0000003a_x0000003aNonDefault(
    ::vec::NonDefault&);
}
inline ::vec::NonDefault::~NonDefault() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_uvec_ugolden_x0000003a_x0000003aNonDefault(
          *this);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_uvec_ugolden_x0000003a_x0000003aNonDefault(
    ::vec::NonDefault const&, ::vec::NonDefault* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_uvec_ugolden_x0000003a_x0000003aNonDefault(
    ::vec::NonDefault&, ::vec::NonDefault const&);
}
inline ::vec::NonDefault::NonDefault(const NonDefault& other) noexcept {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_uvec_ugolden_x0000003a_x0000003aNonDefault(
          other, this);
}
inline ::vec::NonDefault& ::vec::NonDefault::operator=(
    const NonDefault& other) noexcept {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_uvec_ugolden_x0000003a_x0000003aNonDefault(
            *this, other);
  }
  return *this;
}
inline ::vec::NonDefault::NonDefault(::crubit::UnsafeRelocateTag,
                                     NonDefault&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::std::int32_t,
                                   ::vec::NonDefault* __ret_ptr);
}
inline ::vec::NonDefault(NonDefault::new_)(::std::int32_t value) {
  crubit::Slot<::vec::NonDefault> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(value, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_value(::vec::NonDefault const&);
}
inline ::std::int32_t(NonDefault::value)() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_value(self);
}
inline void ::vec::NonDefault::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::vec::NonDefault;
  static_assert(0 == offsetof(__crubit_assert_type, value_));
}
static_assert(
    sizeof(::vec::RustVecOwner) == 12,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::vec::RustVecOwner) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_uvec_ugolden_x0000003a_x0000003aRustVecOwner(
    ::vec::RustVecOwner* __ret_ptr);
}
inline ::vec::RustVecOwner::RustVecOwner() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_uvec_ugolden_x0000003a_x0000003aRustVecOwner(
          this);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Drop_udrop_uvec_ugolden_x0000003a_x0000003aRustVecOwner(
    ::vec::RustVecOwner&);
}
inline ::vec::RustVecOwner::~RustVecOwner() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_uvec_ugolden_x0000003a_x0000003aRustVecOwner(
          *this);
}
inline ::vec::RustVecOwner::RustVecOwner(RustVecOwner&& other) noexcept
    : RustVecOwner() {
  *this = ::std::move(other);
}
inline ::vec::RustVecOwner& ::vec::RustVecOwner::operator=(
    RustVecOwner&& other) noexcept {
  crubit::MemSwap(*this, other);
  return *this;
}
inline ::vec::RustVecOwner::RustVecOwner(::crubit::UnsafeRelocateTag,
                                         RustVecOwner&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::vec::RustVecOwner* __ret_ptr);
}
inline ::vec::RustVecOwner(RustVecOwner::new_)() {
  crubit::Slot<::vec::RustVecOwner> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::rs::Vec<::std::int32_t>& $(__anon1)
    __crubit_thunk_vec_umut(::vec::RustVecOwner&);
}
inline ::rs::Vec<::std::int32_t>& $(__anon1)(RustVecOwner::vec_mut)() &
    $(__anon1) CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_vec_umut(self);
}

namespace __crubit_internal {
extern "C" ::std::uintptr_t __crubit_thunk_len(::vec::RustVecOwner const&);
}
inline ::std::uintptr_t(RustVecOwner::len)() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_len(self);
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_uempty(::vec::RustVecOwner const&);
}
inline bool(RustVecOwner::is_empty)() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_is_uempty(self);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_element(::vec::RustVecOwner const&,
                                                 ::std::uintptr_t);
}
inline ::std::int32_t(RustVecOwner::element)(::std::uintptr_t index) const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_element(self, index);
}
inline void ::vec::RustVecOwner::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::vec::RustVecOwner;
  static_assert(0 == offsetof(__crubit_assert_type, v));
}
static_assert(
    sizeof(::vec::StructWithVec) == 12,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::vec::StructWithVec) == 4,
    "Verify that ADT layout didn't change since this header got generated");

namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::std::int32_t,
                                   ::vec::StructWithVec* __ret_ptr);
}
inline ::vec::StructWithVec(StructWithVec::new_)(::std::int32_t val) {
  crubit::Slot<::vec::StructWithVec> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(val, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void ::vec::StructWithVec::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::vec::StructWithVec;
  static_assert(0 == offsetof(__crubit_assert_type, v));
}
static_assert(
    sizeof(::vec::Tracked) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::vec::Tracked) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_uvec_ugolden_x0000003a_x0000003aTracked(
    ::vec::Tracked* __ret_ptr);
}
inline ::vec::Tracked::Tracked() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_uvec_ugolden_x0000003a_x0000003aTracked(
          this);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Drop_udrop_uvec_ugolden_x0000003a_x0000003aTracked(
    ::vec::Tracked&);
}
inline ::vec::Tracked::~Tracked() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_uvec_ugolden_x0000003a_x0000003aTracked(*this);
}
inline ::vec::Tracked::Tracked(Tracked&& other) noexcept : Tracked() {
  *this = ::std::move(other);
}
inline ::vec::Tracked& ::vec::Tracked::operator=(Tracked&& other) noexcept {
  crubit::MemSwap(*this, other);
  return *this;
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_uvec_ugolden_x0000003a_x0000003aTracked(
    ::vec::Tracked const&, ::vec::Tracked* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_uvec_ugolden_x0000003a_x0000003aTracked(
    ::vec::Tracked&, ::vec::Tracked const&);
}
inline ::vec::Tracked::Tracked(const Tracked& other) noexcept {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_uvec_ugolden_x0000003a_x0000003aTracked(other,
                                                                          this);
}
inline ::vec::Tracked& ::vec::Tracked::operator=(
    const Tracked& other) noexcept {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_uvec_ugolden_x0000003a_x0000003aTracked(
            *this, other);
  }
  return *this;
}
inline ::vec::Tracked::Tracked(::crubit::UnsafeRelocateTag, Tracked&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::vec::Tracked* __ret_ptr);
}
inline ::vec::Tracked(Tracked::new_)() {
  crubit::Slot<::vec::Tracked> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::uintptr_t __crubit_thunk_strong_ucount(::vec::Tracked const&);
}
inline ::std::uintptr_t(Tracked::strong_count)() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_strong_ucount(self);
}
inline void ::vec::Tracked::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::vec::Tracked;
  static_assert(0 == offsetof(__crubit_assert_type, counter));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop_uvec(::rs::Vec<::std::int32_t>*);
}
inline void drop_vec(::rs::Vec<::std::int32_t> _v) {
  crubit::Slot _v_slot((::std::move(_v)));
  return __crubit_internal::__crubit_thunk_drop_uvec(_v_slot.Get());
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_ugrown_uvec(
    ::rs::Vec<::std::int32_t>* __ret_ptr);
}
inline ::rs::Vec<::std::int32_t> return_grown_vec() {
  crubit::Slot<::rs::Vec<::std::int32_t>> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_return_ugrown_uvec(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_uu8_uvec(
    ::rs::Vec<::std::uint8_t>* __ret_ptr);
}
inline ::rs::Vec<::std::uint8_t> return_u8_vec() {
  crubit::Slot<::rs::Vec<::std::uint8_t>> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_return_uu8_uvec(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_uvec(
    ::rs::Vec<::std::int32_t>* __ret_ptr);
}
inline ::rs::Vec<::std::int32_t> return_vec() {
  crubit::Slot<::rs::Vec<::std::int32_t>> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_return_uvec(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_rust_uadd_uelements(::rs::Vec<::std::int32_t>&);
}
inline void rust_add_elements(::rs::Vec<::std::int32_t>& v) {
  return __crubit_internal::__crubit_thunk_rust_uadd_uelements(v);
}

namespace __crubit_internal {
extern "C" ::std::uintptr_t __crubit_thunk_take_umove_uonly_uvec(
    ::rs::Vec<::vec::MoveOnly>*);
}
inline ::std::uintptr_t take_move_only_vec(::rs::Vec<::vec::MoveOnly> v) {
  crubit::Slot v_slot((::std::move(v)));
  return __crubit_internal::__crubit_thunk_take_umove_uonly_uvec(v_slot.Get());
}

namespace __crubit_internal {
extern "C" ::std::uintptr_t __crubit_thunk_take_unon_udefault_uvec(
    ::rs::Vec<::vec::NonDefault>*);
}
inline ::std::uintptr_t take_non_default_vec(::rs::Vec<::vec::NonDefault> v) {
  crubit::Slot v_slot((::std::move(v)));
  return __crubit_internal::__crubit_thunk_take_unon_udefault_uvec(
      v_slot.Get());
}

namespace __crubit_internal {
extern "C" ::std::uintptr_t __crubit_thunk_take_utracked_uvec(
    ::rs::Vec<::vec::Tracked>*);
}
inline ::std::uintptr_t take_tracked_vec(::rs::Vec<::vec::Tracked> v) {
  crubit::Slot v_slot((::std::move(v)));
  return __crubit_internal::__crubit_thunk_take_utracked_uvec(v_slot.Get());
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_take_uvec(::rs::Vec<::std::int32_t>*);
}
inline ::std::int32_t take_vec(::rs::Vec<::std::int32_t> v) {
  crubit::Slot v_slot((::std::move(v)));
  return __crubit_internal::__crubit_thunk_take_uvec(v_slot.Get());
}

}

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003ci32_x0000003e(
    ::rs::Vec<::std::int32_t> const&, ::rs::Vec<::std::int32_t>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003ci32_x0000003e(
    ::rs::Vec<::std::int32_t>&, ::rs::Vec<::std::int32_t> const&);
}
inline ::rs::Vec<::std::int32_t>::Vec(const Vec& other) noexcept {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003ci32_x0000003e(
          other, this);
}
inline ::rs::Vec<::std::int32_t>& ::rs::Vec<::std::int32_t>::operator=(
    const Vec& other) noexcept {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003ci32_x0000003e(
            *this, other);
  }
  return *this;
}
inline ::rs::Vec<::std::int32_t>::Vec(Vec&& other) noexcept : storage_{} {
  ::std::memcpy(storage_, other.storage_, sizeof(storage_));
  other.init_empty();
}
inline ::rs::Vec<::std::int32_t>& ::rs::Vec<::std::int32_t>::operator=(
    Vec&& other) noexcept {
  if (this != &other) {
    destroy();
    crubit::MemSwap(*this, other);
  }
  return *this;
}
inline ::rs::Vec<::std::int32_t>::Vec(::crubit::UnsafeRelocateTag,
                                      Vec&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

inline ::rs::Vec<::std::int32_t>::Vec() noexcept : storage_{} { init_empty(); }
inline ::rs::Vec<::std::int32_t>::~Vec() noexcept { destroy(); }
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cu8_x0000003e(
    ::rs::Vec<::std::uint8_t> const&, ::rs::Vec<::std::uint8_t>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cu8_x0000003e(
    ::rs::Vec<::std::uint8_t>&, ::rs::Vec<::std::uint8_t> const&);
}
inline ::rs::Vec<::std::uint8_t>::Vec(const Vec& other) noexcept {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cu8_x0000003e(
          other, this);
}
inline ::rs::Vec<::std::uint8_t>& ::rs::Vec<::std::uint8_t>::operator=(
    const Vec& other) noexcept {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cu8_x0000003e(
            *this, other);
  }
  return *this;
}
inline ::rs::Vec<::std::uint8_t>::Vec(Vec&& other) noexcept : storage_{} {
  ::std::memcpy(storage_, other.storage_, sizeof(storage_));
  other.init_empty();
}
inline ::rs::Vec<::std::uint8_t>& ::rs::Vec<::std::uint8_t>::operator=(
    Vec&& other) noexcept {
  if (this != &other) {
    destroy();
    crubit::MemSwap(*this, other);
  }
  return *this;
}
inline ::rs::Vec<::std::uint8_t>::Vec(::crubit::UnsafeRelocateTag,
                                      Vec&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

inline ::rs::Vec<::std::uint8_t>::Vec() noexcept : storage_{} { init_empty(); }
inline ::rs::Vec<::std::uint8_t>::~Vec() noexcept { destroy(); }
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020vec_x00000020_x0000003a_x0000003a_x00000020MoveOnly_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020vec_x00000020_x0000003a_x0000003a_x00000020MoveOnly_x00000020_x0000003e
inline ::rs::Vec<::vec::MoveOnly>::Vec(Vec&& other) noexcept : storage_{} {
  ::std::memcpy(storage_, other.storage_, sizeof(storage_));
  other.init_empty();
}
inline ::rs::Vec<::vec::MoveOnly>& ::rs::Vec<::vec::MoveOnly>::operator=(
    Vec&& other) noexcept {
  if (this != &other) {
    destroy();
    crubit::MemSwap(*this, other);
  }
  return *this;
}
inline ::rs::Vec<::vec::MoveOnly>::Vec(::crubit::UnsafeRelocateTag,
                                       Vec&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

inline ::rs::Vec<::vec::MoveOnly>::Vec() noexcept : storage_{} { init_empty(); }
inline ::rs::Vec<::vec::MoveOnly>::~Vec() noexcept { destroy(); }
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020vec_x00000020_x0000003a_x0000003a_x00000020NonDefault_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020vec_x00000020_x0000003a_x0000003a_x00000020NonDefault_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cvec_ugolden_x0000003a_x0000003aNonDefault_x0000003e(
    ::rs::Vec<::vec::NonDefault> const&,
    ::rs::Vec<::vec::NonDefault>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cvec_ugolden_x0000003a_x0000003aNonDefault_x0000003e(
    ::rs::Vec<::vec::NonDefault>&, ::rs::Vec<::vec::NonDefault> const&);
}
inline ::rs::Vec<::vec::NonDefault>::Vec(const Vec& other) noexcept {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cvec_ugolden_x0000003a_x0000003aNonDefault_x0000003e(
          other, this);
}
inline ::rs::Vec<::vec::NonDefault>& ::rs::Vec<::vec::NonDefault>::operator=(
    const Vec& other) noexcept {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cvec_ugolden_x0000003a_x0000003aNonDefault_x0000003e(
            *this, other);
  }
  return *this;
}
inline ::rs::Vec<::vec::NonDefault>::Vec(Vec&& other) noexcept : storage_{} {
  ::std::memcpy(storage_, other.storage_, sizeof(storage_));
  other.init_empty();
}
inline ::rs::Vec<::vec::NonDefault>& ::rs::Vec<::vec::NonDefault>::operator=(
    Vec&& other) noexcept {
  if (this != &other) {
    destroy();
    crubit::MemSwap(*this, other);
  }
  return *this;
}
inline ::rs::Vec<::vec::NonDefault>::Vec(::crubit::UnsafeRelocateTag,
                                         Vec&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

inline ::rs::Vec<::vec::NonDefault>::Vec() noexcept : storage_{} {
  init_empty();
}
inline ::rs::Vec<::vec::NonDefault>::~Vec() noexcept { destroy(); }
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020vec_x00000020_x0000003a_x0000003a_x00000020Tracked_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020vec_x00000020_x0000003a_x0000003a_x00000020Tracked_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cvec_ugolden_x0000003a_x0000003aTracked_x0000003e(
    ::rs::Vec<::vec::Tracked> const&, ::rs::Vec<::vec::Tracked>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cvec_ugolden_x0000003a_x0000003aTracked_x0000003e(
    ::rs::Vec<::vec::Tracked>&, ::rs::Vec<::vec::Tracked> const&);
}
inline ::rs::Vec<::vec::Tracked>::Vec(const Vec& other) noexcept {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cvec_ugolden_x0000003a_x0000003aTracked_x0000003e(
          other, this);
}
inline ::rs::Vec<::vec::Tracked>& ::rs::Vec<::vec::Tracked>::operator=(
    const Vec& other) noexcept {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cvec_ugolden_x0000003a_x0000003aTracked_x0000003e(
            *this, other);
  }
  return *this;
}
inline ::rs::Vec<::vec::Tracked>::Vec(Vec&& other) noexcept : storage_{} {
  ::std::memcpy(storage_, other.storage_, sizeof(storage_));
  other.init_empty();
}
inline ::rs::Vec<::vec::Tracked>& ::rs::Vec<::vec::Tracked>::operator=(
    Vec&& other) noexcept {
  if (this != &other) {
    destroy();
    crubit::MemSwap(*this, other);
  }
  return *this;
}
inline ::rs::Vec<::vec::Tracked>::Vec(::crubit::UnsafeRelocateTag,
                                      Vec&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

inline ::rs::Vec<::vec::Tracked>::Vec() noexcept : storage_{} { init_empty(); }
inline ::rs::Vec<::vec::Tracked>::~Vec() noexcept { destroy(); }
#endif

#pragma clang assume_nonnull end

#pragma clang diagnostic pop
#endif  // CC_BINDINGS_FROM_RS_TEST_VEC_VEC_GOLDEN
