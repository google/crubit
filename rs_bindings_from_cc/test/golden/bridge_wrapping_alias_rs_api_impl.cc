// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:bridge_wrapping_alias_cc

#include "support/bridge.h"
#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/bridge_wrapping_alias.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" void __rust_thunk___Z20bridge_alias_to_instv(
    unsigned char* __return_abi_buffer) {
  ::crubit::Encoder __return_encoder(
      BridgeAbi<::crubit::TransmuteAbi<::TemplateType<int>>>::kSize,
      __return_abi_buffer);
  BridgeAbi<::crubit::TransmuteAbi<::TemplateType<int>>>(
      ::crubit::TransmuteAbi<::TemplateType<int>>())
      .Encode(bridge_alias_to_inst(), __return_encoder);
}

static_assert((struct Bridge<TemplateType<int>> (*)()) &
              ::bridge_alias_to_inst);

static_assert(CRUBIT_SIZEOF(struct TemplateType<int>) == 4);
static_assert(alignof(struct TemplateType<int>) == 4);
static_assert(CRUBIT_OFFSET_OF(value, struct TemplateType<int>) == 0);

extern "C" void __rust_thunk__36c1161d__ZN12TemplateTypeIiEC1Ev(
    struct TemplateType<int>* __this) {
  crubit::construct_at(__this);
}

#pragma clang diagnostic pop
