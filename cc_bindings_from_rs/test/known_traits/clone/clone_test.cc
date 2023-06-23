// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <type_traits>
#include <utility>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/known_traits/clone/rs_clone_cc_api.h"

namespace crubit {
namespace {

template <typename TypeUnderTest>
void MainTestBody(const TypeUnderTest& s,
                  std::int32_t expected_copy_constructed_value,
                  std::int32_t expected_copy_assigned_value) {
  // The next line invokes the copy constructor.
  TypeUnderTest copy(s);
  static_assert(!std::is_trivially_copy_constructible_v<TypeUnderTest>);
  static_assert(std::is_copy_constructible_v<TypeUnderTest>);

  // Minimal verification that the copy constructor worked as expected.
  EXPECT_EQ(expected_copy_constructed_value,
            TypeUnderTest::extract_int(std::move(copy)));

  // The next line invokes the copy assignment operator.
  TypeUnderTest& assignment_result = (copy = s);
  static_assert(!std::is_trivially_copy_assignable_v<TypeUnderTest>);
  static_assert(std::is_copy_assignable_v<TypeUnderTest>);

  // Minimal verification that the copy assignment operator worked as expected.
  EXPECT_EQ(expected_copy_assigned_value,
            TypeUnderTest::extract_int(std::move(copy)));
  EXPECT_EQ(&assignment_result, &copy);

  // The next line invokes the copy assignment operator with the same lhs and
  // rhs operands.  This is an Undefined Behavior risk if such aliasing
  // references are passed to Rust - it needs to be short-circuited before
  // passing such args over the FFI boundary.  The short-circuiting is detected
  // by expecting the same, unchanged value.
  copy = copy;
  EXPECT_EQ(expected_copy_assigned_value,
            TypeUnderTest::extract_int(std::move(copy)));
}

TEST(CloneTest, ExplicitImplOfMandatoryMethod) {
  MainTestBody(
      rs_clone::explicit_impl_of_mandatory_method::SomeStruct::create_struct(
          42),
      10042,   // `Clone::clone` adds 10000
      10042);  // Trait-provided `Clone::clone_from` calls `Clone::clone`
}

TEST(CloneTest, ExplicitImplOfAllMethods) {
  MainTestBody(
      rs_clone::explicit_impl_of_all_methods::SomeStruct::create_struct(42),
      10042,   // `Clone::clone` adds 10000
      20042);  // `Clone::clone_from` adds 20000`
}

TEST(CloneTest, DerivedImpl) {
  MainTestBody(rs_clone::derived_impl::SomeStruct::create_struct(42),
               42,   // Derived `Clone::clone` copies the value
               42);  // Derived `Clone::clone_from` copies the value
}

TEST(CloneTest, DerivedImplWithNonDefaultField) {
  MainTestBody(
      rs_clone::derived_impl_with_non_default_field::SomeStruct::create_struct(
          42),
      42,   // Derived `Clone::clone` copies the value
      42);  // Derived `Clone::clone_from` copies the value
}

TEST(CloneTest, NoImpl) {
  namespace tests = rs_clone::no_impl;
  static_assert(!std::is_copy_constructible_v<tests::SomeStruct>);
  static_assert(!std::is_copy_assignable_v<tests::SomeStruct>);
}

}  // namespace
}  // namespace crubit
