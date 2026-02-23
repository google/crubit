// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// primitive_types_golden
// Features: callables, custom_ffi_types, experimental, fmt, non_unpin_ctor,
// std_unique_ptr, std_vector, supported, unsafe_view, wrapper

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_PRIMITIVE_TYPES_PRIMITIVE_TYPES_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_PRIMITIVE_TYPES_PRIMITIVE_TYPES_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#include "support/annotations_internal.h"
#include "support/internal/cxx20_backports.h"
#include "support/internal/slot.h"
#include "support/lifetime_annotations.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>
#include <utility>

#include "support/ffi_11/ffi_11.h"

namespace primitive_types::argument_types {

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=60
void c_char_mut_ptr_arg(decltype(char(0))* __param_0);

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=59
void c_char_ptr_arg(decltype(char(0)) const* __param_0);

}  // namespace primitive_types::argument_types

namespace primitive_types::field_types {

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=169
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
  ::primitive_types::field_types::Types& operator=(Types&&) = default;

  // `primitive_types_golden::field_types::Types` doesn't implement the `Clone`
  // trait
  Types(const Types&) = delete;
  Types& operator=(const Types&) = delete;
  Types(::crubit::UnsafeRelocateTag, Types&& value) {
    memcpy(this, &value, sizeof(value));
  }
  union {
    // Generated from:
    // cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=200
    crubit::type_identity_t<void(std::int8_t)>* i8_func;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=201
    crubit::type_identity_t<void(decltype(char(0)))>* c_char_func;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=170
    void* c_void_mut_ptr;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=171
    const void* c_void_const_ptr;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=180
    std::int64_t c_long;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=181
    std::uint64_t c_ulong;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=182
    long long c_longlong;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=183
    unsigned long long c_ulonglong;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=185
    double c_double;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=193
    std::int64_t i64;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=194
    std::uint64_t u64;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=195
    std::intptr_t isize;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=196
    std::uintptr_t usize;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=198
    double f64;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=178
    std::int32_t c_int;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=179
    std::uint32_t c_uint;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=184
    float c_float;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=191
    std::int32_t i32;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=192
    std::uint32_t u32;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=197
    float f32;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=176
    std::int16_t c_short;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=177
    std::uint16_t c_ushort;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=189
    std::int16_t i16;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=190
    std::uint16_t u16;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=173
    decltype(char(0)) c_char;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=174
    std::int8_t c_schar;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=175
    std::uint8_t c_uchar;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=187
    std::int8_t i8;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=188
    std::uint8_t u8;
  };

 private:
  unsigned char __padding16[3];

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace primitive_types::field_types

namespace primitive_types::return_types {

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=74
decltype(char(0)) c_char();

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=80
decltype(char(0)) const* c_char_const_ptr();

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=162
crubit::type_identity_t<void(decltype(char(0)))>& c_char_func();

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=77
decltype(char(0))* c_char_mut_ptr();

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=117
double c_double();

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=114
float c_float();

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=96
std::int32_t c_int();

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=102
std::int64_t c_long();

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=108
long long c_longlong();

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=84
std::int8_t c_schar();

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=90
std::int16_t c_short();

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=87
std::uint8_t c_uchar();

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=99
std::uint32_t c_uint();

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=105
std::uint64_t c_ulong();

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=111
unsigned long long c_ulonglong();

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=93
std::uint16_t c_ushort();

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=66
void c_void();

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=70
const void* c_void_const_ptr();

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=67
void* c_void_mut_ptr();

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=151
float f32();

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=154
double f64();

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=127
std::int16_t i16();

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=133
std::int32_t i32();

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=139
std::int64_t i64();

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=121
std::int8_t i8();

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=158
crubit::type_identity_t<void(std::int8_t)>& i8_func();

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=145
std::intptr_t isize();

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=130
std::uint16_t u16();

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=136
std::uint32_t u32();

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=142
std::uint64_t u64();

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=124
std::uint8_t u8();

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=148
std::uintptr_t usize();

}  // namespace primitive_types::return_types

namespace primitive_types::test_c_void_ptr {

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=9
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
  ::primitive_types::test_c_void_ptr::StructWithCVoidPointerMember& operator=(
      StructWithCVoidPointerMember&&) = default;

