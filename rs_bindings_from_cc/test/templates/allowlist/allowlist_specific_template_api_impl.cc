// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/templates/allowlist:allowlist_specific_template

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/templates/allowlist/allowlist_specific_template.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" void __rust_thunk___Z14IntFloatCaller13AlwaysBoundTsIifE(
    struct AlwaysBoundTs<int, float>* i) {
  IntFloatCaller(std::move(*i));
}

static_assert((void (*)(struct AlwaysBoundTs<int, float> const)) &
              ::IntFloatCaller);

extern "C" void __rust_thunk___Z14NotBoundCaller10NotBoundTsIifE(
    struct NotBoundTs<int, float>* i) {
  NotBoundCaller(std::move(*i));
}

static_assert((void (*)(struct NotBoundTs<int, float> const)) &
              ::NotBoundCaller);

static_assert(sizeof(struct AlwaysBoundTs<int, float>) == 1);
static_assert(alignof(struct AlwaysBoundTs<int, float>) == 1);

extern "C" void
__rust_thunk___ZN13AlwaysBoundTsIifEC1Eif__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2ftemplates_2fallowlist_3aallowlist_5fspecific_5ftemplate(
    struct AlwaysBoundTs<int, float>* __this, int t, float s) {
  crubit::construct_at(__this, t, s);
}

extern "C" void
__rust_thunk___ZN13AlwaysBoundTsIifE6MemberEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2ftemplates_2fallowlist_3aallowlist_5fspecific_5ftemplate(
    struct AlwaysBoundTs<int, float>* __this) {
  __this->Member();
}

static_assert((void (::AlwaysBoundTs<int, float>::*)()) &
              ::AlwaysBoundTs<int, float>::Member);

static_assert(sizeof(struct NotBoundTs<int, float>) == 1);
static_assert(alignof(struct NotBoundTs<int, float>) == 1);

extern "C" void
__rust_thunk___ZN10NotBoundTsIifEC1Eif__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2ftemplates_2fallowlist_3aallowlist_5fspecific_5ftemplate(
    struct NotBoundTs<int, float>* __this, int t, float s) {
  crubit::construct_at(__this, t, s);
}

#pragma clang diagnostic pop
