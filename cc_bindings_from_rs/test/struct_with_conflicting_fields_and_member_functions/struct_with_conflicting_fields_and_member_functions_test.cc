// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/struct_with_conflicting_fields_and_member_functions/struct_with_conflicting_fields_and_member_functions.h"

#include <type_traits>

#include "gtest/gtest.h"

namespace {

TEST(StructWithConflictingFieldsAndMemberFunctionsTest, CompileOnly) {
  using namespace struct_with_conflicting_fields_and_member_functions;

  // Verify type properties
  static_assert(sizeof(X) == 12);
  static_assert(std::is_trivially_destructible_v<X>);

  // We can't construct X, but we can check that the members exist.
  X* x = nullptr;
  if (false) {
    // This code is not executed but must compile.
    int a_val = x->a();
    int b_val = x->b();
    int a_field = x->a_;
    (void)a_val;
    (void)b_val;
    (void)a_field;
  }
}

}  // namespace
