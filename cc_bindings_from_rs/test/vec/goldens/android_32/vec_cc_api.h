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
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(":: alloc :: vec :: Vec < i32 >")
    rs_std::Vec<::std::int32_t> : public rs_std::VecBase<::std::int32_t> {
 public:
  // Default::default
  Vec() noexcept;

  // Clone::clone
  Vec(const Vec&);

  // Clone::clone_from
  rs_std::Vec<::std::int32_t>& operator=(const Vec&);

  Vec(Vec&&) noexcept;
  rs_std::Vec<::std::int32_t>& operator=(Vec&&) noexcept;
  Vec(::crubit::UnsafeRelocateTag, Vec&& value);

  ~Vec() noexcept;

 private:
  friend class rs_std::VecBase<::std::int32_t>;
  static constexpr std::size_t kPtrOffset = 4;
  static constexpr std::size_t kCapOffset = 0;
  static constexpr std::size_t kLenOffset = 8;
  unsigned char storage_[12];
};
#endif

namespace vec {

struct CRUBIT_INTERNAL_RUST_TYPE(":: vec_golden :: RustVecOwner") alignas(4)
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

  rs_std::Vec<::std::int32_t>& $(__anon1) vec_mut() &
      $(__anon1) CRUBIT_LIFETIME_BOUND;

  ::std::uintptr_t len() const;

  bool is_empty() const;

  ::std::int32_t element(::std::uintptr_t index) const;

 private:
  union {
    rs_std::Vec<::std::int32_t> v;
  };

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(":: vec_golden :: StructWithVec") alignas(4)
    [[clang::trivial_abi]] StructWithVec final {
 public:
  static ::vec::StructWithVec new_(::std::int32_t val);

  rs_std::Vec<::std::int32_t> v{};

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace vec

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(":: alloc :: vec :: Vec < u8 >")
    rs_std::Vec<::std::uint8_t> : public rs_std::VecBase<::std::uint8_t> {
 public:
  // Default::default
  Vec() noexcept;

  // Clone::clone
  Vec(const Vec&);

  // Clone::clone_from
  rs_std::Vec<::std::uint8_t>& operator=(const Vec&);

  Vec(Vec&&) noexcept;
  rs_std::Vec<::std::uint8_t>& operator=(Vec&&) noexcept;
  Vec(::crubit::UnsafeRelocateTag, Vec&& value);

  ~Vec() noexcept;

 private:
  friend class rs_std::VecBase<::std::uint8_t>;
  static constexpr std::size_t kPtrOffset = 4;
  static constexpr std::size_t kCapOffset = 0;
  static constexpr std::size_t kLenOffset = 8;
  unsigned char storage_[12];
};
#endif

namespace vec {

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
inline ::vec::RustVecOwner(RustVecOwner::new_)() {
  crubit::Slot<::vec::RustVecOwner> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" rs_std::Vec<::std::int32_t>& $(__anon1)
    __crubit_thunk_vec_umut(::vec::RustVecOwner&);
}
inline rs_std::Vec<::std::int32_t>& $(__anon1)(RustVecOwner::vec_mut)() &
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
__crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003ci32_x0000003e(
    rs_std::Vec<::std::int32_t> const&, rs_std::Vec<::std::int32_t>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003ci32_x0000003e(
    rs_std::Vec<::std::int32_t>&, rs_std::Vec<::std::int32_t> const&);
}
inline rs_std::Vec<::std::int32_t>::Vec(const Vec& other) {
  ::__crubit_internal::
      __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003ci32_x0000003e(
          other, this);
}
inline rs_std::Vec<::std::int32_t>& rs_std::Vec<::std::int32_t>::operator=(
    const Vec& other) {
  if (this != &other) {
    ::__crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003ci32_x0000003e(
            *this, other);
  }
  return *this;
}
inline rs_std::Vec<::std::int32_t>::Vec(Vec&& other) noexcept : storage_{} {
  ::std::memcpy(storage_, other.storage_, sizeof(storage_));
  other.init_empty();
}
inline rs_std::Vec<::std::int32_t>& rs_std::Vec<::std::int32_t>::operator=(
    Vec&& other) noexcept {
  if (this != &other) {
    destroy();
    crubit::MemSwap(*this, other);
  }
  return *this;
}
inline rs_std::Vec<::std::int32_t>::Vec(::crubit::UnsafeRelocateTag,
                                        Vec&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

inline rs_std::Vec<::std::int32_t>::Vec() noexcept : storage_{} {
  init_empty();
}
inline rs_std::Vec<::std::int32_t>::~Vec() noexcept { destroy(); }
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
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
  ::__crubit_internal::
      __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cu8_x0000003e(
          other, this);
}
inline rs_std::Vec<::std::uint8_t>& rs_std::Vec<::std::uint8_t>::operator=(
    const Vec& other) {
  if (this != &other) {
    ::__crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cu8_x0000003e(
            *this, other);
  }
  return *this;
}
inline rs_std::Vec<::std::uint8_t>::Vec(Vec&& other) noexcept : storage_{} {
  ::std::memcpy(storage_, other.storage_, sizeof(storage_));
  other.init_empty();
}
inline rs_std::Vec<::std::uint8_t>& rs_std::Vec<::std::uint8_t>::operator=(
    Vec&& other) noexcept {
  if (this != &other) {
    destroy();
    crubit::MemSwap(*this, other);
  }
  return *this;
}
inline rs_std::Vec<::std::uint8_t>::Vec(::crubit::UnsafeRelocateTag,
                                        Vec&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

inline rs_std::Vec<::std::uint8_t>::Vec() noexcept : storage_{} {
  init_empty();
}
inline rs_std::Vec<::std::uint8_t>::~Vec() noexcept { destroy(); }
#endif

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_VEC_VEC_GOLDEN
