// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <utility>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/structs/structs_cc_api.h"

namespace crubit {
namespace {

TEST(StructsTest, ReprCPointReturnedOrTakenByValue) {
  structs::repr_c::Point p = structs::repr_c::create(123, 456);
  EXPECT_EQ(123, structs::repr_c::get_x(std::move(p)));
}

TEST(StructsTest, DefaultReprPointReturnedOrTakenByValue) {
  structs::default_repr::Point p = structs::default_repr::create(123, 456);
  EXPECT_EQ(123, structs::default_repr::get_x(std::move(p)));
}

TEST(StructsTest, ReorderingDefs) {
  namespace m1 = structs::reordering_defs::m1;
  namespace m2 = structs::reordering_defs::m2;

  m1::S1 s1 = m2::create_s1();
  EXPECT_EQ(456, m2::get_int_from_s1(std::move(s1)));

  m2::S2 s2 = m1::create_s2();
  EXPECT_EQ(123, m1::get_int_from_s2(std::move(s2)));
}

TEST(StructsTest, FwdDecls) {
  namespace fwd_decls = structs::fwd_decls;
  fwd_decls::S1 s1 = fwd_decls::create_s1();
  EXPECT_EQ(456, fwd_decls::get_int_from_s1(&s1));
}

}  // namespace
}  // namespace crubit
