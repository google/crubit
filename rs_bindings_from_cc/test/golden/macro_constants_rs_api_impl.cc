// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:macro_constants_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/macro_constants.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"

extern "C" int __rust_thunk___Z20MacroConstantsSquarei(int x) {
  return MacroConstantsSquare(x);
}

static_assert((int (*)(int)) & ::MacroConstantsSquare);

static_assert(CRUBIT_SIZEOF(struct SizeofDeclaredStruct) == 4);
static_assert(alignof(struct SizeofDeclaredStruct) == 4);
static_assert(CRUBIT_OFFSET_OF(field, struct SizeofDeclaredStruct) == 0);

extern "C" void __rust_thunk___ZN20SizeofDeclaredStructC1Ev(
    struct SizeofDeclaredStruct* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct SizeofLaterDeclaredStruct) == 4);
static_assert(alignof(struct SizeofLaterDeclaredStruct) == 4);
static_assert(CRUBIT_OFFSET_OF(field, struct SizeofLaterDeclaredStruct) == 0);

extern "C" void __rust_thunk___ZN25SizeofLaterDeclaredStructC1Ev(
    struct SizeofLaterDeclaredStruct* __this) {
  crubit::construct_at(__this);
}

#pragma clang diagnostic pop
