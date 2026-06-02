// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ASSUME_LIFETIMES_SIMPLE_STRING_VIEW_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ASSUME_LIFETIMES_SIMPLE_STRING_VIEW_H_

#include "rs_bindings_from_cc/test/assume_lifetimes/test_annotations.h"

struct LIFETIME_PARAMS("a") SV {};

// TODO(zarko): We should mark 'unknowns (or equivalent) as unsafe.
SV sv_ident(SV s);
SV $unknown sv_ident_unknown(SV $unknown s);
SV sv_ident_unknown_elided(SV $unknown s);
SV sv_make_raw();

using SVA = SV;
SVA sva_lb(SVA s [[clang::lifetimebound]]);

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ASSUME_LIFETIMES_SIMPLE_STRING_VIEW_H_
