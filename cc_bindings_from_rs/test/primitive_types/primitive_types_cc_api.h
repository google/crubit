// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// primitive_types_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_PRIMITIVE_TYPES_PRIMITIVE_TYPES_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_PRIMITIVE_TYPES_PRIMITIVE_TYPES_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/cxx20_backports.h"
#include "support/internal/slot.h"
#include "support/lifetime_annotations.h"

#include <cstddef>
#include <cstdint>
#include <cstring>
#include <type_traits>
#include <utility>

#include "support/ffi_11/ffi_11.h"

namespace primitive_types::argument_types {

void c_char_mut_ptr_arg(decltype(char(0))* __param_0);

void c_char_ptr_arg(decltype(char(0)) const* __param_0);

}  // namespace primitive_types::argument_types

namespace primitive_types::field_types {

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: primitive_types_golden :: field_types :: Types") alignas(8)
    [[clang::trivial_abi]] Types final {
 public:
  // `primitive_types_golden::field_types::Types` doesn't implement the
  // `Default` trait
  Types() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~Types() = default;
  Types(Types&&) = default;
  Types& operator=(Types&&) = default;

  // `primitive_types_golden::field_types::Types` doesn't implement the `Clone`
  // trait
  Types(const Types&) = delete;
  Types& operator=(const Types&) = delete;
  Types(::crubit::UnsafeRelocateTag, Types&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  union {
    crubit::type_identity_t<void(::std::int8_t)>* i8_func;
  };
  union {
    crubit::type_identity_t<void(decltype(char(0)))>* c_char_func;
  };
  union {
    void* c_void_mut_ptr;
  };
  union {
    const void* c_void_const_ptr;
  };
  union {
    ::std::int64_t c_long;
  };
  union {
    ::std::uint64_t c_ulong;
  };
  union {
    long long c_longlong;
  };
  union {
    unsigned long long c_ulonglong;
  };
  union {
    double c_double;
  };
  union {
    ::std::int64_t i64;
  };
  union {
    ::std::uint64_t u64;
  };
  union {
    ::std::intptr_t isize;
  };
  union {
    ::std::uintptr_t usize;
  };
  union {
    double f64;
  };
  union {
    ::std::int32_t c_int;
  };
  union {
    ::std::uint32_t c_uint;
  };
  union {
    float c_float;
  };
  union {
    ::std::int32_t i32;
  };
  union {
    ::std::uint32_t u32;
  };
  union {
    float f32;
  };
  union {
    ::std::int16_t c_short;
  };
  union {
    ::std::uint16_t c_ushort;
  };
  union {
    ::std::int16_t i16;
  };
  union {
    ::std::uint16_t u16;
  };
  union {
    decltype(char(0)) c_char;
  };
  union {
    ::std::int8_t c_schar;
  };
  union {
    ::std::uint8_t c_uchar;
  };
  union {
    ::std::int8_t i8;
  };
  union {
    ::std::uint8_t u8;
  };

 private:
  unsigned char __padding16[3];

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace primitive_types::field_types

namespace primitive_types::return_types {

decltype(char(0)) c_char();

decltype(char(0)) const* c_char_const_ptr();

crubit::type_identity_t<void(decltype(char(0)))>& c_char_func();

decltype(char(0))* c_char_mut_ptr();

double c_double();

float c_float();

::std::int32_t c_int();

::std::int64_t c_long();

long long c_longlong();

::std::int8_t c_schar();

::std::int16_t c_short();

::std::uint8_t c_uchar();

::std::uint32_t c_uint();

::std::uint64_t c_ulong();

unsigned long long c_ulonglong();

::std::uint16_t c_ushort();

void c_void();

const void* c_void_const_ptr();

void* c_void_mut_ptr();

float f32();

double f64();

::std::int16_t i16();

::std::int32_t i32();

::std::int64_t i64();

::std::int8_t i8();

crubit::type_identity_t<void(::std::int8_t)>& i8_func();

::std::intptr_t isize();

::std::uint16_t u16();

::std::uint32_t u32();

::std::uint64_t u64();

::std::uint8_t u8();

::std::uintptr_t usize();

}  // namespace primitive_types::return_types

namespace primitive_types::test_c_void_ptr {

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: primitive_types_golden :: test_c_void_ptr :: "
    "StructWithCVoidPointerMember") alignas(8) [[clang::trivial_abi]]
StructWithCVoidPointerMember final {
 public:
  // `primitive_types_golden::test_c_void_ptr::StructWithCVoidPointerMember`
  // doesn't implement the `Default` trait
  StructWithCVoidPointerMember() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~StructWithCVoidPointerMember() = default;
  StructWithCVoidPointerMember(StructWithCVoidPointerMember&&) = default;
  StructWithCVoidPointerMember& operator=(StructWithCVoidPointerMember&&) =
      default;

  // `primitive_types_golden::test_c_void_ptr::StructWithCVoidPointerMember`
  // doesn't implement the `Clone` trait
  StructWithCVoidPointerMember(const StructWithCVoidPointerMember&) = delete;
  StructWithCVoidPointerMember& operator=(const StructWithCVoidPointerMember&) =
      delete;
  StructWithCVoidPointerMember(::crubit::UnsafeRelocateTag,
                               StructWithCVoidPointerMember&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  union {
    const void* ptr_const;
  };
  union {
    void* ptr_mut;
  };

 private:
  static void __crubit_field_offset_assertions();
};

const void* identity_const_c_void_ptr(const void* ptr);

void* identity_mut_c_void_ptr(void* ptr);

::primitive_types::test_c_void_ptr::StructWithCVoidPointerMember
new_struct_with_c_void_pointer_member(const void* ptr_const, void* ptr_mut);

}  // namespace primitive_types::test_c_void_ptr

namespace primitive_types::test_maybe_uninit {

::std::int32_t const* maybe_uninit_ptr(::std::int32_t const* maybe_uninit);

::std::int32_t* maybe_uninit_ptr_mut(::std::int32_t* maybe_uninit);

::std::int32_t const& $static
maybe_uninit_ref(::std::int32_t const* $static maybe_uninit);

::std::int32_t& $static
maybe_uninit_ref_mut(::std::int32_t* $static maybe_uninit);

}  // namespace primitive_types::test_maybe_uninit

namespace primitive_types::argument_types {

namespace __crubit_internal {
extern "C" void __crubit_thunk_c_uchar_umut_uptr_uarg(decltype(char(0))*);
}
inline void c_char_mut_ptr_arg(decltype(char(0))* __param_0) {
  return __crubit_internal::__crubit_thunk_c_uchar_umut_uptr_uarg(__param_0);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_c_uchar_uptr_uarg(decltype(char(0)) const*);
}
inline void c_char_ptr_arg(decltype(char(0)) const* __param_0) {
  return __crubit_internal::__crubit_thunk_c_uchar_uptr_uarg(__param_0);
}

}  // namespace primitive_types::argument_types

namespace primitive_types::field_types {

static_assert(
    sizeof(Types) == 152,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Types) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<Types>);
static_assert(::std::is_trivially_move_constructible_v<
              ::primitive_types::field_types::Types>);
static_assert(::std::is_trivially_move_assignable_v<
              ::primitive_types::field_types::Types>);
inline void Types::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Types, i8_func));
  static_assert(8 == offsetof(Types, c_char_func));
  static_assert(16 == offsetof(Types, c_void_mut_ptr));
  static_assert(24 == offsetof(Types, c_void_const_ptr));
  static_assert(32 == offsetof(Types, c_long));
  static_assert(40 == offsetof(Types, c_ulong));
  static_assert(48 == offsetof(Types, c_longlong));
  static_assert(56 == offsetof(Types, c_ulonglong));
  static_assert(64 == offsetof(Types, c_double));
  static_assert(72 == offsetof(Types, i64));
  static_assert(80 == offsetof(Types, u64));
  static_assert(88 == offsetof(Types, isize));
  static_assert(96 == offsetof(Types, usize));
  static_assert(104 == offsetof(Types, f64));
  static_assert(112 == offsetof(Types, c_int));
  static_assert(116 == offsetof(Types, c_uint));
  static_assert(120 == offsetof(Types, c_float));
  static_assert(124 == offsetof(Types, i32));
  static_assert(128 == offsetof(Types, u32));
  static_assert(132 == offsetof(Types, f32));
  static_assert(136 == offsetof(Types, c_short));
  static_assert(138 == offsetof(Types, c_ushort));
  static_assert(140 == offsetof(Types, i16));
  static_assert(142 == offsetof(Types, u16));
  static_assert(144 == offsetof(Types, c_char));
  static_assert(145 == offsetof(Types, c_schar));
  static_assert(146 == offsetof(Types, c_uchar));
  static_assert(147 == offsetof(Types, i8));
  static_assert(148 == offsetof(Types, u8));
}
}  // namespace primitive_types::field_types