  // `primitive_types_golden::test_c_void_ptr::StructWithCVoidPointerMember`
  // doesn't implement the `Clone` trait
  StructWithCVoidPointerMember(const StructWithCVoidPointerMember&) = delete;
  StructWithCVoidPointerMember& operator=(const StructWithCVoidPointerMember&) =
      delete;
  StructWithCVoidPointerMember(::crubit::UnsafeRelocateTag,
                               StructWithCVoidPointerMember&& value) {
    memcpy(this, &value, sizeof(value));
  }
  union {
    // Generated from:
    // cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=10
    const void* ptr_const;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=11
    void* ptr_mut;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=23
const void* identity_const_c_void_ptr(const void* ptr);

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=26
void* identity_mut_c_void_ptr(void* ptr);

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=15
::primitive_types::test_c_void_ptr::StructWithCVoidPointerMember
new_struct_with_c_void_pointer_member(const void* ptr_const, void* ptr_mut);

}  // namespace primitive_types::test_c_void_ptr

namespace primitive_types::test_maybe_uninit {

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=44
std::int32_t const* maybe_uninit_ptr(std::int32_t const* maybe_uninit);

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=48
std::int32_t* maybe_uninit_ptr_mut(std::int32_t* maybe_uninit);

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=34
std::int32_t const& $static
maybe_uninit_ref(std::int32_t const* $static maybe_uninit);

// Generated from:
// cc_bindings_from_rs/test/primitive_types/primitive_types.rs;l=38
std::int32_t& $static maybe_uninit_ref_mut(std::int32_t* $static maybe_uninit);

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
static_assert(std::is_trivially_destructible_v<Types>);
static_assert(std::is_trivially_move_constructible_v<
              ::primitive_types::field_types::Types>);
static_assert(
    std::is_trivially_move_assignable_v<::primitive_types::field_types::Types>);
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
extern "C" std::int32_t __crubit_thunk_c_uint();
}
inline std::int32_t c_int() {
  return __crubit_internal::__crubit_thunk_c_uint();
}

namespace __crubit_internal {
extern "C" std::int64_t __crubit_thunk_c_ulong();
}
inline std::int64_t c_long() {
  return __crubit_internal::__crubit_thunk_c_ulong();
}

namespace __crubit_internal {
extern "C" long long __crubit_thunk_c_ulonglong();
}
inline long long c_longlong() {
  return __crubit_internal::__crubit_thunk_c_ulonglong();
}

namespace __crubit_internal {
extern "C" std::int8_t __crubit_thunk_c_uschar();
}
inline std::int8_t c_schar() {
  return __crubit_internal::__crubit_thunk_c_uschar();
}

namespace __crubit_internal {
extern "C" std::int16_t __crubit_thunk_c_ushort();
}
inline std::int16_t c_short() {
  return __crubit_internal::__crubit_thunk_c_ushort();
}

namespace __crubit_internal {
extern "C" std::uint8_t __crubit_thunk_c_uuchar();
}
inline std::uint8_t c_uchar() {
  return __crubit_internal::__crubit_thunk_c_uuchar();
}

namespace __crubit_internal {
extern "C" std::uint32_t __crubit_thunk_c_uuint();
}
inline std::uint32_t c_uint() {
  return __crubit_internal::__crubit_thunk_c_uuint();
}

namespace __crubit_internal {
extern "C" std::uint64_t __crubit_thunk_c_uulong();
}
inline std::uint64_t c_ulong() {
  return __crubit_internal::__crubit_thunk_c_uulong();
}

namespace __crubit_internal {
extern "C" unsigned long long __crubit_thunk_c_uulonglong();
}
inline unsigned long long c_ulonglong() {
  return __crubit_internal::__crubit_thunk_c_uulonglong();
}

namespace __crubit_internal {
extern "C" std::uint16_t __crubit_thunk_c_uushort();
}
inline std::uint16_t c_ushort() {
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
extern "C" std::int16_t __crubit_thunk_i16();
}
inline std::int16_t i16() { return __crubit_internal::__crubit_thunk_i16(); }

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_i32();
}
inline std::int32_t i32() { return __crubit_internal::__crubit_thunk_i32(); }

namespace __crubit_internal {
extern "C" std::int64_t __crubit_thunk_i64();
}
inline std::int64_t i64() { return __crubit_internal::__crubit_thunk_i64(); }

namespace __crubit_internal {
extern "C" std::int8_t __crubit_thunk_i8();
}
inline std::int8_t i8() { return __crubit_internal::__crubit_thunk_i8(); }

namespace __crubit_internal {
extern "C" crubit::type_identity_t<void(std::int8_t)>&
__crubit_thunk_i8_ufunc();
}
inline crubit::type_identity_t<void(std::int8_t)>& i8_func() {
  return __crubit_internal::__crubit_thunk_i8_ufunc();
}

namespace __crubit_internal {
extern "C" std::intptr_t __crubit_thunk_isize();
}
inline std::intptr_t isize() {
  return __crubit_internal::__crubit_thunk_isize();
}

namespace __crubit_internal {
extern "C" std::uint16_t __crubit_thunk_u16();
}
inline std::uint16_t u16() { return __crubit_internal::__crubit_thunk_u16(); }

namespace __crubit_internal {
extern "C" std::uint32_t __crubit_thunk_u32();
}
inline std::uint32_t u32() { return __crubit_internal::__crubit_thunk_u32(); }

namespace __crubit_internal {
extern "C" std::uint64_t __crubit_thunk_u64();
}
inline std::uint64_t u64() { return __crubit_internal::__crubit_thunk_u64(); }

namespace __crubit_internal {
extern "C" std::uint8_t __crubit_thunk_u8();
}
inline std::uint8_t u8() { return __crubit_internal::__crubit_thunk_u8(); }

namespace __crubit_internal {
extern "C" std::uintptr_t __crubit_thunk_usize();
}
inline std::uintptr_t usize() {
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
static_assert(std::is_trivially_destructible_v<StructWithCVoidPointerMember>);
static_assert(
    std::is_trivially_move_constructible_v<
        ::primitive_types::test_c_void_ptr::StructWithCVoidPointerMember>);
static_assert(
    std::is_trivially_move_assignable_v<
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
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

}  // namespace primitive_types::test_c_void_ptr

namespace primitive_types::test_maybe_uninit {

namespace __crubit_internal {
extern "C" std::int32_t const* __crubit_thunk_maybe_uuninit_uptr(
    std::int32_t const*);
}
inline std::int32_t const* maybe_uninit_ptr(std::int32_t const* maybe_uninit) {
  return __crubit_internal::__crubit_thunk_maybe_uuninit_uptr(maybe_uninit);
}

namespace __crubit_internal {
extern "C" std::int32_t* __crubit_thunk_maybe_uuninit_uptr_umut(std::int32_t*);
}
inline std::int32_t* maybe_uninit_ptr_mut(std::int32_t* maybe_uninit) {
  return __crubit_internal::__crubit_thunk_maybe_uuninit_uptr_umut(
      maybe_uninit);
}

namespace __crubit_internal {
extern "C" std::int32_t const& $static
__crubit_thunk_maybe_uuninit_uref(std::int32_t const* $static);
}
inline std::int32_t const& $static
maybe_uninit_ref(std::int32_t const* $static maybe_uninit) {
  return __crubit_internal::__crubit_thunk_maybe_uuninit_uref(maybe_uninit);
}

namespace __crubit_internal {
extern "C" std::int32_t& $static
__crubit_thunk_maybe_uuninit_uref_umut(std::int32_t* $static);
}
inline std::int32_t& $static
maybe_uninit_ref_mut(std::int32_t* $static maybe_uninit) {
  return __crubit_internal::__crubit_thunk_maybe_uuninit_uref_umut(
      maybe_uninit);
}

}  // namespace primitive_types::test_maybe_uninit

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_PRIMITIVE_TYPES_PRIMITIVE_TYPES_GOLDEN
