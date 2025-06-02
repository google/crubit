// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// str_golden
// Features: supported, unsafe_types

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_STR_STR_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_STR_STR_GOLDEN

#include "support/internal/attribute_macros.h"
#include "support/internal/slot.h"
#include "support/rs_std/str_ref.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>
#include <utility>

namespace str {

// Generated from:
// cc_bindings_from_rs/test/str/str.rs;l=9
struct CRUBIT_INTERNAL_RUST_TYPE(":: str_golden :: TypeWithStr") alignas(8)
    [[clang::trivial_abi]] TypeWithStr final {
 public:
  // Default::default
  TypeWithStr();

  // No custom `Drop` impl and no custom \"drop glue\" required
  ~TypeWithStr() = default;
  TypeWithStr(TypeWithStr&&) = default;
  TypeWithStr& operator=(TypeWithStr&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  TypeWithStr(const TypeWithStr&) = default;
  TypeWithStr& operator=(const TypeWithStr&) = default;
  TypeWithStr(::crubit::UnsafeRelocateTag, TypeWithStr&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/str/str.rs;l=14
  static ::str::TypeWithStr create(rs_std::StrRef s);

  // Generated from:
  // cc_bindings_from_rs/test/str/str.rs;l=18
  std::uintptr_t get_str_len() const
      [[clang::annotate_type("lifetime", "__anon1")]];

  // Generated from:
  // cc_bindings_from_rs/test/str/str.rs;l=22
  std::uint8_t const* get_str_data() const
      [[clang::annotate_type("lifetime", "__anon1")]];

 private:
  // Field type has been replaced with a blob of bytes: Can't format `&'static
  // str`, because references are only supported in function parameter types,
  // return types, and consts (b/286256327)
  unsigned char str_field[16];

 private:
  static void __crubit_field_offset_assertions();
};

// Error generating bindings for `should_not_generate_bindings` defined at
// cc_bindings_from_rs/test/str/str.rs;l=28: support
// for functions taking a mutable reference, and which may alias in C++,
// requires //features:experimental

// Generated from:
// cc_bindings_from_rs/test/str/str.rs;l=30
std::uintptr_t get_str_len(rs_std::StrRef s);

// Generated from:
// cc_bindings_from_rs/test/str/str.rs;l=34
std::uint8_t const* get_str_data(rs_std::StrRef s);

// Generated from:
// cc_bindings_from_rs/test/str/str.rs;l=38
rs_std::StrRef foo_as_str();
constexpr rs_std::StrRef CONST_STR_FOO = rs_std::StrRef("foo");

// Error generating bindings for `STATIC_STR_FOO` defined at
// cc_bindings_from_rs/test/str/str.rs;l=44:
// Unsupported rustc_hir::hir::ItemKind: static item

static_assert(
    sizeof(TypeWithStr) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(TypeWithStr) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(::str::TypeWithStr* __ret_ptr);
}
inline TypeWithStr::TypeWithStr() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(std::is_trivially_destructible_v<TypeWithStr>);
static_assert(std::is_trivially_move_constructible_v<TypeWithStr>);
static_assert(std::is_trivially_move_assignable_v<TypeWithStr>);
static_assert(std::is_trivially_copy_constructible_v<TypeWithStr>);
static_assert(std::is_trivially_copy_assignable_v<TypeWithStr>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_create(rs_std::StrRef,
                                      ::str::TypeWithStr* __ret_ptr);
}
inline ::str::TypeWithStr TypeWithStr::create(rs_std::StrRef s) {
  crubit::Slot<::str::TypeWithStr> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(s, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" std::uintptr_t __crubit_thunk_get_ustr_ulen(
    ::str::TypeWithStr const& [[clang::annotate_type("lifetime", "__anon1")]]);
}
inline std::uintptr_t TypeWithStr::get_str_len() const
    [[clang::annotate_type("lifetime", "__anon1")]] {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_get_ustr_ulen(self);
}

namespace __crubit_internal {
extern "C" std::uint8_t const* __crubit_thunk_get_ustr_udata(
    ::str::TypeWithStr const& [[clang::annotate_type("lifetime", "__anon1")]]);
}
inline std::uint8_t const* TypeWithStr::get_str_data() const
    [[clang::annotate_type("lifetime", "__anon1")]] {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_get_ustr_udata(self);
}
inline void TypeWithStr::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(TypeWithStr, str_field));
}
namespace __crubit_internal {
extern "C" std::uintptr_t __crubit_thunk_get_ustr_ulen(rs_std::StrRef);
}
inline std::uintptr_t get_str_len(rs_std::StrRef s) {
  return __crubit_internal::__crubit_thunk_get_ustr_ulen(s);
}

namespace __crubit_internal {
extern "C" std::uint8_t const* __crubit_thunk_get_ustr_udata(rs_std::StrRef);
}
inline std::uint8_t const* get_str_data(rs_std::StrRef s) {
  return __crubit_internal::__crubit_thunk_get_ustr_udata(s);
}

namespace __crubit_internal {
extern "C" rs_std::StrRef __crubit_thunk_foo_uas_ustr();
}
inline rs_std::StrRef foo_as_str() {
  return __crubit_internal::__crubit_thunk_foo_uas_ustr();
}

}  // namespace str
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_STR_STR_GOLDEN
