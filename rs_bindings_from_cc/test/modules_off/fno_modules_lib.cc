// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/modules_off/fno_modules_lib.h"

#include "rs_bindings_from_cc/test/modules_off/dep_lib.h"

int FnoModulesGetValue(DepStruct s) { return s.value + 1; }
