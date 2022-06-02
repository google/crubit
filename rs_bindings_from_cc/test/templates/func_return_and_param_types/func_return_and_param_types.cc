// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/templates/func_return_and_param_types/func_return_and_param_types.h"

MyTemplate<int> CreateInstanceOfMyTemplate(int value) {
  return MyTemplate<int>::Create(value);
}

int DoubleInstanceOfMyTemplate(const MyTemplate<int>& my_template) {
  return my_template.value() * 2;
}
