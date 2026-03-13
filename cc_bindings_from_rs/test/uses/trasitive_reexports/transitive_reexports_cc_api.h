// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// transitive_reexports_golden
// Features: supported

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_USES_TRASITIVE_REEXPORTS_TRANSITIVE_REEXPORTS_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_USES_TRASITIVE_REEXPORTS_TRANSITIVE_REEXPORTS_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#include "support/internal/slot.h"

#include <utility>

#include "cc_bindings_from_rs/test/uses/trasitive_reexports/direct.h"

namespace transitive_reexports {

// Generated from:
// cc_bindings_from_rs/test/uses/trasitive_reexports/transitive_reexports.rs;l=6
::direct::Transitive direct_to_transitive(::direct::Direct const& direct);

namespace __crubit_internal {
extern "C" void __crubit_thunk_direct_uto_utransitive(
    ::direct::Direct const&, ::direct::Transitive* __ret_ptr);
}
inline ::direct::Transitive direct_to_transitive(
    ::direct::Direct const& direct) {
  crubit::Slot<::direct::Transitive> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_direct_uto_utransitive(
      direct, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

}  // namespace transitive_reexports

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_USES_TRASITIVE_REEXPORTS_TRANSITIVE_REEXPORTS_GOLDEN
