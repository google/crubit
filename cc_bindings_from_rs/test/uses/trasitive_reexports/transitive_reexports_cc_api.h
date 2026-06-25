// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// transitive_reexports_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_USES_TRASITIVE_REEXPORTS_TRANSITIVE_REEXPORTS_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_USES_TRASITIVE_REEXPORTS_TRANSITIVE_REEXPORTS_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/slot.h"

#include <utility>

#include "cc_bindings_from_rs/test/uses/trasitive_reexports/direct.h"

namespace transitive_reexports {

::direct::Transitive direct_to_transitive(::direct::Direct const& direct);

::direct::TransitiveGlobA direct_to_transitive_glob_a(
    ::direct::Direct const& direct);

::direct::Transitive direct_to_transitive_private_type_alias(
    ::direct::Direct const& direct);

::direct::Transitive direct_to_transitive_use_alias(
    ::direct::Direct const& direct);

::direct::Transitive direct_to_transittive_type_alias(
    ::direct::Direct const& direct);

}  // namespace transitive_reexports

namespace transitive_reexports {

using DirectReexportOfTransitive CRUBIT_INTERNAL_RUST_TYPE(
    ":: direct :: TransitiveReexportAndDirectReexport") =
    ::direct::TransitiveReexportAndDirectReexport;
}

namespace transitive_reexports {

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_direct_uto_utransitive(
    ::direct::Direct const&, ::direct::Transitive* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::direct::Transitive direct_to_transitive(
    ::direct::Direct const& direct) {
  crubit::Slot<::direct::Transitive> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_direct_uto_utransitive(
      direct, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_direct_uto_utransitive_uglob_ua(
    ::direct::Direct const&, ::direct::TransitiveGlobA* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::direct::TransitiveGlobA direct_to_transitive_glob_a(
    ::direct::Direct const& direct) {
  crubit::Slot<::direct::TransitiveGlobA> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_direct_uto_utransitive_uglob_ua(
      direct, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_direct_uto_utransitive_uprivate_utype_ualias(
    ::direct::Direct const&, ::direct::Transitive* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::direct::Transitive direct_to_transitive_private_type_alias(
    ::direct::Direct const& direct) {
  crubit::Slot<::direct::Transitive> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_direct_uto_utransitive_uprivate_utype_ualias(
          direct, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_direct_uto_utransitive_uuse_ualias(
    ::direct::Direct const&, ::direct::Transitive* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::direct::Transitive direct_to_transitive_use_alias(
    ::direct::Direct const& direct) {
  crubit::Slot<::direct::Transitive> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_direct_uto_utransitive_uuse_ualias(
      direct, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_direct_uto_utransittive_utype_ualias(
    ::direct::Direct const&, ::direct::Transitive* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::direct::Transitive direct_to_transittive_type_alias(
    ::direct::Direct const& direct) {
  crubit::Slot<::direct::Transitive> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_direct_uto_utransittive_utype_ualias(
      direct, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

}  // namespace transitive_reexports

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_USES_TRASITIVE_REEXPORTS_TRANSITIVE_REEXPORTS_GOLDEN
