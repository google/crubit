// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <utility>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/impls/impls_cc_api.h"

namespace crubit {
namespace {

TEST(ImplsTest, BasicStaticMethod) {
  std::int32_t sum = impls::basic_static_method::Math::add_i32(123, 456);
  EXPECT_EQ(sum, 123 + 456);
}

TEST(ImplsTest, InstanceMethods) {
  namespace test = impls::instance_methods;
  test::Number x = test::Number::create(123);
  EXPECT_EQ(123, x.get_i32());
  x.set_i32(456);
  EXPECT_EQ(456, x.get_i32());
  EXPECT_EQ(456, std::move(x).into_i32());
}

TEST(ImplsTest, StaticMethodTakingSameStructByValue) {
  namespace test = impls::static_method_taking_same_struct_by_value;
  test::Number x = test::Number::create(123);
  EXPECT_EQ(123, test::Number::static_into_i32(std::move(x)));
}

TEST(ImplsTest, NonContiguousMethodDeclsAndDefs) {
  namespace test = impls::non_contiguous_method_decls_and_defs;
  EXPECT_EQ(123 + 456, test::S1::add_structs(test::S1::create(123),
                                             test::S2::create(456)));
  EXPECT_EQ(123 + 456, test::S2::add_structs(test::S1::create(123),
                                             test::S2::create(456)));
}

}  // namespace
}  // namespace crubit
