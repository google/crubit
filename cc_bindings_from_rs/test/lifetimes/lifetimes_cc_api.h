// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// lifetimes_golden
// Features: supported, unsafe_types

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_LIFETIMES_LIFETIMES_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_LIFETIMES_LIFETIMES_GOLDEN

#include "support/internal/attribute_macros.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>
#include <utility>

namespace lifetimes {

// Generated from:
// cc_bindings_from_rs/test/lifetimes/lifetimes.rs;l=9
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: lifetimes_golden :: StructWithLifetime") alignas(8)
    [[clang::trivial_abi]] StructWithLifetime final {
 public:
  // `StructWithLifetime<'_>` doesn't implement the `Default` trait
  StructWithLifetime() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~StructWithLifetime() = default;
  StructWithLifetime(StructWithLifetime&&) = default;
  StructWithLifetime& operator=(StructWithLifetime&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  StructWithLifetime(const StructWithLifetime&) = default;
  StructWithLifetime& operator=(const StructWithLifetime&) = default;
  StructWithLifetime(::crubit::UnsafeRelocateTag, StructWithLifetime&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Error generating bindings for `StructWithLifetime::<'a>::from_ref` defined
  // at
  // cc_bindings_from_rs/test/lifetimes/lifetimes.rs;l=32:
  // support for multiple uses of a lifetime parameter requires
  // //features:experimental

  // Error generating bindings for `StructWithLifetime::<'a>::into_ref` defined
  // at
  // cc_bindings_from_rs/test/lifetimes/lifetimes.rs;l=36:
  // support for references of non-function-param types requires
  // //features:experimental

  // Generated from:
  // cc_bindings_from_rs/test/lifetimes/lifetimes.rs;l=40
  std::int32_t value() const;

  // Generated from:
  // cc_bindings_from_rs/test/lifetimes/lifetimes.rs;l=46
  static ::lifetimes::StructWithLifetime make_static_42();

  // Error generating bindings for
  // `StructWithLifetime::<'static>::from_static_ref` defined at
  // cc_bindings_from_rs/test/lifetimes/lifetimes.rs;l=50:
  // support for multiple uses of a lifetime parameter requires
  // //features:experimental, support for bound reference
  // lifetimes (such as 'static) requires
  // //features:experimental

  // Generated from:
  // cc_bindings_from_rs/test/lifetimes/lifetimes.rs;l=54
  static ::lifetimes::StructWithLifetime from_static_ref_where_bound(
      std::int32_t const& [[clang::annotate_type("lifetime",
                                                 "a")]] field_with_lifetime);

 private:
  // Field type has been replaced with a blob of bytes: Can't format `&i32`,
  // because references are only supported in function parameter types, return
  // types, and consts (b/286256327)
  unsigned char field_with_lifetime[8];

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/lifetimes/lifetimes.rs;l=62
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: lifetimes_golden :: StructWithLifetimeAndDropGlue") alignas(8)
    [[clang::trivial_abi]] StructWithLifetimeAndDropGlue final {
 public:
  // `StructWithLifetimeAndDropGlue<'_>` doesn't implement the `Default` trait
  StructWithLifetimeAndDropGlue() = delete;

  // Drop::drop
  ~StructWithLifetimeAndDropGlue();

  // C++ moves are deleted because there's no non-destructive implementation
  // available.
  StructWithLifetimeAndDropGlue(StructWithLifetimeAndDropGlue&&) = delete;
  StructWithLifetimeAndDropGlue& operator=(StructWithLifetimeAndDropGlue&&) =
      delete;
  // `StructWithLifetimeAndDropGlue<'_>` doesn't implement the `Clone` trait
  StructWithLifetimeAndDropGlue(const StructWithLifetimeAndDropGlue&) = delete;
  StructWithLifetimeAndDropGlue& operator=(
      const StructWithLifetimeAndDropGlue&) = delete;
  StructWithLifetimeAndDropGlue(::crubit::UnsafeRelocateTag,
                                StructWithLifetimeAndDropGlue&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/lifetimes/lifetimes.rs;l=68
  static ::lifetimes::StructWithLifetimeAndDropGlue make_static_42();

 private:
  // Field type has been replaced with a blob of bytes: Type
  // `std::string::String` comes from the `alloc` crate, but no `--crate-header`
  // was specified for this crate
  unsigned char field_with_drop_glue[24];
  // Field type has been replaced with a blob of bytes: Can't format `&i32`,
  // because references are only supported in function parameter types, return
  // types, and consts (b/286256327)
  unsigned char field_with_lifetime[8];

 private:
  static void __crubit_field_offset_assertions();
};

static_assert(
    sizeof(StructWithLifetime) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(StructWithLifetime) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<StructWithLifetime>);
static_assert(std::is_trivially_move_constructible_v<StructWithLifetime>);
static_assert(std::is_trivially_move_assignable_v<StructWithLifetime>);
static_assert(std::is_trivially_copy_constructible_v<StructWithLifetime>);
static_assert(std::is_trivially_copy_assignable_v<StructWithLifetime>);
namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_value(::lifetimes::StructWithLifetime*);
}
inline std::int32_t StructWithLifetime::value() const {
  auto& self = const_cast<std::remove_cvref_t<decltype(*this)>&>(*this);
  return __crubit_internal::__crubit_thunk_value(&self);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_make_ustatic_u42(
    ::lifetimes::StructWithLifetime* __ret_ptr);
}
inline ::lifetimes::StructWithLifetime StructWithLifetime::make_static_42() {
  crubit::Slot<::lifetimes::StructWithLifetime> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_make_ustatic_u42(__return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_from_ustatic_uref_uwhere_ubound(
    std::int32_t const& [[clang::annotate_type("lifetime", "a")]],
    ::lifetimes::StructWithLifetime* __ret_ptr);
}
inline ::lifetimes::StructWithLifetime
StructWithLifetime::from_static_ref_where_bound(
    std::int32_t const& [[clang::annotate_type("lifetime",
                                               "a")]] field_with_lifetime) {
  crubit::Slot<::lifetimes::StructWithLifetime> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_from_ustatic_uref_uwhere_ubound(
      field_with_lifetime, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void StructWithLifetime::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(StructWithLifetime, field_with_lifetime));
}
static_assert(
    sizeof(StructWithLifetimeAndDropGlue) == 32,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(StructWithLifetimeAndDropGlue) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(
    ::lifetimes::StructWithLifetimeAndDropGlue& [[clang::annotate_type(
        "lifetime", "__anon1")]]);
}
inline StructWithLifetimeAndDropGlue::~StructWithLifetimeAndDropGlue() {
  __crubit_internal::__crubit_thunk_drop(*this);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_make_ustatic_u42(
    ::lifetimes::StructWithLifetimeAndDropGlue* __ret_ptr);
}
inline ::lifetimes::StructWithLifetimeAndDropGlue
StructWithLifetimeAndDropGlue::make_static_42() {
  crubit::Slot<::lifetimes::StructWithLifetimeAndDropGlue>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_make_ustatic_u42(__return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void StructWithLifetimeAndDropGlue::__crubit_field_offset_assertions() {
  static_assert(0 ==
                offsetof(StructWithLifetimeAndDropGlue, field_with_drop_glue));
  static_assert(24 ==
                offsetof(StructWithLifetimeAndDropGlue, field_with_lifetime));
}
}  // namespace lifetimes
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_LIFETIMES_LIFETIMES_GOLDEN
