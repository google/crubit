// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:template_instantiation_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/template_instantiation.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" void __rust_thunk___Z3RTSv(struct TS<int>* __return) {
  new (__return) auto(RTS());
}

static_assert((struct TS<int> (*)()) & ::RTS);

static_assert(sizeof(struct TS<int>) == 1);
static_assert(alignof(struct TS<int>) == 1);

extern "C" void
__rust_thunk___ZN2TSIiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplate_5finstantiation_5fcc(
    struct TS<int>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk___ZN2TSIiE1fEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplate_5finstantiation_5fcc(
    struct TS<int>* __this) {
  __this->f();
}

static_assert((void (::TS<int>::*)()) & ::TS<int>::f);

#pragma clang diagnostic pop
