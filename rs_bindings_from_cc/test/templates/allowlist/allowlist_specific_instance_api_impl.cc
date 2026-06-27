// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/templates/allowlist:allowlist_specific_instance

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/templates/allowlist/allowlist_specific_instance.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" void __rust_thunk___Z14IntFloatCaller2TsIifE(
    struct Ts<int, float>* i) {
  IntFloatCaller(std::move(*i));
}

static_assert((void (*)(struct Ts<int, float> const)) & ::IntFloatCaller);

extern "C" void __rust_thunk___Z17ShortDoubleCaller2TsIsdE(
    struct Ts<short, double>* i) {
  ShortDoubleCaller(std::move(*i));
}

static_assert((void (*)(struct Ts<short, double> const)) & ::ShortDoubleCaller);

static_assert(sizeof(struct Ts<int, float>) == 1);
static_assert(alignof(struct Ts<int, float>) == 1);

extern "C" void
__rust_thunk___ZN2TsIifEC1Eif__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2ftemplates_2fallowlist_3aallowlist_5fspecific_5finstance(
    struct Ts<int, float>* __this, int t, float s) {
  crubit::construct_at(__this, t, s);
}

extern "C" void
__rust_thunk___ZN2TsIifE6MemberEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2ftemplates_2fallowlist_3aallowlist_5fspecific_5finstance(
    struct Ts<int, float>* __this) {
  __this->Member();
}

static_assert((void (::Ts<int, float>::*)()) & ::Ts<int, float>::Member);

static_assert(sizeof(struct Ts<short, double>) == 1);
static_assert(alignof(struct Ts<short, double>) == 1);

extern "C" void
__rust_thunk___ZN2TsIsdEC1Esd__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2ftemplates_2fallowlist_3aallowlist_5fspecific_5finstance(
    struct Ts<short, double>* __this, short t, double s) {
  crubit::construct_at(__this, t, s);
}

#pragma clang diagnostic pop
