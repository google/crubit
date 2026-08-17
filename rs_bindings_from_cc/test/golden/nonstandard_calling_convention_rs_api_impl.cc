// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:nonstandard_calling_convention_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/nonstandard_calling_convention.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" float __rust_thunk___Z19function_vectorcallff(float a, float b) {
  return ::function_vectorcall(a, b);
}

static_assert((float(__attribute__((vectorcall)) *)(float, float)) &
              ::function_vectorcall);

extern "C" int __rust_thunk___Z14function_win64i(int a) {
  return ::function_win64(a);
}

static_assert((int(__attribute__((ms_abi)) *)(int)) & ::function_win64);

static_assert(sizeof(class ::SomeClass) == 1);
static_assert(alignof(class ::SomeClass) == 1);

extern "C" void __rust_thunk___ZN9SomeClassC1Ev(class ::SomeClass* __this) {
  crubit::construct_at(__this);
}

extern "C" float __rust_thunk___ZN9SomeClass19function_vectorcallEff(
    class ::SomeClass* __this, float a, float b) {
  return __this->function_vectorcall(a, b);
}

static_assert((float (__attribute__((vectorcall)) ::SomeClass::*)(float,
                                                                  float)) &
              ::SomeClass::function_vectorcall);

extern "C" int __rust_thunk___ZN9SomeClass14function_win64Ei(
    class ::SomeClass* __this, int a) {
  return __this->function_win64(a);
}

static_assert((int (__attribute__((ms_abi)) ::SomeClass::*)(int)) &
              ::SomeClass::function_win64);

#pragma clang diagnostic pop
