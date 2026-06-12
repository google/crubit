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
#include "support/internal/offsetof.h"
#include "support/internal/slot.h"
#include "support/rs_std/vec.h"

#include <bit>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <utility>

namespace vec {

rs_std::Vec<::std::int32_t> return_vec();

::std::int32_t take_vec(rs_std::Vec<::std::int32_t> v);

}  // namespace vec

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    ":: alloc :: vec :: Vec < i32 >") rs_std::Vec<::std::int32_t> {
 public:
  // Default::default
  Vec();

  // Clone::clone
  Vec(const Vec&);

  // Clone::clone_from
  rs_std::Vec<::std::int32_t>& operator=(const Vec&);

  Vec(Vec&&);
  rs_std::Vec<::std::int32_t>& operator=(Vec&&);
  Vec(::crubit::UnsafeRelocateTag, Vec&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  ~Vec() noexcept;
  ::std::int32_t* data() noexcept;
  const ::std::int32_t* data() const noexcept;
  std::size_t size() const noexcept;
  ::std::int32_t& operator[](std::size_t index) noexcept;
  const ::std::int32_t& operator[](std::size_t index) const noexcept;
  ::std::int32_t* begin() noexcept;
  const ::std::int32_t* begin() const noexcept;
  ::std::int32_t* end() noexcept;
  const ::std::int32_t* end() const noexcept;

 private:
  unsigned char storage_[24];
};
#endif

namespace vec {

struct CRUBIT_INTERNAL_RUST_TYPE(":: vec_golden :: StructWithVec") alignas(8)
    [[clang::trivial_abi]] StructWithVec final {
 public:
  // `vec_golden::StructWithVec` doesn't implement the `Default` trait
  StructWithVec() = delete;

  // Drop::drop
  ~StructWithVec();

  // C++ move operations are unavailable for this type. See
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  StructWithVec(StructWithVec&&) = delete;
  ::vec::StructWithVec& operator=(StructWithVec&&) = delete;
  // `vec_golden::StructWithVec` doesn't implement the `Clone` trait
  StructWithVec(const StructWithVec&) = delete;
  StructWithVec& operator=(const StructWithVec&) = delete;
  StructWithVec(::crubit::UnsafeRelocateTag, StructWithVec&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  static ::vec::StructWithVec new_(::std::int32_t val);

  union {
    rs_std::Vec<::std::int32_t> v;
  };

 private:
  static void __crubit_field_offset_assertions();
};

static_assert(
    sizeof(StructWithVec) == 24,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(StructWithVec) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(::vec::StructWithVec&);
}
inline StructWithVec::~StructWithVec() {
  __crubit_internal::__crubit_thunk_drop(*this);
}
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
  CRUBIT_WARNING_PUSH("-Wno-invalid-offsetof")
  static_assert(0 == offsetof(StructWithVec, v));
  CRUBIT_WARNING_POP
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
extern "C" void __crubit_thunk_default(rs_std::Vec<::std::int32_t>* __ret_ptr);
}
inline rs_std::Vec<::std::int32_t>::Vec() {
  __crubit_internal::__crubit_thunk_default(this);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone(rs_std::Vec<::std::int32_t> const&,
                                     rs_std::Vec<::std::int32_t>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone_ufrom(rs_std::Vec<::std::int32_t>&,
                                           rs_std::Vec<::std::int32_t> const&);
}
inline rs_std::Vec<::std::int32_t>::Vec(const Vec& other) {
  __crubit_internal::__crubit_thunk_clone(other, this);
}
inline rs_std::Vec<::std::int32_t>& rs_std::Vec<::std::int32_t>::operator=(
    const Vec& other) {
  if (this != &other) {
    __crubit_internal::__crubit_thunk_clone_ufrom(*this, other);
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
extern "C" void
__crubit_drop_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e(
    void* vec) noexcept;
inline rs_std::Vec<::std::int32_t>::~Vec() noexcept {
  __crubit_drop_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e(
      this);
}
inline ::std::int32_t* rs_std::Vec<::std::int32_t>::data() noexcept {
  return std::bit_cast<::std::int32_t*>(
      *reinterpret_cast<const std::uintptr_t*>(&storage_[8]));
}
inline const ::std::int32_t* rs_std::Vec<::std::int32_t>::data()
    const noexcept {
  return std::bit_cast<::std::int32_t*>(
      *reinterpret_cast<const std::uintptr_t*>(&storage_[8]));
}
inline std::size_t rs_std::Vec<::std::int32_t>::size() const noexcept {
  return std::bit_cast<std::size_t>(
      *reinterpret_cast<const std::size_t*>(&storage_[16]));
}
inline ::std::int32_t& rs_std::Vec<::std::int32_t>::operator[](
    std::size_t index) noexcept {
  CRUBIT_CHECK(index < size());
  return data()[index];
}
inline const ::std::int32_t& rs_std::Vec<::std::int32_t>::operator[](
    std::size_t index) const noexcept {
  CRUBIT_CHECK(index < size());
  return data()[index];
}
inline ::std::int32_t* rs_std::Vec<::std::int32_t>::begin() noexcept {
  return data();
}
inline const ::std::int32_t* rs_std::Vec<::std::int32_t>::begin()
    const noexcept {
  return data();
}
inline ::std::int32_t* rs_std::Vec<::std::int32_t>::end() noexcept {
  return data() + size();
}
inline const ::std::int32_t* rs_std::Vec<::std::int32_t>::end() const noexcept {
  return data() + size();
}
#endif

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_VEC_VEC_GOLDEN
