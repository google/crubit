// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/function/calling_conventions/calling_conventions.h"

// https://godbolt.org/z/4srMjcrab shows that `function_with_default_cc`
// produces assembly with different ABI expectations from
// `function_with_swiftcall_cc`.  This difference is implicitly depended on
// by the tests in this directory.

uintptr_t function_with_default_cc(UnusualSwiftcallStruct s) {
  constexpr uintptr_t kFuncDifferentiator = 0xffff0000;
  return s.x0 + s.x1 + s.x2 + kFuncDifferentiator;
}

__attribute__((swiftcall)) uintptr_t function_with_swiftcall_cc(
    UnusualSwiftcallStruct s) {
  constexpr uintptr_t kFuncDifferentiator = 0x0000ffff;
  return s.x0 + s.x1 + s.x2 + kFuncDifferentiator;
}
