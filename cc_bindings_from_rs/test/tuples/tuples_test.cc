// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/tuples/tuples.h"

#include <tuple>
#include <type_traits>
#include <utility>

#include "gtest/gtest.h"
#include "support/rs_std/rs_alloc.h"

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

// TODO(jeanpierreda): enable non-movable types inside compound data types like
// tuples?
//
// TEST(TuplesTest, NonCppMovableInTuple) {
//   std::tuple<tuples::NonCppMovable> v =
//       tuples::return_new_non_cpp_movable_in_tuple();
//   EXPECT_EQ(std::get<0>(v).value, 42);
//   std::tuple<std::tuple<tuples::NonCppMovable>> nested_v =
//       tuples::return_new_non_cpp_movable_in_nested_tuple();
//   EXPECT_EQ(std::get<0>(std::get<0>(nested_v)).value, 42);
// }

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

template <typename T>
concept HasBadTupleMethod = requires(T t) { t.tuple_not_by_value(); };

TEST(TuplesTest, TupleStruct) {
  // Can't directly test the tuple fields, because binary blob fields are
  // private, and ZST fields don't exist at all.

  EXPECT_FALSE(HasBadTupleMethod<tuples::TupleStruct>)
      << "Tuples cannot be bridged to std::tuple except when used by value";
}

TEST(TuplesTest, GetsTuple) {
  auto res = tuples::GetsTuple::new_(42);
  std::tuple<uint32_t, uint32_t> t = std::move(res.value);
  EXPECT_EQ(std::get<0>(t), 42);
  EXPECT_EQ(std::get<1>(t), 42);
}

TEST(TuplesTest, NestedTupleStruct) {
  auto res = tuples::NestedTupleStruct::new_(42);
  std::tuple<rs_std::Tuple<rs_std::Tuple<uint32_t, uint32_t>, uint32_t>,
             uint32_t>
      t1 = std::move(res.in_tuple1);
  EXPECT_EQ(std::get<1>(t1), 42);
  std::tuple<uint32_t,
             rs_std::Tuple<uint32_t, rs_std::Tuple<uint32_t, uint32_t>>>
      t2 = std::move(res.in_tuple2);
  EXPECT_EQ(std::get<0>(t2), 42);
}

TEST(TuplesTest, CopyNoDefaultTuple) {
  auto res = tuples::CopyNoDefaultTuple::new_(42);
  std::tuple<tuples::CopyNoDefault, uint8_t> t1 = std::move(res.in_tuple1);
  EXPECT_EQ(std::get<0>(t1).val, 42);
  std::tuple<uint8_t, tuples::CopyNoDefault> t2 = std::move(res.in_tuple2);
  EXPECT_EQ(std::get<1>(t2).val, 42);
}

TEST(TuplesTest, CloneNoDefaultTuple) {
  auto res = tuples::CloneNoDefaultTuple::new_(42);
  std::tuple<tuples::CloneNoDefault, uint8_t> t1 = std::move(res.in_tuple1);
  EXPECT_EQ(std::get<0>(t1).val, 42);
  std::tuple<uint8_t, tuples::CloneNoDefault> t2 = std::move(res.in_tuple2);
  EXPECT_EQ(std::get<1>(t2).val, 42);
}

TEST(TuplesTest, HasDefaultTuple) {
  auto res = tuples::HasDefaultTuple::new_("hello");
  std::tuple<tuples::HasDefault, uint8_t> t1 = std::move(res.in_tuple1);
  EXPECT_EQ(std::get<0>(t1).val().to_string_view(), "hello");
  std::tuple<uint8_t, tuples::HasDefault> t2 = std::move(res.in_tuple2);
  EXPECT_EQ(std::get<1>(t2).val().to_string_view(), "hello");
}

TEST(TuplesTest, ConstructAndPassTupleCopyNoDefault) {
  rs_std::Tuple<tuples::CopyNoDefault, std::uint8_t> res(
      std::make_tuple(tuples::CopyNoDefault::new_(42), std::uint8_t{10}));
  EXPECT_EQ(tuples::take_tuple_copy_no_default_1(res), 42);
}

TEST(TuplesTest, ConstructAndPassTupleCloneNoDefault) {
  rs_std::Tuple<std::uint8_t, tuples::CloneNoDefault> res(
      std::make_tuple(std::uint8_t{10}, tuples::CloneNoDefault::new_(42)));
  EXPECT_EQ(tuples::take_tuple_clone_no_default_2(res), 42);
}

TEST(TuplesTest, ConstructAndPassTupleHasDefault) {
  rs_std::Tuple<tuples::HasDefault, std::uint8_t> res(std::make_tuple(
      tuples::HasDefault::new_("halo strategy"), std::uint8_t{15}));
  EXPECT_EQ(tuples::take_tuple_has_default(&res), "halo strategy");
  std::tuple<tuples::HasDefault, std::uint8_t> t = std::move(res);
  EXPECT_EQ(std::get<0>(t).val().to_string_view(), "halo strategy");
}

TEST(TuplesTest, ConstructAndPassCppMovedTupleHasDefault) {
  rs_std::Tuple<tuples::HasDefault, std::uint8_t> res(std::make_tuple(
      tuples::HasDefault::new_("halo strategy"), std::uint8_t{15}));
  std::tuple<tuples::HasDefault, std::uint8_t> t = std::move(res);
  EXPECT_EQ(std::get<0>(t).val().to_string_view(), "halo strategy");
  // This should be the default empty string, after C++ move.
  EXPECT_EQ(tuples::take_tuple_has_default(&res), "");
}

TEST(TuplesTest, OptionInTupleRef) {
  rs_std::Tuple<rs_std::Option<std::int32_t>> res(
      std::make_tuple(rs_std::Option<std::int32_t>(42)));
  std::optional<int32_t> opt = tuples::return_option_in_tuple_ref(res);
  EXPECT_EQ(opt, 42);
}

TEST(TuplesTest, StructWithOptionTuple) {
  auto s = tuples::StructWithOptionTuple::new_(42);
  std::tuple<rs_std::Option<int32_t>,
             rs_std::Result<int32_t, rs::alloc::string::String>>
      t = std::move(s.opt_tuple);
  EXPECT_EQ(std::optional<int32_t>(std::move(std::get<0>(t))), 42);
  rs_std::Result<int32_t, rs::alloc::string::String> res =
      std::move(std::get<1>(t));
  EXPECT_TRUE(res.has_value());
  EXPECT_EQ(res.value(), 42);
}

}  // namespace
}  // namespace crubit
