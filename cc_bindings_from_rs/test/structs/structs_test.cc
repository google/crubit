// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <utility>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/structs/structs_cc_api.h"

namespace crubit {
namespace {

// Import all the APIs from `structs_cc_api.h`
using namespace structs;

TEST(StructsTest, ReprCPointReturnedOrTakenByValue) {
  ReprCPoint p = create_repr_c_point_via_free_function(123, 456);
  EXPECT_EQ(123, get_x_of_repr_c_point_via_free_function(std::move(p)));
}

}  // namespace
}  // namespace crubit
