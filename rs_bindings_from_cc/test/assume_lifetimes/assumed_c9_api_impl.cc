// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/assume_lifetimes:assumed_c9
// Features: assume_lifetimes, assume_this_lifetimes, callables, check_default_initialized, experimental, fmt, leading_colons_for_cpp_type, supported, types, unsafe_view, wrapper

#include "support/bridge.h"
#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

#include "util/c9/internal/rust/co_crubit_abi.h"

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/assume_lifetimes/assumed_c9.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" void __rust_thunk___Z17CoReturnReferencev(
    unsigned char* __return_abi_buffer) {
  ::crubit::Encoder __return_encoder(
      ::c9::internal::rust::CoCrubitAbi<::crubit::TransmuteAbi<int*>>::kSize,
      __return_abi_buffer);
  ::c9::internal::rust::CoCrubitAbi<::crubit::TransmuteAbi<int*>>(
      &::c9::internal::rust::StartCoroutineFromRust<[]() {
        return ::crubit::TransmuteAbi<int*>();
      }>)
      .Encode(CoReturnReference(), __return_encoder);
}

static_assert((struct c9::Co<int&> (*)()) & ::CoReturnReference);

#pragma clang diagnostic pop
