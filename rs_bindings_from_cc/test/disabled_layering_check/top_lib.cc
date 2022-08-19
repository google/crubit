// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/disabled_layering_check/top_lib.h"

int GetValFromMyStruct(MyStruct my_struct) { return my_struct.GetVal(); }
