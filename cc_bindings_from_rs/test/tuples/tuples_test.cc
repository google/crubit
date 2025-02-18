// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/tuples/tuples.h"

#include <type_traits>
#include <utility>

#include "gtest/gtest.h"

namespace crubit {
namespace {

TEST(TuplesTest, ReturnUnitIsNotTuple) {
  static_assert(
      std::is_same_v<decltype(tuples::return_unit_is_not_tuple()), void>);
  tuples::return_unit_is_not_tuple();
}

TEST(TuplesTest, ReturnCAbiCompatibleFiveInTuple) {
  std::tuple<uint32_t> v = tuples::return_c_abi_compatible_five_in_tuple();
  EXPECT_EQ(std::get<0>(v), 5);
}

TEST(TuplesTest, ParamCAbiCompatibleFiveInTuple) {
  tuples::param_c_abi_compatible_five_in_tuple(std::make_tuple(5));
}

TEST(TuplesTest, AdtInTuple) {
  std::tuple<tuples::AdtHoldingFiveAndSix> v = tuples::return_adt_in_tuple();
  tuples::param_adt_in_tuple(std::move(v));
}

TEST(TuplesTest, NontrivialDropInTuple) {
  // NOTE: `assert_nontrivial_drop_count` accesses a global variable
  // mutated by the `NontrivialDrop` destructor.
  //
  // Copying this test will result in assertion failures.
  tuples::assert_nontrivial_drop_count(0);
  {
    std::tuple<tuples::NontrivialDrop> v =
        tuples::return_new_nontrivial_drop_in_tuple();
    tuples::assert_nontrivial_drop_count(0);
  }
  tuples::assert_nontrivial_drop_count(1);
  tuples::param_nontrivial_drop_in_tuple(
      tuples::return_new_nontrivial_drop_in_tuple());
  tuples::assert_nontrivial_drop_count(2);
}

TEST(TuplesTest, NestedTupleParameters) {
  tuples::param_nested_tuples(std::make_tuple(std::make_tuple(1, 2), 3));
}

TEST(TuplesTest, NestedTupleReturns) {
  std::tuple<std::tuple<int32_t, int32_t>, int32_t> v =
      tuples::return_nested_tuples();
  EXPECT_EQ(std::get<0>(std::get<0>(v)), 1);
  EXPECT_EQ(std::get<1>(std::get<0>(v)), 2);
  EXPECT_EQ(std::get<1>(v), 3);
}

TEST(TuplesTest, TriplyNestedTupleParameters) {
  tuples::param_triply_nested_tuple(
      std::make_tuple(std::make_tuple(std::make_tuple(57))));
}

TEST(TuplesTest, TriplyNestedTupleReturns) {
  std::tuple<std::tuple<std::tuple<int32_t>>> v =
      tuples::return_triply_nested_tuple();
  EXPECT_EQ(std::get<0>(std::get<0>(std::get<0>(v))), 57);
}

TEST(TuplesTest, FfiAliasInTuple) {
  std::tuple<char> v = tuples::return_ffi_alias_in_tuple();
  EXPECT_EQ(std::get<0>(v), 5);
  tuples::param_ffi_alias_in_tuple(std::make_tuple<char>(5));
}

}  // namespace
}  // namespace crubit