namespace primitive_types::return_types {

namespace __crubit_internal {
extern "C" decltype(char(0)) __crubit_thunk_c_uchar();
}
inline decltype(char(0)) c_char() {
  return __crubit_internal::__crubit_thunk_c_uchar();
}

namespace __crubit_internal {
extern "C" decltype(char(0)) const* __crubit_thunk_c_uchar_uconst_uptr();
}
inline decltype(char(0)) const* c_char_const_ptr() {
  return __crubit_internal::__crubit_thunk_c_uchar_uconst_uptr();
}

namespace __crubit_internal {
extern "C" crubit::type_identity_t<void(decltype(char(0)))>&
__crubit_thunk_c_uchar_ufunc();
}
inline crubit::type_identity_t<void(decltype(char(0)))>& c_char_func() {
  return __crubit_internal::__crubit_thunk_c_uchar_ufunc();
}

namespace __crubit_internal {
extern "C" decltype(char(0))* __crubit_thunk_c_uchar_umut_uptr();
}
inline decltype(char(0))* c_char_mut_ptr() {
  return __crubit_internal::__crubit_thunk_c_uchar_umut_uptr();
}

namespace __crubit_internal {
extern "C" double __crubit_thunk_c_udouble();
}
inline double c_double() {
  return __crubit_internal::__crubit_thunk_c_udouble();
}

namespace __crubit_internal {
extern "C" float __crubit_thunk_c_ufloat();
}
inline float c_float() { return __crubit_internal::__crubit_thunk_c_ufloat(); }

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_c_uint();
}
inline ::std::int32_t c_int() {
  return __crubit_internal::__crubit_thunk_c_uint();
}

