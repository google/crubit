// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:c_abi_compatible_type_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/c_abi_compatible_type.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(struct X) == 4);
static_assert(alignof(struct X) == 4);
static_assert(CRUBIT_OFFSET_OF(a, struct X) == 0);

extern "C" void __rust_thunk___ZN1XC1Ev(struct X* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN1XC1EOS_(struct X* __this,
                                          struct X* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct X* __rust_thunk___ZN1XaSERKS_(struct X* __this,
                                                const struct X* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct X* __rust_thunk___ZN1XaSEOS_(struct X* __this,
                                               struct X* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

extern "C" struct MyI8 __rust_thunk___Z3ffi4MyI81X(struct MyI8 a, struct X* b) {
  return ffi(std::move(a), std::move(*b));
}

extern "C" void __rust_thunk___Z1fiPvi(MyTypedefDecl a, void* b, int c) {
  f(a, b, c);
}

#pragma clang diagnostic pop
