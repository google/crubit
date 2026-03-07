// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "cc_bindings_from_rs/test/traits/generic_traits.h"

#include "gtest/gtest.h"

namespace generic_traits {

template <typename Self, typename T, typename U>
  requires(rs_std::where_v<Self, TraitWithTwoGenerics<T, U>>)
T generic_do_something(Self const& self, T t, U u) {
  return TraitWithTwoGenerics<T, U>::template impl<Self>::bar(self, t, u);
}

TEST(GenericTraitTest, FullySpecifiedTraitImplBound) {
  StructGeneric s = StructGeneric::new_(10);
  EXPECT_TRUE(TraitWithGeneric<int32_t>::impl<StructGeneric>::kIsImplemented);
  EXPECT_EQ(TraitWithGeneric<int32_t>::impl<StructGeneric>::foo(s, 5), 15);
}

TEST(GenericTraitTest, FullySpecifiedMultipleGenericsTraitImplBound) {
  StructGeneric s = StructGeneric::new_(123);
  EXPECT_TRUE(
      (TraitWithTwoGenerics<int32_t,
                            int32_t>::impl<StructGeneric>::kIsImplemented));
  EXPECT_EQ((TraitWithTwoGenerics<int32_t, int32_t>::impl<StructGeneric>::bar(
                s, 5, 7)),
            135);
  EXPECT_EQ(generic_do_something(s, 5, 7), 135);
}

TEST(GenericTraitTest, AnotherStructDoesNotImplementTrait) {
  static_assert(
      !TraitWithTwoGenerics<int32_t,
                            int32_t>::impl<AnotherStruct>::kIsImplemented);
}

// TraitWithConstant doesn't receive bindings.

}  // namespace generic_traits