namespace __crubit_internal {
extern "C" ::std::int64_t __crubit_thunk_c_ulong();
}
inline ::std::int64_t c_long() {
  return __crubit_internal::__crubit_thunk_c_ulong();
}

namespace __crubit_internal {
extern "C" long long __crubit_thunk_c_ulonglong();
}
inline long long c_longlong() {
  return __crubit_internal::__crubit_thunk_c_ulonglong();
}

namespace __crubit_internal {
extern "C" ::std::int8_t __crubit_thunk_c_uschar();
}
inline ::std::int8_t c_schar() {
  return __crubit_internal::__crubit_thunk_c_uschar();
}

namespace __crubit_internal {
extern "C" ::std::int16_t __crubit_thunk_c_ushort();
}
inline ::std::int16_t c_short() {
  return __crubit_internal::__crubit_thunk_c_ushort();
}

namespace __crubit_internal {
extern "C" ::std::uint8_t __crubit_thunk_c_uuchar();
}
inline ::std::uint8_t c_uchar() {
  return __crubit_internal::__crubit_thunk_c_uuchar();
}

namespace __crubit_internal {
extern "C" ::std::uint32_t __crubit_thunk_c_uuint();
}
inline ::std::uint32_t c_uint() {
  return __crubit_internal::__crubit_thunk_c_uuint();
}

