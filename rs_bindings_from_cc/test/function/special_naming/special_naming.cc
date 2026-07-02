// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/function/special_naming/special_naming.h"

int llvm_no_mangle_marker() { return 42; }

int asm_name_with_dollar_sign() { return 42; }

extern "C" SimpleStruct my_asm_conflict_func() { return SimpleStruct{42}; }
