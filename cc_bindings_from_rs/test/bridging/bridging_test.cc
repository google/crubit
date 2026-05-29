// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstdint>
#include <tuple>

#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/bridging/cc_generics.h"
#include "cc_bindings_from_rs/test/bridging/cc_type.h"
#include "cc_bindings_from_rs/test/bridging/layout_equivalent_generics.h"
#include "cc_bindings_from_rs/test/bridging/rust_pointer_types.h"
#include "cc_bindings_from_rs/test/bridging/rust_type.h"

namespace crubit {
namespace {

TEST(TypeBridging, StructToStructTest) {
  crubit::test::TheCppType cpp_type = ::rust_type::create_new(1);

  EXPECT_EQ(::rust_type::get_x(cpp_type), 1);
}

TEST(TypeBridging, StructToPointerTest) {
  crubit::test::TheCppType cpp_type = ::rust_type::create_new(2);

  crubit::test::TheCppType* ptr = rust_pointer_types::pass_through(&cpp_type);
  EXPECT_EQ(ptr, &cpp_type);

  int x = rust_pointer_types::get_x_from_view(&cpp_type);
  EXPECT_EQ(x, 2);
}

TEST(TypeBridging, StructConversionTest) {
  crubit::test::TheCppType cpp_type = ::rust_type::create_new(1);

  EXPECT_EQ(::rust_type::into_something_else(cpp_type).field, 1);
}

TEST(TypeBridging, NestedInTest) {
  std::tuple<crubit::test::TheCppType> cpp_type =
      ::rust_type::create_in_tuple(37);
  EXPECT_EQ(::rust_type::get_x(std::get<0>(cpp_type)), 37);
  EXPECT_EQ(::rust_type::get_x_from_tuple(cpp_type), 37);
}

TEST(TypeBridging, GenericStructToStructTest) {
  crubit::test::MyOptional<int> opt =
      ::layout_equivalent_generics::return_optional_by_value(42);
  EXPECT_TRUE(opt.has_value);
  EXPECT_EQ(opt.value, 42);

  EXPECT_EQ(::layout_equivalent_generics::accept_optional_by_value(opt), 42);
  EXPECT_EQ(::layout_equivalent_generics::accept_optional_by_reference(opt),
            42);
}

TEST(TypeBridging, GenericSpecializationTest) {
  crubit::test::MyStatus status = ::layout_equivalent_generics::return_status();
  EXPECT_TRUE(status.ok);

  EXPECT_TRUE(::layout_equivalent_generics::accept_status(status));
}

TEST(TypeBridging, GenericSpecializationWithAliasTest) {
  crubit::test::MyStatus status =
      ::layout_equivalent_generics::return_status_alias();
  EXPECT_TRUE(status.ok);
  EXPECT_TRUE(::layout_equivalent_generics::accept_status(status));
}

TEST(TypeBridging, GenericStatusOrNonUnitTest) {
  crubit::test::MyStatusOr<uint32_t> input{true, 42};
  crubit::test::MyStatusOr<uint64_t> result =
      ::layout_equivalent_generics::return_status_non_unit(input);
  EXPECT_TRUE(result.has_value);
  EXPECT_EQ(result.value, 42);
}

TEST(TypeBridging, GenericSpecializationWithUnitAliasTest) {
  crubit::test::MyStatus status =
      ::layout_equivalent_generics::return_status_or_unit_alias();
  EXPECT_TRUE(status.ok);
  EXPECT_TRUE(::layout_equivalent_generics::accept_status(status));
}

TEST(TypeBridging, GenericSpecializationWithMultipleParamsTest) {
  crubit::test::MyIntBoolPair pair =
      ::layout_equivalent_generics::create_int_bool_pair();
  EXPECT_EQ(pair.first, 42);
  EXPECT_TRUE(pair.second);
}

TEST(TypeBridging, GenericSpecializationWithPubUseTest) {
  crubit::test::MyStatus status =
      ::layout_equivalent_generics::create_status_with_secret_alias();
  EXPECT_TRUE(status.ok);
  EXPECT_TRUE(::layout_equivalent_generics::is_ok_secret(status));
}

}  // namespace
}  // namespace crubit