namespace __crubit_internal {
extern "C" ::std::uint64_t __crubit_thunk_c_uulong();
}
inline ::std::uint64_t c_ulong() {
  return __crubit_internal::__crubit_thunk_c_uulong();
}

namespace __crubit_internal {
extern "C" unsigned long long __crubit_thunk_c_uulonglong();
}
inline unsigned long long c_ulonglong() {
  return __crubit_internal::__crubit_thunk_c_uulonglong();
}

namespace __crubit_internal {
extern "C" ::std::uint16_t __crubit_thunk_c_uushort();
}
inline ::std::uint16_t c_ushort() {
  return __crubit_internal::__crubit_thunk_c_uushort();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_c_uvoid();
}
inline void c_void() { return __crubit_internal::__crubit_thunk_c_uvoid(); }

namespace __crubit_internal {
extern "C" const void* __crubit_thunk_c_uvoid_uconst_uptr();
}
inline const void* c_void_const_ptr() {
  return __crubit_internal::__crubit_thunk_c_uvoid_uconst_uptr();
}

namespace __crubit_internal {
extern "C" void* __crubit_thunk_c_uvoid_umut_uptr();
}
inline void* c_void_mut_ptr() {
  return __crubit_internal::__crubit_thunk_c_uvoid_umut_uptr();
}

namespace __crubit_internal {
extern "C" float __crubit_thunk_f32();
}
inline float f32() { return __crubit_internal::__crubit_thunk_f32(); }

namespace __crubit_internal {
extern "C" double __crubit_thunk_f64();
}
inline double f64() { return __crubit_internal::__crubit_thunk_f64(); }

namespace __crubit_internal {
extern "C" ::std::int16_t __crubit_thunk_i16();
}
inline ::std::int16_t i16() { return __crubit_internal::__crubit_thunk_i16(); }

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_i32();
}
inline ::std::int32_t i32() { return __crubit_internal::__crubit_thunk_i32(); }

namespace __crubit_internal {
extern "C" ::std::int64_t __crubit_thunk_i64();
}
inline ::std::int64_t i64() { return __crubit_internal::__crubit_thunk_i64(); }

namespace __crubit_internal {
extern "C" ::std::int8_t __crubit_thunk_i8();
}
inline ::std::int8_t i8() { return __crubit_internal::__crubit_thunk_i8(); }

namespace __crubit_internal {
extern "C" crubit::type_identity_t<void(::std::int8_t)>&
__crubit_thunk_i8_ufunc();
}
inline crubit::type_identity_t<void(::std::int8_t)>& i8_func() {
  return __crubit_internal::__crubit_thunk_i8_ufunc();
}

namespace __crubit_internal {
extern "C" ::std::intptr_t __crubit_thunk_isize();
}
inline ::std::intptr_t isize() {
  return __crubit_internal::__crubit_thunk_isize();
}

namespace __crubit_internal {
extern "C" ::std::uint16_t __crubit_thunk_u16();
}
inline ::std::uint16_t u16() { return __crubit_internal::__crubit_thunk_u16(); }

namespace __crubit_internal {
extern "C" ::std::uint32_t __crubit_thunk_u32();
}
inline ::std::uint32_t u32() { return __crubit_internal::__crubit_thunk_u32(); }

namespace __crubit_internal {
extern "C" ::std::uint64_t __crubit_thunk_u64();
}
inline ::std::uint64_t u64() { return __crubit_internal::__crubit_thunk_u64(); }

namespace __crubit_internal {
extern "C" ::std::uint8_t __crubit_thunk_u8();
}
inline ::std::uint8_t u8() { return __crubit_internal::__crubit_thunk_u8(); }

