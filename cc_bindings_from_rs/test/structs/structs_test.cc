// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/structs/structs.h"

#include <type_traits>
#include <utility>

#include "gtest/gtest.h"

namespace crubit {
namespace {

TEST(StructsTest, ReprCPoint) {
  structs::repr_c::Point p = structs::repr_c::create(123, 456);
  EXPECT_EQ(123, p.x);
  EXPECT_EQ(456, p.y);
  EXPECT_EQ(123, structs::repr_c::get_x(std::move(p)));
}

TEST(StructsTest, NonCppMovable) {
  structs::non_cpp_movable::Point p =
      structs::non_cpp_movable::create(123, 456);
  EXPECT_EQ(123, p.x);
  EXPECT_EQ(456, p.y);
  EXPECT_EQ(123, structs::non_cpp_movable::get_x(p));
}

TEST(StructsTest, NonCppMovableIntoUniquePtr) {
  std::unique_ptr<structs::non_cpp_movable::Point> p(
      new auto(structs::non_cpp_movable::create(123, 456)));
  EXPECT_EQ(123, p->x);
  EXPECT_EQ(456, p->y);
  EXPECT_EQ(123, structs::non_cpp_movable::get_x(*p));
}

TEST(StructsTest, DefaultRepr) {
  structs::default_repr::Point p = structs::default_repr::create(123, 456);
  EXPECT_EQ(123, p.x);
  EXPECT_EQ(456, p.y);
  EXPECT_EQ(123, structs::default_repr::get_x(std::move(p)));
}

TEST(StructsTest, ZstFields) {
  structs::zst_fields::ZstFields x = structs::zst_fields::create(42);
  EXPECT_EQ(42, x.value);
  EXPECT_EQ(structs::zst_fields::get_value(std::move(x)), 42);
}

TEST(StructsTest, StructInteger) {
  namespace test = structs::abi_classification;
  test::StructInteger x = test::StructInteger::create(123);
  test::StructInteger y = test::StructInteger::create(456);
  test::StructInteger product =
      test::StructInteger::multiply(std::move(x), std::move(y));
  EXPECT_EQ(123 * 456, test::StructInteger::inspect(std::move(product)));
}

TEST(StructsTest, StructFloat) {
  namespace test = structs::abi_classification;
  test::StructFloat x = test::StructFloat::create(456.0);
  test::StructFloat y = test::StructFloat::create(789.0);
  test::StructFloat product =
      test::StructFloat::multiply(std::move(x), std::move(y));
  EXPECT_EQ(456.0 * 789.0, test::StructFloat::inspect(std::move(product)));
}

TEST(StructsTest, StructMemory) {
  namespace test = structs::abi_classification;
  test::StructMemory x = test::StructMemory::create(321);
  test::StructMemory y = test::StructMemory::create(654);
  test::StructMemory product =
      test::StructMemory::multiply(std::move(x), std::move(y));
  EXPECT_EQ(321 * 654, test::StructMemory::inspect(std::move(product)));
}

TEST(StructsTest, DefinitionlessStructFloat) {
  namespace test = structs::struct_by_float_passing_with_no_cc_definition;
  test::StructFloat x = test::no_mangle_create(111.0);
  test::StructFloat y = test::no_mangle_create(222.0);
  test::StructFloat product =
      test::no_mangle_multiply(std::move(x), std::move(y));
  EXPECT_EQ(111.0 * 222.0, test::no_mangle_inspect(std::move(product)));
}

TEST(StructsTest, ThunklessStructFloat) {
  namespace test = structs::struct_by_float_passing_with_no_thunk;
  test::StructFloat x = test::thunkless_create(111.0);
  test::StructFloat y = test::thunkless_create(222.0);
  test::StructFloat product =
      test::thunkless_multiply(std::move(x), std::move(y));
  EXPECT_EQ(111.0 * 222.0, test::thunkless_inspect(std::move(product)));
}

extern "C" {

structs::struct_by_float_passing_with_no_thunk::StructFloat
struct_by_float_passing_with_no_thunk__thunkless_create(float);

structs::struct_by_float_passing_with_no_thunk::StructFloat
    struct_by_float_passing_with_no_thunk__thunkless_multiply(
        structs::struct_by_float_passing_with_no_thunk::StructFloat,
        structs::struct_by_float_passing_with_no_thunk::StructFloat);

float struct_by_float_passing_with_no_thunk__thunkless_inspect(
    structs::struct_by_float_passing_with_no_thunk::StructFloat);

}  // extern "C"

TEST(StructsTest, DirectFfiThunklessStructFloat) {
  namespace test = structs::struct_by_float_passing_with_no_thunk;
  test::StructFloat x =
      struct_by_float_passing_with_no_thunk__thunkless_create(111.0);
  test::StructFloat y =
      struct_by_float_passing_with_no_thunk__thunkless_create(222.0);
  test::StructFloat product =
      struct_by_float_passing_with_no_thunk__thunkless_multiply(std::move(x),
                                                                std::move(y));
  EXPECT_EQ(111.0 * 222.0,
            struct_by_float_passing_with_no_thunk__thunkless_inspect(
                std::move(product)));
}

// This is a regression test for b/286876315 - it verifies that the mutability
// qualifiers of nested pointers / pointees are correctly propagated.
TEST(StructsTest, NestedPtrTypeMutabilityQualifiers) {
  namespace test = structs::nested_ptr_type_mutability_qualifiers;
  test::SomeStruct s;
  ASSERT_EQ(nullptr, s.mut_const_ptr);
  ASSERT_EQ(nullptr, s.const_mut_ptr);

  // Verify that the `const` qualifiers got propagated correctly.
  static_assert(std::is_same_v<decltype(s.mut_const_ptr), float const**>);
  static_assert(std::is_same_v<decltype(s.const_mut_ptr), float* const*>);
}

// Structs can use unsupported types, and it doesn't affect whether they exist.
// The parts that are unsupported aren't generated, that's all.
TEST(StructsTest, UnsupportedTypes) {
  namespace test = structs::unsupported_types;
  test::SomeStruct s;
  (void)s;
}

}  // namespace
}  // namespace crubit
