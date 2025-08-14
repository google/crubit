// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/structs/unit_structs/unit_structs.h"

#include <type_traits>
#include <utility>

#include "gtest/gtest.h"

namespace crubit {
namespace {

TEST(UnitStructsTest, HasAnyBindings) {
  // TODO: 258259459 - add support for ZSTs and unit structs, then add a call
  // to the default constructor here.
}

}  // namespace
}  // namespace crubit
