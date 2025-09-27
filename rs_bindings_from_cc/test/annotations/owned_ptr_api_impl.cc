// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/annotations:owned_ptr
// Features: non_unpin_ctor, std_unique_ptr, std_vector, supported

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/annotations/owned_ptr.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(struct Thing) == 4);
static_assert(alignof(struct Thing) == 4);
static_assert(CRUBIT_OFFSET_OF(value, struct Thing) == 0);

extern "C" void __rust_thunk___ZN5ThingC1Ei(struct Thing* __this,
                                            int32_t value) {
  crubit::construct_at(__this, value);
}

extern "C" void __rust_thunk___ZN5Thing5CloseEv(struct Thing* __this) {
  __this->Close();
}

static_assert((void (::Thing::*)())&Thing::Close);

#pragma clang diagnostic pop
