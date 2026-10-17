// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Concatenated into the generated `_rust_api_impl.cc` of `:cpp_srcs_lib`.
// It includes a header of a transitive dep (not a direct dep of
// `:cpp_srcs_lib`). This has always compiled, because this code was never
// subject to `layering_check`, and must keep compiling.
#include "rs_bindings_from_cc/test/modules_off/dep_lib.h"

int CppSrcsTransitiveGetValue(int value) { return MakeDepStruct(value).value; }
