// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "gtest/gtest.h"
#include "rs_bindings_from_cc/test/templates/failed_template_instantiation_member_function/failed_template_instantiation_member_function.h"

namespace {

TEST(TestCallFunc, BuildTest) { Func(A<NoMethod>{}); }

}  // namespace
