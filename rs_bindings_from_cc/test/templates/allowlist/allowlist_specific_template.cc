// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/templates/allowlist/allowlist_specific_template.h"

void IntFloatCaller(AlwaysBoundTs<int, float> i) {}
void NotBoundCaller(NotBoundTs<int, float> i) {}
