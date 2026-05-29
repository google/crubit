// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// layout_equivalent_generics_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_LAYOUT_EQUIVALENT_GENERICS_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_LAYOUT_EQUIVALENT_GENERICS_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"

#include <cstdint>

#include "cc_bindings_from_rs/test/bridging/cc_generics.h"

namespace layout_equivalent_generics {

// Error generating bindings for struct
// `layout_equivalent_generics_golden::MyOptional` defined at
// cc_bindings_from_rs/test/bridging/layout_equivalent_generics.rs;l=12:
// Type bindings for layout_equivalent_generics_golden::MyOptional suppressed
// due to being mapped to an existing C++ type (crubit::test::MyOptional<{T}>)

// Error generating bindings for struct
// `layout_equivalent_generics_golden::MyPair` defined at
// cc_bindings_from_rs/test/bridging/layout_equivalent_generics.rs;l=105:
// Type bindings for layout_equivalent_generics_golden::MyPair suppressed due to
// being mapped to an existing C++ type (crubit::test::MyPair<{T1}, {T2}>)

using MyStatusAlias CRUBIT_INTERNAL_RUST_TYPE(
    ":: layout_equivalent_generics_golden :: MyStatusAlias") =
    crubit::test::MyStatus;

// Error generating bindings for struct
// `layout_equivalent_generics_golden::MyStatusOr` defined at
// cc_bindings_from_rs/test/bridging/layout_equivalent_generics.rs;l=51:
// Type bindings for layout_equivalent_generics_golden::MyStatusOr suppressed
// due to being mapped to an existing C++ type (crubit::test::MyStatusOr<{T}>)

// Error generating bindings for type alias
// `layout_equivalent_generics_golden::UnitAlias` defined at
// cc_bindings_from_rs/test/bridging/layout_equivalent_generics.rs;l=79:
// Tuple type `()` is not supported in this context

// CRUBIT_ANNOTATE: must_bind=
::std::int32_t accept_optional_by_reference(
    crubit::test::MyOptional<::std::int32_t> const& opt);

// CRUBIT_ANNOTATE: must_bind=
::std::int32_t accept_optional_by_value(
    crubit::test::MyOptional<::std::int32_t> opt);

// CRUBIT_ANNOTATE: must_bind=
bool accept_status(crubit::test::MyStatus status);

// CRUBIT_ANNOTATE: must_bind=
bool accept_status_or_unit(crubit::test::MyStatus status);

// CRUBIT_ANNOTATE: must_bind=
crubit::test::MyPair<bool, bool> create_bool_bool_pair();

// CRUBIT_ANNOTATE: must_bind=
crubit::test::MyIntBoolPair create_int_bool_pair();

// CRUBIT_ANNOTATE: must_bind=
crubit::test::MyStatus create_status_with_private_secret();

// CRUBIT_ANNOTATE: must_bind=
crubit::test::MyStatus create_status_with_secret();

// CRUBIT_ANNOTATE: must_bind=
crubit::test::MyStatus create_status_with_secret_alias();

// CRUBIT_ANNOTATE: must_bind=
bool is_ok_secret(crubit::test::MyStatus status);

// CRUBIT_ANNOTATE: must_bind=
crubit::test::MyOptional<::std::int32_t> return_optional_by_value(
    ::std::int32_t x);

// CRUBIT_ANNOTATE: must_bind=
crubit::test::MyStatus return_status();

// CRUBIT_ANNOTATE: must_bind=
crubit::test::MyStatus return_status_alias();

// CRUBIT_ANNOTATE: must_bind=
crubit::test::MyStatusOr<::std::uint64_t> return_status_non_unit(
    crubit::test::MyStatusOr<::std::uint32_t> status);

// CRUBIT_ANNOTATE: must_bind=
crubit::test::MyStatus return_status_or_unit();

// CRUBIT_ANNOTATE: must_bind=
crubit::test::MyStatus return_status_or_unit_alias();

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_accept_uoptional_uby_ureference(
    crubit::test::MyOptional<::std::int32_t> const&);
}
inline ::std::int32_t accept_optional_by_reference(
    crubit::test::MyOptional<::std::int32_t> const& opt) {
  return __crubit_internal::__crubit_thunk_accept_uoptional_uby_ureference(opt);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_accept_uoptional_uby_uvalue(
    crubit::test::MyOptional<::std::int32_t>*);
}
inline ::std::int32_t accept_optional_by_value(
    crubit::test::MyOptional<::std::int32_t> opt) {
  return __crubit_internal::__crubit_thunk_accept_uoptional_uby_uvalue(&opt);
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_accept_ustatus(crubit::test::MyStatus*);
}
inline bool accept_status(crubit::test::MyStatus status) {
  return __crubit_internal::__crubit_thunk_accept_ustatus(&status);
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_accept_ustatus_uor_uunit(
    crubit::test::MyStatus*);
}
inline bool accept_status_or_unit(crubit::test::MyStatus status) {
  return __crubit_internal::__crubit_thunk_accept_ustatus_uor_uunit(&status);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_create_ubool_ubool_upair(
    crubit::test::MyPair<bool, bool>* __ret_ptr);
}
inline crubit::test::MyPair<bool, bool> create_bool_bool_pair() {
  union __return_value_crubit_return_union {
    constexpr __return_value_crubit_return_union() {}
    ~__return_value_crubit_return_union() { ::std::destroy_at(&this->val); }
    crubit::test::MyPair<bool, bool> val;
  } __return_value_ret_val_holder;
  auto* __return_value_storage = &__return_value_ret_val_holder.val;
  __crubit_internal::__crubit_thunk_create_ubool_ubool_upair(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder.val);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_create_uint_ubool_upair(
    crubit::test::MyIntBoolPair* __ret_ptr);
}
inline crubit::test::MyIntBoolPair create_int_bool_pair() {
  union __return_value_crubit_return_union {
    constexpr __return_value_crubit_return_union() {}
    ~__return_value_crubit_return_union() { ::std::destroy_at(&this->val); }
    crubit::test::MyIntBoolPair val;
  } __return_value_ret_val_holder;
  auto* __return_value_storage = &__return_value_ret_val_holder.val;
  __crubit_internal::__crubit_thunk_create_uint_ubool_upair(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder.val);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_create_ustatus_uwith_uprivate_usecret(
    crubit::test::MyStatus* __ret_ptr);
}
inline crubit::test::MyStatus create_status_with_private_secret() {
  union __return_value_crubit_return_union {
    constexpr __return_value_crubit_return_union() {}
    ~__return_value_crubit_return_union() { ::std::destroy_at(&this->val); }
    crubit::test::MyStatus val;
  } __return_value_ret_val_holder;
  auto* __return_value_storage = &__return_value_ret_val_holder.val;
  __crubit_internal::__crubit_thunk_create_ustatus_uwith_uprivate_usecret(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder.val);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_create_ustatus_uwith_usecret(
    crubit::test::MyStatus* __ret_ptr);
}
inline crubit::test::MyStatus create_status_with_secret() {
  union __return_value_crubit_return_union {
    constexpr __return_value_crubit_return_union() {}
    ~__return_value_crubit_return_union() { ::std::destroy_at(&this->val); }
    crubit::test::MyStatus val;
  } __return_value_ret_val_holder;
  auto* __return_value_storage = &__return_value_ret_val_holder.val;
  __crubit_internal::__crubit_thunk_create_ustatus_uwith_usecret(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder.val);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_create_ustatus_uwith_usecret_ualias(
    crubit::test::MyStatus* __ret_ptr);
}
inline crubit::test::MyStatus create_status_with_secret_alias() {
  union __return_value_crubit_return_union {
    constexpr __return_value_crubit_return_union() {}
    ~__return_value_crubit_return_union() { ::std::destroy_at(&this->val); }
    crubit::test::MyStatus val;
  } __return_value_ret_val_holder;
  auto* __return_value_storage = &__return_value_ret_val_holder.val;
  __crubit_internal::__crubit_thunk_create_ustatus_uwith_usecret_ualias(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder.val);
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_uok_usecret(crubit::test::MyStatus*);
}
inline bool is_ok_secret(crubit::test::MyStatus status) {
  return __crubit_internal::__crubit_thunk_is_uok_usecret(&status);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_uoptional_uby_uvalue(
    ::std::int32_t, crubit::test::MyOptional<::std::int32_t>* __ret_ptr);
}
inline crubit::test::MyOptional<::std::int32_t> return_optional_by_value(
    ::std::int32_t x) {
  union __return_value_crubit_return_union {
    constexpr __return_value_crubit_return_union() {}
    ~__return_value_crubit_return_union() { ::std::destroy_at(&this->val); }
    crubit::test::MyOptional<::std::int32_t> val;
  } __return_value_ret_val_holder;
  auto* __return_value_storage = &__return_value_ret_val_holder.val;
  __crubit_internal::__crubit_thunk_return_uoptional_uby_uvalue(
      x, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder.val);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_ustatus(
    crubit::test::MyStatus* __ret_ptr);
}
inline crubit::test::MyStatus return_status() {
  union __return_value_crubit_return_union {
    constexpr __return_value_crubit_return_union() {}
    ~__return_value_crubit_return_union() { ::std::destroy_at(&this->val); }
    crubit::test::MyStatus val;
  } __return_value_ret_val_holder;
  auto* __return_value_storage = &__return_value_ret_val_holder.val;
  __crubit_internal::__crubit_thunk_return_ustatus(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder.val);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_ustatus_ualias(
    crubit::test::MyStatus* __ret_ptr);
}
inline crubit::test::MyStatus return_status_alias() {
  union __return_value_crubit_return_union {
    constexpr __return_value_crubit_return_union() {}
    ~__return_value_crubit_return_union() { ::std::destroy_at(&this->val); }
    crubit::test::MyStatus val;
  } __return_value_ret_val_holder;
  auto* __return_value_storage = &__return_value_ret_val_holder.val;
  __crubit_internal::__crubit_thunk_return_ustatus_ualias(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder.val);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_ustatus_unon_uunit(
    crubit::test::MyStatusOr<::std::uint32_t>*,
    crubit::test::MyStatusOr<::std::uint64_t>* __ret_ptr);
}
inline crubit::test::MyStatusOr<::std::uint64_t> return_status_non_unit(
    crubit::test::MyStatusOr<::std::uint32_t> status) {
  union __return_value_crubit_return_union {
    constexpr __return_value_crubit_return_union() {}
    ~__return_value_crubit_return_union() { ::std::destroy_at(&this->val); }
    crubit::test::MyStatusOr<::std::uint64_t> val;
  } __return_value_ret_val_holder;
  auto* __return_value_storage = &__return_value_ret_val_holder.val;
  __crubit_internal::__crubit_thunk_return_ustatus_unon_uunit(
      &status, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder.val);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_ustatus_uor_uunit(
    crubit::test::MyStatus* __ret_ptr);
}
inline crubit::test::MyStatus return_status_or_unit() {
  union __return_value_crubit_return_union {
    constexpr __return_value_crubit_return_union() {}
    ~__return_value_crubit_return_union() { ::std::destroy_at(&this->val); }
    crubit::test::MyStatus val;
  } __return_value_ret_val_holder;
  auto* __return_value_storage = &__return_value_ret_val_holder.val;
  __crubit_internal::__crubit_thunk_return_ustatus_uor_uunit(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder.val);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_ustatus_uor_uunit_ualias(
    crubit::test::MyStatus* __ret_ptr);
}
inline crubit::test::MyStatus return_status_or_unit_alias() {
  union __return_value_crubit_return_union {
    constexpr __return_value_crubit_return_union() {}
    ~__return_value_crubit_return_union() { ::std::destroy_at(&this->val); }
    crubit::test::MyStatus val;
  } __return_value_ret_val_holder;
  auto* __return_value_storage = &__return_value_ret_val_holder.val;
  __crubit_internal::__crubit_thunk_return_ustatus_uor_uunit_ualias(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder.val);
}

}  // namespace layout_equivalent_generics

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_LAYOUT_EQUIVALENT_GENERICS_GOLDEN
