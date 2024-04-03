// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/extern_c/has_bindings.h"

extern "C" void crubit_non_inline_function() {}
extern "C" void crubit_extern_c_directly_function() {}
