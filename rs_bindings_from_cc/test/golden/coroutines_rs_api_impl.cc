// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:coroutines_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

#include "util/c9/internal/pass_key.h"
#include "util/c9/internal/rust/co_vtable.h"
#include "util/c9/internal/rust/destroy_coroutine_frame_from_rust.h"
#include "util/c9/internal/rust/start_coroutine_from_rust.h"

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/coroutines.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" void __rust_thunk___ZN2c97SetBoolERb(
    c9::internal::rust::CoVTable* __return_co_vtable, bool* b) {
  __return_co_vtable->addr =
      c9::SetBool(*b).release_handle(c9::internal::PassKey()).address();
  __return_co_vtable->destroy_coroutine_frame_from_rust =
      &c9::internal::rust::DestroyCoroutineFrameFromRust<void>;
  __return_co_vtable->start_coroutine_from_rust =
      &c9::internal::rust::StartCoroutineFromRust;
  ;
}

static_assert((struct c9::Co<void> (*)(bool&))&c9::SetBool);

extern "C" void __rust_thunk___ZN2c99ReturnIntEv(
    c9::internal::rust::CoVTable* __return_co_vtable) {
  __return_co_vtable->addr =
      c9::ReturnInt().release_handle(c9::internal::PassKey()).address();
  __return_co_vtable->destroy_coroutine_frame_from_rust =
      &c9::internal::rust::DestroyCoroutineFrameFromRust<int>;
  __return_co_vtable->start_coroutine_from_rust =
      &c9::internal::rust::StartCoroutineFromRust<::crubit::TransmuteAbi<int>>;
  ;
}

static_assert((struct c9::Co<int> (*)())&c9::ReturnInt);

#pragma clang diagnostic pop
