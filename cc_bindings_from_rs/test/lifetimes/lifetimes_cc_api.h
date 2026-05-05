// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// lifetimes_golden
// Features: fmt, supported, types

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_LIFETIMES_LIFETIMES_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_LIFETIMES_LIFETIMES_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/slot.h"
#include "support/lifetime_annotations.h"

#include <cstddef>
#include <cstdint>
#include <cstring>
#include <type_traits>
#include <utility>

namespace lifetimes {

// Generated from:
// cc_bindings_from_rs/test/lifetimes/lifetimes.rs;l=9
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: lifetimes_golden :: StructWithLifetime") alignas(8)
    [[clang::trivial_abi]] StructWithLifetime final {
 public:
  // `lifetimes_golden::StructWithLifetime` doesn't implement the `Default`
  // trait
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
    ::std::memcpy(this, &value, sizeof(value));
  }

  // Error generating bindings for associated function
  // `lifetimes_golden::StructWithLifetime::<'a>::from_ref` defined at
  // cc_bindings_from_rs/test/lifetimes/lifetimes.rs;l=32:
  // Error formatting function return type
  // `lifetimes_golden::StructWithLifetime<'a>`: Types with non-'static
  // lifetimes are not supported yet (b/500486197)

  // Error generating bindings for associated function
  // `lifetimes_golden::StructWithLifetime::<'a>::into_ref` defined at
  // cc_bindings_from_rs/test/lifetimes/lifetimes.rs;l=36:
  // Error handling parameter #0 of type
  // `lifetimes_golden::StructWithLifetime<'a>`: Types with non-'static
  // lifetimes are not supported yet (b/500486197)

  // Error generating bindings for associated function
  // `lifetimes_golden::StructWithLifetime::<'a>::value` defined at
  // cc_bindings_from_rs/test/lifetimes/lifetimes.rs;l=40:
  // Error handling parameter #0 of type
  // `lifetimes_golden::StructWithLifetime<'a>`: Types with non-'static
  // lifetimes are not supported yet (b/500486197)

  // Error generating bindings for associated function
  // `lifetimes_golden::StructWithLifetime::<'a>::borrow_from_self` defined at
  // cc_bindings_from_rs/test/lifetimes/lifetimes.rs;l=44:
  // Error handling parameter #0 of type `&'__anon1
  // lifetimes_golden::StructWithLifetime<'a>`: Failed to format the referent of
  // the reference type `&'__anon1 lifetimes_golden::StructWithLifetime<'a>`:
  // Types with non-'static lifetimes are not supported yet (b/500486197)

  // Generated from:
  // cc_bindings_from_rs/test/lifetimes/lifetimes.rs;l=50
  static ::lifetimes::StructWithLifetime make_static_42();

  // Generated from:
  // cc_bindings_from_rs/test/lifetimes/lifetimes.rs;l=54
  static ::lifetimes::StructWithLifetime from_static_ref(
      ::std::int32_t const* $static crubit_nonnull field_with_lifetime);

  // Generated from:
  // cc_bindings_from_rs/test/lifetimes/lifetimes.rs;l=58
  static ::lifetimes::StructWithLifetime from_static_ref_where_bound(
      ::std::int32_t const* $a crubit_nonnull field_with_lifetime);

  // Generated from:
  // cc_bindings_from_rs/test/lifetimes/lifetimes.rs;l=65
  ::std::int32_t const& $static borrow_from_static_self() const& $static;

  union {
    // Generated from:
    // cc_bindings_from_rs/test/lifetimes/lifetimes.rs;l=10
    ::std::int32_t const* crubit_nonnull field_with_lifetime;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Error generating bindings for struct
// `lifetimes_golden::StructWithLifetimeAndDropGlue` defined at
// cc_bindings_from_rs/test/lifetimes/lifetimes.rs;l=72:
// Types with non-'static lifetimes that need drop are not supported yet
// (b/500486197)

// Generated from:
// cc_bindings_from_rs/test/lifetimes/lifetimes.rs;l=70
void function_with_trivial_unnamed_lifetime_param(
    ::std::int32_t const& __param_0);

static_assert(
    sizeof(StructWithLifetime) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(StructWithLifetime) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<StructWithLifetime>);
static_assert(
    ::std::is_trivially_move_constructible_v<::lifetimes::StructWithLifetime>);
static_assert(
    ::std::is_trivially_move_assignable_v<::lifetimes::StructWithLifetime>);
static_assert(
    ::std::is_trivially_copy_constructible_v<::lifetimes::StructWithLifetime>);
static_assert(
    ::std::is_trivially_copy_assignable_v<::lifetimes::StructWithLifetime>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_make_ustatic_u42(
    ::lifetimes::StructWithLifetime* __ret_ptr);
}
inline ::lifetimes::StructWithLifetime StructWithLifetime::make_static_42() {
  crubit::Slot<::lifetimes::StructWithLifetime> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_make_ustatic_u42(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_from_ustatic_uref(
    ::std::int32_t const* $static crubit_nonnull,
    ::lifetimes::StructWithLifetime* __ret_ptr);
}
inline ::lifetimes::StructWithLifetime StructWithLifetime::from_static_ref(
    ::std::int32_t const* $static crubit_nonnull field_with_lifetime) {
  crubit::Slot<::lifetimes::StructWithLifetime> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_from_ustatic_uref(field_with_lifetime,
                                                      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_from_ustatic_uref_uwhere_ubound(
    ::std::int32_t const* $a crubit_nonnull,
    ::lifetimes::StructWithLifetime* __ret_ptr);
}
inline ::lifetimes::StructWithLifetime
StructWithLifetime::from_static_ref_where_bound(
    ::std::int32_t const* $a crubit_nonnull field_with_lifetime) {
  crubit::Slot<::lifetimes::StructWithLifetime> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_from_ustatic_uref_uwhere_ubound(
      field_with_lifetime, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::int32_t const& $static
__crubit_thunk_borrow_ufrom_ustatic_uself(
    ::lifetimes::StructWithLifetime const&);
}
inline ::std::int32_t const& $static
StructWithLifetime::borrow_from_static_self() const& $static {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_borrow_ufrom_ustatic_uself(self);
}
inline void StructWithLifetime::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(StructWithLifetime, field_with_lifetime));
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_function_uwith_utrivial_uunnamed_ulifetime_uparam(
    ::std::int32_t const&);
}
inline void function_with_trivial_unnamed_lifetime_param(
    ::std::int32_t const& __param_0) {
  return __crubit_internal::
      __crubit_thunk_function_uwith_utrivial_uunnamed_ulifetime_uparam(
          __param_0);
}

}  // namespace lifetimes

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_LIFETIMES_LIFETIMES_GOLDEN
