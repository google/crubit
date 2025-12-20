// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:template_inst_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/template_inst.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" void __rust_thunk___Z13GetMyTemplatev(
    struct MyTemplate<int>* __return) {
  new (__return) auto(GetMyTemplate());
}

static_assert((struct MyTemplate<int> (*)()) & ::GetMyTemplate);

static_assert(CRUBIT_SIZEOF(struct MyTemplate<int>) == 4);
static_assert(alignof(struct MyTemplate<int>) == 4);
static_assert(CRUBIT_OFFSET_OF(field, struct MyTemplate<int>) == 0);

#pragma clang diagnostic pop
