// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// rs_default_golden
// Features: custom_ffi_types, non_unpin_ctor, std_unique_ptr, std_vector,
// supported

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_DEFAULT_RS_DEFAULT_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_DEFAULT_RS_DEFAULT_GOLDEN

#include "support/annotations_internal.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>

namespace rs_default {

namespace derived_impl {

// Generated from:
// cc_bindings_from_rs/test/known_traits/default/default.rs;l=28
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: rs_default_golden :: derived_impl :: SomeStruct") alignas(4)
    [[clang::trivial_abi]] SomeStruct final {
 public:
  // Default::default
  SomeStruct();

  // No custom `Drop` impl and no custom "drop glue" required
  ~SomeStruct() = default;
  SomeStruct(SomeStruct&&) = default;
  SomeStruct& operator=(SomeStruct&&) = default;

  // `rs_default_golden::derived_impl::SomeStruct` doesn't implement the `Clone`
  // trait
  SomeStruct(const SomeStruct&) = delete;
  SomeStruct& operator=(const SomeStruct&) = delete;
  SomeStruct(::crubit::UnsafeRelocateTag, SomeStruct&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/known_traits/default/default.rs;l=31
  static std::int32_t extract_int(::rs_default::derived_impl::SomeStruct s);

 private:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/known_traits/default/default.rs;l=28
    std::int32_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace derived_impl

namespace explicit_impl {

// Generated from:
// cc_bindings_from_rs/test/known_traits/default/default.rs;l=10
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: rs_default_golden :: explicit_impl :: SomeStruct") alignas(4)
    [[clang::trivial_abi]] SomeStruct final {
 public:
  // Default::default
  SomeStruct();

  // No custom `Drop` impl and no custom "drop glue" required
  ~SomeStruct() = default;
  SomeStruct(SomeStruct&&) = default;
  SomeStruct& operator=(SomeStruct&&) = default;

  // `rs_default_golden::explicit_impl::SomeStruct` doesn't implement the
  // `Clone` trait
  SomeStruct(const SomeStruct&) = delete;
  SomeStruct& operator=(const SomeStruct&) = delete;
  SomeStruct(::crubit::UnsafeRelocateTag, SomeStruct&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/known_traits/default/default.rs;l=19
  static std::int32_t extract_int(::rs_default::explicit_impl::SomeStruct s);

 private:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/known_traits/default/default.rs;l=10
    std::int32_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace explicit_impl

namespace field_with_no_default {

//  It is important that `StructWithoutDefault` is `pub` so that `field`
//
//  above is typed correctly in the C++ bindings and not replaced with a
//
//  blob of bytes.
//
// Generated from:
// cc_bindings_from_rs/test/known_traits/default/default.rs;l=53
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: rs_default_golden :: field_with_no_default :: "
    "StructWithoutDefault") alignas(4) [[clang::trivial_abi]]
StructWithoutDefault final {
 public:
  // `rs_default_golden::field_with_no_default::StructWithoutDefault` doesn't
  // implement the `Default` trait
  StructWithoutDefault() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~StructWithoutDefault() = default;
  StructWithoutDefault(StructWithoutDefault&&) = default;
  StructWithoutDefault& operator=(StructWithoutDefault&&) = default;

  // `rs_default_golden::field_with_no_default::StructWithoutDefault` doesn't
  // implement the `Clone` trait
  StructWithoutDefault(const StructWithoutDefault&) = delete;
  StructWithoutDefault& operator=(const StructWithoutDefault&) = delete;
  StructWithoutDefault(::crubit::UnsafeRelocateTag,
                       StructWithoutDefault&& value) {
    memcpy(this, &value, sizeof(value));
  }

 private:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/known_traits/default/default.rs;l=53
    std::int32_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/known_traits/default/default.rs;l=40
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: rs_default_golden :: field_with_no_default :: "
    "StructWithFieldWithNoDefault") alignas(4) [[clang::trivial_abi]]
StructWithFieldWithNoDefault final {
 public:
  // Default::default
  StructWithFieldWithNoDefault();

  // No custom `Drop` impl and no custom "drop glue" required
  ~StructWithFieldWithNoDefault() = default;
  StructWithFieldWithNoDefault(StructWithFieldWithNoDefault&&) = default;
  StructWithFieldWithNoDefault& operator=(StructWithFieldWithNoDefault&&) =
      default;

  // `rs_default_golden::field_with_no_default::StructWithFieldWithNoDefault`
  // doesn't implement the `Clone` trait
  StructWithFieldWithNoDefault(const StructWithFieldWithNoDefault&) = delete;
  StructWithFieldWithNoDefault& operator=(const StructWithFieldWithNoDefault&) =
      delete;
  StructWithFieldWithNoDefault(::crubit::UnsafeRelocateTag,
                               StructWithFieldWithNoDefault&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/known_traits/default/default.rs;l=56
  static std::int32_t extract_int(
      ::rs_default::field_with_no_default::StructWithFieldWithNoDefault s);

 private:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/known_traits/default/default.rs;l=41
    ::rs_default::field_with_no_default::StructWithoutDefault field;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace field_with_no_default

namespace no_impl {

// Generated from:
// cc_bindings_from_rs/test/known_traits/default/default.rs;l=64
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: rs_default_golden :: no_impl :: SomeStruct") alignas(4)
    [[clang::trivial_abi]] SomeStruct final {
 public:
  // `rs_default_golden::no_impl::SomeStruct` doesn't implement the `Default`
  // trait
  SomeStruct() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~SomeStruct() = default;
  SomeStruct(SomeStruct&&) = default;
  SomeStruct& operator=(SomeStruct&&) = default;

  // `rs_default_golden::no_impl::SomeStruct` doesn't implement the `Clone`
  // trait
  SomeStruct(const SomeStruct&) = delete;
  SomeStruct& operator=(const SomeStruct&) = delete;
  SomeStruct(::crubit::UnsafeRelocateTag, SomeStruct&& value) {
    memcpy(this, &value, sizeof(value));
  }

 private:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/known_traits/default/default.rs;l=64
    std::int32_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace no_impl

namespace transparent_struct {

// Generated from:
// cc_bindings_from_rs/test/known_traits/default/default.rs;l=70
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: rs_default_golden :: transparent_struct :: SomeStruct") alignas(4)
    [[clang::trivial_abi]] SomeStruct final {
 public:
  // Default::default
  SomeStruct();

  // No custom `Drop` impl and no custom "drop glue" required
  ~SomeStruct() = default;
  SomeStruct(SomeStruct&&) = default;
  SomeStruct& operator=(SomeStruct&&) = default;

  // `rs_default_golden::transparent_struct::SomeStruct` doesn't implement the
  // `Clone` trait
  SomeStruct(const SomeStruct&) = delete;
  SomeStruct& operator=(const SomeStruct&) = delete;
  SomeStruct(::crubit::UnsafeRelocateTag, SomeStruct&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/known_traits/default/default.rs;l=73
  std::int32_t extract_int() const;

 private:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/known_traits/default/default.rs;l=70
    std::int32_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace transparent_struct

namespace derived_impl {

static_assert(
    sizeof(SomeStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(SomeStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(
    ::rs_default::derived_impl::SomeStruct* __ret_ptr);
}
inline SomeStruct::SomeStruct() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(std::is_trivially_destructible_v<SomeStruct>);
static_assert(std::is_trivially_move_constructible_v<SomeStruct>);
static_assert(std::is_trivially_move_assignable_v<SomeStruct>);
namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_extract_uint(
    ::rs_default::derived_impl::SomeStruct*);
}
inline std::int32_t SomeStruct::extract_int(
    ::rs_default::derived_impl::SomeStruct s) {
  return __crubit_internal::__crubit_thunk_extract_uint(&s);
}
inline void SomeStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(SomeStruct, __field0));
}
}  // namespace derived_impl

namespace explicit_impl {

static_assert(
    sizeof(SomeStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(SomeStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(
    ::rs_default::explicit_impl::SomeStruct* __ret_ptr);
}
inline SomeStruct::SomeStruct() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(std::is_trivially_destructible_v<SomeStruct>);
static_assert(std::is_trivially_move_constructible_v<SomeStruct>);
static_assert(std::is_trivially_move_assignable_v<SomeStruct>);
namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_extract_uint(
    ::rs_default::explicit_impl::SomeStruct*);
}
inline std::int32_t SomeStruct::extract_int(
    ::rs_default::explicit_impl::SomeStruct s) {
  return __crubit_internal::__crubit_thunk_extract_uint(&s);
}
inline void SomeStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(SomeStruct, __field0));
}
}  // namespace explicit_impl

namespace field_with_no_default {

static_assert(
    sizeof(StructWithFieldWithNoDefault) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(StructWithFieldWithNoDefault) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(
    ::rs_default::field_with_no_default::StructWithFieldWithNoDefault*
        __ret_ptr);
}
inline StructWithFieldWithNoDefault::StructWithFieldWithNoDefault() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(std::is_trivially_destructible_v<StructWithFieldWithNoDefault>);
static_assert(
    std::is_trivially_move_constructible_v<StructWithFieldWithNoDefault>);
static_assert(
    std::is_trivially_move_assignable_v<StructWithFieldWithNoDefault>);
namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_extract_uint(
    ::rs_default::field_with_no_default::StructWithFieldWithNoDefault*);
}
inline std::int32_t StructWithFieldWithNoDefault::extract_int(
    ::rs_default::field_with_no_default::StructWithFieldWithNoDefault s) {
  return __crubit_internal::__crubit_thunk_extract_uint(&s);
}
inline void StructWithFieldWithNoDefault::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(StructWithFieldWithNoDefault, field));
}
static_assert(
    sizeof(StructWithoutDefault) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(StructWithoutDefault) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<StructWithoutDefault>);
static_assert(std::is_trivially_move_constructible_v<StructWithoutDefault>);
static_assert(std::is_trivially_move_assignable_v<StructWithoutDefault>);
inline void StructWithoutDefault::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(StructWithoutDefault, __field0));
}
}  // namespace field_with_no_default

namespace no_impl {

static_assert(
    sizeof(SomeStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(SomeStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<SomeStruct>);
static_assert(std::is_trivially_move_constructible_v<SomeStruct>);
static_assert(std::is_trivially_move_assignable_v<SomeStruct>);
inline void SomeStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(SomeStruct, __field0));
}
}  // namespace no_impl

namespace transparent_struct {

static_assert(
    sizeof(SomeStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(SomeStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" ::rs_default::transparent_struct::SomeStruct
__crubit_thunk_default();
}
inline SomeStruct::SomeStruct() {
  *this = __crubit_internal::__crubit_thunk_default();
}
static_assert(std::is_trivially_destructible_v<SomeStruct>);
static_assert(std::is_trivially_move_constructible_v<SomeStruct>);
static_assert(std::is_trivially_move_assignable_v<SomeStruct>);
namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_extract_uint(
    ::rs_default::transparent_struct::SomeStruct const&);
}
inline std::int32_t SomeStruct::extract_int() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_extract_uint(self);
}
inline void SomeStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(SomeStruct, __field0));
}
}  // namespace transparent_struct

}  // namespace rs_default
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_DEFAULT_RS_DEFAULT_GOLDEN
