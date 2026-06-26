// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/assume_lifetimes:simple_string_view

#include "support/internal/cxx20_backports.h"
#include "support/internal/fmt.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"
#include "support/rs_std/lossy_formatter_for_bindings.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/assume_lifetimes/simple_string_view.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(sizeof(struct ::SV) == 1);
static_assert(alignof(struct ::SV) == 1);

extern "C" void __rust_thunk___ZN2SVC1Ev(struct ::SV* __this) {
  crubit::construct_at(__this);
}

extern "C" bool
__crubit_fmt__2SV___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fassume_5flifetimes_3asimple_5fstring_5fview(
    const struct ::SV& value, ::lossy_formatter::LossyFormatter& formatter) {
  return ::crubit::Fmt(value, formatter);
}

extern "C" void __rust_thunk___Z8sv_ident2SV(struct ::SV* __return,
                                             struct ::SV* s) {
  new (__return) auto(::sv_ident(std::move(*s)));
}

static_assert((struct ::SV (*)(struct ::SV)) & ::sv_ident);

extern "C" void __rust_thunk___Z16sv_ident_unknown2SV(struct ::SV* __return,
                                                      struct ::SV* s) {
  new (__return) auto(::sv_ident_unknown(std::move(*s)));
}

static_assert((struct ::SV (*)(struct ::SV)) & ::sv_ident_unknown);

extern "C" void __rust_thunk___Z23sv_ident_unknown_elided2SV(
    struct ::SV* __return, struct ::SV* s) {
  new (__return) auto(::sv_ident_unknown_elided(std::move(*s)));
}

static_assert((struct ::SV (*)(struct ::SV)) & ::sv_ident_unknown_elided);

extern "C" void __rust_thunk___Z11sv_make_rawv(struct ::SV* __return) {
  new (__return) auto(::sv_make_raw());
}

static_assert((struct ::SV (*)()) & ::sv_make_raw);

extern "C" void __rust_thunk___Z6sva_lb2SV(::SVA* __return, ::SVA* s) {
  new (__return) auto(::sva_lb(std::move(*s)));
}

static_assert((::SVA (*)(::SVA)) & ::sva_lb);

#pragma clang diagnostic pop
