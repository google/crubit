// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/assume_lifetimes/simple_string_view.h"

SV sv_ident(SV s) { return s; }
SV sv_make_raw() { return SV{}; }
SVA sva_lb(SVA s) { return s; }
