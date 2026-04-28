// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:displayables_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/fmt.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"
#include "support/rs_std/lossy_formatter_for_bindings.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/displayables.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(sizeof(struct DisplayableStruct) == 1);
static_assert(alignof(struct DisplayableStruct) == 1);

extern "C" void __rust_thunk___ZN17DisplayableStructC1Ev(
    struct DisplayableStruct* __this) {
  crubit::construct_at(__this);
}

extern "C" bool
__crubit_fmt__17DisplayableStruct___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3adisplayables_5fcc(
    const struct DisplayableStruct& value,
    ::lossy_formatter::LossyFormatter& formatter) {
  return ::crubit::Fmt(value, formatter);
}

extern "C" bool
__crubit_fmt__DisplayableEnum___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3adisplayables_5fcc(
    const DisplayableEnum& value,
    ::lossy_formatter::LossyFormatter& formatter) {
  return ::crubit::Fmt(value, formatter);
}

#pragma clang diagnostic pop
