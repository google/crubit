// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:composable_bridging_template_type_cc

#include "support/bridge.h"
#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/composable_bridging_template_type.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" void __rust_thunk___Z12ReturnsValuev(
    unsigned char* __return_abi_buffer) {
  ::crubit::Encoder __return_encoder(
      ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::Value<int>>>::kSize,
      __return_abi_buffer);
  ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::Value<int>>>(
      ::crubit::TransmuteAbi<::Value<int>>())
      .Encode(ReturnsValue(), __return_encoder);
}

static_assert((struct MyOption<Value<int>> (*)()) & ::ReturnsValue);

static_assert(CRUBIT_SIZEOF(struct Value<int>) == 4);
static_assert(alignof(struct Value<int>) == 4);
static_assert(CRUBIT_OFFSET_OF(value, struct Value<int>) == 0);

#pragma clang diagnostic pop
