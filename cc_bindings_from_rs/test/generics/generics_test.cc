// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/generics/generics.h"

#include <array>
#include <compare>
#include <cstdint>
#include <functional>
#include <sstream>
#include <type_traits>
#include <utility>

#include "gtest/gtest.h"

namespace {

TEST(GenericsTest, InherentMethodsAndImplSelection) {
  auto s = generics::instantiate_generic_struct_at_i32();
  EXPECT_EQ(s.value.__field0, 10);
  EXPECT_EQ(s.get_ref().__field0, 10);
  EXPECT_EQ(s.deref_value(), 10);
  EXPECT_EQ(s.borrow_with_explicit_lt().__field0, 10);
  EXPECT_FALSE(s.is_default());

  s.set(generics::PartialEqAndDefault{0});
  EXPECT_TRUE(s.is_default());

  auto inner = std::move(s).into_inner();
  EXPECT_EQ(inner.__field0, 0);

  auto s2 = generics::GenericStruct<generics::PartialEqAndDefault>::new_(
      generics::PartialEqAndDefault{42});
  EXPECT_EQ(s2.value.__field0, 42);

  // `NotPartialEqOrDefault` does not satisfy `PartialEq + Default`, so
  // `is_default` is not generated on `GenericStruct<NotPartialEqOrDefault>`.
  auto s_no_trait =
      generics::instantiate_generic_struct_at_not_partial_eq_or_default();
  EXPECT_EQ(s_no_trait.get_ref().__field0, 10);
  static_assert(
      []<typename T = generics::GenericStruct<generics::PartialEqAndDefault>> {
        return requires(const T& x) { x.is_default(); };
      }());
  static_assert(
      ![]<typename T =
              generics::GenericStruct<generics::NotPartialEqOrDefault>> {
        return requires(const T& x) { x.is_default(); };
      }());
  static_assert(
      ![]<typename T =
              generics::GenericStruct<generics::NotPartialEqOrDefault>> {
        return requires(const T& x) { x.deref_value(); };
      }());

  // Concrete `impl GenericStruct<i32>` only applies to
  // `GenericStruct<int32_t>`.
  auto s_i32 = generics::instantiate_generic_struct_at_primitive_i32();
  EXPECT_EQ(s_i32.add_one(), 11);
  EXPECT_EQ(generics::GenericStruct<int32_t>::type_size(), sizeof(int32_t));
  EXPECT_EQ(generics::GenericStruct<int64_t>::type_size(), sizeof(int64_t));
  EXPECT_EQ(generics::GenericStruct<int32_t>::VALUE_SIZE, sizeof(int32_t));
  EXPECT_EQ(generics::GenericStruct<int64_t>::VALUE_SIZE, sizeof(int64_t));
  EXPECT_EQ(generics::GenericStruct<int32_t>::ZERO.value, 0);
  EXPECT_EQ(generics::GenericStruct<int32_t>::concrete_id(), 32);
  EXPECT_EQ(generics::GenericStruct<int64_t>::concrete_id(), 64);
  static_assert(
      ![]<typename T = generics::GenericStruct<generics::PartialEqAndDefault>> {
        return requires(const T& x) { x.add_one(); };
      }());

  // Partially specialized `impl<T: Copy> MultiGenericStruct<T, i32>` applies to
  // `<bool, int32_t>` but not `<bool, double>`.
  auto multi_i32 = generics::instantiate_multi_generic_at_bool_i32();
  auto [first, len] = multi_i32.first_and_len();
  EXPECT_TRUE(first);
  EXPECT_EQ(len, 3u);
  static_assert(![]<typename T = generics::MultiGenericStruct<bool, double>> {
    return requires(const T& m) { m.first_and_len(); };
  }());
}

TEST(GenericsTest, GenericTraitImpls) {
  auto d1 = generics::instantiate_derived_generic_at_i32();
  auto d2 = generics::DerivedGeneric<int32_t>{100};
  EXPECT_EQ(d1.value, 42);
  EXPECT_EQ(d1 <=> d2, std::strong_ordering::less);
  EXPECT_NE(std::hash<generics::DerivedGeneric<int32_t>>{}(d1), 0u);
  struct MockAbslHasher {
    uint64_t combined = 0;
    static MockAbslHasher combine(MockAbslHasher h, uint64_t v) {
      h.combined = v;
      return h;
    }
  };
  EXPECT_NE(AbslHashValue(MockAbslHasher{}, d1).combined, 0u);

  // `DerivedGeneric<NotPartialEqOrDefault>` does not get `Ord` or `std::hash`.
  static_assert(
      ![]<typename T =
              generics::DerivedGeneric<generics::NotPartialEqOrDefault>> {
        return requires(const T& d) { d <=> d; };
      }());

  // `GenericWithDrop<int32_t>` implements `Default`, so it is default- and
  // move-constructible in C++.
  static_assert(
      std::is_default_constructible_v<generics::GenericWithDrop<int32_t>>);
  static_assert(
      std::is_move_constructible_v<generics::GenericWithDrop<int32_t>>);
  auto drop_movable = generics::instantiate_generic_with_drop_movable();
  EXPECT_EQ(drop_movable.value, 7);
  auto drop_moved = std::move(drop_movable);
  EXPECT_EQ(drop_moved.value, 7);

  // Generic `From<GenericWithDrop<T>> for Wrapper<T>` (`operator Wrapper<T>`)
  // and `Into<GenericWithDrop<T>> for Wrapper<T>` (converting constructor).
  auto wrapped = static_cast<generics::Wrapper<int32_t>>(std::move(drop_moved));
  EXPECT_EQ(wrapped.__field0, 7);
  generics::GenericWithDrop<int32_t> drop_from_wrapped(std::move(wrapped));
  EXPECT_EQ(drop_from_wrapped.value, 7);

  // `GenericWithDrop<NotPartialEqOrDefault>` does not implement `Default` or
  // `Clone`, so it is not C++-movable.
  static_assert(!std::is_default_constructible_v<
                generics::GenericWithDrop<generics::NotPartialEqOrDefault>>);
  static_assert(!std::is_move_constructible_v<
                generics::GenericWithDrop<generics::NotPartialEqOrDefault>>);
  auto drop_unmovable = generics::instantiate_generic_with_drop_unmovable();
  EXPECT_EQ(drop_unmovable.value.__field0, 7);

  // `std::fmt::Display` on `GenericStruct<T>`.
  auto s_i32 = generics::instantiate_generic_struct_at_primitive_i32();
  std::ostringstream os;
  os << s_i32;
  EXPECT_EQ(os.str(), "GenericStruct(10)");
}

TEST(GenericsTest, TransitiveSpecializationsAndAliases) {
  // `GenericStruct<uint32_t>` is only referenced transitively via
  // `OuterWithNestedGenericField`.
  generics::OuterWithNestedGenericField outer{
      .field = generics::GenericStruct<generics::GenericStruct<uint32_t>>::new_(
          generics::GenericStruct<uint32_t>::new_(55u)),
  };
  EXPECT_EQ(outer.field.get_ref().get_ref(), 55u);

  // `MultiGenericStruct<int32_t, int32_t>` is only discovered via
  // `GenericStruct<int32_t>::into_multi`.
  auto s_i32 = generics::instantiate_generic_struct_at_primitive_i32();
  auto multi = s_i32.into_multi();
  auto [first, len] = multi.first_and_len();
  EXPECT_EQ(first, 10);
  EXPECT_EQ(len, 1u);

  // Type aliases and `pub use` re-exports.
  auto aliased = generics::AliasedGenericI64::new_(99);
  auto aliased_out = generics::use_aliased_generic(aliased);
  EXPECT_EQ(aliased_out.value, 99);

  auto sub = generics::instantiate_sub_generic();
  static_assert(std::is_same_v<decltype(sub), generics::SubGeneric<int32_t>>);
  static_assert(
      std::is_same_v<decltype(sub), generics::submodule::SubGeneric<int32_t>>);
  EXPECT_EQ(sub.item, 5);

  // Generic `enum`, `union`, and lifetime + type generic struct.
  auto e = generics::instantiate_generic_enum();
  EXPECT_EQ(e.tag, generics::GenericEnum<int32_t>::Tag::Value);
  EXPECT_EQ(e.Value.__field0, 99);

  auto u = generics::instantiate_generic_union();
  EXPECT_EQ(u.val, 123u);

  int32_t x = 77;
  auto lt = generics::instantiate_lifetime_and_type_generic(&x);
  EXPECT_EQ(*lt.reference, 77);
}

TEST(GenericsTest, CornerCasesAndSafetyGuards) {
  // Recursive inherent methods are omitted without breaking non-recursive
  // methods on `GenericStruct<int32_t>`.
  static_assert(![]<typename T = generics::GenericStruct<int32_t>> {
    return requires(T s) { std::move(s).nest_in_self(); };
  }());
  static_assert(![]<typename T = generics::GenericStruct<int32_t>> {
    return requires(T s) { std::move(s).wrap_type_param(); };
  }());
  static_assert(![]<typename T = generics::GenericStruct<int32_t>> {
    return requires(T s) { std::move(s).wrap_self(); };
  }());
  static_assert(![]<typename T = generics::GenericStruct<int32_t>> {
    return requires(T s) { -std::move(s); };
  }());

  // Mutual recursion (`Ping<T> <-> Pong<T>`) terminates via depth limit.
  auto ping = generics::instantiate_ping();
  auto pong = ping.to_pong();
  EXPECT_EQ(pong.__field0.__field0, 1);

  // Layout-independent `PhantomData<T>` breaks self-referential cycle.
  auto sr = generics::instantiate_self_referential_phantom();
  EXPECT_EQ(sr.phantom.id, 1);

  // Fixed-size array specialization.
  auto arr_s = generics::instantiate_with_array();
  EXPECT_EQ(arr_s.value, (std::array<int32_t, 4>{1, 2, 3, 4}));
}

}  // namespace
