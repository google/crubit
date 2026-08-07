// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// shared_ptr_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_SHARED_PTR_SHARED_PTR_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_SHARED_PTR_SHARED_PTR_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/internal/slot.h"

#include <cstdint>
#include <memory>

namespace shared_ptr {

// CRUBIT_ANNOTATE: must_bind=
::std::shared_ptr<::std::int32_t> clone_shared_ptr(
    ::std::shared_ptr<::std::int32_t> const& val);

// CRUBIT_ANNOTATE: must_bind=
void consume_shared_ptr(::std::shared_ptr<::std::int32_t> _val);

// CRUBIT_ANNOTATE: must_bind=
::std::shared_ptr<::std::int32_t> roundtrip_shared_ptr(
    ::std::shared_ptr<::std::int32_t> val);

namespace __crubit_internal {
extern "C" void __crubit_thunk_clone_ushared_uptr(
    ::std::shared_ptr<::std::int32_t> const&,
    ::std::shared_ptr<::std::int32_t>* __ret_ptr);
}
inline ::std::shared_ptr<::std::int32_t> clone_shared_ptr(
    ::std::shared_ptr<::std::int32_t> const& val) {
  crubit::Slot<::std::shared_ptr<::std::int32_t>> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_clone_ushared_uptr(val,
                                                       __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_consume_ushared_uptr(
    ::std::shared_ptr<::std::int32_t>*);
}
inline void consume_shared_ptr(::std::shared_ptr<::std::int32_t> _val) {
  crubit::Slot _val_slot((::std::move(_val)));
  return __crubit_internal::__crubit_thunk_consume_ushared_uptr(
      _val_slot.Get());
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_roundtrip_ushared_uptr(
    ::std::shared_ptr<::std::int32_t>*,
    ::std::shared_ptr<::std::int32_t>* __ret_ptr);
}
inline ::std::shared_ptr<::std::int32_t> roundtrip_shared_ptr(
    ::std::shared_ptr<::std::int32_t> val) {
  crubit::Slot val_slot((::std::move(val)));
  crubit::Slot<::std::shared_ptr<::std::int32_t>> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_roundtrip_ushared_uptr(
      val_slot.Get(), __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

}  // namespace shared_ptr

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_SHARED_PTR_SHARED_PTR_GOLDEN
