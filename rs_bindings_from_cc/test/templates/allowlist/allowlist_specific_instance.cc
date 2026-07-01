// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/templates/allowlist/allowlist_specific_instance.h"

void IntFloatCaller(Ts<int, float> i) {}
void ShortDoubleCaller(Ts<short, double> i) {}
