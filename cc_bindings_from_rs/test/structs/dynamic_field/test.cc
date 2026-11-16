// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstdint>
#include <type_traits>

#include "gtest/gtest.h"
#include "cc_bindings_from_rs/test/structs/dynamic_field/dynamic_field.h"

namespace {

static_assert(!std::is_constructible_v<
              dynamic_field::StructWithDynamicField, crubit::UnsafeRelocateTag,
              dynamic_field::StructWithDynamicField&&>);
static_assert(!std::is_constructible_v<dynamic_field::DynamicNoDtorWrapper,
                                       crubit::UnsafeRelocateTag,
                                       dynamic_field::DynamicNoDtorWrapper&&>);
static_assert(!std::is_constructible_v<
              dynamic_field::TransparentDynamicField, crubit::UnsafeRelocateTag,
              dynamic_field::TransparentDynamicField&&>);
static_assert(!std::is_constructible_v<
              dynamic_field::EnumWithDynamicVariant, crubit::UnsafeRelocateTag,
              dynamic_field::EnumWithDynamicVariant&&>);
static_assert(!std::is_constructible_v<
              rs::Tuple<DynamicWithVirtualDtor, std::int32_t>,
              crubit::UnsafeRelocateTag,
              rs::Tuple<DynamicWithVirtualDtor, std::int32_t>&&>);
static_assert(
    !std::is_constructible_v<rs::Tuple<DynamicMovable, std::int32_t>,
                             crubit::UnsafeRelocateTag,
                             rs::Tuple<DynamicMovable, std::int32_t>&&>);
static_assert(std::is_constructible_v<dynamic_field::BoxedDynamic,
                                      crubit::UnsafeRelocateTag,
                                      dynamic_field::BoxedDynamic&&>);

TEST(DynamicFieldTest, EnumWithDynamicVariantConstructsInPlace) {
  auto int_enum = dynamic_field::EnumWithDynamicVariant::Int::Make(123);
  const auto* int_variant =
      rs::get_if<dynamic_field::EnumWithDynamicVariant::Int>(&int_enum);
  ASSERT_NE(int_variant, nullptr);
  EXPECT_EQ(int_variant->__field0, 123);

  auto dyn_enum =
      dynamic_field::EnumWithDynamicVariant::Dynamic::Make(DynamicMovable());
  const auto* dyn_variant =
      rs::get_if<dynamic_field::EnumWithDynamicVariant::Dynamic>(&dyn_enum);
  ASSERT_NE(dyn_variant, nullptr);
  EXPECT_EQ(dyn_variant->__field0.value(), 7);

  auto boxed = dynamic_field::make_boxed_int_variant(456);
  const auto* boxed_int =
      rs::get_if<dynamic_field::EnumWithDynamicVariant::Int>(&*boxed);
  ASSERT_NE(boxed_int, nullptr);
  EXPECT_EQ(boxed_int->__field0, 456);
}

}  // namespace
