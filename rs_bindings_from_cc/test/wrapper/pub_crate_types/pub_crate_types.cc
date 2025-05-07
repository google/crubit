// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/wrapper/pub_crate_types/pub_crate_types.h"

struct ForwardDeclared {
  int value;
};

ForwardDeclared* ForwardDeclaredConstant = nullptr;

ForwardDeclared* CreateForwardDeclared() { return new ForwardDeclared{42}; }

int ConsumeCompoundDataType(CompoundDataType container) {
  int value = container.forward_declared->value;
  delete container.forward_declared;
  return value;
}
