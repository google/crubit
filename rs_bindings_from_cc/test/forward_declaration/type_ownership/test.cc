// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "gtest/gtest.h"
#include "rs_bindings_from_cc/test/forward_declaration/type_ownership/definition.h"
#include "rs_bindings_from_cc/test/forward_declaration/type_ownership/forward_declaration.h"

namespace {

TEST(TestCallFunc, BuildTest) {
  ForwardDeclaredStruct forward_declared_struct;
  FuncA(&forward_declared_struct);
  FuncB(B{});
  FuncB(forward_declared_struct);
}

}  // namespace
