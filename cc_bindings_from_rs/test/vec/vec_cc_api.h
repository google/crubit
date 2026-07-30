// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// vec_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_VEC_VEC_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_VEC_VEC_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/check.h"
#include "support/internal/memswap.h"
#include "support/internal/slot.h"
#include "support/lifetime_annotations.h"
#include "support/rs_std/vec.h"

#include <bit>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <memory>
#include <new>
#include <type_traits>
#include <utility>

namespace vec {

void drop_vec(rs_std::Vec<::std::int32_t> _v);

rs_std::Vec<::std::int32_t> return_grown_vec();

rs_std::Vec<::std::uint8_t> return_u8_vec();

rs_std::Vec<::std::int32_t> return_vec();

void rust_add_elements(rs_std::Vec<::std::int32_t>& v);

::std::int32_t take_vec(rs_std::Vec<::std::int32_t> v);

}  // namespace vec

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    ":: alloc :: vec :: Vec < i32 >") rs_std::Vec<::std::int32_t>
    : public rs_std::VecBase<rs_std::Vec<::std::int32_t>, ::std::int32_t> {
 public:
  // Default::default
  Vec();

  // Clone::clone
  Vec(const Vec&);

  // Clone::clone_from
  rs_std::Vec<::std::int32_t>& operator=(const Vec&);

  Vec(Vec&&);
  rs_std::Vec<::std::int32_t>& operator=(Vec&&);
  Vec(::crubit::UnsafeRelocateTag, Vec&& value);

  ~Vec() noexcept;
  ::std::int32_t* data() noexcept;
  ::std::int32_t const* data() const noexcept;
  std::size_t size() const noexcept;
  std::size_t capacity() const noexcept;

 private:
  friend class rs_std::VecBase<rs_std::Vec<::std::int32_t>, ::std::int32_t>;
  void set_ptr(::std::int32_t* ptr) noexcept;
  void set_len(std::size_t len) noexcept;
  void set_cap(std::size_t cap) noexcept;

 private:
  unsigned char storage_[24];
};
#endif

namespace vec {

struct CRUBIT_INTERNAL_RUST_TYPE(":: vec_golden :: RustVecOwner") alignas(8)
    [[clang::trivial_abi]] RustVecOwner final {
 public:
  // Default::default
  RustVecOwner();

  // Drop::drop
  ~RustVecOwner();

  RustVecOwner(RustVecOwner&&);
  ::vec::RustVecOwner& operator=(RustVecOwner&&);

  // `vec_golden::RustVecOwner` doesn't implement the `Clone` trait
  RustVecOwner(const RustVecOwner&) = delete;
  RustVecOwner& operator=(const RustVecOwner&) = delete;
  RustVecOwner(::crubit::UnsafeRelocateTag, RustVecOwner&& value);

  static ::vec::RustVecOwner new_();

  rs_std::Vec<::std::int32_t>& $(__anon1) get_mut_vec() &
      $(__anon1) CRUBIT_LIFETIME_BOUND;

  ::std::uintptr_t get_len() const;

  ::std::int32_t get_element(::std::uintptr_t index) const;

 private:
  union {
    rs_std::Vec<::std::int32_t> v;
  };

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(":: vec_golden :: StructWithVec") alignas(8)
    [[clang::trivial_abi]] StructWithVec final {
 public:
  static ::vec::StructWithVec new_(::std::int32_t val);

  rs_std::Vec<::std::int32_t> v = {};

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace vec

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    ":: alloc :: vec :: Vec < u8 >") rs_std::Vec<::std::uint8_t>
    : public rs_std::VecBase<rs_std::Vec<::std::uint8_t>, ::std::uint8_t> {
 public:
  // Default::default
  Vec();

  // Clone::clone
  Vec(const Vec&);

  // Clone::clone_from
  rs_std::Vec<::std::uint8_t>& operator=(const Vec&);

  Vec(Vec&&);
  rs_std::Vec<::std::uint8_t>& operator=(Vec&&);
  Vec(::crubit::UnsafeRelocateTag, Vec&& value);

  ~Vec() noexcept;
  ::std::uint8_t* data() noexcept;
  ::std::uint8_t const* data() const noexcept;
  std::size_t size() const noexcept;
  std::size_t capacity() const noexcept;

 private:
  friend class rs_std::VecBase<rs_std::Vec<::std::uint8_t>, ::std::uint8_t>;
  void set_ptr(::std::uint8_t* ptr) noexcept;
  void set_len(std::size_t len) noexcept;
  void set_cap(std::size_t cap) noexcept;

 private:
  unsigned char storage_[24];
};
#endif

namespace vec {

static_assert(
    sizeof(RustVecOwner) == 24,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(RustVecOwner) == 8,
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
inline RustVecOwner::~RustVecOwner() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_uvec_ugolden_x0000003a_x0000003aRustVecOwner(
          *this);
}
inline ::vec::RustVecOwner::RustVecOwner(RustVecOwner&& other)
    : RustVecOwner() {
  *this = ::std::move(other);
}
inline ::vec::RustVecOwner& ::vec::RustVecOwner::operator=(
    RustVecOwner&& other) {
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
inline ::vec::RustVecOwner RustVecOwner::new_() {
  crubit::Slot<::vec::RustVecOwner> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" rs_std::Vec<::std::int32_t>& $(__anon1)
    __crubit_thunk_get_umut_uvec(::vec::RustVecOwner&);
}
inline rs_std::Vec<::std::int32_t>& $(__anon1) RustVecOwner::get_mut_vec() &
    $(__anon1) CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_get_umut_uvec(self);
}

namespace __crubit_internal {
extern "C" ::std::uintptr_t __crubit_thunk_get_ulen(::vec::RustVecOwner const&);
}
inline ::std::uintptr_t RustVecOwner::get_len() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_get_ulen(self);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_get_uelement(
    ::vec::RustVecOwner const&, ::std::uintptr_t);
}
inline ::std::int32_t RustVecOwner::get_element(::std::uintptr_t index) const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_get_uelement(self, index);
}
inline void RustVecOwner::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(RustVecOwner, v));
}
static_assert(
    sizeof(StructWithVec) == 24,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(StructWithVec) == 8,
    "Verify that ADT layout didn't change since this header got generated");

namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::std::int32_t,
                                   ::vec::StructWithVec* __ret_ptr);
}
inline ::vec::StructWithVec StructWithVec::new_(::std::int32_t val) {
  crubit::Slot<::vec::StructWithVec> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(val, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void StructWithVec::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(StructWithVec, v));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop_uvec(rs_std::Vec<::std::int32_t>*);
}
inline void drop_vec(rs_std::Vec<::std::int32_t> _v) {
  crubit::Slot _v_slot((::std::move(_v)));
  return __crubit_internal::__crubit_thunk_drop_uvec(_v_slot.Get());
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_ugrown_uvec(
    rs_std::Vec<::std::int32_t>* __ret_ptr);
}
inline rs_std::Vec<::std::int32_t> return_grown_vec() {
  crubit::Slot<rs_std::Vec<::std::int32_t>> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_return_ugrown_uvec(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_uu8_uvec(
    rs_std::Vec<::std::uint8_t>* __ret_ptr);
}
inline rs_std::Vec<::std::uint8_t> return_u8_vec() {
  crubit::Slot<rs_std::Vec<::std::uint8_t>> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_return_uu8_uvec(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_uvec(
    rs_std::Vec<::std::int32_t>* __ret_ptr);
}
inline rs_std::Vec<::std::int32_t> return_vec() {
  crubit::Slot<rs_std::Vec<::std::int32_t>> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_return_uvec(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_rust_uadd_uelements(
    rs_std::Vec<::std::int32_t>&);
}
inline void rust_add_elements(rs_std::Vec<::std::int32_t>& v) {
  return __crubit_internal::__crubit_thunk_rust_uadd_uelements(v);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_take_uvec(
    rs_std::Vec<::std::int32_t>*);
}
inline ::std::int32_t take_vec(rs_std::Vec<::std::int32_t> v) {
  crubit::Slot v_slot((::std::move(v)));
  return __crubit_internal::__crubit_thunk_take_uvec(v_slot.Get());
}

}  // namespace vec

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003ci32_x0000003e(
    rs_std::Vec<::std::int32_t>* __ret_ptr);
}
inline rs_std::Vec<::std::int32_t>::Vec() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003ci32_x0000003e(
          this);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003ci32_x0000003e(
    rs_std::Vec<::std::int32_t> const&, rs_std::Vec<::std::int32_t>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003ci32_x0000003e(
    rs_std::Vec<::std::int32_t>&, rs_std::Vec<::std::int32_t> const&);
}
inline rs_std::Vec<::std::int32_t>::Vec(const Vec& other) {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003ci32_x0000003e(
          other, this);
}
inline rs_std::Vec<::std::int32_t>& rs_std::Vec<::std::int32_t>::operator=(
    const Vec& other) {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003ci32_x0000003e(
            *this, other);
  }
  return *this;
}
inline rs_std::Vec<::std::int32_t>::Vec(Vec&& other) : Vec() {
  *this = ::std::move(other);
}
inline rs_std::Vec<::std::int32_t>& rs_std::Vec<::std::int32_t>::operator=(
    Vec&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
inline rs_std::Vec<::std::int32_t>::Vec(::crubit::UnsafeRelocateTag,
                                        Vec&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

inline rs_std::Vec<::std::int32_t>::~Vec() noexcept { destroy(); }
inline ::std::int32_t* rs_std::Vec<::std::int32_t>::data() noexcept {
  return std::bit_cast<::std::int32_t*>(
      *reinterpret_cast<const std::uintptr_t*>(&storage_[8]));
}
inline ::std::int32_t const* rs_std::Vec<::std::int32_t>::data()
    const noexcept {
  return std::bit_cast<::std::int32_t*>(
      *reinterpret_cast<const std::uintptr_t*>(&storage_[8]));
}
inline std::size_t rs_std::Vec<::std::int32_t>::size() const noexcept {
  return std::bit_cast<std::size_t>(
      *reinterpret_cast<const std::size_t*>(&storage_[16]));
}
inline std::size_t rs_std::Vec<::std::int32_t>::capacity() const noexcept {
  return std::bit_cast<std::size_t>(
      *reinterpret_cast<const std::size_t*>(&storage_[0]));
}
inline void rs_std::Vec<::std::int32_t>::set_ptr(::std::int32_t* ptr) noexcept {
  *reinterpret_cast<std::uintptr_t*>(&storage_[8]) =
      std::bit_cast<std::uintptr_t>(ptr);
}
inline void rs_std::Vec<::std::int32_t>::set_len(std::size_t len) noexcept {
  *reinterpret_cast<std::size_t*>(&storage_[16]) = len;
}
inline void rs_std::Vec<::std::int32_t>::set_cap(std::size_t cap) noexcept {
  *reinterpret_cast<std::size_t*>(&storage_[0]) = cap;
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cu8_x0000003e(
    rs_std::Vec<::std::uint8_t>* __ret_ptr);
}
inline rs_std::Vec<::std::uint8_t>::Vec() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cu8_x0000003e(
          this);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cu8_x0000003e(
    rs_std::Vec<::std::uint8_t> const&, rs_std::Vec<::std::uint8_t>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cu8_x0000003e(
    rs_std::Vec<::std::uint8_t>&, rs_std::Vec<::std::uint8_t> const&);
}
inline rs_std::Vec<::std::uint8_t>::Vec(const Vec& other) {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cu8_x0000003e(
          other, this);
}
inline rs_std::Vec<::std::uint8_t>& rs_std::Vec<::std::uint8_t>::operator=(
    const Vec& other) {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cu8_x0000003e(
            *this, other);
  }
  return *this;
}
inline rs_std::Vec<::std::uint8_t>::Vec(Vec&& other) : Vec() {
  *this = ::std::move(other);
}
inline rs_std::Vec<::std::uint8_t>& rs_std::Vec<::std::uint8_t>::operator=(
    Vec&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
inline rs_std::Vec<::std::uint8_t>::Vec(::crubit::UnsafeRelocateTag,
                                        Vec&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

inline rs_std::Vec<::std::uint8_t>::~Vec() noexcept { destroy(); }
inline ::std::uint8_t* rs_std::Vec<::std::uint8_t>::data() noexcept {
  return std::bit_cast<::std::uint8_t*>(
      *reinterpret_cast<const std::uintptr_t*>(&storage_[8]));
}
inline ::std::uint8_t const* rs_std::Vec<::std::uint8_t>::data()
    const noexcept {
  return std::bit_cast<::std::uint8_t*>(
      *reinterpret_cast<const std::uintptr_t*>(&storage_[8]));
}
inline std::size_t rs_std::Vec<::std::uint8_t>::size() const noexcept {
  return std::bit_cast<std::size_t>(
      *reinterpret_cast<const std::size_t*>(&storage_[16]));
}
inline std::size_t rs_std::Vec<::std::uint8_t>::capacity() const noexcept {
  return std::bit_cast<std::size_t>(
      *reinterpret_cast<const std::size_t*>(&storage_[0]));
}
inline void rs_std::Vec<::std::uint8_t>::set_ptr(::std::uint8_t* ptr) noexcept {
  *reinterpret_cast<std::uintptr_t*>(&storage_[8]) =
      std::bit_cast<std::uintptr_t>(ptr);
}
inline void rs_std::Vec<::std::uint8_t>::set_len(std::size_t len) noexcept {
  *reinterpret_cast<std::size_t*>(&storage_[16]) = len;
}
inline void rs_std::Vec<::std::uint8_t>::set_cap(std::size_t cap) noexcept {
  *reinterpret_cast<std::size_t*>(&storage_[0]) = cap;
}
#endif

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_VEC_VEC_GOLDEN