namespace __crubit_internal {
extern "C" ::std::uintptr_t __crubit_thunk_usize();
}
inline ::std::uintptr_t usize() {
  return __crubit_internal::__crubit_thunk_usize();
}

}  // namespace primitive_types::return_types

namespace primitive_types::test_c_void_ptr {

static_assert(
    sizeof(StructWithCVoidPointerMember) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(StructWithCVoidPointerMember) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<StructWithCVoidPointerMember>);
static_assert(
    ::std::is_trivially_move_constructible_v<
        ::primitive_types::test_c_void_ptr::StructWithCVoidPointerMember>);
static_assert(
    ::std::is_trivially_move_assignable_v<
        ::primitive_types::test_c_void_ptr::StructWithCVoidPointerMember>);
inline void StructWithCVoidPointerMember::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(StructWithCVoidPointerMember, ptr_const));
  static_assert(8 == offsetof(StructWithCVoidPointerMember, ptr_mut));
}
namespace __crubit_internal {
extern "C" const void* __crubit_thunk_identity_uconst_uc_uvoid_uptr(
    const void*);
}
inline const void* identity_const_c_void_ptr(const void* ptr) {
  return __crubit_internal::__crubit_thunk_identity_uconst_uc_uvoid_uptr(ptr);
}

namespace __crubit_internal {
extern "C" void* __crubit_thunk_identity_umut_uc_uvoid_uptr(void*);
}
inline void* identity_mut_c_void_ptr(void* ptr) {
  return __crubit_internal::__crubit_thunk_identity_umut_uc_uvoid_uptr(ptr);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_new_ustruct_uwith_uc_uvoid_upointer_umember(
    const void*, void*,
    ::primitive_types::test_c_void_ptr::StructWithCVoidPointerMember*
        __ret_ptr);
}
inline ::primitive_types::test_c_void_ptr::StructWithCVoidPointerMember
new_struct_with_c_void_pointer_member(const void* ptr_const, void* ptr_mut) {
  crubit::Slot<::primitive_types::test_c_void_ptr::StructWithCVoidPointerMember>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new_ustruct_uwith_uc_uvoid_upointer_umember(
      ptr_const, ptr_mut, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

}  // namespace primitive_types::test_c_void_ptr

namespace primitive_types::test_maybe_uninit {

namespace __crubit_internal {
extern "C" ::std::int32_t const* __crubit_thunk_maybe_uuninit_uptr(
    ::std::int32_t const*);
}
inline ::std::int32_t const* maybe_uninit_ptr(
    ::std::int32_t const* maybe_uninit) {
  return __crubit_internal::__crubit_thunk_maybe_uuninit_uptr(maybe_uninit);
}

namespace __crubit_internal {
extern "C" ::std::int32_t* __crubit_thunk_maybe_uuninit_uptr_umut(
    ::std::int32_t*);
}
inline ::std::int32_t* maybe_uninit_ptr_mut(::std::int32_t* maybe_uninit) {
  return __crubit_internal::__crubit_thunk_maybe_uuninit_uptr_umut(
      maybe_uninit);
}

namespace __crubit_internal {
extern "C" ::std::int32_t const& $static
__crubit_thunk_maybe_uuninit_uref(::std::int32_t const* $static);
}
inline ::std::int32_t const& $static
maybe_uninit_ref(::std::int32_t const* $static maybe_uninit) {
  return __crubit_internal::__crubit_thunk_maybe_uuninit_uref(maybe_uninit);
}

namespace __crubit_internal {
extern "C" ::std::int32_t& $static
__crubit_thunk_maybe_uuninit_uref_umut(::std::int32_t* $static);
}
inline ::std::int32_t& $static
maybe_uninit_ref_mut(::std::int32_t* $static maybe_uninit) {
  return __crubit_internal::__crubit_thunk_maybe_uuninit_uref_umut(
      maybe_uninit);
}

}  // namespace primitive_types::test_maybe_uninit

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_PRIMITIVE_TYPES_PRIMITIVE_TYPES_GOLDEN
