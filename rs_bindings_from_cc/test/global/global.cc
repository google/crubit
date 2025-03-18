// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/global/global.h"

int extern_int{1};
const int kExternConstInt{2};

namespace foo {
int extern_int_namespaced{3};
extern "C" int extern_c_int_namespaced{4};
}  // namespace foo

int GetIntVal() { return extern_int; }

int GetNamespacedIntVal() { return foo::extern_int_namespaced; }

int GetCNamespacedIntVal() { return foo::extern_c_int_namespaced; }

int GetInlineIntVal() { return inline_int; }
